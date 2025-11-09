# Layer caching during build

- When you build an image from a Dockerfile, Docker executes instructions **one by one**. Each instruction creates a **read-only** layer.
- Docker **caches every layer** and next time you build the same Dockerfile, Docker checks of instruction and its **context** have not changed. If unchanged - use the **cached layer**, otherwise **rebuild that layer and all layers after it**.

# Best practices for layer caching and build optimization

# Multi-stage builds
