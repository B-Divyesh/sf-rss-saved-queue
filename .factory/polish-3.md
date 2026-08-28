# Polish round 3 — complete acceptance record

Candidate repaired: `07b05d962b74c926f4740d7d4b826922073ecc8b`.
Review baseline: `007518f8353559868304e37b96b27b168d611f65`.
Deployed application commit: `8d96723406022a10046d535cd8f2eb0a89a4f18f`.

The evidence for every row includes the clean-clone 30/30 browser suite, the
independent claim sweep, and the cold production check at
<https://rss-saved-queue.sociobot.in>. Production screenshots are
`/work/.evidence/rss-saved-queue-polish-3/live-root-390.png`,
`live-demo-390.png`, and `live-extension-390.png`.

| Finding id | Change made | Evidence |
| --- | --- | --- |
| B1 | Preserved the plain job headline, audience, first demo action, result sentence, and three facts; the audit now guards that wording. | Live `/` cold check: headline, audience, and primary action above 390 px fold; `landing copy inventory matches the rendered cold landing page`. |
| B2 | Preserved the one-click `/demo` and `?demo=1` isolated workspace, persistent banner, reset, exit, sample data, RSS, and mobile-first sample row. | `@claim:demo-isolation`; live `/?demo=1` has only `demo:` session keys, no real device key, same-origin requests, banner/reset/exit, and above-fold sample. |
| B3 | Preserved the ten one-to-one claim records/tests and made the new copy evidence part of `npm test`. | Every exact registry command passed independently from `/tmp/rss-polish3-clean.iAo9Ja/repo`; full browser suite 30/30. |
| B4 | Preserved listed tests for every public privacy, data, RSS, extension, import, free-access, and connectivity promise. | `.factory/claims.json`; ten independent tagged claim passes; `same-origin-privacy` and `device-isolation` assertions. |
| B5 | Preserved real routes, route titles/metadata, canonical tags, focus/announcements, 404, discovery assets, legal links, and consistent footer. | `real routes set titles, canonical metadata, focus, announcements, and history`; live `/privacy`, `/terms`, `/extension-setup` 200 and `/not-a-route` designed 404. |
| M1 | Preserved the explanatory first-screen facts, queue preview, three steps, and privacy boundary section. | `landing copy inventory matches the rendered cold landing page`; live root screenshot. |
| M2 | Preserved queue/link terminology and added a source-validated terminology table. | `npm run audit:copy`; `.factory/copy-audit.md`; live root copy check. |
| M3 | Preserved concise README setup copy and corrected its Markdown code fences. | `npm run audit:copy`; every audited README sentence is at most 15 words. |
| M4 | Preserved result-naming RSS actions. | `@claim:rss-feed-revocation`; live demo. |
| F-2-1 | Preserved compact mobile demo layout with a complete realistic sample link above the fold. | `mobile demo shows a complete sample link without scrolling`; live demo screenshot. |
| F-2-2 | Preserved self-hosted sample destinations. | Live checks: all three `/samples/*.html` routes return 200; `sample destinations and extension package are available from the live product surface`. |
| F-2-3 | Preserved cold-server allowance and per-run database isolation. | Independent clean-clone claims: first cold command passed in 1m 22s. |
| F-2-4 | Preserved claims for connection, memory-only demo, key hash, extension, and CSV import behavior. | Ten registry entries and independently passing tagged tests. |
| F-2-5 | Replaced stale/contradictory terminology proof with generated inventory and DOM coverage. | `npm run audit:copy`; `landing copy inventory matches the rendered cold landing page`; live root has “saved links.” |
| F-2-6 | Preserved live extension setup/download/device-key path and rewrote it with visitor-facing Chrome labels. | `@claim:extension-save`; live `/extension-setup` screenshot and `/extension.zip` 200. |
| F-2-7 | Preserved specific offline recovery messages. | `@claim:internet-connection mutations explain that a connection is required`. |
| F-2-8 | Preserved visible theme-action labels. | `mobile controls and links meet touch sizing without horizontal overflow`; live root check. |
| F-2-9 | Preserved CSV preview, import, restored state/priority, and duplicate handling. | `@claim:csv-import imports an exported CSV with state and skips duplicates`. |
| F-3-1 | Added `.factory/copy-inventory.json`, deterministic generator/check, regenerated `.factory/copy-audit.md`, and DOM test; `npm test` runs it. | `npm run audit:copy` passed in the clean clone; inventory now records the 13-word audience sentence, import copy, and current README text. |
| F-3-2 | Rewrote README, setup page, and extension controls to say **Developer mode**, **Load unpacked**, and **this site’s address**. | `extension setup uses Chrome’s visible install label and this site’s address`; live `/extension-setup` confirms both instructions and a populated address/key. |

## Live recheck result

`verify-url.sh` passed cold with no console errors. The explicit live browser
recheck found root 200, demo 200, legal/extension routes 200, styled 404, one
main landmark per route, zero serious/critical axe findings, no external demo
requests, and all discovery/sample/package links returning 200. The live health
response reports the deployed application commit above.
