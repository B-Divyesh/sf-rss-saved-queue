# Adversarial first-read review 4 — PASS

Reviewed 2026-08-28 UTC against <https://rss-saved-queue.sociobot.in> from new Chromium contexts at 390 × 844 and 1440 × 950. The repository began at `b5b0f3c58bc991fb0287eee7385879fa6d175391`. Product source was not changed.

**Verdict: PASS.** No blocking, major, minor, unlisted-claim, routing, copy, demo, accessibility, or history-regression finding remained.

## First screen, before scrolling

At both viewport sizes, the first screen answers all three required questions.

- **What it does:** It saves web links in a private queue and provides an RSS feed for that queue. The exact headline is “Save web links in a private RSS queue.”
- **For whom:** It is for people who save too many links and use an RSS reader. The exact audience sentence is “For people who save too many links and read in an RSS reader.”
- **What to click first:** Click **“Try it with sample data.”** The adjacent result text is “See three saved links and their RSS feed.” The real-data alternative, **“Save your first link,”** is visibly secondary.

The four hero elements are visible at 390 px: the eight-word job headline, 13-word audience sentence, primary sample action/result, and the three plain facts. No ambiguous metaphor or competing primary action was observed.

## Copy audit

Counts treat hyphenated terms, URLs, and code paths as one word. The checked inventory in `.factory/copy-inventory.json` is generated into `.factory/copy-audit.md`; `npm run audit:copy` passed and the browser inventory test confirms each cold-landing sentence renders. No sentence exceeds 22 words. No banned marketing term, unexplained configuration term, or inconsistent entry name was found.

### Landing-page sentences

| Sentence | Words | Check |
| --- | ---: | --- |
| Save web links in a private RSS queue. | 8 | Pass |
| For people who save too many links and read in an RSS reader. | 13 | Pass |
| See three saved links and their RSS feed. | 8 | Pass |
| Private: each browser has its own queue. | 7 | `device-isolation` |
| No page fetching or tracking. | 5 | `saved-metadata-no-fetch`; `same-origin-privacy` |
| Free. | 1 | `free-access` |
| Internet connection required. | 3 | `internet-connection` |
| Saved links will appear here. | 5 | Pass |
| Save a link here or with the browser extension. | 9 | `extension-save` |
| Save a link. | 3 | Pass |
| Enter a title, link, and optional tags. | 7 | `saved-metadata-no-fetch` |
| Choose its place. | 3 | Pass |
| Set each link to next, soon, or later. | 8 | `saved-metadata-no-fetch` |
| Read the private RSS feed. | 5 | Pass |
| Create a private RSS link for your reader. | 8 | `rss-feed-revocation` |
| It does not fetch link contents or import public feeds. | 10 | `saved-metadata-no-fetch` |
| It uses no analytics, ads, external fonts, or third-party scripts. | 10 | `same-origin-privacy` |
| A private RSS queue for saved links. | 7 | Pass |

### README sentences

| Sentence | Words | Check |
| --- | ---: | --- |
| Save web links in a private queue, then read them in your RSS reader. | 14 | Pass |
| It is for people who save too many links and want a smaller reading list. | 15 | Pass |
| The queue is free to use and requires no account or payment. | 12 | `free-access` |
| Open the sample queue. | 4 | Pass |
| It starts with three links, varied priorities, two queue states, and a working RSS preview. | 15 | `demo-isolation`; `rss-feed-revocation` |
| Demo changes stay in a separate memory-only workspace and never touch your real queue. | 14 | `demo-isolation` |
| Use Reset demo for a fresh sample. | 7 | `demo-isolation` |
| Use Start for real to discard the demo. | 8 | `demo-isolation` |
| Save a title, link, and optional tags. | 7 | `saved-metadata-no-fetch` |
| Set its priority and queue state. | 6 | `saved-metadata-no-fetch` |
| Create a private RSS link for queued links and revoke it later. | 12 | `rss-feed-revocation` |
| Export CSV writes one row for every saved link. | 9 | `csv-export` |
| Import CSV restores exported links and skips duplicates. | 8 | `csv-import` |
| Each browser keeps a device key for its queue. | 9 | `device-isolation` |
| The server stores a one-way hash of that key. | 9 | `device-isolation` |
| A second device key cannot open or change the first queue. | 11 | `device-isolation` |
| The queue stores entered link metadata. | 6 | `saved-metadata-no-fetch` |
| It does not fetch link contents or import public feeds. | 10 | `saved-metadata-no-fetch` |
| Open the extension setup page to download the package and copy your device key. | 14 | `extension-save` |
| In Chrome’s Extensions page, turn on Developer mode. | 8 | `extension-save` |
| Choose Load unpacked and select the unzipped folder. | 8 | `extension-save` |
| Chrome then asks permission for this site’s address. | 8 | `extension-save` |
| The extension saves the active tab title, link, and entered tags. | 11 | `extension-save` |
| Install Node 22+ and stable Rust. | 6 | Developer setup |
| Open http://localhost:8080. | 2 | Developer setup |
| Mount /data in production. | 4 | Developer setup |
| Run each listed claim command from a clean checkout. | 9 | Verified below |
| Build the root Dockerfile with BUILD_SHA set to the source commit. | 11 | Developer setup |
| Run it on PORT, which defaults to 8080. | 8 | Developer setup |
| Mount persistent storage at /data. | 5 | Developer setup |
| The health endpoint is /health. | 5 | Developer setup |
| The app uses no analytics, ads, external fonts, or third-party scripts. | 11 | `same-origin-privacy` |
| Read the live privacy policy and terms. | 7 | Both routes returned 200 |
| MIT licensed. | 2 | `LICENSE` present |
| See LICENSE. | 2 | Local repository link |

Headings are specific out of context: “Your saved links,” “How the queue works,” and “What the queue does not do.” The application consistently calls a saved entry a **link**, its collection a **queue**, its output a **private RSS link**, and its sample environment **demo**. Visible controls name their result: **Try it with sample data**, **Save a link**, **Save your first link**, **Create private RSS link**, **Create RSS feed link**, **Import CSV**, **Export CSV**, **Reset demo**, **Start for real**, and **Retry loading queue**. The queue-status controls are view tabs, not verbs.

## Demo and sandbox

The root action opened `/demo` in one click. At 390 px, the first post-click viewport already contained the complete realistic sample row “A field guide to calmer web typography,” its next priority, tags, state, and self-hosted article URL. The demo banner was persistent and exact: **“Demo — sample data, nothing is saved.”**

`Reset demo` issued demo-only reset calls, displayed “Demo reset to its three sample links,” and restored two queued links plus one read link. A fresh demo used only `demo:rss-saved-queue:workspace` and `demo:rss-saved-queue:feed` in session storage; the only local-storage entry was the visual theme. It sent only `/api/demo/*` requests, did not write or send the supplied real device-key sentinel, and `Start for real` destroyed the demo workspace before returning to `/`. Code confirmation: `src/App.svelte` uses the `demo:` keys and separate `/api/demo` target; `.factory/demo.md` records the in-memory backend workspace and reset/exit semantics.

The sample article links and sample RSS link returned 200. The sample is opinionated rather than placeholder text: it has distinct titles, tags, next/soon/later priorities, queued/read states, CSV export, and two RSS items.

## Claims

`.factory/claims.json` contains ten entries and exactly one `@claim:<id>` test per entry. I cloned the repository to a new temporary directory, ran `npm ci`, and invoked every listed command separately. The initial `demo-isolation` command completed after its cold Cargo dependency build; each command passed.

| Claim id | Result | Observable evidence exercised |
| --- | --- | --- |
| `demo-isolation` | Pass | Separate demo storage/API, real-key sentinel unchanged, Reset and exit destroy workspace |
| `saved-metadata-no-fetch` | Pass | Saves unreachable URL as metadata; no fetch; state and priority persist |
| `csv-export` | Pass | Header plus one row for each of three sample links |
| `rss-feed-revocation` | Pass | Private RSS works, has two queued items, then returns 404 after revoke |
| `device-isolation` | Pass | Second device cannot read/change first queue; raw keys absent and hashes present |
| `same-origin-privacy` | Pass | Complete demo flow made only same-origin requests |
| `extension-save` | Pass | Download/settings and active-tab title, link, tags flow exercised |
| `free-access` | Pass | Save, RSS, and CSV work without login, checkout, billing, or payment request |
| `internet-connection` | Pass | Offline save remains unsent and gives reconnect guidance |
| `csv-import` | Pass | Preview/import restores state; duplicate re-import is skipped |

The landing and README claim-like sentences in the copy table map to one of these entries. Deployment instructions and local-file facts were checked directly; no visitor-facing product claim lacked a registry entry. No claim test failed or remained untested.

## Structure, routing, accessibility, and identity

Direct visits to `/`, `/demo`, `/privacy`, `/terms`, `/extension-setup`, and an unknown route worked as intended. The unknown route returned HTTP 404 with the designed “This page is not in the queue.” recovery screen. Every checked application route had one `<h1>`, one `<main>`, `lang="en"`, a concise route-specific title, description, canonical URL, Open Graph/Twitter metadata, and favicon declarations. The root title is 51 characters: “RSS Saved Queue — save links to a private RSS queue.” Legal and demo routes follow the specified “Route — Product” pattern.

The complete crawl of links rendered across root, demo, Privacy, Terms, extension setup, and 404 returned 200 where applicable: product routes, self-hosted sample articles, the generated demo RSS URL, and the extension ZIP. `robots.txt`, `sitemap.xml`, SVG/ICO/apple favicons, and the 1200 × 630 social image also returned 200. The header/footer are consistent and include Privacy and Terms.

Privacy navigation and Back both put focus on the new route's `<h1>` and updated the polite route announcement. The supplied `verify-url.sh` completed successfully: HTTP 200, 586 ms navigation, no console/page errors, title/lang/one h1/main present, no missing image alt text, and no unnamed button. Axe found zero serious or critical violations on live demo in light and dark theme. The mobile check found no horizontal overflow or undersized interactive target. The quiet reading-room ledger identity is distinct: warm-paper and ink palette, editorial type, ruled queue rows, coral bookmark accent, and no generic SaaS hero/card/gradient treatment.

## Earlier-finding audit

Every prior finding was rechecked live and against its implementation/tests.

| Earlier finding | Recheck result |
| --- | --- |
| Review-1 B1 | Fixed: job, audience, first click, and result are above the fold. |
| Review-1 B2 | Fixed: one-click isolated demo, persistent banner, realistic above-fold mobile sample, Reset, exit, and no real storage. |
| Review-1 B3 | Fixed: claims registry exists; all ten exact clean-clone commands passed. |
| Review-1 B4 | Fixed: visitor-facing privacy, data, RSS, extension, free, connectivity, and import claims are registered and tested. |
| Review-1 B5 | Fixed: deep routes, metadata, discovery files, designed 404, focus/announcement, and legal skeleton all work. |
| Review-1 M1–M4 | Fixed: explanation/privacy structure, consistent terms, concise README, and result-naming RSS controls remain present. |
| Review-2 F-2-1–F-2-9 | Fixed: mobile demo content, self-hosted samples, cold claim setup, claim coverage, terminology, extension setup/download, offline recovery, visible theme label, and CSV import all verified. |
| Review-3 F-3-1 | Fixed: generated copy inventory/audit is current and checked by `npm test`. |
| Review-3 F-3-2 | Fixed: README and setup use Chrome’s visible “Developer mode,” “Load unpacked,” and “this site’s address” labels. |

## Missed leverage

No omission was found. CSV import/export covers the expected migration and backup path; the private RSS link covers reading in a preferred reader; the extension covers saving the current tab. An AI feature would not improve the stated queue-to-RSS job enough to justify transmitting saved-link data, a key, cost, and a new privacy surface. No AI provider key, decorative AI copy, or provider endpoint is present.

## What would make this perfect

Keep the generated copy inventory, independent claim commands, and the 390 px demo-row assertion in the release gate. No product change is requested from this review.
