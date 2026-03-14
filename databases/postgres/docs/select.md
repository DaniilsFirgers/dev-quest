# SELECT clause

1. Basic select `SELECT * FROM <table_name>;`, when we want to select all columns (\* is similar similar to {} in NoSQL);

2. Selecting specific columns with `SELECT name, email FROM users;` (this is equivalent to projecting only certain columns in NoSQL);

3. Filtering with `WHERE` as `SELECT * FROM users WHERE name = 'Alice'`;

- Similar to `.find({condition})`
- Should use '' not "" for **string literals** (actual values that match in the database);
- Comparisonm operators used are `=`, `!=`, `>`, `<`, `<=` and `>=`
- Can combine conditions with logical operators `AND` and `OR` like this:

```
SELECT * FROM users
WHERE name = 'Alice' AND email LIKE '%example.com';
```

4. Sorting results is done by using `ORDER BY`:

```
SELECT * FROM users
ORDER BY name ASC;
```

OR

```
SELECT * FROM orders
ORDER BY total DESC, created_at ASC;
```

- `DESC` means from largest to smallest (numbers go from high to low, text from Z to A, and dates from latest to earliest);
- `ASC` means from smallest to largest (numbers go from low to high, text from A to Z, and dates from earliest to latest);

5. Limiting results is done using `LIMIT` aka `.limit(n)` in NoSQL:

```
SELECT * FROM users
WHERE name = 'Alice'
ORDER BY name DESC
LIMIT 2;
```

---

## Additional SELECT features

1. Better filtering with `IN`, `BETWEEN`, `IS NULL`

- To select multiple values use `IN` aka:

```
SELECT * FROM users WHERE name IN ('Alice', 'Bob');
```

- To select values from the range use `BETWEEN`:

```
SELECT * FROM orders WHERE amount BETWEEN 100 AND 500;
```

- To select rows with `NULL` values:

```
SELECT * FROM users
WHERE email is NULL;
```

2. For pattern matching use `%`:

```
--- starts with A
WHERE name LIKE 'A%';

--- ends with .com
WHERE email LIKE '%.com';

--- contains "ali"
WHERE name LIKE '%ali%';
```

3. Aggregation functions

- To count rows use `COUNT(*)`:

```
SELECT COUNT(*) FROM users;
```

- To get average/sum/max of a column use `AVG/SUM/MAX(column_name)`:

```
SELECT AVG(total) FROM orders;

SELECT SUM(total) FROM orders;

SELECT MAX(total) FROM orders;

```

4. Grouping by

```
SELECT user_id, SUM(total)
FROM orders
GROUP BY user_id;
```

5. Aliases to make queries more readable

```
SELECT name as user_name
FROM users;
```

OR

```
SELECT u.name
FROM users u;
```

6. Distinct to remove duplicates

7. Limit + offset (pagination)

8. WHERE vs HAVING
