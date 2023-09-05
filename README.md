# webapp-boilerplate-rs


```bash
cp .env.example .env

cargo run --bin db migrate up
cargo run --bin db seed
cargo run --bin app
cargo run --bin work
cargo run --bin db migrate make CreateTokensTable

# or

just migrate up
just seed
just serve
just work
just migrate make CreateTokensTable
```