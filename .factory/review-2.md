# Adversarial first-read review 2 — FAIL

Reviewed 2026-08-28 UTC against <https://rss-saved-queue.sociobot.in> at deployed build
`1b792bcc7972fbf1a38636a9a7536d71621436dd`. Fresh Chromium contexts used 390 × 844 and
1440 × 1000 viewports. Product code was not modified.

The verdict is **FAIL**. There are five blocking findings, two major findings, and two minor
findings. Four blocking findings reopen or regress findings from review 1. There are also
unlisted claims, so this round cannot say every public claim was tested.

`.factory/brief.json` is absent from the repository. Scope and missed-leverage judgments therefore
use the live product, README, claims registry, design thesis, and prior review history.

## First screen, before scrolling

Both cold first screens answer the required questions:

- **What does it do?** It saves web links in a private queue that can be read as RSS. Evidence:
  **“Save web links in a private RSS queue.”**
- **For whom?** People who save too many links and use an RSS reader. Evidence:
  **“For people who save too many links and read in an RSS reader.”**
- **What should I click first?** **“Try it with sample data”**, followed by
  **“See three saved articles and their RSS feed.”**

The primary action is visible without scrolling at 390 px and desktop. The competing header action
**“Save a link”** is visually secondary. This closes review-1 B1.

## Findings, ordered by severity

### BLOCKING — F-2-1 / review-1 B2 reopened: the mobile demo does not show sample data in its first screen

- **Quote/location:** 390 × 844, immediately after the one allowed click: **“Explore a sample
  private RSS queue.”**, **“2 links in queue”**, and **“Sample RSS is ready.”** The viewport ends
  there. Neither sample title nor an article row is visible.
- **Evidence:** the first actual title, **“A field guide to calmer web typography”**, is below the
  fold. The banner, Reset, Start for real, two queued items, one read item, RSS preview, CSV, and
  isolated workspace otherwise work.
- **Why this fails a first-time visitor:** the result screen describes a populated product but does
  not visibly show realistic sample data. The demo rule requires the first post-click screen to
  already look like the product in use. This is a half-fix of B2, whose required outcome was
  explicitly “already visible.”
- **Concrete fix:** compact or remove the repeated demo intro on phones and place the first complete
  sample row above the 844 px fold. Add a 390 × 844 assertion that a sample title and its priority,
  tags, and state are visible without scrolling.

### BLOCKING — F-2-2: the realistic demo contains two dead article links

- **Quote/location:** `/demo`: **“A field guide to calmer web typography”** links to
  `https://example.com/library/web-typography`; **“Why a shorter reading list improves recall”**
  links to `https://example.com/notes/reading-and-recall`.
- **Evidence:** a fresh GET returned `404` for both destinations. The current crawler checks only
  root-page internal links, so it reports green without crawling the demo links.
- **Why this fails a first-time visitor:** the samples look realistic until clicked, then reveal
  placeholder data. This weakens the mandatory demo and violates the no-dead-links rule.
- **Concrete fix:** seed reachable, stable articles or self-hosted sample article routes. Extend the
  crawler across every route, including demo-generated links, and assert an expected successful
  status for every navigable destination.

### BLOCKING — F-2-3 / review-1 B3 reopened: a registered claim command fails from a cold clean clone

- **Quote/location:** `.factory/claims.json`, `@claim:demo-isolation` command:
  `npm run test:browser -- --grep @claim:demo-isolation`.
- **Evidence:** in clean clone `/tmp/rss-review2.KY4nQb/repo` at the deployed commit, the command
  failed with **“Timed out waiting 90000ms from config.webServer.”** while Cargo performed its cold
  dependency build. After compilation completed, the same claim passed in 5.0 seconds. The other
  seven claim commands passed. `playwright.config.ts` also reuses the fixed path
  `/tmp/rss-saved-queue-browser.db`, rather than a per-run temporary database.
- **Why this fails verification:** the registered command is the product contract. A warm rerun does
  not erase a cold-clone failure, and a shared database path does not guarantee a clean sandbox.
- **Concrete fix:** prebuild the Rust service before Playwright starts or raise the web-server timeout
  enough for a cold build. Give each run a unique temporary SQLite path and clean it up. Prove all
  eight exact registry commands in a new clone with empty Cargo and npm caches.

### BLOCKING — F-2-4 / review-1 B4 reopened: public claims remain outside the claims registry

- **Exact quotes/locations:** landing: **“Internet connection required.”** README:
  **“Demo changes stay in a separate memory-only workspace”**, **“The server stores a one-way hash
  of that key”**, and **“Mount `/data` in production to keep queues after container replacement.”**
  README also says **“Every public product promise appears in `.factory/claims.json`.”**
- **Evidence:** no claim entry declares or tests the online-only behavior or restart persistence.
  `demo-isolation` declares an ephemeral workspace, but not the stronger “memory-only” promise.
  `device-isolation` happens to inspect hashes, but the registry claim itself never declares the
  one-way-hash promise.
- **Why this misleads a visitor/operator:** the registry cannot be used as the promised complete list
  of reliance statements. Incidentally asserting an undeclared property does not make the registry
  complete.
- **Concrete fix:** add explicit entries and observable tests for each retained promise. Exercise
  online-only behavior with `context.setOffline(true)`, prove demo storage never enters SQLite, make
  hashing part of the declared device claim, and test persistence across service restart. Otherwise
  remove or narrow the sentences. Remove the “every promise” sentence until the cross-check passes.

### BLOCKING — F-2-5 / review-1 M2 regressed: the saved-entry term changes from “link” to “article”

- **Quote/location:** landing action explanation: **“See three saved articles and their RSS feed.”**
  Every other collection entry is a **“link.”** `.factory/copy-audit.md` incorrectly says
  **“Entries are always links.”**
- **Why this fails the history rule:** review-1 M2 required one term for one concept. The regression
  is small in isolation, but the work order requires any regressed earlier finding to become blocking
  again under the same prior ID.
- **Concrete fix:** use **“See three saved links and their RSS feed.”** Update the copy audit only
  after an automated terminology scan confirms the claim.

### MAJOR — F-2-6: the promoted browser-extension path is not usable from the live product

- **Quote/location:** empty state: **“Save a link here or with the browser extension.”** Connection
  sheet: **“Load `extension/`, then add these values.”**
- **Evidence:** in a fresh real browser, opening **“Create private RSS link”** shows an empty Device
  key. The extension box contains no link. Live `/extension` and `/extension/` return `404`; only
  individual implementation files such as `/extension/manifest.json` return `200`.
- **Why this loses a first-time visitor:** the product promotes an extension but provides neither an
  install/download route nor the credential needed by its own setup instructions. Repository-aware
  developers can reconstruct the process; a live visitor cannot.
- **Concrete fix:** link a plain installation page and downloadable extension package from the empty
  state and connection sheet. Ensure opening setup creates and displays a non-empty device key.
  Add a fresh-browser test that follows the live instructions, downloads the package, configures it,
  and saves the active tab.

### MAJOR — F-2-7: offline actions expose a raw network error without recovery guidance

- **Quote/location:** `/demo` save form after the page was loaded, the browser was put offline, and
  **“Save link”** was pressed: **“Failed to fetch”.**
- **Why this fails the error-copy rule:** the message does not say that the connection was lost, that
  the link was not saved, or what to do next. The landing page explicitly says Internet is required,
  so this is a foreseeable state.
- **Concrete fix:** show **“This link was not saved because you are offline. Reconnect, then save it
  again.”** Map network exceptions for save, state changes, RSS actions, Reset, and export. Add an
  offline-after-load browser test for each mutation class.

### MINOR — F-2-8: the visible theme button is a cryptic symbol, not a result-naming label

- **Quote/location:** header button visible text: **“◐”** in light mode and **“☀”** in dark mode.
  The accessible names **“Use dark theme”** and **“Use light theme”** are present, but sighted visitors
  receive only the symbols.
- **Why this slows a first-time visitor:** the half-circle is not a self-evident outcome and has no
  visible word or tooltip.
- **Concrete fix:** visibly label it **“Use dark theme”** / **“Use light theme”**, or add a persistent
  adjacent label that changes with the action.

### MINOR — F-2-9: CSV export has no corresponding import path

- **Location:** queue toolbar and README provide **“Export CSV”** only.
- **Why this is missed leverage:** people described as already having too many saved links must add
  them one at a time or install the extension. Importing a prior export is the obvious recovery and
  migration counterpart. AI would add no necessary value here; provider keys or decorative AI would
  be inappropriate.
- **Concrete fix:** add **“Import CSV”** with a preview, per-row validation, duplicate handling, and
  an explicit confirmation. Accept the exported columns without fetching any link contents. In demo
  mode, import only into the ephemeral workspace. Register and test the claim.

## Copy audit

Counts use whitespace-separated words; hyphenated terms, code paths, and URLs count as one word.
No landing or README sentence exceeds 22 words. No banned marketing adjective appears.

### Landing-page sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Save web links in a private RSS queue. | 8 | Pass. |
| For people who save too many links and read in an RSS reader. | 13 | Pass. |
| See three saved articles and their RSS feed. | 8 | **Flag F-2-5:** “articles” conflicts with “links.” Rewrite: “See three saved links and their RSS feed.” |
| Private: each browser has its own queue. | 7 | Covered by `device-isolation`. |
| No page fetching or tracking. | 5 | Covered by `saved-metadata-no-fetch` and `same-origin-privacy`. |
| Free. | 1 | Covered by `free-access`. |
| Internet connection required. | 3 | **Flag F-2-4:** unlisted claim. Add an offline test or remove it. |
| Saved links will appear here. | 5 | Pass. |
| Save a link here or with the browser extension. | 9 | **Flag F-2-6:** promoted path is not actionable from the live site. |
| Save a link. | 3 | Pass. |
| Enter a title, link, and optional tags. | 8 | Pass. |
| Choose its place. | 3 | Pass. |
| Set each link to next, soon, or later. | 8 | Pass. |
| Read the private RSS feed. | 5 | Pass. |
| Create a private RSS link for your reader. | 8 | Pass. |
| It does not fetch link contents or import public feeds. | 10 | Covered by `saved-metadata-no-fetch`. |
| It uses no analytics, ads, external fonts, or third-party scripts. | 10 | Covered by `same-origin-privacy`. |
| A private RSS queue for saved links. | 7 | Pass. |

### README sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Save web links in a private queue, then read them in your RSS reader. | 14 | Pass. |
| It is for people who save too many links and want a smaller reading list. | 15 | Pass. |
| The queue is free to use and requires no account or payment. | 12 | Covered by `free-access`. |
| Open the sample queue. | 4 | Pass. |
| It starts with three links, varied priorities, two queue states, and a working RSS preview. | 15 | Covered by the demo behavior test. |
| Demo changes stay in a separate memory-only workspace and never touch your real queue. | 14 | **Flag F-2-4:** “memory-only” is stronger than the registered claim. |
| Use Reset demo for a fresh sample. | 7 | Covered by `demo-isolation`. |
| Use Start for real to discard the demo. | 8 | Covered by `demo-isolation`. |
| Save a title, link, and optional tags. | 7 | Covered by `saved-metadata-no-fetch`. |
| Set its priority and queue state. | 6 | Covered by `saved-metadata-no-fetch`. |
| Create a private RSS link for queued links and revoke it later. | 12 | Covered by `rss-feed-revocation`. |
| Export CSV writes one row for every saved link. | 9 | Covered by `csv-export`. |
| Each browser keeps a device key for its queue. | 9 | Covered by `device-isolation`; “device key” is defined by the next sentence. |
| The server stores a one-way hash of that key. | 9 | **Flag F-2-4:** technical privacy promise is absent from the claim text. Rewrite: “The server cannot recover the original key from its stored copy.” |
| A second device key cannot open or change the first queue. | 11 | Covered by `device-isolation`. |
| The queue stores entered link metadata. | 6 | Covered by `saved-metadata-no-fetch`. |
| It does not fetch link contents or import public feeds. | 10 | Covered by `saved-metadata-no-fetch`. |
| Load the `extension/` folder as an unpacked Chrome extension. | 9 | **Flag F-2-6:** jargon-heavy and no live download/install destination. |
| Chrome asks permission for the service URL you enter. | 9 | Covered by `extension-save`, but “service URL” needs the installation guide proposed in F-2-6. |
| The extension saves the active tab title, link, and entered tags. | 11 | Covered by `extension-save`. |
| Install Node 22+ and stable Rust. | 6 | Pass for developer setup. |
| Open `http://localhost:8080`. | 2 | Pass. |
| Mount `/data` in production to keep queues after container replacement. | 10 | **Flag F-2-4:** unlisted persistence claim. |
| Every public product promise appears in `.factory/claims.json`. | 7 | **Flag F-2-4:** contradicted by this audit. |
| Run each listed command from a clean checkout. | 8 | **Flag F-2-3:** the first exact command failed cold. |
| Build the root `Dockerfile` with `BUILD_SHA` set to the source commit. | 11 | Pass for deployment setup. |
| Run it on `PORT`, which defaults to `8080`. | 8 | Pass for deployment setup. |
| Mount persistent storage at `/data`. | 5 | Duplicates the unlisted persistence statement above. |
| The health endpoint is `/health`. | 5 | Pass for deployment setup; live response is 200. |
| The app uses no analytics, ads, external fonts, or third-party scripts. | 11 | Covered by `same-origin-privacy`. |
| Read the live privacy policy and terms. | 7 | Both links return 200. |
| MIT licensed. | 2 | Confirmed by `LICENSE`. |
| See LICENSE. | 2 | The repository link resolves. |

### Headings and controls

The headline is eight words and names the job. **“Your saved links,” “How the queue works,”** and
**“What the queue does not do”** make sense out of context. Eyebrows such as **“THREE STEPS”** and
**“CLEAR BOUNDARIES”** are supporting labels, not heading elements. No heading defect was found.

Result-naming actions pass for **“Try it with sample data,” “Save a link,” “Save your first link,”
“Create private RSS link,” “Export CSV,” “Reset demo,” “Start for real,”** and
**“Retry loading queue.”** The queue-view buttons **“In queue,” “Read,”** and **“Archived”** are tabs,
not commands. The visible theme-symbol control is the sole control-copy failure and is F-2-8.

## Demo and sandbox evidence

- One click from `/` opens `/demo` and shows the persistent required banner.
- The initial workspace has three records: two queued and one read, covering next/soon/later.
- A ready sample RSS URL returns two items. CSV and RSS claim tests pass.
- Demo requests use `/api/demo/*`, an `X-Demo-Workspace` token, and `demo:` session-storage keys.
- A sentinel `rss-saved-queue:device-key` remained unchanged. No demo mutation sent the real
  Authorization key.
- Archiving changed the visible queue from two items to one. Reset created a new workspace, restored
  two queued items and one read item, and made the old workspace return `401`.
- The backend stores demo workspaces in an in-memory `HashMap`. This source observation does not cure
  the missing registry declaration in F-2-4.
- With the network disabled after load, save exposed **“Failed to fetch”** (F-2-7).

Isolation and Reset therefore pass. First-screen proof and realistic destinations fail.

## Claims record

Clean clone: `/tmp/rss-review2.KY4nQb/repo` at
`1b792bcc7972fbf1a38636a9a7536d71621436dd`. Each exact registry command was invoked separately.

| Claim | Cold command result | Evidence |
| --- | --- | --- |
| `demo-isolation` | **FAIL** | Web server exceeded 90 seconds during cold Cargo build. Warm rerun passed 1/1 in 5.0 s. |
| `saved-metadata-no-fetch` | PASS | 1/1; unreachable URL stored without a fetch and removed import route stayed 404. |
| `csv-export` | PASS | 1/1; header plus three data rows and representative fields asserted. |
| `rss-feed-revocation` | PASS | 1/1; RSS content, item count, dates, no-store, and revoked 404 asserted. |
| `device-isolation` | PASS | 1/1; second key could neither list nor mutate the first queue; hash checked. |
| `same-origin-privacy` | PASS | 1/1; complete demo request origins remained local product origin. |
| `extension-save` | PASS | 1/1; fixture popup/settings flow asserted fields, permission, key, and payload. |
| `free-access` | PASS | 1/1; core demo flow produced no login, checkout, billing, or payment request. |

The full warm suite later passed 24/24. That establishes behavior after compilation; it does not turn
the cold registry command into a pass. F-2-4 lists claim-like sentences that remain untested as
declared claims.

## Earlier-finding audit

| Prior ID | Live and code confirmation | Status this round |
| --- | --- | --- |
| B1 | Exact job headline, audience sentence, first demo action, and adjacent result are above the fold at both sizes. | Fixed. |
| B2 | `/demo`, banner, separate namespace, Reset, exit, samples, RSS, and CSV exist; no sample row is visible in the first mobile screen. | **Half-fixed; reopened as F-2-1/B2.** |
| B3 | Eight registry entries and eight tags exist; one exact command fails from a cold clone and the DB path is shared. | **Half-fixed; reopened as F-2-3/B3.** |
| B4 | Main product/privacy outcomes are registered, but four stronger/public sentences remain outside the registry. | **Half-fixed; reopened as F-2-4/B4.** |
| B5 | Demo/legal/404 routes, per-route metadata, discovery assets, focus, announcements, history, header/footer, and build ID work. | Fixed. |
| M1 | Three facts, live queue, three-step explanation, and explicit boundaries are present. | Fixed. |
| M2 | Room/shelf/pieces metaphors are gone, but “articles” reappears for saved “links.” | **Regressed; reopened as F-2-5/M2.** |
| M3 | Every audited README sentence is at most 15 words; headings are specific. | Fixed. |
| M4 | Controls now say “Create private RSS link,” “Create RSS feed link,” and “Retry loading queue.” | Fixed. |

## Structure, accessibility, and visual identity

- Root title is **“RSS Saved Queue — save links to a private RSS queue”** (51 characters).
  Demo, Privacy, Terms, and 404 use route-specific titles.
- Every checked route has `lang="en"`, one `<h1>`, one `<main>`, a description, canonical URL,
  route-updated Open Graph/Twitter copy, and the 1200 × 630 product artwork.
- SVG/ICO favicons, 180 px apple-touch icon, robots, sitemap, and static fallback configuration return
  200. The unknown route returns the designed queue-style 404 with HTTP 404.
- Push navigation and browser Back focus the destination `<h1>` and update the polite live region.
- Header/footer content is consistent; Privacy and Terms are present. Internal route links return 200.
  Demo article links fail as F-2-2.
- `/opt/fleet/lib/verify-url.sh` passed in 555 ms with no console/page errors.
- Live 390 px dark-scheme axe scans found zero serious/critical violations on root, Demo, Privacy,
  Terms, and 404. No visible link, button, input, or select measured below 44 × 44 px.
- The paper ledger, editorial type, ruled layout, coral bookmark action, and serial priorities are
  distinct and match `.factory/design.md`; this is not a generic SaaS template.

## Supplemental quality gates

After the cold claim attempt completed compilation, the same clean clone produced:

| Command | Result |
| --- | --- |
| `npm test` | PASS — 2/2 |
| `npm run check` | PASS — 0 errors, 0 warnings |
| `npm run build` | PASS — `dist/`; JS 61.60 kB / 23.06 kB gzip; CSS 12.35 kB / 3.40 kB gzip |
| `cargo test --locked` | PASS — 9/9 |
| `npm run test:browser -- --reporter=line` | PASS — 24/24 warm |

## What would make this perfect

Show a complete sample row in the first 390 px demo viewport and replace both placeholder destinations
with working links. Make every exact claim command pass from a genuinely cold clone and unique test
database. Register or remove every stronger privacy, connectivity, and persistence promise. Restore
**link** as the only saved-entry term. Provide a linked extension download/install flow with a
non-empty key. Replace raw network exceptions with actionable offline messages, label the theme
action visibly, and add a sandboxed CSV import with round-trip tests. Re-run the complete checklist;
only zero remaining findings and zero untested claims qualifies for PASS.

## Verdict

**FAIL.** Five blocking, two major, and two minor findings remain. The product is substantially clearer
and more complete than review 1, but it does not meet the required zero-finding standard.
