# Card Game API

[![Rust](https://github.com/Leynaic/card-game-api-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Leynaic/card-game-api-rs/actions/workflows/rust.yml)

"Card Game API" is a project made in [Rust](https://www.rust-lang.org/) with [Actix](https://actix.rs/).
The API allows you to manage a simple card game deck _(shuffle, take a card, put a card back)_.

# How to use it
## First step

Creates the table of the decks :

```sql
-- Add this extension for the uuid.
CREATE extension IF NOT EXISTS "uuid-ossp";
-- Create the table to store the decks state.
CREATE TABLE decks
(
    id         uuid                     DEFAULT uuid_generate_v4() NOT NULL
        CONSTRAINT decks_pk
            PRIMARY KEY,
    discarded  integer[]                                           NOT NULL,
    cards      integer[],
    created_at timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    updated_at timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  NOT NULL
);
```

Before running the API you must create an environment file at the project root
Next, insert two lines : _DATABASE_URL_ and _ASSET_URL_.

```shell
touch .env
```

Exemple :

```dotenv
DATABASE_URL="postgres://postgres:username@localhost/database"
ASSET_URL="my_asset_url"
ASSET_EXTENSION=".png"
HOST="127.0.0.1"
PORT="8000"
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
