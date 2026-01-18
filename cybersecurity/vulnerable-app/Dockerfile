# Use bullseye node 22.16 as the base image
FROM node:22.16-bullseye-slim

# Enable Corepack (bundled with Node)
RUN corepack enable && corepack prepare pnpm@10.28.0 --activate

# set working directory
WORKDIR /app

# Copy required files
COPY package.json tsconfig.json pnpm-lock.yaml ./
COPY src/ /app/src/
COPY public/ /app/public/

# Install dependencies
RUN pnpm install --frozen-lockfile

# Run application
CMD ["pnpm", "dev"]