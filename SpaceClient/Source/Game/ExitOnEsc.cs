using FlaxEngine;
using SpacetimeDB;
using SpacetimeDB.Types;
using System.Collections.Concurrent;

public class ExitOnEsc : Script
{
    public override void OnUpdate()
    {
        if (Input.GetKeyUp(KeyboardKeys.Escape))
            Engine.RequestExit();
    }
}
