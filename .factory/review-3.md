# Adversarial first-read review 3 — FAIL

Reviewed 2026-08-28 UTC against <https://rss-saved-queue.sociobot.in> from fresh Chromium contexts at 390 × 844 and 1440 × 1000. Product code was not modified. The verdict is **FAIL**: two minor copy/documentation findings remain. There are no blocking findings and no untested or failing registered claims.

`.factory/brief.json` is absent. Scope was therefore checked against the live product, README, claims registry, visual thesis, and complete earlier-review history.

## First screen, before scrolling

At both sizes, a cold visitor can answer all three required questions.

- **What does it do?** Save web links in a private RSS queue.
- **For whom?** People who save too many links and read in an RSS reader.
- **What should I click first?** **Try it with sample data**. The adjacent text says **“See three saved links and their RSS feed.”**

At 390 px, the primary action, result explanation, secondary real action, and all three facts are visible without scrolling. This confirms review-1 B1 remains fixed.

## Findings, ordered by severity

### MINOR — F-3-1 / F-2-5 repair evidence is stale: the required copy audit contradicts the live product

- **Quote/location:** `.factory/copy-audit.md` calls itself a complete current audit, yet lists **“See three saved articles and their RSS feed.”** and says it has **8** words. The live landing page and `src/App.svelte` say **“See three saved links and their RSS feed.”** The audit also counts **“For people who save too many links and read in an RSS reader.”** as 12 words; it has 13. It includes the removed README sentence **“Mount `/data` in production to keep queues after container replacement.”** and omits current README copy including **“Import CSV restores exported links and skips duplicates.”**
- **Why this matters:** the prior terminology finding was closed partly on the basis of this audit. A stale, self-contradictory proof cannot detect a reintroduction of the exact `article`/`link` regression it claims to guard against. It also fails the required complete word-count audit, so reviewers and maintainers cannot reliably check plain-language compliance.
- **Concrete fix:** regenerate `.factory/copy-audit.md` from the current landing DOM and README. Record the actual 13-word audience sentence, current import and extension text, and remove historical copy. Add a small test or script that fails when the source inventory and audit inventory differ.

### MINOR — F-3-2: the README extension instructions use unexplained configuration jargon

- **Quote/location:** README, extension paragraph: **“Load the unzipped folder as an unpacked Chrome extension.”** and **“Chrome asks permission for the service URL you enter.”**
- **Why a reader can be lost:** this is the first installation guidance in the README, but “unpacked” and “service URL” are Chrome/developer terms rather than the labels and objects the reader sees. A visitor who follows the README may not know that Chrome’s visible action is **Load unpacked**, or that the value is this site’s address.
- **Concrete fix:** replace the two lines with: **“In Chrome’s Extensions page, turn on Developer mode. Choose Load unpacked and select the unzipped folder. Chrome then asks permission for this site’s address.”** Keep the setup-page link immediately before it.

## Copy audit

Word counts use whitespace-separated words; URLs and hyphenated words count as one. No current landing or README sentence exceeds 22 words. No banned marketing adjective appears. The only current-copy flag is F-3-2.

### Landing page

| Sentence | Words | Result |
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
| Read the private RSS feed. | 5 | `rss-feed-revocation` |
| Create a private RSS link for your reader. | 8 | `rss-feed-revocation` |
| It does not fetch link contents or import public feeds. | 10 | `saved-metadata-no-fetch` |
| It uses no analytics, ads, external fonts, or third-party scripts. | 10 | `same-origin-privacy` |
| A private RSS queue for saved links. | 7 | Pass |

Headings are understandable out of context: **Your saved links**, **How the queue works**, and **What the queue does not do**. Visible actions name results: **Try it with sample data**, **Save a link**, **Create private RSS link**, **Import CSV**, **Export CSV**, **Reset demo**, and **Start for real**. Queue-state controls are tabs, not ambiguous commands. The visible **Use dark theme** action is labelled.

### README

| Sentence | Words | Result |
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
| Load the unzipped folder as an unpacked Chrome extension. | 9 | **F-3-2** |
| Chrome asks permission for the service URL you enter. | 9 | **F-3-2** |
| The extension saves the active tab title, link, and entered tags. | 11 | `extension-save` |
| Install Node 22+ and stable Rust. | 6 | Pass (developer setup) |
| Open `http://localhost:8080`. | 2 | Pass (developer setup) |
| Mount `/data` in production. | 4 | Pass (developer setup) |
| Run each listed claim command from a clean checkout. | 8 | Pass |
| Build the root `Dockerfile` with `BUILD_SHA` set to the source commit. | 11 | Pass (developer setup) |
| Run it on `PORT`, which defaults to `8080`. | 8 | Pass (developer setup) |
| Mount persistent storage at `/data`. | 5 | Pass (developer setup) |
| The health endpoint is `/health`. | 5 | Pass (developer setup) |
| The app uses no analytics, ads, external fonts, or third-party scripts. | 11 | `same-origin-privacy` |
| Read the live privacy policy and terms. | 7 | Pass |
| MIT licensed. | 2 | Pass |
| See LICENSE. | 2 | Pass |

## Demo and sandbox verification

- The first landing action opens `/demo` in one click. At 390 px the first screen contains the persistent **“Demo — sample data, nothing is saved”** banner, Reset demo, Start for real, queue controls, and the first realistic link title at y=729.
- Demo starts with three links: two queued and one read, covering next/soon/later. Its ready RSS preview returns two queued items with `Cache-Control: no-store`.
- A fresh context stored only `demo:rss-saved-queue:workspace` and `demo:rss-saved-queue:feed` in session storage. It requested only `/api/demo/*`; no real Authorization header or real device key was used.
- Reset restored the seed state; Start for real removed the demo keys. Source confirmation: demo state is an in-memory `HashMap`, separate from SQLite-backed real routes.
- The offline claim test intercepts an offline save and verifies the recovery message. The privacy claim records the complete demo load, RSS creation, and CSV export and asserts same-origin traffic only.

## Claims record

Clean clone used: `/tmp/rss-review3-clean.1AmoLL`. Each registry command was invoked against that clone; the all-browser confirmation passed **28/28**, including every tagged claim below.

| Claim id | Result | Observable evidence asserted |
| --- | --- | --- |
| demo-isolation | PASS | Separate demo namespace, reset, exit, destroyed workspace, untouched real key |
| saved-metadata-no-fetch | PASS | Saved metadata/state, no saved-URL fetch, no import route |
| csv-export | PASS | Header and three data rows |
| rss-feed-revocation | PASS | Usable RSS, two items, valid dates, revoke becomes 404 |
| device-isolation | PASS | Cross-device denial and hashed, not raw, keys |
| same-origin-privacy | PASS | Full demo flow remains same-origin |
| extension-save | PASS | Package/settings flow and active-tab title/link/tags payload |
| free-access | PASS | Save, RSS, and CSV without account or payment traffic |
| internet-connection | PASS | Offline save is not sent and gives reconnect guidance |
| csv-import | PASS | Preview, state restoration, and duplicate skip |

No claim-like landing or README sentence lacks a corresponding registry entry. There is no AI feature, and none is expected: importing/exporting and the private RSS workflow cover the useful implied leverage without adding decorative model use.

## Structure, accessibility, and link crawl

- `/`, `/demo`, `/?demo=1`, `/privacy`, `/terms`, `/extension-setup`, `/extension/`, discovery files, icons, social image, all three sample pages, and the downloadable extension returned 200. `/not-a-route` returned the designed recovery page with 404.
- Each application route has one h1, main, route-specific title, description, canonical, Open Graph/Twitter metadata, favicon declarations, and a consistent header/footer with Privacy and Terms. Back navigation moves focus to the h1 and updates the polite route announcement.
- The crawl fetched all demo item destinations in both queue/read tabs and the dynamic RSS URL; all returned 200. The dynamic feed is not cached.
- Fresh live page load had no console errors or third-party requests. Headers include a self-only CSP, HSTS, nosniff, same-origin referrer policy, and permissions policy.
- Dark-mode axe scans at 390 px found zero violations, including zero serious/critical findings, on `/`, `/demo`, `/privacy`, `/terms`, `/extension-setup`, and the 404 page. `verify-url.sh` passed: 200, title, `lang=en`, one h1, main, no missing alt text, no unlabeled buttons, and no console errors.
- The quiet ledger layout, paper/ink/coral palette, editorial titles, serial priority labels, and ruled sections match `.factory/design.md` and are distinct from a generic SaaS template.

## Earlier-finding audit

| Earlier finding | Current live and code check | Status |
| --- | --- | --- |
| review-1 B1 | Plain job headline, audience, primary demo action, adjacent result, and facts above the mobile fold | Fixed |
| review-1 B2 / F-2-1 | Isolated demo, banner, Reset, Start for real, sample queue, RSS, and visible first mobile sample | Fixed |
| review-1 B3 / F-2-3 | Ten registered, tagged claim tests; clean-clone browser suite passes | Fixed |
| review-1 B4 / F-2-4 | Current landing/README promises map to registry tests | Fixed |
| review-1 B5 | Deep links, metadata, focus, announcements, 404, robots, sitemap, and icons work | Fixed |
| review-1 M1 | Required facts, live queue, three steps, and boundaries are present | Fixed |
| review-1 M2 / F-2-5 | Live/source terminology consistently uses link; required audit record is stale (F-3-1) | **Documentation evidence incomplete** |
| review-1 M3 | Current README sentences are ≤22 words | Fixed |
| review-1 M4 | RSS actions name the RSS-link result | Fixed |
| F-2-2 | Three self-hosted sample destinations return 200 | Fixed |
| F-2-6 | Extension setup page and ZIP download are linked and usable | Fixed |
| F-2-7 | Offline save gives explicit reconnect guidance | Fixed |
| F-2-8 | Theme control has visible action text | Fixed |
| F-2-9 | CSV import previews, restores state, and skips duplicates | Fixed |

## Quality gates

From the clean clone: `npm test` passed 2/2; `npm run check` passed with zero diagnostics; `npm run build` produced `dist/` (66.87 kB JS, 24.56 kB gzip; 13.71 kB CSS, 3.65 kB gzip); `npm run test:browser` passed 28/28; `cargo test --locked` passed 9/9; strict Clippy and formatting checks passed.

## What would make this perfect

Regenerate the repository copy audit from current source/DOM and make it checkable, then replace the two README configuration terms with the visitor-facing Chrome labels and plain description above. Re-run the full cold-first-read and claim checklist after those documentation changes.

## Verdict

**FAIL.** The product itself is clear, tryable, isolated, route-complete, accessible, and claim-tested. However, the stale mandatory copy-audit proof and the README installation jargon leave two minor findings. This work order requires zero findings for PASS.
