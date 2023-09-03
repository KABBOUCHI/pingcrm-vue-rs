build:
  cargo build

clean:
  cargo clean

serve:
  cargo run -q --bin app

migrate *args:
  cargo run -q --bin db -- migrate {{args}}

seed *args:
  cargo run -q --bin db -- seed {{args}}

db *args:
  cargo run -q --bin db -- {{args}}