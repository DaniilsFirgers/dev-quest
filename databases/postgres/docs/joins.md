# JOINS

A **JOIN** combines rows from **multiple tables** based on a related column.

1. `INNER JOIN`

👉 Returns only rows where **there is a match in both tables**

👉 `INNER` is often optional (default), so you can write just `JOIN`

    ```
    SELECT u.name, o.amount
    FROM users u
    JOIN orders o ON u.id = o.user_id;
    ```

👉 Typically works with ONE-TO-MANY relationships (users to orders), but can also work in MANY-TO-MANY relationships via a `junction table`:

    ```
    SELECT u.name, r.role
    FROM users u
    JOIN user_roles ur ON u.id = ur.user_id
    JOIN roles r ON ur.role_id = r.id;
    ```

    ✅ Result:

    ```
    | name    | role |

    | -------- | ------- |
    | Alice | admin |
    | Alice | editor |
    | Bob | editor |

    ```

👉 Order of table does NOT matter!

2. `LEFT JOIN`

👉 Left join means -> all left rows + matches (or NULL)!
👉 Left table is **the one after `FROM`**!

🧠 Simple rule:

```
FROM table1
LEFT JOIN table2
```

➡️ `table1` = LEFT
➡️ `table2` = RIGHT

    ```
    SELECT u.name, o.amount
    FROM users u
    LEFT JOIN order o ON u.id = o.order_id;
    ```

TOP cases:

- Show all users + optional data;
- Find missing relationships (very common) -> users who never purchased;
- Aggregation with LEFT JOIN
- Filtering joined data correctly ('show all users, but only expensive orders')
- Joining a lookup table
- Multi table joins (very common)

3. `RIGHT JOIN`

👉 Same logic as for `RIGHT JOIN`, but keeps all right rows + matches
👉 Right table is the in the `RIGHT JOIN` clause;

    ```
    SELECT u.name, o.amount
    FROM users u
    LEFT JOIN order o ON u.id = o.order_id;
    ```

🔥 Right join is mostly avoided by developers in favour of left join (swap tables to get the right join)
