# Adversarial first-read review 2 handoff — FAIL

## What was done

Completed a read-only adversarial review of deployed build
`1b792bcc7972fbf1a38636a9a7536d71621436dd` at
<https://rss-saved-queue.sociobot.in>. The full report is `.factory/review-2.md`.
No product source was modified.

The report includes cold mobile/desktop first-read checks, complete landing/README copy counts,
one-click demo and isolation exercises, every registered claim command, offline/network interception,
all prior finding rechecks, route metadata/history/focus checks, a full link crawl, accessibility and
touch-size checks, visual-identity review, and missed leverage.

## Outcome

**FAIL:** five blocking, two major, and two minor findings.

Most importantly:

- no realistic sample row appears in the first 390 px post-click demo screen;
- both demo article destinations return 404;
- `@claim:demo-isolation` times out under the required cold-clone command;
- several stronger public promises are not declared in `.factory/claims.json`;
- the earlier “link” terminology finding regressed to “articles”;
- live extension setup has no install link and shows an empty device key;
- offline save reports only “Failed to fetch.”

## How it was verified

- Live cold viewports: 390 × 844 and 1440 × 1000, fresh Chromium contexts.
- Clean clone: `/tmp/rss-review2.KY4nQb/repo` at the deployed commit.
- Seven claim commands passed cold/warming; `demo-isolation` failed the first cold invocation at the
  90-second web-server timeout, then passed after compilation.
- Warm gates passed: `npm test` 2/2; `npm run check`; `npm run build`; `cargo test --locked` 9/9;
  `npm run test:browser -- --reporter=line` 24/24.
- Live `verify-url.sh` passed with no console/page errors.
- Live dark-scheme axe checks found zero serious/critical issues on root, Demo, Privacy, Terms, and
  404; all visible controls measured at least 44 × 44 px.
- Demo Reset replaced the workspace, restored the samples, preserved the real-key sentinel, and made
  the old workspace return 401.
- The complete route crawl found the two demo `example.com` destinations returning 404.

## Known limitations

`.factory/brief.json` is missing, so missed-leverage review used the deployed product, README, claims,
design thesis, and earlier verification history. No deployment or product changes were authorized or
made. Unlisted claims remain untested as declared claims, so the report cannot certify complete claim
coverage.
