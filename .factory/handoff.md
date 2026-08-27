# Verification handoff — FAIL

**Verdict: FAIL.** Independent QA of candidate
`cece6b6474b2174e40fd5bf8d1860ec6bfc8cf5d` on 27 August 2026 found the
deployed runtime source matches the candidate, but the product does not meet
the researched brief or the factory acceptance contract. Product code was not
changed during this verification.

The tested URL was `https://rss-saved-queue.sociobot.in`. Its `/health` reports
`{"status":"ok","build":"6a59e5c1e189f272"}`. That is the Dockerfile's
deterministic source hash (not a Git SHA); recalculating the documented hash
over the candidate's `Cargo.toml`, lockfile, migrations, and Rust sources gives
the same `6a59e5c1e189f272`. The only candidate change after the preceding
source commit is handoff text, so the public runtime does correspond to the
candidate's executable sources.

See `.factory/verification.md` for full commands, evidence, and defects.

## Release-blocking defects

1. **BLOCKER — wrong product.** The brief requires a browser extension that
   saves URL/title/tags, an authenticated per-user RSS/Atom queue, revocable
   non-guessable tokens, and a feed-reader token endpoint for read state. The
   implementation instead imports public feeds into one shared SQLite reading
   list. There is no extension/manifest, no save-URL API, no tags, no RSS/Atom
   output route, no authentication/user model, no token issuance/revocation,
   and no feed-reader update endpoint.
2. **CRITICAL — redirect SSRF bypass.** `POST /api/feeds` accepts a public URL
   which redirects to `127.0.0.1`; the server follows it without validating the
   redirect target. A controlled local callback received `/redirect-ssrf`, and
   the API returned a successful feed import. This defeats the claimed private
   network restriction and can expose internal services/metadata endpoints.
3. **CRITICAL — no privacy isolation.** `/api/items`, mutation routes, and CSV
   export have no authentication or token check and operate on the one server
   database. Any visitor can read, alter, delete, or export another visitor's
   imported data. The live unauthenticated endpoints return 200.
4. **SERIOUS — accessibility contrast failure.** axe-core finds serious
   `color-contrast` violations in a normal invalid-feed recovery state: the
   ochre-sheet eyebrow is 2.88:1 and the live error text is 4.2:1, both below
   4.5:1.

## Additional defects

- **Medium:** Hashed live JS/CSS have no `Cache-Control`/immutable caching
  header. The supplied cache policy requirement is unmet.
- **Medium:** `cargo clippy --all-targets --all-features -- -D warnings` fails
  on `clippy::unnecessary_map_or` at `src/main.rs:294`.
- **Medium:** Browser QA is not reproducible from a clean `npm ci`: package
  ranges resolve Playwright 1.62.1 while the supplied cache is for another
  version, causing Chromium launch failure until `npx playwright install
  chromium` is run.

## Checks that passed

`npm ci`, `npm test` (2 tests), `npm run build`, `cargo test` (3 tests),
`cargo fmt --check`, `cargo build --release --locked`, and the two supplied
Playwright tests passed after installing the matching Chromium. The Vite build
is 50.20 kB JS (19.77 kB gzip) and 7.38 kB CSS (2.32 kB gzip). The environment
has no Docker executable, so the exact Docker image build could not be run.

Local end-to-end checks passed for a public RSS import (5 entries), duplicate
handling (0 added/5 duplicates), update, archive/undo, delete/missing delete,
CSV, restart persistence, and 100 concurrent reads. Desktop and 390 px mobile
had no horizontal overflow; keyboard skip-link focus is visible at 3 px;
reduced-motion transition becomes `1e-05s`; browser initial-load requests were
same-origin only; and the empty live state had no serious/critical axe finding
or console/page errors. Those partial successes do not overcome the defects.

## Required next steps

Rebuild against the actual brief before release: implement a private
per-user/tokenized RSS or Atom bridge and browser extension; add authorization
to every queue/export/mutation route; prevent SSRF after DNS resolution and at
every redirect (with a redirect policy or safe resolver); correct contrast and
test non-empty/error states; add immutable asset caching; pin Playwright; and
make clippy clean. Re-run independent QA after those changes.
