# Techneosis Site API

## Simple API for my website
I used rust for some reason. (It's not because I know what I'm doing)

## Development
sqlx-cli is used to create database migrations in the proper format:
`cargo install sqlx-cli`
**Note:** I had to debug compilation of the openssl crate using it's [docs](https://docs.rs/openssl/latest/openssl/).

`sqlx migrate add -r some-migration-description`
**Note:** Migrations are expected to always have an "up" and a "down" version, hence `-r`

## Deploy/Run
To run the project, you'll need to have a sqlite database created already and point to it using the ROCKET_DATABASES environment variable.

### Create DB Using sqlx cli
`DATABASE_URL=/url/to/db.sqlite sqlx database create`

### Run the Project
`ROCKET_DATABASES='{techneosis={url="/url/to/db.sqlite"}}' cargo run --release`
