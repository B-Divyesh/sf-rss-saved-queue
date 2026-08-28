# Review 1 handoff — FAIL

Completed the requested adversarial first-read review without changing product code. The review is in `.factory/review-1.md`.

Verified with fresh live browser contexts at 390 px and desktop, route and metadata probes, request capture for `?demo=1`, repository/source inspection, and local frontend checks.

Completed local commands:

```sh
npm ci
npm test
npm run check
npm run build
cargo test --locked
npx playwright test --reporter=line
```

All completed commands passed: frontend unit tests 2/2, Rust tests 6/6, and browser tests 8/8. `.factory/claims.json`, `.factory/demo.md`, and tagged `@claim:` tests are absent, so no declared claim tests could be run. `/demo` returns 404; `?demo=1` uses the real localStorage namespace and real APIs. The review verdict is FAIL with five blocking findings.

The only committed changes should be this handoff and `.factory/review-1.md`.
