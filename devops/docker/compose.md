# Docker compose file

## Environment variables vs docker secrets

- **Environment variables** are key-values pars passed to the container at runtime. They are used for non-sensitive information like database port, non-sensitive urls and toggle features, because they can be seen using `docker inspect` or `docker exec`.

a) Using docker run

```
docker run -e DB_USER=myuser -e DB_PASS=PASSWORD myimage
```

- Sets variables using `-e` and can be access, for example, using `PROCESS.ENV` in the container of Node.js.

b) Using Docker Compose you have two options, either to use `- ENV_NAME=value` or `ENV_NAME: value`

```
services:
  app:
    image: myimage
    environment:
      - DB_USER=myuser
      - DB_PASS=mypassword
```

- or using `.env` file

```
env_file:
    - .env
```

c) Using a `.env` file

```
docker run --env-file .env myimage
```

- **Docker secrets** in the docker compose allows you to safely manage sensitive data without exposing them as environment variables or in the image.

```
version: "3.8"

services:
  app:
    image: busybox
    secrets:
      - db_password
    command: tail -f /dev/null

secrets:
  db_password:
    file: ../configs/db_password.txt
```

- The secret is hold in `/run/secrets/<name_of_file>`.

// Cover yaml anchors and aliases

// Lifecycle hooks
