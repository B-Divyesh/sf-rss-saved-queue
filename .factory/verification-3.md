# Independent verification 3 — PASS

## Scope and verdict

**PASS — candidate `41085519b2b806c7ac568857f9f00325d8fc16e7` satisfies the researched brief and the factory acceptance contract.** Tested 2026-08-28 UTC against <https://rss-saved-queue.sociobot.in>. The checkout began clean at the candidate; no product source was modified during verification.

The candidate itself changes only handoff text. Its executable source is the repaired revision deployed at the tested URL. Live `GET /health` returned `{"status":"ok","build":"a1c49a7bd390bfed"}`; recomputing the Dockerfile's fallback source digest over `Cargo.toml`, `Cargo.lock`, `migrations/*`, and `src/*.rs` produced `a1c49a7bd390bfed`. The live JS and CSS SHA-256 values exactly match the freshly built `dist/` assets.

## Clean local gates

| Command | Result |
| --- | --- |
| `npm ci` | PASS — 87 packages, 0 vulnerabilities |
| `npm test` | PASS — 2 tests |
| `npm run check` | PASS — 0 errors, 0 warnings |
| `npm run build` | PASS — JS 54.05 kB (21.00 kB gzip); CSS 7.33 kB (2.25 kB gzip) |
| `npx playwright test --reporter=line` | PASS — 8 tests |
| `cargo fmt --check` | PASS |
| `cargo test` | PASS — 6 tests |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `git diff --check` before documentation changes | PASS |

`docker` and `podman` are not installed in this worker, so an image build could not be run locally. The exact locked frontend and release-binary stages were built, and live build identity/assets match the candidate's Dockerfile build input. Dockerfile inspection confirms a multi-stage build, no `.git` dependency, non-root runtime user, `PORT` 8080, and persisted `/data` SQLite path.

## Product and backend evidence

- A fresh live account receives a 43-character URL-safe device key. Unauthenticated `GET /api/items` returned `401`; a second account received `[]` and its PATCH against the first account's item returned `404`.
- Normal save of `QA & <reader>`, an HTTPS URL, and tags returned `201`. The product accepted a literal loopback URL as metadata without fetching it, consistent with the no-scraping/privacy constraint.
- Invalid `ftp:` URL, a 301-character title, and 13 non-empty tags each returned `400` with actionable recovery messages. The UI exposes the 12-tag limit and browser coverage confirms the error remains visible.
- A new 43-character feed token served `application/rss+xml`; title markup was escaped, `pubDate` was RFC 2822-shaped, and `POST /reader/<token>/items/<id>/read` returned `200` and changed the owner-visible item to `read`. Owner deletion returned `204`; revoking the feed token returned `204` and subsequent feed access returned `404`.
- The local release binary started with only `PORT=8091` supplied and served `/health`; no secret environment variable was required. A saved entry remained readable after graceful shutdown/restart. A 20-way, 100-request authenticated list smoke completed 100/100 successfully. The release-binary API also saved `http://127.0.0.1:9999/no-fetch` without issuing a fetch.
- The extension manifest is MV3 and uses `activeTab`, local storage, and optional `http`/`https` host permissions. Its options flow requests the exact configured service origin, and its popup sends only active-tab title/URL plus entered tags using the device-key Authorization header. The automated extension contract test passed. (Browser loading of the unpacked extension itself is not supported by the worker's headless Chrome configuration.)

## Browser, accessibility, privacy, and performance

- `/opt/fleet/lib/verify-url.sh` on the live URL passed: HTTP 200, 787 ms load, no console/page errors, title, `lang="en"`, exactly one `<h1>`, `<main>`, no images lacking `alt`, and no unlabeled buttons.
- At 390 × 844, there was no horizontal overflow. Keyboard Tab visibly focused the skip link and Enter moved focus to `<main>`. With reduced motion, the save sheet transition measured `1e-05s`.
- Live axe runs on light invalid-save and dark invalid-save states found **zero serious or critical violations**. The deliberate invalid save produces its expected 400 resource console message; a normal load had zero console/page errors.
- First-load traffic was same-origin only. Fresh browser context: zero cookies and zero service workers. This is intentionally not a PWA; the retryable offline state is tested rather than stale private-data caching.
- Live headers: HTML `no-cache`; API and health `no-store`; hashed assets `public, max-age=31536000, immutable`; self-only CSP, `nosniff`, DENY framing, same-origin referrer policy, and camera/microphone/geolocation-denying Permissions-Policy. No remote fonts, third-party scripts, analytics, or trackers were observed.
- Fresh live mobile Lighthouse: **99 performance**, **100 accessibility**, FCP 1,123 ms, LCP 1,276 ms, TBT 103 ms, CLS 0. Initial JS/CSS are within the 200/50 kB budgets.

## Defects by severity

None found in the tested scope.

## Known verification limitation

The worker lacks Docker/Podman, so container-image assembly was not independently executed locally. Deployment identity was nevertheless independently confirmed from the live health build digest and byte-identical built assets.
