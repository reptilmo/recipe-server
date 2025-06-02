# recipe-server
This server uses Tokio, Axum, Askama, Sqlx, Sqlite, Utoipa to serve recipes.


### Building
- Ensure that the environment variable `DATABASE_URL` is set.
```
    echo $DATABASE_URL
```
or 
```
    export DATABASE_URL=sqlite://{your path}/recipe-server.db
```
- `cd` into the repository folder
- Install `sqlx-cli`: `cargo install sqlx-cli`
- Make sure the directories in DATABASE_URL exist
- Run `cargo sqlx database setup`.
- Then `cargo run -- --init-from=assets/static/recipes.json`

### Testing
There is a very simple API test scrip in folder `test`.
- `cd` into `test`
- `python3 -m venv testenv` create Python virtual environment under `test`
- `source testenv\bin\activate` activate the virtual environment
- `pip3 install requests` install the rewquests package used by `api.py`
- `python3 api.py` run `api.py`

### Routes
Following routes are currently implemented. \
Web: \
/ - supports parameters 'id', 'tags'

Api: \
/api/redoc \
/api/v1/recipe/{recipe_id} \
/api/v1/recipe/with-tags \
/api/v1/recipe/random


### Todo
Leptos client side app\
Add API routes, POST, DELETE\
Docker\
CI


## License
This project is made available under "MIT License".
See the file `LICENSE.txt` in this repository.
