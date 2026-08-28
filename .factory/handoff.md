# Perfection loop round 1 handoff — PASS

## Outcome

Polished released candidate `fd638a94b897131b0dced9f76373ef152c66d049` against every finding in
`.factory/review-1.md`. The executable repair is `1af906fc8ec595827a111788e7ae0fb848079fb5`.
It is pushed to `origin/main` and deployed at <https://rss-saved-queue.sociobot.in>.

The first screen states the job and audience, leads to a one-click isolated demo, and explains the click.
The demo is an in-memory 24-hour workspace with a separate `demo:` session-storage namespace.
Reset reseeds three realistic links. Start for real destroys the demo without reading or writing a real queue.

Eight claims are registered in `.factory/claims.json`, each with one executable tagged test.
The extension claim runs the shipped popup and settings behavior with recorded fixtures.
Routes, titles, metadata, navigation focus, announcements, discovery files, 404 recovery, legal links, and mobile layout are covered.
The quiet reading-room ledger identity remains unchanged.

This pass also enabled compressed text responses, removed the obsolete HTTP client, made startup configuration provenance explicit,
and moved extension CSS out of inline markup so CSP cannot block its target sizing and focus treatment.

The finding-by-finding record is in `.factory/polish-1.md`.

## Clean-clone claim evidence

Clone: `/tmp/rss-polish-claims-VBUggz/repo` at `1af906fc8ec595827a111788e7ae0fb848079fb5`.

All eight commands from `.factory/claims.json` passed separately:

- `@claim:demo-isolation`
- `@claim:saved-metadata-no-fetch`
- `@claim:csv-export`
- `@claim:rss-feed-revocation`
- `@claim:device-isolation`
- `@claim:same-origin-privacy`
- `@claim:extension-save`
- `@claim:free-access`

## Full clean-clone verification

```text
npm ci                                             PASS — 86 packages; 0 vulnerabilities
npm test                                           PASS — 2/2
npm run check                                      PASS — 0 errors; 0 warnings
npm run build                                      PASS — JS 61.60 kB / 23.06 kB gzip
                                                       CSS 12.35 kB / 3.40 kB gzip
cargo fmt --check                                  PASS
cargo test --locked                                PASS — 9/9
cargo clippy --locked --all-targets --all-features -- -D warnings
                                                   PASS
cargo build --release --locked                     PASS
npm run test:browser -- --reporter=line            PASS — 24/24
git status --short                                 PASS — clean
```

Browser coverage includes every claim, both themes, invalid forms, offline recovery, demo reset and exit, direct `?demo=1`,
CSV content, RSS creation/revocation and caching, route history/focus/announcements, link crawling, 404, privacy, 390 px layout,
extension behavior, extension landmarks, designed focus, and 44 px controls.

Docker and Podman were unavailable locally. The successful factory ACR build exercised the complete multi-stage Dockerfile.

## Local runtime evidence

The service ran on `PORT=4181` and logged `database=supplied` and `static_dir=supplied` without printing values or secrets.

`verify-url.sh` passed: HTTP 200, 561 ms, title present, `lang=en`, one h1, main present,
zero missing image alternatives, zero unlabeled buttons, and zero console/page errors.

Local compressed JS returned `Content-Encoding: gzip`, immutable caching, HSTS, CSP, nosniff, same-origin referrer,
frame denial, and restrictive permissions policy.

Local Lighthouse mobile:

```text
Performance 99; Accessibility 100; Best Practices 100; SEO 100
LCP 1.4 s; TBT 110 ms; CLS 0; transfer 52 KiB
```

## Deployment and live evidence

Deployment command:

```sh
/opt/fleet/lib/deploy-container.sh rss-saved-queue /work/repo Dockerfile 8080
```

Result:

```text
ACR run chgb: succeeded after 5m26s
Image tag: 1af906fc8ec5
Container app: sf-rss-saved-queue
https://rss-saved-queue.sociobot.in -> 200
/health -> 1af906fc8ec595827a111788e7ae0fb848079fb5
```

Cold live checks confirmed:

- root h1 and first action use the required plain wording;
- `/demo` and `/?demo=1` show the banner/reset/exit and use only same-origin `/api/demo/*` requests;
- no real device key is created in demo; exiting clears session state and makes the old workspace return 401;
- demo save performs no request to the saved URL; CSV has one row per link; RSS has two queued items, `no-store`, and revokes to 404;
- Privacy navigation and browser history focus the route h1;
- Privacy, Terms, Demo, icons, robots, sitemap, and social image resolve; an unknown route returns the styled 404 with HTTP 404;
- 390 px demo has no overflow and no visible target below 44 px;
- live axe found zero serious/critical violations and zero total violations on Demo dark-error, Privacy, Terms, 404, and both extension pages;
- compressed live JS returns `Content-Encoding: gzip` with immutable caching and HSTS;
- `verify-url.sh` passed in 539 ms with zero console/page errors.

Live Lighthouse mobile:

```text
Performance 100; Accessibility 100; Best Practices 100; SEO 100
LCP 1.1 s; TBT 30 ms; CLS 0; transfer 50 KiB; console errors 0
```

Screenshots are under `.factory/screenshots/polish-1/`.

## Run and verify

```sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
npm test
npm run check
cargo test --locked
npm run test:browser
```

## Known gaps

None.
