// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using SpacetimeDB.BSATN;
using SpacetimeDB.ClientApi;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    public sealed partial class RemoteTables
    {
        public sealed class UpdatePhysicsTimerHandle : RemoteTableHandle<EventContext, UpdatePhysicsTimer>
        {
            protected override string RemoteTableName => "update_physics_timer";

            public sealed class ScheduledIdUniqueIndex : UniqueIndexBase<ulong>
            {
                protected override ulong GetKey(UpdatePhysicsTimer row) => row.ScheduledId;

                public ScheduledIdUniqueIndex(UpdatePhysicsTimerHandle table) : base(table) { }
            }

            public readonly ScheduledIdUniqueIndex ScheduledId;

            internal UpdatePhysicsTimerHandle(DbConnection conn) : base(conn)
            {
                ScheduledId = new(this);
            }

            protected override object GetPrimaryKey(UpdatePhysicsTimer row) => row.ScheduledId;
        }

        public readonly UpdatePhysicsTimerHandle UpdatePhysicsTimer;
    }
}
