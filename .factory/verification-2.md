# Independent verification 2 — FAIL

## Scope and identity

- Candidate: `770122766ee02b191147dd74d6f55572c9000353`
- Tested URL: <https://rss-saved-queue.sociobot.in>
- Date: 2026-08-28 UTC
- Checkout: clean detached worktree at the candidate SHA; no product source was modified.
- Deployment identity: live `GET /health` was `{"status":"ok","build":"e271f860a7c038b2"}`. Recomputing the Dockerfile source digest over `Cargo.toml`, `Cargo.lock`, `migrations/*`, and `src/*.rs` produced the same value. Live hashed assets and extension manifest also match this build.

## Verdict

**FAIL — do not release.** The repaired product implements the core private-queue flow, but the deployed dark-theme save state has an axe **serious** contrast failure. The acceptance contract requires zero serious/critical axe findings.

## Clean local gates

| Command | Result |
| --- | --- |
| `npm ci` | PASS — 87 packages; audit 0 vulnerabilities |
| `npm test` | PASS — 2 tests |
| `npm run check` | PASS — 0 errors/warnings |
| `npm run build` | PASS — JS 54.06 kB / 21.00 kB gzip; CSS 6.94 kB / 2.17 kB gzip |
| `npx playwright test --reporter=line` | PASS — 5 tests |
| `cargo fmt --check` | PASS |
| `cargo test` | PASS — 3 tests |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `git diff --check` | PASS |

Neither `docker` nor `podman` is installed in this worker, so the exact image build could not run. The locked release binary was built and run against a fresh SQLite DB.

## Core-flow and backend evidence

- Fresh live sessions returned 43-character URL-safe device keys. Unauthenticated list was `401`; a second account saw `[]`; its PATCH against another account's item was `404`.
- Save of title/HTTPS URL/tags returned `201`; `ftp:` URL returned `400`; 300-character title returned `201`, 301 returned `400`, and a 41-character tag returned `400`.
- A 43-character feed token served private RSS (`200`), XML-escaped `&`/`<` title text, marked its own item read, and returned `404` after owner revocation. Owner delete returned `204`.
- 100 concurrent authenticated list requests all succeeded (20-way parallel). A local release-binary run saved one entry, shut down, restarted with the same new SQLite database, and returned one entry after restart.
- Saving `http://127.0.0.1:9999/not-fetched` worked; the product stores user URLs and contains no server-side fetch path, satisfying the no-scraping privacy constraint.

Test entries/tokens were deleted/revoked after API checks; no customer data was accessed. One inaccessible disposable browser account can retain its ephemeral test entry because the headless context closed before its confirmation dialog was accepted.

## Browser, privacy, and performance evidence

- Desktop and 390 px mobile: no horizontal overflow; save/export controls remain visible. Keyboard Tab visibly focuses the skip link and Enter focuses `<main>`. Reduced motion reports `0.01ms` transition duration.
- Live light normal, light invalid-input, and mobile axe runs had zero serious/critical issues. Normal-load console/page errors were empty; invalid input's deliberate HTTP 400 emits the expected browser resource message.
- First-load browser traffic was same-origin only. No third-party font/script/tracker request, cookie, or service worker was found. This is not a PWA.
- Headers: API/health `no-store`, HTML `no-cache`, hashed assets `public, max-age=31536000, immutable`; CSP is self-only; nosniff, DENY framing, same-origin referrer policy, and Permissions-Policy are present.
- Lighthouse mobile live: performance **97**, accessibility **100**, LCP **1.4 s**, CLS **0**, TBT **190 ms**. Initial JS/CSS are under 200/50 kB budgets.

## Release-blocking axe proof

On live: switch to **dark theme**, open **Save a page**, submit `ftp://example.com/nope`, then run axe. It reports one serious `color-contrast` violation affecting seven nodes. Examples: the `INBOX DOOR` eyebrow is `#9cbfb0` on `#e7c65f` (**1.20:1**); save title/form text is `#f4efe5` on `#e7c65f` (**1.44:1**); optional text is `#c4bdb1` on `#e7c65f` (**1.12:1**). These fail both the 4.5:1 normal-text and 3:1 large-text requirements.

## Defects by severity

| Severity | Defect |
| --- | --- |
| SERIOUS | Live dark save sheet has the serious axe contrast failure above. |
| HIGH | RSS writes RFC 3339 to `<pubDate>` (observed `2026-08-28T00:09:30.281280959+00:00`), not the RFC 822-style date required by RSS 2.0. Strict readers may not accept the core interoperability feed. |
| HIGH | The extension offers configurable/self-host service URLs, but Manifest V3 `host_permissions` permits only `https://rss-saved-queue.sociobot.in/*`. A normal installation cannot fetch an arbitrary self-hosted endpoint without a changed/repackaged manifest. |
| MEDIUM | Public `POST /api/session` creates persistent accounts with no rate-limiting middleware/dependency, allowing database-exhaustion abuse and missing the backend-service rate-limit requirement. |
| MEDIUM | Submitting 13 tags returns `201` but silently stores 12; the UI does not state the limit or offer recovery feedback. |

## Required remediation

Fix dark save-sheet tokens and add a dark-theme axe regression test. Emit RFC 822/RFC 2822 RSS dates. Make extension permissions match documented self-host support (or document hosted-only behavior), rate-limit session/mutation routes, and reject or visibly explain over-limit tags. Re-run clean, live, both-theme axe, feed-reader, and extension QA.
