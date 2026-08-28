# Review 3 handoff — not accepted

Review commit records a read-only adversarial review of the deployed product. Product source was not modified.

## What was verified

- Fresh 390 px and desktop visits clearly state the job, audience, and first action.
- The one-click `/demo` path has visible sample data, the persistent sandbox banner, Reset demo, and Start for real. Demo storage uses its separate in-memory workspace and `demo:` session keys.
- All ten registered claims were exercised in a clean clone; the complete browser suite passed 28/28.
- Clean-clone checks passed: `npm test` (2/2), `npm run check`, `npm run build`, `cargo test --locked` (9/9), strict Clippy, formatting, and browser tests. Built JS is 24.56 kB gzip; CSS is 3.65 kB gzip.
- Live route, metadata, link, privacy, headers, keyboard, mobile, and dark-mode axe checks passed. `verify-url.sh` passed with no console errors.

## Remaining work

The verdict in `.factory/review-3.md` is **FAIL** with two minor findings:

1. `.factory/copy-audit.md` is stale and internally contradictory. Regenerate it from current landing/README copy and add an inventory check.
2. Rewrite the README extension-install sentences to use the visible Chrome label **Load unpacked** and describe the service URL as this site’s address.

`.factory/brief.json` remains absent; this review used the live product, README, claims, design thesis, and earlier review history for scope.
