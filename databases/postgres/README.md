# Postgres and SQL general cheatsheet

---

- Postgres terminal commands
  1. `psql -U username -d database_name` - where -U is username and -d is database;
  2. `\conninfo` - to get connection info like _You are connected to database "test" as user "admin" via socket in "/var/run/postgresql" at port "5432"._;
  3. `\q` - to quit psql;
  4. `\dt` - outputs tables in a specific database/schema;
  5. `\dt+` - outputs tables in a specifc database/schema along with details like size and description;
  6. `\d table_name` - outputs info about table structure, types, indexes and relationships;
  7. `\di` - lists all indexes in the current schema;
  8. `\l` - prints out all tables;
  9. `\copy table_name to 'file_name'` - copies a table into a file
  10. `\i command_file` - executes a command file liek init.sql
