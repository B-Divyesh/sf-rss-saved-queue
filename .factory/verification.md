# Independent verification — FAIL

## Scope and identity

- Candidate: `cece6b6474b2174e40fd5bf8d1860ec6bfc8cf5d`
- Tested URL: <https://rss-saved-queue.sociobot.in>
- Date: 2026-08-27 UTC
- Checkout: clean, at the candidate SHA before verification. Only this report
  and `.factory/handoff.md` were changed afterward.
- Deployment identity: live `GET /health` returned build
  `6a59e5c1e189f272`; the candidate's Dockerfile computes that exact source
  digest when `BUILD_SHA` is absent. Recalculation produced the same value.
  `cece6b6` changes only handoff text from `e6a7cff`, so this confirms the
  deployed executable source is the candidate source; it is not evidence of a
  Git-SHA build label.

## Verdict

**FAIL — do not release.** The primary user job in the brief is absent, and
there are critical server-side request forgery and data-isolation failures.

## Acceptance-contract assessment

| Contracted capability | Evidence | Result |
| --- | --- | --- |
| Browser extension saves URL, title, tags | No extension manifest/directory, no save route, and no tag model in tree/schema | FAIL |
| Private authenticated per-user RSS/Atom queue | Routes are only item CRUD, import, CSV, health; no feed output, auth, user, or token code | FAIL |
| Revocable non-guessable private tokens | No token generation, storage, authentication, or revocation | FAIL |
| Feed-reader token endpoint updates read state | No such route or credential verification | FAIL |
| Retain only user-supplied metadata | The app fetches and stores public feed entries; this is a different ingestion model from the brief | FAIL |
| Privacy by default | One unauthenticated shared database is readable/mutable/exportable via public endpoints | FAIL |
| Normal RSS reading-list operations | Local import, duplicate handling, state changes, archive/undo, delete, search, export, persistence worked | Partial only |

## Reproducible evidence

### Clean install, build, and automated checks

| Command | Result |
| --- | --- |
| `npm ci` | PASS; 80 packages; npm audit reported 0 vulnerabilities |
| `npm test` | PASS; 1 file / 2 tests |
| `npm run build` | PASS; JS 50.20 kB / 19.77 kB gzip, CSS 7.38 kB / 2.32 kB gzip |
| `cargo test -q` | PASS; 3 tests |
| `cargo fmt --check` | PASS |
| `cargo build --release --locked` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | **FAIL**: `clippy::unnecessary_map_or` at `src/main.rs:294` |
| `npx playwright test --reporter=line` | PASS; 2 tests after `npx playwright install chromium` |
| Exact `docker build ...` | Not run: `docker` is not installed in this worker (`command not found`) |

There is no repository lint or standalone typecheck script (`npm run` exposes
only dev/build/test/browser/preview). The first browser run after clean `npm ci`
failed because the unpinned Playwright range resolved 1.62.1 while the supplied
browser cache was from another revision; installing the matching Chromium made
the test pass.

### End-to-end local service QA

A fresh SQLite database and a local candidate server were used. A representative
public RSS 2.0 source (`https://www.rssboard.org/files/sample-rss-2.xml`) gave
`200 {"feed_title":"NASA Space Station News","added":5,"duplicates":0}`;
a second import gave `added:0, duplicates:5`. Item update to `read`/`soon`,
CSV download (`text/csv`, attachment), delete `204`, repeat delete `404`,
and restart persistence (5 entries remained) all worked. One hundred concurrent
`GET /api/items` calls (20-way parallel) all succeeded.

Invalid/recovery checks:

- Empty feed URL: `400 Enter a complete http or https feed URL.`
- Literal loopback: `400 Private network addresses are not allowed.`
- `ftp:` URL: `400 Enter a public http or https feed URL.`
- Bad status: `400 Unknown status.`; missing item update: `404`.
- Keyboard-only start: Tab reaches “Skip to queue” with a visible 3 px outline;
  Enter moves to the main landmark.
- Desktop 1440 px and 390 px mobile: no horizontal overflow; add-feed and CSV
  controls remain visible at 390 px. Reduced motion changes the item transition
  duration to `1e-05s`.
- Browser initial-load requests were only to the page origin; local browser
  console/page errors were empty except for the expected failed 400 request in
  the invalid-input test.

### Critical SSRF proof

The literal-IP guard is bypassable. A temporary local HTTP server at
`127.0.0.1:9010` recorded requests. Then the local candidate service received:

```sh
curl -X POST http://127.0.0.1:8090/api/feeds \
  -H 'content-type: application/json' \
  --data '{"url":"https://httpbin.org/redirect-to?url=http%3A%2F%2F127.0.0.1%3A9010%2Fredirect-ssrf"}'
```

It returned `200 {"feed_title":"QA","added":0,"duplicates":0}`, and the
callback log recorded `SSRF_CALLBACK /redirect-ssrf`. `reqwest` follows the
redirect after validation; no redirect target or resolved address validation is
performed. DNS rebinding is likewise not addressed.

### Accessibility and browser behavior

The supplied empty-state axe test passes, and live empty state on 390 px had no
serious/critical axe finding, no console/page errors, one `<h1>`, one
`<main>`, and `lang="en"`. It does not cover error/populated states.

In a normal error recovery (open Add a feed, submit `ftp://example.com/feed`),
axe-core reported a serious `color-contrast` violation:

- `.feed-sheet > div > .eyebrow`: #587064 on #e2b94f, **2.88:1** at 12 px.
- `.live`: #c9462f on #f5f0e7, **4.2:1** at 16 px.

Both are under the 4.5:1 requirement. No critical axe issue was found in that
state.

### Live deployment, privacy, headers, and caching

Fresh live checks returned `200` for `/`, `/privacy`, `/terms`, `/health`,
`/api/items`, and `/api/export.csv`. The live unauthenticated
`GET /api/items` returned `[]` at test time and `GET /api/export.csv`
returned a 55-byte CSV header. Since the server has no auth, user/account key,
or token check in any route, an import by one visitor would be globally
readable, mutable, deletable, and exportable by another. This violates the
brief's private-feed requirement.

Live first-load browser requests were same-origin only; no third-party fonts,
analytics, or scripts were observed. CSP is `default-src 'self'; base-uri
'self'; form-action 'self'; frame-ancestors 'none'; object-src 'none'`; also
present were `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, and
`Referrer-Policy: same-origin`. No cookies were set in these checks.

The hashed asset responses lack `Cache-Control` (including immutable caching),
as do HTML responses. `Permissions-Policy`, `Strict-Transport-Security`, and
an explicit rate limit were absent from the application responses. Absence of
HSTS may be deployment-termination policy, but immutable asset caching is a
stated product requirement and is not met.

## Defects by severity

| Severity | Defect |
| --- | --- |
| BLOCKER | Missing the required product: extension save flow, metadata/tags, private per-user RSS/Atom output, revocable token, and reader-token state endpoint. |
| CRITICAL | Redirect-based SSRF allows requests to loopback/private targets after the initial URL check. |
| CRITICAL | Shared unauthenticated API/database exposes all queue data and mutations to any visitor. |
| SERIOUS | Error/add-feed state has axe serious color-contrast failures. |
| MEDIUM | Hashed assets lack immutable cache headers. |
| MEDIUM | Strict clippy gate fails. |
| MEDIUM | Playwright version/browser dependency is not reproducible from clean install without a browser download. |

## Required remediation before re-verification

Implement the brief rather than iterating the importer: browser extension save
flow, per-user identity/data isolation, cryptographically strong revocable feed
tokens, authenticated RSS/Atom generation and reader-token read updates. Block
SSRF for every redirect and resolved address (including private, link-local,
loopback, IPv6 private/local, and metadata ranges), ideally with a safe DNS
resolution/connect policy and a no-unvalidated-redirect client. Add authz tests,
token-revocation tests, extension tests, and populated/error-state axe tests.
Fix contrast, cache hashed assets immutably, pin Playwright and its browser
revision, and make clippy pass. Then repeat clean-build and deployment QA.
