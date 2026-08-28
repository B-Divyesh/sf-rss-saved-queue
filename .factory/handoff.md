# Polish round 2 handoff — deployed

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

Deployed through `/opt/fleet/lib/deploy-container.sh` on 28 August 2026 UTC.
The ACR build and Container App update completed successfully for `7ea46c6065e6`.

- Cold `verify-url.sh https://rss-saved-queue.sociobot.in` passed in 625 ms: title, `lang=en`, one h1,
  main, no missing image alt text, no unlabeled buttons, and no console/page errors.
- Screenshots: `.factory/evidence/polish-2-live/screenshot-desktop.png`,
  `.factory/evidence/polish-2-live/screenshot-mobile.png`, and
  `.factory/evidence/polish-2-live/live-demo-mobile.png`.
- Cold mobile axe had zero serious/critical issues on `/`, `/demo`, `/privacy`, `/terms`, `/not-a-route`,
  and `/extension-setup`. The first sample link began at y=729.09 in a 390 × 844 viewport.
- `/`, `/demo`, `/privacy`, `/terms`, and `/extension-setup` returned 200 with correct route titles.
  `/not-a-route` returned the styled page with HTTP 404. `?demo=1` showed the persistent banner,
  Reset demo, and Start for real.
- Both queued self-hosted samples returned 200. `/extension/` returned 200 and `/extension.zip`
  returned 200 `application/zip`; the live setup page generated a non-empty device key.

## Known gap

`.factory/brief.json` is absent. Scope was therefore checked against the review history, README,
claims registry, and visual thesis. No product finding remains locally.
