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
