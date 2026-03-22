# SELECT clause

1. Basic select `SELECT * FROM <table_name>;`, when we want to select all columns (**\*** is similar similar to {} in NoSQL);

2. Selecting specific columns with `SELECT name, email FROM users;` (this is equivalent to projecting only certain columns in NoSQL);

3. Filtering with `WHERE` as `SELECT * FROM users WHERE name = 'Alice'`;

- Similar to `.find({condition})`
- Should use '' not "" for **string literals** (actual values that match in the database);
- Comparison operators used are `=`, `!=`, `>`, `<`, `<=` and `>=`
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

5. Limiting results is done using `LIMIT` **aka** `.limit(n)` in NoSQL:

   ```
   SELECT * FROM users
   WHERE name = 'Alice'
   ORDER BY name DESC
   LIMIT 2;
   ```

---

## Additional SELECT features

1. Better filtering with `IN`, `BETWEEN`, `IS NULL`

- To select multiple values use `IN` **aka**:

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

3. Aggregation functions like `COUNT`, `AVG`, `SUM`, `MAX`, `MIN`

- To count rows use `COUNT(*)`:

  ```
  SELECT COUNT(\*) FROM users;
  ```

- To get average/sum/max of a column use `AVG/SUM/MAX(column_name)`:

  ```
  SELECT AVG(total) FROM orders;
  SELECT SUM(total) FROM orders;
  SELECT MAX(total) FROM orders;
  ```

4. Grouping by with `GROUP_BY`

- Group by **one column**:

  ```
  SELECT user_id, SUM(total)
  FROM orders
  GROUP BY user_id;
  ```

- Group by **multiple columns**:

5. `AS` alias to make queries more readable

   ```
   SELECT name as user_name
   FROM users;
   ```

   OR

   ```
   SELECT u.name
   FROM users u;
   ```

6. `DISTINCT` to remove duplicates

Removes **duplicate rows** from the result set. Works on **one column** or **multiple columns**.

- Single column

  ```
  SELECT DISTINCT user_id
  FROM orders;
  ```

  ⚡ Returns each `user_id` **once** - **list of customers who made orders**.

- Multiple columns

  ```
  SELECT DISTINCT user_id, product_id
  FROM orders;
  ```

  ⚡ Returns each unique **user-product pair**.

- With aggregate functions

  ```
  SELECT COUNT(DISTINCT user_id) AS unique_customers
  FROM orders;
  ```

  ⚡ Counts how many **unique customers** exist.

💡 **Pro tip**: Use `DISTINCT` when you need **uniqeness**, and `GROUP BY` when you need **aggregates per group**.

7. `LIMIT` + `OFFSET` for pagination

8. `WHERE` vs `HAVING`

Both are used to filter data, but operate at **different stages of a query** and are used for different purposes.

- `WHERE` filters rows before grouping:

1. Used to filter **individual rows**
2. Applied **before** `GROUP BY`
3. Cannot use aggregate functions (like `SUM`, `COUNT`, etc.)

   ```
   SELECT \*
   FROM orders
   WHERE price > 100;
   ```

👉 This returns rows where `price > 100`.

- `HAVING` filters groups after grouping:

1. Used to filter **groups of rows**
2. Applied **after** `GROUP BY`
3. Can use **aggregate functions**

   ```
   SELECT customer_id, COUNT(*) AS total_orders
   FROM orders
   GROUP BY customer_id
   HAVING COUNT(*) > 5;
   ```

👉 This returns customers who have **more than 5 orders**.

- Can use a combined query with both `WHERE` and `HAVING`:

  ```
  SELECT customer_id, SUM(price) as total_spent
  FROM orders
  WHERE price > 50
  GROUP BY customer_id
  HAVING SUM(price) > 500;
  ```

👉 Filter rows first (`price > 50`), then filter grouped results (`total_spent > 500`). Can use `HAVING total_spent > 500` in some databases.
