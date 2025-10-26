# Docker overview

## Virtual machine vs Docker container

- Conainters **share the host OS system**, while VM **runs its own OS**. Analogy - everyone is renting their own separate flat vs everyone sharing one house, but have their own rooms.

## Image basics

- You can **search** for an image with the command, where you will see stars and 'official' status:

```
docker search <image_name>
```

- To **list downloaded** images use:

```
docker image ls
```

- To see **docker image layers** use the following command (add --no-truc flag to the full commands):

```
docker image history <image_name>
```

- To **tag** (give an existing docker image a new name or label) the image use the following command:

```
docker tag SOURCE_IMAGE:TAG TARGET_IMAGE:TAG

---or---

docker tag ubuntu:22.04 myrepo/ubuntu:prod
```

- To see how much **disk space Docker is using**, run this command:

```
docker system df
```

## Metadata

To inspect image metadata:

```
docker image inspect <image_name>:<image_tag>
```

In the respective `.json` file you can find the image digest, which is a **unique cryptographic** image identifier, which represents the exact Docker image. It guarantees that the image has not been tampered with and that deployments use the same version every time.

You can also find out what command `Config.Cmd` is fired when you run the container, startup command `Entrypoint` before cmd, user `User` under which the container is running, ports `ExposedPorts` that container exposes, environmental variables `Env` baked into the image. Also, you can see layers `RootFS.Layers` that the image is comprised of.

When you see an array of `Layers`, each layer represents an instruction from a Dockerfile (like `RUN`, `COPY` or `ADD`). Each layer is **read-only** and when you run a container Docker stacks all these layers and adds a **writable** layer on top.

View layers with:

```
docker history <image_name>
```
