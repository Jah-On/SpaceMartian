// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    [SpacetimeDB.Type]
    [DataContract]
    public sealed partial class Ball
    {
        [DataMember(Name = "id")]
        public uint Id;
        [DataMember(Name = "radius")]
        public float Radius;
        [DataMember(Name = "pos")]
        public DbPos Pos;
        [DataMember(Name = "velocity")]
        public DbVec2 Velocity;

        public Ball(
            uint Id,
            float Radius,
            DbPos Pos,
            DbVec2 Velocity
        )
        {
            this.Id = Id;
            this.Radius = Radius;
            this.Pos = Pos;
            this.Velocity = Velocity;
        }

        public Ball()
        {
            this.Pos = new();
            this.Velocity = new();
        }
    }
}
