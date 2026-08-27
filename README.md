# RSS Saved Queue

RSS Saved Queue is a calm, durable reading list for people who use RSS. Add an RSS
or Atom feed, choose what to read next, mark items read or archived, and export your
list when you need it. It has no accounts, ads, or analytics.

## Run locally

Requires Node 22+ and Rust.

```
npm install
npm run build
DATABASE_URL=sqlite://rss-saved-queue.db?mode=rwc STATIC_DIR=dist cargo run
```

Open `http://localhost:8080`. During UI development, run `cargo run` in one terminal
and `npm run dev` in another; Vite proxies API requests to the Rust service.

## Verify

```
npm test
npm run build
cargo test
```

The Docker image is a multi-stage non-root image and listens on `PORT` (default
`8080`). SQLite is stored under `/data/rss-saved-queue.db`; mount `/data` for durable
container storage.

## Deploy

The factory deploys the root `Dockerfile` through its container path. The image build
identity is baked into `/health` as `build`, using the `BUILD_SHA` build argument.

## Data and legal

See `/privacy` and `/terms`. Imported feed URLs and their public entries are stored
only to provide the queue. The browser also keeps a local recovery snapshot.
