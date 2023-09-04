build:
  cargo build

clean:
  cargo clean

serve:
  cargo run -q --bin app

serve-watch:
  cargo watch -x "run -q --bin app" -c

migrate *args:
  cargo run -q --bin db -- migrate {{args}}

seed *args:
  cargo run -q --bin db -- seed {{args}}

db *args:
  cargo run -q --bin db -- {{args}}

work *args:
  cargo run -q --bin work -- {{args}}