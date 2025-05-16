# recipe-server
This server uses Tokio, Axum, Askama, Sqlx, Sqlite to serve recipes.

### Building
- Ensure that the environment variable `DATABASE_URL` is set.
```
    echo $DATABASE_URL
```
or 
```
    export DATABASE_URL=sqlite://{your path}/recipe-server.db
```
- `cd` into the repository folder.
- Install `sqlx-cli`: `cargo install sqlx-cli`.
- Make sure the directories in `{your path}` exist.
- Run `cargo sqlx database setup`.
- Then cargo run -- --init-from=assets/static/recipes.json

## Todo
Better tag query
API routes
More recipes
Client side app
