-----

For a server and a client, I need to have two different projects as they both have different functions, however, making another seperating them completely would not make sense as they are still linked so I have to re-organise my project folder so that I can have a headless program for the server and then a client build for players. 
Using the command: "git ls-tree -r --name-only HEAD | tree --fromfile"
Currently the file tree is:
```
.
├── Cargo.toml
└── src
    ├── main.rs
    ├── tileset.rs
    └── tiles.rs
```
To add multiple programs, I use workspace members in the root Cargo.toml file and I also created a server crate: 

```toml
[workspace]
resolver = "2"
members  = ["client", "server"]
```


Finally the directory (removing assets) looks like:
```
.
├── client
│   ├── Cargo.toml
│   └── src
│       ├── main.rs
│       ├── tileset.rs
│       └── tiles.rs
└── server
    ├── Cargo.toml
    └── src
        └── main.rs
```

-----
