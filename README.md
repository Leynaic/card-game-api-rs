# Card Game API
"Card Game API" is a project made in [Rust](https://www.rust-lang.org/) with [Actix](https://actix.rs/).
The API allows you to manage a simple card game deck _(shuffle, take a card, put a card back)_.

# How to use it
## First step
Before running the API you must create an environment file at the project root
Next, insert two lines : _DATABASE_URL_ and _ASSET_URL_.

```shell
touch .env
```

Exemple :

```dotenv
DATABASE_URL=postgres://postgres:username@localhost/database
ASSET_URL=my_asset_url
```


## Build
You can build the project and run it with the generated executable:

```shell
cargo build
```

## Run
You can also run it directly with `cargo`:

```shell
cargo run
```