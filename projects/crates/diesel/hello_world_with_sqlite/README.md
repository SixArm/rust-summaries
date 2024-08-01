# Diesel demo with SQLite

Diesel connects to databases via these libraries:

* libpq for the PostgreSQL backend
* libmysqlclient for the Mysql backend
* libsqlite3 for the SQlite backend

To install one or more of these on macOS via brew:

* `brew install postgresql libpq`
* `brew install mysql`
* `brew install sqlite3`


## Setup

This demonstration uses SQLite because it's the easiest portable database, and uses the SQLite bundled database rather than any SQLite system-wide database.

Run:

```sh
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
diesel setup --database-url='hello'
diesel migration generate create_table_posts
```

Diesel creates a directory `migrations` and two new migration files: the up migration file and down migration file. Edit these files.

up:

```sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
);
```

down:

```sql
DROP TABLE posts;
```

Run:

```sh
diesel migration run
```
