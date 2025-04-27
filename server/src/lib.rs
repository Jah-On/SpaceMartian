use std::time::Duration;

use parry2d::math::{Isometry, Translation};
use spacetimedb::{
    reducer, table, Identity, ReducerContext, ScheduleAt, SpacetimeType, Table, Timestamp,
};
#[derive(SpacetimeType, Clone, Debug)]
pub struct DbPos {
    pub rotation: f32,
    pub translation: DbVec2,
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct DbVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<Translation<f32>> for DbVec2 {
    fn from(value: Translation<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<DbVec2> for Translation<f32> {
    fn from(value: DbVec2) -> Self {
        Translation::new(value.x, value.y)
    }
}
impl From<Isometry<f32>> for DbPos {
    fn from(value: Isometry<f32>) -> Self {
        Self {
            rotation: value.rotation.angle(),
            translation: value.translation.into(),
        }
    }
}
impl From<DbPos> for Isometry<f32> {
    fn from(value: DbPos) -> Self {
        Self::from_parts(
            value.translation.into(),
            parry2d::na::Unit::<parry2d::na::Complex<f32>>::from_angle(value.rotation),
        )
    }
}

#[table(name = ball, public)]
pub struct Ball {
    #[primary_key]
    id: u32,
    radius: f32,
    pos: DbPos,
    velocity: DbVec2,
}

#[table(name = bumper, public)]
pub struct Bumper {
    #[primary_key]
    id: u32,
    radius: f32,
    pos: DbPos,
}

#[table(name = user, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    // Called when the module is initially published
    ctx.db.ball().try_insert(Ball {
        id: 0,
        radius: 0.015,
        pos: Isometry::from_parts(Translation::new(0., 3.), Default::default()).into(),
        velocity: DbVec2 { x: 0., y: 0. },
    })?;

    ctx.db
        .update_physics_timer()
        .try_insert(UpdatePhysicsTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Interval(Duration::from_millis(16).into()),
        })?;

    Ok(())
}

#[reducer(client_connected)]
// Called when a client connects to a SpacetimeDB database server
pub fn client_connected(ctx: &ReducerContext) {
    ctx.db.user().insert(Player {
        name: None,
        identity: ctx.sender,
    });
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB database server
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().delete(user);
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        );
    }
}
#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(Player {
            name: Some(name),
            ..user
        });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

/// Takes a name and checks if it's acceptable as a user's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[spacetimedb::table(name = update_physics_timer, scheduled(update_physics))]
pub struct UpdatePhysicsTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
}

#[reducer]
pub fn update_physics(ctx: &ReducerContext, _timer: UpdatePhysicsTimer) -> Result<(), String> {
    if ctx.db.user().count() == 0 {
        // Are there no logged in players? Skip physics
        return Ok(());
    }

    let mut balls: Vec<_> = ctx.db.ball().iter().collect();
    let bumpers: Vec<_> = ctx.db.bumper().iter().collect();

    for b in balls.iter_mut() {
        b.pos.translation.x += b.velocity.x;
        b.pos.translation.y += b.velocity.y;

        b.velocity.x *= 0.99; // friction (is that remotely correct)
        b.velocity.y *= 0.99; // friction

        b.velocity.y += 0.1; // gravity
    }

    // TODO: ball out of bounds => reset ball

    for b in balls.into_iter() {
        ctx.db.ball().insert(b);
    }

    Ok(())
}
