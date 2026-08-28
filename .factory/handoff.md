# Polish round 2 handoff — ready for deployment

Repair commit: `74e038e09653ce9163654fc089c606c1b33187bc`.

## What changed

Closed every finding in `.factory/review-2.md` and rechecked the earlier review history.
The one-click demo now shows a real sample link in the first phone screen, uses self-hosted sample
destinations, and remains memory-only. The product now has CSV import with preview, validation,
confirmation, and duplicate handling. The extension has an install page, downloadable package, and
a real device key. Offline actions give clear reconnect guidance. The visible theme action is labelled.

`.factory/claims.json` now has ten claims, each with exactly one tagged browser test. The full
finding map is `.factory/polish-2.md`.

## Verification

From clean clone `/tmp/rss-polish2-clean.iN904M/repo`:

- All ten exact claim commands passed. The cold `@claim:demo-isolation` command finished in 1m20s.
- `npm test` passed 2/2; `npm run check` passed; `npm run build` produced `dist/`.
- `npm run test:browser -- --reporter=line` passed 28/28, including axe, mobile, routes, privacy,
  offline, import, and extension checks.
- `cargo fmt --check`, `cargo test --locked` (9/9), and strict Clippy passed.
- `cargo build --release --locked` passed in the repair worktree.

The built JavaScript is 66.87 kB (24.56 kB gzip) and CSS is 13.71 kB (3.65 kB gzip).

## Deployment and live evidence

Run `/opt/fleet/lib/deploy-container.sh rss-saved-queue /work/repo Dockerfile 8080`.
After deployment, run `verify-url.sh`, axe, cold desktop/mobile checks, and the live route/link crawl.
Record the live build SHA and screenshots here before final handoff.

## Known gap

`.factory/brief.json` is absent. Scope was therefore checked against the review history, README,
claims registry, and visual thesis. No product finding remains locally.
