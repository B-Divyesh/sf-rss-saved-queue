# Polish round 1 — all findings closed

Scope: candidate `fd638a94b897131b0dced9f76373ef152c66d049`, adversarial report
`de6b1eee269497222c0af0d90aafab1a1505503a` (`.factory/review-1.md`).
No earlier `.factory/polish-*.md` file existed.

| Finding | Change made | Evidence |
| --- | --- | --- |
| B1 — unclear job, audience, and first click | The first screen uses “Save web links in a private RSS queue.”, names RSS-reader users, leads with “Try it with sample data”, explains the result beside it, and keeps “Save your first link” secondary. | Browser test `@claim:demo-isolation sample mode uses only its ephemeral namespace and resets`; [live 390 px first screen](screenshots/polish-1/live-first-screen-390.png); cold check of <https://rss-saved-queue.sociobot.in/> confirmed the exact h1, action, adjacent explanation, three facts, no overflow, and HTTP 200. |
| B2 — no isolated one-click demo | `/demo` and `/?demo=1` create random in-memory workspaces with three links, all priorities, two states, and a ready two-item RSS feed. Demo state uses `demo:` session storage and `/api/demo/*`. Reset reseeds it. Start for real destroys it. | Browser test `@claim:demo-isolation sample mode uses only its ephemeral namespace and resets`; backend test `demo_workspace_is_ephemeral_and_cannot_touch_real_queue`; [live 390 px demo](screenshots/polish-1/live-demo-390.png); live `/?demo=1` used only `/api/demo/session`, `/api/demo/items`, and `/api/demo/feed-tokens`, created no real device key, and the exited workspace returned 401. |
| B3 — no claims registry or claim tests | `.factory/claims.json` registers eight reliance claims. Every id appears in exactly one executable `@claim:` test. Tests now exercise outcomes, including the extension popup and permission flow rather than inspecting source alone. | All eight registry commands passed independently in clean clone `/tmp/rss-polish-claims-VBUggz/repo`; full browser run passed 24/24. |
| B4 — unlisted privacy and feature promises | Registered and tested metadata/no-fetch/no-import, CSV rows, RSS creation and revocation, device isolation and hashing, same-origin privacy, extension save fields, demo isolation, and free access. | Tests `@claim:saved-metadata-no-fetch`, `@claim:csv-export`, `@claim:rss-feed-revocation`, `@claim:device-isolation`, `@claim:same-origin-privacy`, `@claim:extension-save`, and `@claim:free-access`; live demo saved an unreachable URL without requesting it, exported four data rows plus a header, served/revoked RSS, and made only same-origin requests. |
| B5 — routes, titles, metadata, focus, 404, and discovery files | Added real `/demo`, `/privacy`, `/terms`, and styled 404 handling; route-specific title/description/canonical/OG/Twitter metadata; SVG/ICO/apple icon and 1200×630 social image; robots and sitemap; history focus and polite announcements; header/footer legal links and build id. | Tests `real routes set titles, canonical metadata, focus, announcements, and history`, `demo deep links, discovery files, icons, and the designed 404 route work`, and `all internal landing links resolve without dead ends`; [live privacy](screenshots/polish-1/live-privacy-390.png); [live 404](screenshots/polish-1/live-404-390.png); live discovery files returned 200 and unknown route returned the styled page with HTTP 404. |
| M1 — missing explanatory skeleton | Added three first-screen facts, the live queue, “How the queue works” in three steps, and plain privacy boundaries. | [live first screen and full mobile page](screenshots/polish-1/live-first-screen-390.png); test `mobile controls and links meet touch sizing without horizontal overflow`; live page had one h1, main landmark, no overflow, and no undersized visible target. |
| M2 — metaphor-heavy, changing terminology | Replaced shelf/room/pieces/line language with queue, link, private RSS link, device key, and demo. The reading-room ledger remains only as visual identity. | `.factory/copy-audit.md` terminology table and sentence inventory; `rg` audit; [live first screen](screenshots/polish-1/live-first-screen-390.png). |
| M3 — long, technical README copy | Rewrote setup and extension copy into short direct sentences. Every audited README sentence is at most 15 words. | `.factory/copy-audit.md` README table; clean-clone README link and command review. |
| M4 — “Connect reader” does not name the result | Replaced it with “Create private RSS link”; the submit action is “Create RSS feed link”; recovery uses “Retry loading queue”. | Test `@claim:rss-feed-revocation creates a usable RSS link and revokes it`; [live demo](screenshots/polish-1/live-demo-390.png). |

## Additional acceptance hardening

- Text responses now negotiate gzip/Brotli/Zstd. The live JS returned `Content-Encoding: gzip`, cutting Lighthouse transfer to 50 KiB.
- The obsolete HTTP client dependency was removed, reducing the backend fetch surface.
- Startup logs now distinguish supplied and generated-default database/static configuration without exposing values.
- Extension styles moved to a self-hosted stylesheet so the application CSP does not block them.
- Backend test `private_responses_are_not_cached_and_text_can_be_compressed` proves compression, `no-store`, and HSTS.
- Browser test `extension pages have landmarks, designed focus, and 44px controls` proves the hosted extension surfaces meet the baseline.

## Final live evidence

- Deployment health: `1af906fc8ec595827a111788e7ae0fb848079fb5`.
- `verify-url.sh`: HTTP 200, 539 ms, one h1, `lang=en`, main present, no missing alt text, no unlabeled buttons, zero console/page errors.
- Live axe: zero violations on dark validation state, Privacy, Terms, 404, extension popup, and extension settings.
- Live Lighthouse: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.1 s, TBT 30 ms, CLS 0, 50 KiB transfer, zero console errors.

Every finding in `.factory/review-1.md` is resolved. No known finding remains.
