# Docker users

- Like on the most linux systems, the first normal user created gets `uid=1000, gid=1000`, most official base images create a non-root user with UID=1000 and name it something like `ubuntu`, `node`, `nginx`, etc. This is a good **least-privilege** security practice.

**That ensures compatibility with host-mounted volumes!**

When you mount a directory from your host into a container:

```
docker run -v $(pwd):/app myimage
```

**The container and the host are looking at the same files**. Just two different filesystem namespaces.

- As Linux file ownership works by numeric IDs, not names, thus inside the container Docker does not translate usernames, but will **use the same UIDs/GIDs** and use these files as its own!
- Id you use UID 1000 inside the container, tehn it will match host UID 1000 and **you can edit the file freely** in the container!

**Define user in the Dockerfile**:

```
USER 1001:1001
```

**Pass user run `run` command**:

```
docker run -it -u 0:0 -v $(pwd):/data usertest
```

# `--privileged` and docker socket bind

- Mounting `/var/run/docker.sock` into the container is a big security breach and it basically giving the container the root access to the host. So, generally speaking, deamon will have an ability to perform actions that have consequences on the host. Also, **avoid** adding users to the `docker` group as it also gives them near root capabilities.

# Volumes

1. Use **named** volumes instead of **host path** mount for dynamic/persistent data

_GOOD!_

```
services:
  db:
    image: postgres
    volumes:
      - db_data:/var/lib/postgresql/data

volumes:
  db_data:
```

This way `db_data` is Docker managed volume and it comes with an improved security, portability, backup/restore (can call list, prune and inspect native commands)

_BAD!_

```
services:
  db:
    image: postgres
    volumes:
      - ./data:/var/lib/postgresql/data

```

**When host paths make sense?**

- For **development**, when you need live code reloading (static data)!
- For **logs** that you want to be visible on the host!

2. Use `/tmp` inside the container file system for **short container** tasks or **during the image build**

- The data will be written to the image **writable layer** and removed when you stop the container.

_Compiling code_:

```
FROM ubuntu

RUN apt-get update && apt-get install -y build-essential
RUN mkdir /tmp/build && cd /tmp/build && \
    gcc /src/app.c -o /usr/local/bin/app && \
    rm -rf /tmp/build

```

_Test runners_:

```
services:
  tests:
    image: python:3.11
    command: >
      bash -c "pytest tests/ --junitxml=/tmp/results.xml && cat /tmp/results.xml"
```

3. Use volume **access modes** correctly

- The default access mode for mount/bind volume is `:rw` or read-write, but if you only want to allow the container to read from the volume use `:r0` - read-only.

```
volumes:
  - ./data:/app/data:ro
```

4. Better use docker-managed volumes (external: true) for **production** environments

- Inits a temporary container and copies config from host to the volume **only if empty**.
- Mounts `shared_config_example` as **read-only** and `depends_on` ensures initialization happens before the app starts.
- `shared_config_example` in compose will **NOT** create the volume. Docker expects it to already exist in **the Docker daemon**! You have to initialize docker volume with `docker volume create shared_config`.
- IMPORTANT: changes done to **config.yaml** inside the Docker container will be reflected in the **named volume** , so carefully choose whether to use `shared_config_example:/app/config:ro` read-only flag.

```
volumes:
  shared_config_example:
    external: true  # assumes volume already exists

services:
  init_config:
    image: busybox
    # Only runs once to populate the volume if empty
    volumes:
      - shared_config:/data
      - ./default-config.yaml:/default-config.yaml:ro
    entrypoint: sh -c '
      # Only copy if the volume is empty
      if [ -z "$(ls -A /data)" ]; then
        cp /default-config.yaml /data/config.yaml
      fi
    '
    # Remove the container after running
    restart: "no"

  app:
    image: usertest
    depends_on:
      - init_config  # ensure volume is initialized first
    volumes:
      - shared_config_example:/app/config:ro  # read-only for safety
    environment:
      - CONFIG_PATH=/app/config/config.yaml
    restart: always
```

_Safely edit config in the volume_

- If you want to change the config **in the named volume**, do it through a temproray container that mounts the volume as **writable**:

```
docker run --rm -it \
  -v shared_config_example:/data \
  busybox sh
```

then

```
vi /data/config.yaml
```
