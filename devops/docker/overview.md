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

# Volumes

Test this: Do not not bind mount docker socket volume (security breach), it will allows to control host Docker deamon directly, meaning, give root access to host. While `--priviledged` allows to run docker inside docker, but without root socket access.
