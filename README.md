# RSS Saved Queue

RSS Saved Queue is a private, deliberately small reading queue for people who
save links from around the web and want to read them in their own RSS reader.
It stores only the title, URL, tags, and queue state you provide; it never
fetches saved pages or imports public feeds.

Each browser receives a random device key stored locally. The server retains
only its SHA-256 hash, so every API request and export is isolated to that
device key. Treat it like a password. The app can create long random,
revocable RSS links for feed readers; the reader state endpoint is
`POST /reader/<feed-token>/items/<id>/read`.

## Use it

Open the app, save a page, then choose **Connect reader** to create a private
RSS link. The newly generated link is shown exactly once; paste it into your
reader and revoke it from the app whenever needed.

To use the browser extension, load the repository's `extension/` directory as
an unpacked Manifest V3 extension. In its options, paste the service URL and
device key shown under **Connect reader**. Chrome then asks you to allow that
exact service origin, so the extension can work with this hosted instance or a
self-hosted one without gaining automatic access to every site. It saves the
active tab's title and URL, plus tags you enter.

## Run locally

Requires Node 22+ and Rust.

```sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
```

Open <http://localhost:8080>. The default production database is
`/data/rss-saved-queue.db`; mount `/data` when running the container to keep
saved queues across replacements.

## Verify

```sh
npm ci
npm test
npm run check
npm run build
npx playwright test --reporter=line
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release --locked
```

The root Dockerfile builds the Svelte frontend and Rust service, runs as a
non-root user, and listens on `PORT` (default `8080`). Built hashed assets are
served with immutable caching; API and private-feed responses are not stored.

## Privacy and legal

There are no trackers, analytics, external fonts, or third-party scripts.
See `/privacy` and `/terms` in the running app.
