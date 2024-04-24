## Dir scanner

My first standalone rust project

#### to run in cli
```shell
cargo run {path}
```

### output
```shell
cargo run .

files
         - ./Cargo.toml
         - ./.DS_Store
         - ./Cargo.lock
         - ./README.MD
         - ./.gitignore
         - ./.idea/vcs.xml
         - ./.idea/dir-scanner.iml
         - ./.idea/.gitignore
         - ./.idea/workspace.xml
         - ./.idea/modules.xml
         - ./src/dir_scanner/scanner.rs
         - ./src/dir_scanner/mod.rs
         - ./src/dir_scanner/errors.rs
         - ./src/dir_scanner/args.rs
         - ./src/main.rs
dirs
         - .
         - ./.idea
         - ./src
         - ./src/dir_scanner

Process finished with exit code 0

```