# RSS Saved Queue repair handoff

## Delivered

Repaired every release-blocking finding recorded in
`.factory/verification.md` for candidate `cece6b6474b2174e40fd5bf8d1860ec6bfc8cf5d`.

- Rebuilt the product around the researched job: a private saved-link queue.
  It accepts only user-entered title, HTTP(S) URL, and tags; it no longer fetches
  public feeds or stores publisher content. Existing queue handling remains:
  priority, read/archive, undo, delete, search, and authenticated CSV export.
- Added an included Manifest V3 browser extension at `/extension/`. It saves the
  active tab’s title/URL plus user-entered tags through the same private API. Its
  device key is stored in extension-local storage, not sync storage.
- Added per-device private identity. `POST /api/session` creates a 256-bit random
  device key; only its SHA-256 hash is stored. Every queue, token, and CSV route
  requires `Authorization: Bearer <device-key>` and scopes SQL by account.
- Added cryptographically random, hash-at-rest, revocable private RSS links at
  `GET /feed/<token>/rss` and reader state changes at
  `POST /reader/<token>/items/<id>/read`. Tokens are shown only on creation and
  cannot serve or mutate data after revocation.
- Removed the server-side public-feed importer entirely. This removes the prior
  redirect/DNS SSRF class rather than attempting to maintain a risky fetcher.
- Corrected error/add-sheet contrast (including the verifier’s exact ochre state),
  added populated/error-state axe coverage, immutable hashed-asset cache headers,
  permissions policy, reproducible `npm ci`, and exact Playwright 1.58.2 pinning.
- Updated the reading-room visual thesis, privacy/terms copy, README, Docker build
  (`npm ci`), and build output so the extension ships at `/extension/`.

## Verification performed

Clean-install and release gates on 2026-08-27 UTC:

```sh
npm ci                                      # pass; 87 packages, 0 vulnerabilities
npm test                                    # pass; 2 tests
npm run check                               # pass; 0 errors, 0 warnings
npm run build                               # pass; JS 54.06 kB / 21.00 kB gzip, CSS 6.94 kB / 2.17 kB gzip
npx playwright test --reporter=line         # pass; 5 tests
cargo fmt --check                           # pass
cargo test                                  # pass; 3 tests
cargo clippy --all-targets --all-features -- -D warnings  # pass
cargo build --release --locked              # pass
git diff --check                            # pass
```

Browser checks cover desktop save/populated flow with axe, invalid-URL recovery
with axe, 390 px no-horizontal-overflow controls, keyboard skip-link focus and
activation, and the extension manifest/save contract. Both axe runs have zero
serious/critical violations. The supplied URL verifier against local port 8090
reported title/lang/one-h1/main/alt checks and no console errors (582 ms load).

API regression smoke against a fresh local SQLite DB proved:

```text
cross-account PATCH: 404
unauthenticated GET /api/items: 401
private feed: 200
reader marks item read: 200
owner revoke: 204
revoked feed: 404
```

Response checks confirmed `Cache-Control: public, max-age=31536000, immutable`
on hashed assets; API responses are `no-store`; and CSP, nosniff,
same-origin referrer policy, DENY framing, and Permissions-Policy are present.
There are no third-party scripts, fonts, trackers, analytics, page images, or
service worker. Offline is intentionally a clear reconnect/error state rather
than a cached copy of private queue data.

## Run and deploy

```sh
npm ci && npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run

/opt/fleet/lib/deploy-container.sh rss-saved-queue /work/repo Dockerfile 8080
```

The Dockerfile is a multi-stage Node/Rust build, uses `npm ci`, runs as UID
10001, and serves on `PORT=8080`. Mount `/data` when the container platform
offers persistent volume storage. It was deployed through that exact factory
container path on 2026-08-27. Live `GET /health` reports build
`e271f860a7c038b2`, matching the Dockerfile source digest recalculated from
this repair. Live `/api/items` is unauthenticated `401`, `/extension/manifest.json`
is `200`, and a hashed JS asset has immutable cache control. The live URL verifier
passed at 684 ms with no console errors and title/lang/one-h1/main/alt checks.

## Known gaps

No local Docker daemon is available in this worker, so the exact Docker build
was not exercised locally; the successful factory ACR/container deployment is
the production-image verification.
