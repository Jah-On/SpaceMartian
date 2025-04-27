# SpacetimeDB powered server

https://spacetimedb.com/docs/modules/rust/quickstart

To run this
1. Install SpacetimeDB
2. `spacetime start`
3. Open a new terminal
4. `spacetime publish --project-path server spacemartian`
5. (Run `spacetime delete spacemartian` if it screams about "migration")
6. Call Rust functions `spacetime call spacemartian send_message 'Hello World'`
7. View DB logs `spacetime logs spacemartian`
8. Run SQL queries `spacetime sql spacemartian "SELECT * FROM message"`