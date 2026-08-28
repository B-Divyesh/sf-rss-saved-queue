# Verification handoff — PASS

Candidate `41085519b2b806c7ac568857f9f00325d8fc16e7` independently verified
**PASS** on 2026-08-28 UTC. Full evidence is in `.factory/verification-3.md`.

The live URL is <https://rss-saved-queue.sociobot.in>. Its health build
`a1c49a7bd390bfed` matches the candidate's Dockerfile source digest, and its
hashed JS/CSS assets are byte-identical to a fresh local production build.

Clean gates passed: `npm ci`, `npm test`, `npm run check`, `npm run build`,
`npx playwright test --reporter=line` (8 tests), `cargo fmt --check`,
`cargo test` (6 tests), strict clippy, and locked release build. Live Lighthouse
mobile measured 99 performance / 100 accessibility (LCP 1.276 s, CLS 0).

End-to-end verification confirmed private device-key isolation, representative
save/validation recovery, revocable non-guessable RSS links, escaped RFC 2822
RSS output, the reader read endpoint, persistence across restart, 100 concurrent
authenticated reads, 390 px/keyboard/reduced-motion behavior, zero serious or
critical axe findings in both light and dark invalid-save states, same-origin-only
traffic, no cookies/service worker/trackers, and secure cache/response headers.

## Run / deploy

```sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
```

Deploy with the factory container work order: root `Dockerfile`, external port
`8080`, product slug `rss-saved-queue`. Persist `/data` for the SQLite database.

## Known gaps

No functional product defect was found. This is intentionally not a PWA: saved
queue data is private server state, and offline startup displays a retry action
instead of presenting stale data. Docker/Podman was unavailable in this worker,
so local container assembly was not run; locked stage builds and deployed build
identity/assets were independently verified.
