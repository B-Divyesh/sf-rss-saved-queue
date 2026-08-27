# RSS Saved Queue handoff

## Delivered

- Recovered the empty scaffold into a working Rust/Axum + SQLite + Svelte product.
  Users can import public RSS/Atom URLs, deduplicate up to 100 recent entries per
  import, rank items as next/soon/later, mark them read, archive with undo, remove
  with confirmation, search queue state, and export CSV.
- Data is stored in SQLite (`DATABASE_URL`, default `/data/rss-saved-queue.db`),
  with a browser-local queue snapshot used for a temporary server outage. Mount
  `/data` when running a container to preserve the server database across container
  replacement.
- Added `/health` (including compile-time build identity), `/privacy`, and `/terms`.
  There are no trackers, CDNs, remote fonts, or third-party images. Feed URL input
  rejects private literal addresses and uses parameterized SQLite queries.
- Added an original reading-room ledger visual system in `.factory/design.md`, light
  and dark themes, keyboard focus styling, skip link, responsive phone layout,
  reduced-motion treatment, useful loading/error/empty states, and accessible labels.
- Added root multi-stage `Dockerfile`: Node build + Rust release build, Debian runtime,
  non-root UID 10001, `PORT=8080`, `/data`, and a deterministic source-hash build
  identity when `BUILD_SHA` is not supplied.

## Verification performed

- `cargo fmt --check` — pass.
- `cargo test` — 3 tests pass (public/private feed URL validation and HTML stripping).
- `npm test` — 2 presentation tests pass.
- `npm run build` — pass; production JS is 50.20 kB (19.77 kB gzip) and CSS is
  7.38 kB (2.32 kB gzip), within budget.
- `npm run test:browser` — 2 Playwright checks pass: desktop empty flow plus 390 px
  mobile controls. The desktop test runs axe-core and has zero serious/critical
  violations.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8080 evidence` — pass: HTTP 200,
  title/lang/one h1/main/alt checks pass, no browser console errors; measured local
  load was 588 ms. Desktop and mobile screenshots were visually reviewed.
- Product/API smoke: imported `https://hnrss.org/newest` (20 entries), confirmed CSV
  export, valid item transition, delete (204), missing-item response (404), and
  private feed rejection (400). `/health` returns the build value. Response headers
  include CSP, `X-Frame-Options: DENY`, `nosniff`, and same-origin referrer policy.

`npx @axe-core/cli` was attempted but cannot use the environment’s Playwright Chrome
binary; the equivalent axe-core Playwright test is included and passing. Lighthouse
also could not attach to headless Chrome in this worker environment, so no synthetic
Lighthouse score is claimed; the built asset sizes and browser-load evidence are above.

## Run and deploy

```sh
npm install
npm run build
DATABASE_URL=sqlite://rss-saved-queue.db?mode=rwc STATIC_DIR=dist cargo run

npm test && cargo test && npm run test:browser
```

Deploy the root Dockerfile with the factory container path:

```sh
/opt/fleet/lib/deploy-container.sh rss-saved-queue /work/repo Dockerfile 8080
```

No platform storage resource was changed by this work order. The application is ready
to use a mounted `/data` volume where the deployment environment provides one; browser
snapshot recovery remains available without it.
