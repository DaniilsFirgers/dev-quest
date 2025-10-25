### Docker overview

# Metadata

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
