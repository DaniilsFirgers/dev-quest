# Postgres and SQL general cheatsheet

- ## Postgres terminal commands

  1. `psql -U username -d database_name` - where -U is username and -d is database;
  2. `\conninfo` - to get connection info like _You are connected to database "test" as user "admin" via socket in "/var/run/postgresql" at port "5432"_;
  3. `\q` - to quit psql;
  4. `\dt` - outputs tables in a specific database/schema;
  5. `\dt+` - outputs tables in a specifc database/schema along with details like size and description;
  6. `\d table_name` - outputs info about table structure, types, indexes and relationships;
  7. `\di` - lists all indexes in the current schema;
  8. `\l` - prints out all tables;
  9. `\copy table_name to 'file_name'` - copies a table into a file;
  10. `\i command_file` - executes a command file like init.sql;
  11. `\du` - lists all database users;

- ## Init scripts and Docker

Initialization scripts can be either .sql or .sh and will run only once on the first start up of the container. If db was initialized they are ignored. Stored inside docker-entrypoint-initdb.d.

It is possible to run multiple scripts in a succession if they are named - 01-some_stuff.sql ... 02-some_stuff.sql and so on.

To overwrite default username, password and database use the following environment variables in the docker-compose:

```
environment:
  POSTGRES_USER: your_username
  POSTGRES_PASSWORD: your_password
  POSTGRES_DB: your_database
```

- ## SQL commands and stuff

- ### Schema vs database

- ### INDEX vs CONSTRAINT

  Index is created primarily for search optimisations, while constraint is used
  for data integrity (I in ACID), so that we add only data that adheres to our restrictions.

  Both indexes and contraints can be UNIQUE. A unique constraint automatically creates a unique
  index, while unique index standalone should be created explicitly.

  When a constraint is dropped, a related index is also dropped automatically. Examples:

  a. During Table Creation (indirect index via primary key)

  ```
  CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE,
    age INT NOT NULL CHECK (age > 0)
  )
  ```

  b. After table creation

  ```
  CREATE INDEX isx_users_age ON users (age);

  ALTER TABLE users ADD CONSTRAINT chk_username_no_start_digit CHECK (username !~ '^[0-9]');
  ```

  Overall INDEX suggestions:

  1. Use indexes for columsn that frequently use WHERE cluses, JOIN conditions,
     sorting (ORDER BY) and grouping (GROUP BY);
  2. For a B-TREE index generally requires 10-20% of the table's data size;
  3. Try not to use more than 10 indexes as write operation will take significatly
     longer.
  4. DO NOT index highly volatile columns like timestamps;
  5. Monitor unused indexes with:

     ```
      SELECT
          schemaname,
          relname AS table_name,
          indexrelname AS index_name,
          idx_scan,
          pg_size_pretty(pg_relation_size(indexrelid)) AS index_size
      FROM
          pg_stat_user_indexes
      JOIN
          pg_index
          ON pg_stat_user_indexes.indexrelid = pg_index.indexrelid
      WHERE
          idx_scan = 0 -- No scans
          AND NOT indisunique; -- Exclude unique indexes
     ```

     And index sizes with:

     ```
      SELECT
          schemaname,
          relname AS table_name,
          indexrelname AS index_name,
          pg_size_pretty(pg_relation_size(indexrelid)) AS index_size
      FROM
          pg_stat_user_indexes
      ORDER BY
          pg_relation_size(indexrelid) DESC;
     ```

  6. Index maintenance

  - Use VACUUM to reclaim space occupied by 'dead tuples', which are created
    after rows deletion or updates and should be cleaned up (as data is marked
    as dead, not removed instantly);

    _Clean up dead tupels and recover space without a lock_

    ```
    VACUUM
    ```

    _Reclaim all space, but with a table lock:_

    ```
    VACUUM FULL
    ```

    _Remove dead space and get statistics in one go_

    ```
    VACUUM ANALYZE
    ```
