default:
  @just --list

# Run 'cargo run' on the project
run *ARGS:
  cargo run {{ARGS}}

migratedb:
  cargo sqlx migrate run

db:
  psql ${DATABASE_URL}

redis:
  redis-cli -u $REDIS_URL

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
  cargo watch -s 'just tailwind' -x "run -- {{ARGS}}"

tailwind:
  tailwindcss -o static/main.css styles/tailwind.css

format:
  treefmt

lint:
  cargo clippy -- -D warnings

test:
  cargo test

sqlxprepare:
  cargo sqlx prepare -- --lib

pre-commit: tailwind format lint test sqlxprepare

install-git-hooks:
  grep "just pre-commit" .git/hooks/pre-commit || (echo "just pre-commit" >> .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit)

deploy:
  cargo sqlx prepare
  nix build '.#docker'
  cat result | docker load
  flyctl auth docker
  docker tag ${APP_NAME}:${APP_VERSION} registry.fly.io/${APP_NAME_FLY}:${APP_VERSION}
  docker push registry.fly.io/${APP_NAME_FLY}:${APP_VERSION}
  flyctl deploy
