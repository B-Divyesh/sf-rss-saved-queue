# Repair handoff — ready for deployment

## Scope

Repaired the release-blocking findings in independent verification report
`.factory/verification-2.md` for candidate
`770122766ee02b191147dd74d6f55572c9000353`. The private device-key queue,
account isolation, revocable feeds, reader read endpoint, export, and no-fetch
privacy model were retained.

## Repairs

- Dark-mode save sheet now scopes high-contrast light-card tokens: `#25231f`
  ink, `#3f3b34` muted copy, and `#29483d` ledger labels on ochre. The exact
  dark invalid-save axe reproduction is covered in Playwright.
- RSS `<pubDate>` now converts the stored RFC 3339 timestamp to RFC 2822.
  The Rust feed test parses the emitted value with `parse_from_rfc2822`.
- The MV3 extension now requests an optional permission for the exact configured
  `http` or `https` service origin. It can save to a self-hosted service without
  shipping a blanket granted host permission; denial gives an explicit recovery
  message.
- Added `tower_governor` per-peer rate limits: a burst of 8 session creations
  then one every 6 seconds, plus a separate 24-burst / one-per-2-seconds bucket
  shared by state-changing routes. The service supplies connection info and a
  Rust regression asserts the ninth immediate session request is `429`.
- A 13th non-empty tag is now rejected with `400 Use up to 12 tags per saved
  page.` rather than truncated. The save form states the limit and its browser
  regression verifies the visible recovery message.
- First-load connection failures now enter the existing retryable queue error
  state; the browser suite covers it. This is appropriate offline behavior for
  this server-backed (non-PWA) product.

## Verification

Clean install and product gates run on 2026-08-28 UTC:

```text
npm ci                                             PASS — 87 packages, 0 vulnerabilities
npm test                                           PASS — 2 tests
npm run check                                      PASS — 0 errors/warnings
npm run build                                      PASS — JS 54.05 kB / 21.00 kB gzip; CSS 7.33 kB / 2.25 kB gzip
npx playwright test --reporter=line                PASS — 8 tests
cargo fmt --check                                  PASS
cargo test                                         PASS — 5 tests
cargo clippy --all-targets --all-features -- -D warnings  PASS
cargo build --release --locked                     PASS
git diff --check                                   PASS
```

Browser coverage includes a populated private queue, invalid URL recovery,
the verifier's dark-theme save sheet, tag-limit recovery, 390 px mobile without
overflow, keyboard skip-link activation, and offline/connection recovery. Axe
is exercised for populated, light-invalid, and dark-invalid save states; all
have zero serious/critical violations.

Fresh SQLite release-binary API smoke evidence:

```text
reader token marks its item read                         200
13 tags are rejected                                    400
9th immediate POST /api/session from one peer           429
private RSS XML escapes title text and has RFC 2822 pubDate  PASS
```

`/opt/fleet/lib/verify-url.sh http://127.0.0.1:8091` passed: 584 ms local
load, no console/page errors, `lang=en`, one `<h1>`, a `<main>`, zero missing
image alts, and zero unlabeled buttons. A Playwright traffic smoke saw only the
local origin, zero cookies, zero service workers, and zero errors. Local HTTP
checks confirm self-only CSP; `nosniff`, DENY framing, same-origin referrer,
Permissions-Policy; `no-store` for API/health, `no-cache` HTML, and immutable
hashed-asset caching. There are no trackers, external fonts, or remote scripts.

Docker and Podman are unavailable in this worker, so exact local image creation
was not possible; the locked production binary was built successfully. Cloud
deployment completed through the supplied container work order on 2026-08-28:
ACR image `sf-rss-saved-queue:c3aae35f03ea`, root `Dockerfile`, port 8080, and
the public TLS URL `https://rss-saved-queue.sociobot.in` all returned success.
Live `/health` is `{"status":"ok","build":"d5160be55d9ab20a"}`, matching
the Dockerfile source digest over `Cargo.toml`, `Cargo.lock`, migrations, and
Rust source.

Live `/opt/fleet/lib/verify-url.sh` passed at 695 ms with no console/page
errors and the same document/a11y basics. A live dark invalid-save axe check
reported zero serious/critical violations; 390 px had no overflow and keyboard
Tab focused the skip link. Live request smoke observed only the product origin,
zero cookies, zero service workers, and zero errors. A disposable live private
item/feed test confirmed escaped XML, RFC 2822 `pubDate`, reader mark-read
`200`, then cleanup delete `204` and token revoke `204`.

## Run / deploy

```sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
```

Deploy with the factory container work order: root `Dockerfile`, external port
`8080`, product slug `rss-saved-queue`. Persist `/data` for the SQLite database.

## Known gaps

No functional product gap is known. This is intentionally not a PWA: saved
queue data is private server state, and offline startup displays a retry action
instead of presenting stale data. Local Docker/Podman was unavailable, but the
factory cloud container build and live identity validation passed.
