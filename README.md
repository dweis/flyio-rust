## Run PostgreSQL for local development

```bash
docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```

## Connect to PostgrSQL on CLI

```bash
psql postgres://postgres:mysecretpassword@localhost:5432/postgres
```

## Add a new migration script

Make sure cargo sqlx is installed:


```bash
cargo install sqlx
```

Add a migration:
```bash
cargo sqlx migrate add
```

Run migrations:
```bash
cargo sqlx migrate run
```

