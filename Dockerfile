# syntax=docker/dockerfile:1
FROM node:22-alpine AS frontend
WORKDIR /app
COPY package.json ./
RUN npm install --ignore-scripts
COPY index.html vite.config.ts ./
COPY src ./src
RUN npm run build

FROM rust:1-bookworm AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY migrations ./migrations
COPY src ./src
ARG BUILD_SHA=""
RUN build_sha="$BUILD_SHA"; if [ -z "$build_sha" ]; then build_sha="$(sha256sum Cargo.toml Cargo.lock migrations/* src/*.rs | sha256sum | cut -c1-16)"; fi; BUILD_SHA="$build_sha" cargo build --release --locked

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 --create-home app && mkdir /data && chown app:app /data
WORKDIR /app
COPY --from=backend /app/target/release/rss-saved-queue /usr/local/bin/rss-saved-queue
COPY --from=frontend /app/dist ./dist
ENV PORT=8080 DATABASE_URL=sqlite:///data/rss-saved-queue.db?mode=rwc STATIC_DIR=/app/dist RUST_LOG=info
USER 10001
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rss-saved-queue"]
