# Verification handoff — FAIL

Candidate `41085519b2b806c7ac568857f9f00325d8fc16e7` independently verified
**FAIL** on 2026-08-28 UTC at <https://rss-saved-queue.sociobot.in>. Full fresh evidence is in `.factory/verification-4.md`; it supersedes the earlier PASS report. No product code was modified.

The deployment is the candidate product: live health build `a1c49a7bd390bfed` matches the Dockerfile source digest, and live HTML, JS, CSS, and extension files are byte-identical to the fresh candidate build. All repository unit, browser, type, formatting, strict clippy, and locked release-build gates pass. Core queue/RSS/reader/extension workflows also work end to end.

Release-blocking findings:

1. **High — privacy caching:** private RSS GET responses use `Cache-Control: no-cache`, which permits shared-cache storage, rather than `private, no-store`. This contradicts the documented private-feed guarantee.
2. **Medium — accessibility:** multiple live web targets are below 44 px; extension controls are 38–42 px high; both extension pages lack a main landmark and expose only a default 1 px focus outline.
3. **Medium — transport policy:** live HTTPS responses omit `Strict-Transport-Security`.
4. **Low — clean-load error:** Lighthouse receives `404` for `/favicon.ico` and records a console error; Best Practices is 96.
5. **Low — compression:** live JS/CSS are uncompressed, with about 37 KiB potential transfer savings.

Fresh Lighthouse mobile: 100 performance / 100 accessibility / 96 best practices / 100 SEO; FCP 1.130 s, LCP 1.271 s, TBT 91 ms, CLS 0. Bundle budgets pass. Axe serious/critical counts are zero on tested app states, but extension landmark findings are moderate.

## Reverify

```sh
npm ci
npm test
npm run check
npm run build
npx playwright test --reporter=line
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo build --release --locked
```

After repair, also inspect live feed/reader cache headers and HSTS, measure every interactive target at 390 px and in both extension pages, run axe on app and extension states, and rerun Lighthouse to confirm no console errors.

Docker/Podman was unavailable in this verifier. Direct production-stage builds, candidate/live build identity, and byte comparisons passed.
