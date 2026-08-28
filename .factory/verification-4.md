# Independent verification 4 — FAIL

## Scope and verdict

**FAIL — candidate `41085519b2b806c7ac568857f9f00325d8fc16e7` does not satisfy the full factory acceptance contract.** Verification ran independently on 2026-08-28 UTC against a clean detached checkout of that exact commit and against <https://rss-saved-queue.sociobot.in>. No product source was modified.

The queue, private RSS bridge, reader endpoint, extension, and deployment are functional. The verdict is nevertheless FAIL because the private RSS response is cacheable, mandatory accessibility/touch-target requirements are not met, the live HTTPS response lacks HSTS, and Lighthouse observes a load-time 404 console error. These are fresh findings; the earlier PASS report is superseded.

## Candidate and deployment identity

- Candidate checkout: `41085519b2b806c7ac568857f9f00325d8fc16e7` (clean, detached worktree).
- Live `GET /health`: `{"status":"ok","build":"a1c49a7bd390bfed"}`.
- Recomputing the Dockerfile fallback digest over `Cargo.toml`, `Cargo.lock`, `migrations/*`, and `src/*.rs` produced `a1c49a7bd390bfed`.
- Freshly built and live JS SHA-256: `d3528e2b259545642e5ee87013805b052a5b7c556d63610e5b49887e4398da71`.
- Freshly built and live CSS SHA-256: `1ec852523b54a2de34dafba92b3cf46f02340eb7741b9265e9197682b3ebe586`.
- Live `index.html` and all five `/extension/` files were byte-identical to the candidate build/source.
- Plain HTTP redirects `301` to the tested HTTPS URL.

This establishes that the live executable/static product matches the candidate's product inputs even though health uses the Dockerfile source digest rather than the documentation-only candidate SHA.

## Clean local gates

| Command | Fresh result |
| --- | --- |
| `npm ci` | PASS — 87 packages audited, 0 vulnerabilities |
| `npm test` | PASS — 2/2 tests |
| `npm run check` | PASS — 0 errors, 0 warnings |
| `npm run build` | PASS — JS 54.05 kB (21.00 kB gzip), CSS 7.33 kB (2.25 kB gzip), extension copied to `dist/` |
| `npx playwright test --reporter=line` | PASS — 8/8 tests |
| `cargo fmt --check` | PASS |
| `cargo test --locked` | PASS — 6/6 tests |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | PASS |
| `cargo build --release --locked` | PASS |
| `BUILD_SHA=a1c49a7bd390bfed cargo build --release --locked` | PASS — reproduces the production build identity |
| `git diff --check` | PASS |

Docker and Podman are unavailable in the worker, so the image could not be assembled locally. The frontend and locked release-binary stages were built directly. Dockerfile inspection confirms a multi-stage build with no `.git` dependency, a non-root runtime user, `/data` persistence, port 8080, and build identity injection.

## Backend and product behavior

The release binary started with only `PORT=8091` in an otherwise empty environment. It created its default `/data` database, served the expected build identity, shut down cleanly, restarted, and retained all five saved test items. The verifier-created database was removed afterward.

Fresh local API evidence:

- unauthenticated item listing returned `401`;
- two sessions returned distinct 43-character URL-safe device keys;
- a second account saw an empty queue and could not mutate the first account's item (`404`);
- a normal title/HTTPS URL/tags save returned `201` and trimmed tags;
- title length 300 was accepted and 301 rejected;
- 12 tags were accepted and 13 rejected;
- a 40-character tag was accepted and 41 rejected;
- `ftp:` and credential-bearing URLs were rejected with `400`;
- a loopback URL was saved as metadata while a listening probe observed zero outbound requests;
- valid priority updates, search, and status filters worked; unknown and empty updates returned `400`;
- RSS tokens were distinct 43-character values; RSS content type, XML escaping, and RFC 2822 `pubDate` were valid;
- the reader-token endpoint marked the correct item read, CSV export was valid, revoke returned `204`, and the revoked feed returned `404`;
- 100/100 concurrent authenticated local reads returned `200`; 100/100 live reads returned `200` in 383 ms;
- the anonymous-session burst limiter allowed the remaining six requests in its eight-request bucket, then returned `429`.

Raw device/feed tokens were not present in the local SQLite file; 64-character SHA-256 hashes were. No tracker, analytics, remote font/script, or backend fetch surface was found.

## Browser and extension evidence

`/opt/fleet/lib/verify-url.sh` passed locally (688 ms) and live (807 ms): HTTP 200, title, `lang=en`, exactly one `<h1>`, `<main>`, no missing image alternatives, no unlabeled buttons, and no console/page errors in that normal Playwright load.

The live UI completed the full hosted workflow: save with invalid-URL recovery, successful save, priority change, read/unread, archive/undo, empty search recovery, create and consume a private feed, revoke it, export and inspect CSV, and confirmed deletion. A separate keyboard-only run used Tab/Enter for the skip link, save form, and mark-read action; the web app's active control had a designed 3 px focus outline.

At 390 × 844 there was no horizontal overflow and primary controls remained available. Reduced-motion computed transition and animation duration were both `1e-05s`. Fresh contexts observed only the product origin, no cookies, and no service worker. This is intentionally not a PWA; the repository's offline recovery test passed.

The unpacked MV3 extension was loaded in Chromium. In a headed Xvfb run, the native exact-origin permission prompt was accepted, options persisted the hosted endpoint/device key, the popup saved a representative page to the live queue, and the API-confirmed item was deleted. This verifies the extension beyond its source-contract test.

Axe 4.13 found zero serious/critical violations in the web app's light invalid, dark invalid, populated, privacy, and terms states. It also found zero serious/critical extension findings, but did report moderate `landmark-one-main` and `region` violations on both extension pages.

## Performance, requests, and policies

Fresh Lighthouse 12.8.2 mobile scores were **100 performance / 100 accessibility / 96 best practices / 100 SEO**. Metrics: FCP 1,130 ms, LCP 1,271 ms, TBT 91 ms, CLS 0, Speed Index 1,130 ms, and 62,686 bytes transferred. Initial JS and CSS satisfy the 200/50 kB budgets.

Live caching is correct for public shell/API assets: HTML `no-cache`, API and health `no-store`, and hashed assets `public, max-age=31536000, immutable`. The live product supplies a self-only CSP, `nosniff`, DENY framing, same-origin referrer policy, and camera/microphone/geolocation-denying Permissions-Policy. Defects in private-response caching and HTTPS policy are below.

## Defects by severity

### High — private RSS responses may be stored by shared caches

Fresh live `GET /feed/<token>/rss` responses returned `Cache-Control: no-cache`, not `no-store` or `private`. Under HTTP caching semantics, `no-cache` permits a GET response to be stored and merely requires revalidation before reuse. The URL authenticates via a capability token in the path and returns private queue metadata; without `private` it may be retained by shared intermediaries. This contradicts README's statement that private-feed responses are not stored and violates the privacy-by-default contract. The reader-token POST response also lacks an explicit private/no-store policy, although normal HTTP caches do not store POST responses without explicit freshness. Reproduce by creating a feed token and inspecting `GET /feed/<token>/rss` with `curl -D -`.

### Medium — mandatory accessibility baseline is incomplete

Measured live web targets below the required 44 × 44 CSS px include the 42 px search input, 48 × 28 priority selector, 212 × 25 article link, 47 × 15 Privacy link, and 38 × 15 Terms link. Extension popup inputs are 38 px high and its button 42 px; options inputs/buttons are 42 px. Both extension pages omit a `<main>` landmark and have only the browser-default 1 px focus outline (`rgb(16,16,16) auto 1px`) rather than the required designed focus treatment. Axe classifies the landmark/region findings as moderate. These measurements violate the attached accessibility and design-principles contract even though serious/critical axe counts are zero.

### Medium — HTTPS responses do not set HSTS

Live HTML, API, health, feed, reader, and static-asset responses all lacked `Strict-Transport-Security`. HTTP redirects to HTTPS, but without HSTS a first connection remains susceptible to downgrade/SSL-stripping. Other inspected security headers are present.

### Low — Lighthouse observes a load-time console error

The live site has no favicon resource or explicit favicon declaration. Lighthouse requested `/favicon.ico`, received `404`, and recorded `Failed to load resource: the server responded with a status of 404`, reducing Best Practices to 96. This violates the no-console-errors-on-load acceptance gate even though the supplied Playwright verifier did not request a favicon.

### Low — live text assets are not compressed

Lighthouse's text-compression audit reported approximately 37 KiB potential savings: the 54,053-byte JS and 7,333-byte CSS were transferred uncompressed. The absolute bundle and performance budgets still pass, so this is not independently release-blocking.

## Verification limitations and cleanup

- No Docker-compatible engine was installed; exact stage builds, Dockerfile audit, live build digest, and byte-identical static assets provide the available image evidence.
- One first-pass live automation run aborted after creating an item and revoking its feed token; its ephemeral random browser key was destroyed before the item could be deleted. All later live test items were deleted and all later feed tokens revoked. The orphan is isolated behind an unguessable lost key and demonstrates why a retention/account-deletion path would be useful.
