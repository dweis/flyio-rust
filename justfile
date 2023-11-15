default:
  @just --list

# Run 'cargo run' on the project
run *ARGS:
  cargo run {{ARGS}}

startdb:
  (docker start some-postgres || docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres) && echo "started"

stopdb:
  docker stop some-postgres 

migratedb:
  cargo sqlx migrate run

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
  cargo watch -x "run -- {{ARGS}}"

tailwind:
  tailwindcss -o static/main.css styles/tailwind.css

test:
  cargo fmt
  cargo clippy -- -D warnings
  cargo test

sqlxprepare:
  cargo sqlx prepare -- --lib

pre-commit: tailwind test sqlxprepare

install-git-hooks:
  echo "just pre-commit" > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
