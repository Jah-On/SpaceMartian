using System;
using System.Collections.Generic;
using FlaxEngine;
using SpacetimeDB;
using SpacetimeDB.Types;
using System.Collections.Concurrent;
using System.Threading;

namespace Game;

/// <summary>
/// ServerConnector Script.
/// </summary>
public class ServerConnector : Script
{
    // our local client SpacetimeDB identity
    Identity? local_identity = null;
    // declare a thread safe queue to store commands
    ConcurrentQueue<(string Command, string Args)>? input_queue = new();

    /// The URI of the SpacetimeDB instance hosting our chat database and module.
    const string HOST = "http://localhost:3000";

    /// The database name we chose when we published our module.
    const string DBNAME = "spacemartian";

    /// <inheritdoc/>
    public override void OnStart()
    {
        // Initialize the `AuthToken` module
        AuthToken.Init(".spacemartian");
        // Builds and connects to the database
        DbConnection? conn = null;
        conn = ConnectToDB();
        // Registers to run in response to database events.
        RegisterCallbacks(conn);
    }

    /// Load credentials from a file and connect to the database.
    DbConnection ConnectToDB()
    {
        DbConnection? conn = null;
        conn = DbConnection.Builder()
            .WithUri(HOST)
            .WithModuleName(DBNAME)
            .WithToken(AuthToken.Token)
            .OnConnect(OnConnected)
            .OnConnectError(OnConnectError)
            .OnDisconnect(OnDisconnected)
            .Build();
        return conn;
    }

    /// Our `OnConnected` callback: save our credentials to a file.
    void OnConnected(DbConnection conn, Identity identity, string authToken)
    {
        local_identity = identity;
        AuthToken.SaveToken(authToken);
    }

    /// Our `OnConnectError` callback: print the error, then exit the process.
    void OnConnectError(Exception e)
    {
        Console.Write($"Error while connecting: {e}");
    }

    /// Our `OnDisconnect` callback: print a note, then exit the process.
    void OnDisconnected(DbConnection conn, Exception? e)
    {
        if (e != null)
        {
            Console.Write($"Disconnected abnormally: {e}");
        }
        else
        {
            Console.Write($"Disconnected normally.");
        }
    }

    /// Register all the callbacks our app will use to respond to database events.
    void RegisterCallbacks(DbConnection conn)
    {
        // TODO:
    }

    /// <inheritdoc/>
    public override void OnEnable()
    {
        // Here you can add code that needs to be called when script is enabled (eg. register for events)
    }

    /// <inheritdoc/>
    public override void OnDisable()
    {
        // Here you can add code that needs to be called when script is disabled (eg. unregister from events)
    }

    /// <inheritdoc/>
    public override void OnUpdate()
    {
        // Here you can add code that needs to be called every frame
    }
}
