# RSS Saved Queue

Save web links in a private queue, then read them in your RSS reader.

It is for people who save too many links and want a smaller reading list.
The queue is free to use and requires no account or payment.

## Try the sample queue

Open [the sample queue](https://rss-saved-queue.sociobot.in/demo).

It starts with three links, varied priorities, two queue states, and a working RSS preview.
Demo changes stay in a separate memory-only workspace and never touch your real queue.
Use **Reset demo** for a fresh sample.
Use **Start for real** to discard the demo.

## Save and read links

Save a title, link, and optional tags.
Set its priority and queue state.
Create a private RSS link for queued links and revoke it later.
Export CSV writes one row for every saved link.

Each browser keeps a device key for its queue.
The server stores a one-way hash of that key.
A second device key cannot open or change the first queue.

The queue stores entered link metadata.
It does not fetch link contents or import public feeds.

Load the \`extension/\` folder as an unpacked Chrome extension.
Chrome asks permission for the service URL you enter.
The extension saves the active tab title, link, and entered tags.

## Run locally

Install Node 22+ and stable Rust.

\`\`\`sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
\`\`\`

Open <http://localhost:8080>.
Mount \`/data\` in production to keep queues after container replacement.

## Run checks

\`\`\`sh
npm test
npm run check
npm run build
npm run test:browser
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --release --locked
\`\`\`

Every public product promise appears in [\`.factory/claims.json\`](.factory/claims.json).
Run each listed command from a clean checkout.

## Deploy the container

Build the root \`Dockerfile\` with \`BUILD_SHA\` set to the source commit.
Run it on \`PORT\`, which defaults to \`8080\`.
Mount persistent storage at \`/data\`.
The health endpoint is \`/health\`.

## Privacy and legal

The app uses no analytics, ads, external fonts, or third-party scripts.
Read the live [privacy policy](https://rss-saved-queue.sociobot.in/privacy) and [terms](https://rss-saved-queue.sociobot.in/terms).

MIT licensed. See [LICENSE](LICENSE).
