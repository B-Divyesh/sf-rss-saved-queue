# Polish round 2 — repair record

Candidate reviewed: `1b792bcc7972fbf1a38636a9a7536d71621436dd`.
Repair commits: `74e038e09653ce9163654fc089c606c1b33187bc` and this handoff update.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 / B2 | Compacted the mobile demo header, hides duplicate demo detail on phones, and adds a 390 × 844 assertion that the complete first sample link is above the fold. | `mobile demo shows a complete sample link without scrolling`; live screenshot `.factory/evidence/polish-2-live/live-demo-mobile.png`, title at y=729.09. |
| F-2-2 | Replaced placeholder `example.com` sample destinations with three self-hosted reading samples and test-crawled them. | `sample destinations and extension package are available from the live product surface`; live `/samples/web-typography.html`, `/samples/reading-recall.html`, and `/samples/private-rss.html` return 200. |
| F-2-3 / B3 | Raised cold server allowance to five minutes and gives each Playwright run a fresh SQLite path. | Every exact `claims.json` command passed in clean clone `/tmp/rss-polish2-clean.iN904M/repo`; cold `demo-isolation` completed in 1m20s. |
| F-2-4 / B4 | Declared and tested connectivity, memory-only demo storage, key hashing, extension download, and CSV import promises; removed the unprovable restart-persistence sentence. | Ten one-to-one `@claim:` tests; clean-clone claim sweep passed. |
| F-2-5 / M2 | Rewrote the hero result as “See three saved links and their RSS feed,” then updated the terminology audit. | `rg` terminology audit and `.factory/copy-audit.md`; cold live `/` check passed. |
| F-2-6 | Added `/extension-setup`, an in-site ZIP download, a static `/extension/` landing page, and session creation before showing the device key. | `@claim:extension-save`; live `/extension-setup` showed a non-empty key and `/extension.zip` returned 200 `application/zip`. |
| F-2-7 | Maps offline fetch failures to specific reconnect guidance for save, updates, removal, RSS, reset, export, and import. | `@claim:internet-connection`; full browser suite. |
| F-2-8 | Replaced the cryptic theme glyph-only control with the visible action label “Use dark theme” / “Use light theme.” | `mobile controls and links meet touch sizing without horizontal overflow`; live mobile screenshot `.factory/evidence/polish-2-live/live-demo-mobile.png`. |
| F-2-9 | Added CSV import preview, exported-column validation, explicit confirmation, status/priority restore, and duplicate skipping in real and demo workspaces. | `@claim:csv-import`; full browser suite. |
| B1, B5, M1, M3, M4 | Preserved the prior plain first screen, isolated demo URL, metadata/routing/focus/404/legal skeleton, explanatory sections, short README, and result-naming controls. | `real routes set titles, canonical metadata, focus, announcements, and history`; `demo deep links, discovery files, icons, and the designed 404 route work`; full browser suite. |

## Verification before deployment

- Clean clone (`/tmp/rss-polish2-clean.iN904M/repo`): all ten exact claim commands passed; first cold command completed inside the 300-second server allowance.
- Clean clone: `npm test` (2/2), `npm run check`, `npm run build` (66.87 kB JS / 24.56 kB gzip; 13.71 kB CSS / 3.65 kB gzip), `npm run test:browser` (28/28), `cargo fmt --check`, `cargo test --locked` (9/9), and strict Clippy all passed.
- Original worktree: `cargo build --release --locked` passed.

## Live evidence

Cold live checks passed on 28 August 2026 UTC. `/`, `/demo`, `/privacy`, `/terms`, and
`/extension-setup` returned 200; `/not-a-route` returned the designed 404. Each had one h1 and
main landmark, correct route title, and zero serious/critical axe findings. `?demo=1` showed the
banner, Reset demo, and Start for real. `verify-url.sh` recorded no browser errors and saved desktop
and mobile screenshots in `.factory/evidence/polish-2-live/`.
