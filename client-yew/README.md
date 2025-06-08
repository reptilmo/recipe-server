# client-yew
Yew client for the recipe server.


### Building
Install WebAssembly target.
```
    rustup target add wasm32-unknown-unknown
```
Install `trunk`.
```
    cargo install trunk
```
Run trunk to serve the client.
```
    trunk serve
```

### Notes
By default the server listens on port 8888, and the client app is served on port 3111.

## License
This project is made available under "MIT License".
See the file `LICENSE.txt` in this repository.
