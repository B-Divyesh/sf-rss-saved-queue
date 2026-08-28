# Polish round 3 handoff

Status: accepted. The deployed application revision is `8d96723406022a10046d535cd8f2eb0a89a4f18f` at <https://rss-saved-queue.sociobot.in>.

## What changed

- Replaced the stale hand-maintained copy proof with a generated inventory.
  `.factory/copy-inventory.json` is the source inventory for the cold landing
  screen and README; `npm run audit:copy` validates source text, word counts,
  banned terms, and the generated `.factory/copy-audit.md`. `npm test` runs it.
- Rewrote the README and in-product extension setup around Chrome’s visible
  labels: **Developer mode**, **Load unpacked**, and **this site’s address**.
  The extension settings and recovery messages use the same language.
- Kept the existing private demo, queue, routing, metadata, legal, extension,
  CSV, RSS, privacy, mobile, and ledger visual-system repairs intact. The
  catalog sentence is now verb-first and 69 characters.

## Exact verification evidence

Fresh clone used: `/tmp/rss-polish3-clean.iAo9Ja/repo` at `8d96723`.

| Check | Result |
| --- | --- |
| Every exact command in `.factory/claims.json` | 10/10 passed independently; first cold `@claim:demo-isolation` completed in 1m 22s, then the other nine claims passed independently. |
| `npm test` | Passed: copy-audit inventory check plus 2/2 Vitest tests. |
| `npm run check` and `npm run build` | Passed with 0 Svelte diagnostics; build produced `dist/` with 66.90 kB JS (24.56 kB gzip) and 13.71 kB CSS (3.65 kB gzip). |
| `npm run test:browser` | Passed 30/30: all claims, private/demo flows, CSV/RSS/extension behavior, privacy interception, offline recovery, keyboard, mobile, metadata/routing, link crawl, and axe serious/critical scans. |
| `cargo fmt --check`, `cargo test --locked`, strict Clippy, release build | Passed; Rust tests 9/9. |
| Production `verify-url.sh` | Passed at 2026-08-28 UTC: HTTP 200, 661 ms cold navigation, correct title/lang/one h1/main, no missing image alt, no unlabeled button, and no console/page errors. Evidence: `/work/.evidence/rss-saved-queue-polish-3/verify.json`. |
| Production cold route/demo/axe review | Passed: root and demo first-screen checks, `?demo=1` isolation, legal/extension/404 titles and headings, discovery/sample/package URLs, and zero serious/critical axe findings. Screenshots: `/work/.evidence/rss-saved-queue-polish-3/live-root-390.png`, `live-demo-390.png`, and `live-extension-390.png`. |
| Live deployment identity and headers | `/health` returned build `8d96723406022a10046d535cd8f2eb0a89a4f18f`. Hashed JS has `Cache-Control: public, max-age=31536000, immutable`; security headers include CSP, HSTS, nosniff, referrer policy, and permissions policy. |

## Run locally

```sh
npm ci
npm test
npm run check
npm run build
npm run test:browser
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --release --locked
```

Run each command declared in `.factory/claims.json` from a clean checkout as
well. Start the service with `DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run`.

## Known gaps and next steps

None. The failed adversarial finding set is fully addressed. No AI feature was
added because importing/exporting and a private RSS link meet the product’s
actual job without introducing an unrelated model dependency.
