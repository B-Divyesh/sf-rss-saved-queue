# Adversarial first-read review 1 — FAIL

Reviewed 2026-08-28 UTC against `https://rss-saved-queue.sociobot.in` from fresh Chromium contexts at 390 × 844 and 1440 × 1000. This was a read-only review of the product. The verdict is **FAIL**: there are five blocking findings.

## First screen, before scrolling

My best interpretation was: this is a private list of web links, possibly for people who use an RSS reader. I could not determine who specifically needs it, and I could not identify a single first click. The visible controls are **“Save a page ↗”**, **“Connect reader”**, **“Export CSV”**, and **“Save your first page”**. Nothing says which action starts the intended workflow or what happens after it.

The text that failed the first-read check was:

> “Make a smaller later.”

> “Save the links worth your attention, give them a place in line, then read them in the feed reader you already use.”

The headline is a metaphor rather than the job, and the paragraph does not name a concrete audience or first action. The reading-room visual system is distinct and legible at 390 px; this failure is about clarity and the missing try-out path, not a generic-template appearance.

## Findings, ordered by severity

### BLOCKING — B1. A cold visitor cannot establish the job, audience, and first click

- **Quote:** “Make a smaller later.”; “A PRIVATE READING ROOM FOR THE INTERNET”; “Save a page ↗”; “Connect reader”.
- **Why this loses or misleads a first-time visitor:** the headline does not say that the product saves links to a private RSS queue. “Reading room,” “shelf,” and “line” are competing metaphors. A visitor cannot tell whether to save a link, connect an existing reader, or export an empty queue. The only sentence mentioning a feed reader assumes the visitor already uses one and does not say who this is for.
- **Concrete fix:** use the ≤9-word job headline **“Save web links in a private RSS queue.”** Use the subhead **“For people who save too many links and read in an RSS reader.”** Put **“Try it with sample data”** first, with **“See three saved articles and their RSS feed.”** beside it. Retain a clearly secondary **“Save your first link”** action.

### BLOCKING — B2. No one-click demo exists, and `?demo=1` is real storage

- **Quote/evidence:** the live first screen has no “Try it with sample data” control. `GET /demo` returned `404`. A fresh `/?demo=1` visit showed no text beginning “Demo,” no Reset control, and wrote `rss-saved-queue:device-key` to localStorage while requesting `/api/session`, `/api/items`, and `/api/feed-tokens`.
- **Why this loses or misleads a first-time visitor:** a visitor must create a real anonymous queue before seeing the product with any content. There is no realistic sample, no “nothing is saved” banner, no reset, and no way to verify that a try-out cannot touch real data.
- **Concrete fix:** implement `/demo` (and make `?demo=1` enter the same mode) with at least three realistic saved articles, queue states, priorities, tags, and a working RSS-preview/export outcome already visible. Use a separate `demo:` browser-storage namespace and an isolated ephemeral backend tenant. Show the persistent banner **“Demo — sample data, nothing is saved”** with **“Reset demo”** and **“Start for real.”** Add `.factory/demo.md` and browser tests proving demo writes cannot reach a real account.

### BLOCKING — B3. Claims are undocumented and therefore unverified

- **Quote/evidence:** `.factory/claims.json` is absent; `rg '@claim:'` found no tagged claim tests. Consequently there were no listed claim commands to run from a clean clone. `npm test` passed 2/2 presentation-helper tests, but it does not establish product claims.
- **Why this loses or misleads a first-time visitor:** the product asks a visitor to rely on privacy, RSS, export, extension, device-key, and caching promises without a claim-to-observable-test record. The missing demo also makes the required clean-sandbox verification impossible.
- **Concrete fix:** add `.factory/claims.json`, one `@claim:<id>` test per claim, and execute each against `/demo` in a fresh context. At minimum cover: saving title/URL/tags without fetching the URL; CSV export; RSS feed creation/revocation; device isolation; and the privacy/no-third-party-request statement. Intercept all requests throughout the demo flow and assert the allowed origins. Remove any statement that cannot be tested.

### BLOCKING — B4. Claim-like landing and README statements are unlisted

- **Quote:** “RSS Saved Queue stores only the link details you provide to run your private queue.”; “Your queue remains private to this device key.”; README: “It stores only the title, URL, tags, and queue state you provide; it never fetches saved pages or imports public feeds.”; “There are no trackers, analytics, external fonts, or third-party scripts.”
- **Why this loses or misleads a first-time visitor:** these are specific privacy and feature promises, not decorative copy. With no claims file, none has an associated sandbox test.
- **Concrete fix:** list each promise in `claims.json` with its exact location and tagged test. For example, test that a demo save performs no outbound request to the saved URL, and test that the complete demo request log stays same-origin. Test CSV content rather than only the existence of **“Export CSV.”**

### BLOCKING — B5. Required routes and route metadata are missing

- **Quote/evidence:** `/demo`, `/robots.txt`, `/sitemap.xml`, and `/favicon.ico` each returned `404`. `/not-a-route` returned an unstyled server `404`, not a designed recovery page. `/`, `/privacy`, and `/terms` all had the identical title **“RSS Saved Queue — private reading, in a smaller line”**; none had canonical, Open Graph, Twitter, or favicon tags. After a Privacy navigation and Back, `document.activeElement` was `BODY` and there was no route announcement (`[aria-live]` count 0 on Privacy).
- **Why this loses or misleads a first-time visitor:** direct demo and discovery links fail. Privacy and Terms do not identify themselves in browser history or shared previews. A keyboard or screen-reader user receives no focus destination or route announcement. The absence of a designed 404 leaves a dead end.
- **Concrete fix:** add `/demo`, designed `/404`, `robots.txt`, and `sitemap.xml`; set per-route titles such as **“Privacy — RSS Saved Queue”** and **“Demo — RSS Saved Queue.”** Add canonical, OG/Twitter image metadata and SVG/apple-touch favicon. Use real navigation or History API navigation that puts focus on the new `<h1>` and announces the change. Add route, back-button, focus, metadata, favicon, and link-crawl tests.

### Major — M1. The landing page does not provide the required explanatory skeleton

- **Quote/evidence:** after the hero it jumps directly into an empty **“YOUR PRIVATE SHELF”** queue. There is no visible “How it works” sequence, no three plain privacy/offline/price facts, and no explanation of what the product does not do.
- **Why this loses or misleads a first-time visitor:** the empty state repeats the metaphor instead of proving the workflow. A visitor cannot evaluate RSS output, privacy boundaries, or whether this is useful before entering real data.
- **Concrete fix:** after the demo preview, add a three-step section: **“Save a link,” “Choose its place,” “Read the private RSS feed.”** Add short factual lines such as **“Stores title, link, tags, and queue state,” “Does not fetch saved pages,”** and an accurate price statement. Keep these statements in the claims registry if they are promises.

### Major — M2. The core copy is metaphor-heavy and terminology changes

- **Quote:** “A PRIVATE READING ROOM FOR THE INTERNET”; “Make a smaller later.”; “YOUR PRIVATE SHELF”; “SHELF CLEAR”; “0 pieces waiting”; README: “reading queue,” “links,” “saved pages,” and “feed links.”
- **Why this loses or misleads a first-time visitor:** “room,” “shelf,” “pieces,” “pages,” “links,” “queue,” and “line” describe the same or overlapping concepts. The job needs no metaphor to be understood. **“Made for deliberate reading.”** is a slogan, not an explanation.
- **Concrete fix:** use **queue** for the collection and **link** for its entries everywhere. Replace headings with **“Your saved links”** and **“No saved links yet.”** Replace the slogan with a factual product line or omit it.

### Major — M3. Long and technical README copy prevents a quick independent setup decision

- **Quote:** “RSS Saved Queue is a private, deliberately small reading queue for people who save links from around the web and want to read them in their own RSS reader.” (29 words); “Chrome then asks you to allow that exact service origin, so the extension can work with this hosted instance or a self-hosted one without gaining automatic access to every site.” (30 words).
- **Why this loses or misleads a first-time visitor:** both exceed the 22-word hard cap. The first repeats the unclear “reading queue” framing; the second forces a reader through extension permission detail before establishing the simple outcome.
- **Concrete fix:** replace the first with **“Save web links in a private queue, then read them in your RSS reader.”** Split the second into **“Chrome asks permission for the service URL you enter. The extension cannot access other sites automatically.”**

### Major — M4. “Connect reader” is not a result-naming primary control

- **Quote:** **“Connect reader.”**
- **Why this loses or misleads a first-time visitor:** it opens a configuration sheet; it does not connect a reader. The visitor does not learn that the result is a private RSS link.
- **Concrete fix:** label the entry action **“Create private RSS link”** and the form action **“Create RSS feed link.”** Keep **“Export CSV”** and **“Save your first link”**, which do name their results. Change **“Try again”** to **“Retry loading queue”** where practical.

## Copy audit

Word counts treat URLs, code paths, and hyphenated terms as one word. The landing inventory is the completed cold root state; controls, headings, and sentence copy are listed separately so fragments are not misrepresented as sentences.

### Landing-page sentences

| Copy | Words | Result |
| --- | ---: | --- |
| “Save the links worth your attention, give them a place in line, then read them in the feed reader you already use.” | 22 | Flag: vague audience and “line” metaphor. Rewrite: “For people who save links and read them in an RSS reader.” |
| “Your next good read starts with a saved page.” | 9 | Flag: marketing adjective and “page” conflicts with “link.” Rewrite: “Saved links will appear here.” |
| “Save a page yourself or use the included extension.” | 10 | Flag: “page” conflicts with “link.” Rewrite: “Save a link here or with the browser extension.” |
| “Your queue remains private to this device key.” | 9 | Flag: unlisted privacy claim and unexplained technical noun. Rewrite: “This queue opens only with this browser’s device key.” Add a claim test. |
| “RSS Saved Queue stores only the link details you provide to run your private queue.” | 15 | Flag: unlisted storage/privacy claim. Rewrite: “It stores the title, link, tags, and queue state you save.” Add a claim test. |
| “Made for deliberate reading.” | 4 | Flag: slogan/marketing adjective; does not explain the product. Rewrite: omit, or “A private RSS queue for saved links.” |

### Landing headings and labels

| Copy | Words | Result |
| --- | ---: | --- |
| “A PRIVATE READING ROOM FOR THE INTERNET” | 7 | Flag: metaphor and heading does not name the job. Rewrite: “Private RSS queue for saved links.” |
| “Make a smaller later.” | 4 | Flag: blocking ambiguous headline. Rewrite: “Save web links in a private RSS queue.” |
| “YOUR PRIVATE SHELF” | 3 | Flag: “shelf” conflicts with queue. Rewrite: “Your saved links.” |
| “SHELF CLEAR” | 2 | Flag: contextual metaphor. Rewrite: “No saved links yet.” |
| “0 pieces waiting” | 3 | Flag: “pieces” conflicts with links/pages/articles. Rewrite: “0 links in queue.” |

### Landing controls

| Control | Result |
| --- | --- |
| “Save a page ↗” / “Save your first page” | Flag terminology. Rewrite: “Save a link.” |
| “Connect reader” | Flag: does not name its result. Rewrite: “Create private RSS link.” |
| “Export CSV” | Passes result-naming-verb check. |
| “In queue”, “Read”, “Archive” | View tabs, not action buttons; use one noun consistently with “queue.” |
| “Try again” | Flag: recovery result is vague. Rewrite: “Retry loading queue.” |

### README sentences

| Copy | Words | Result |
| --- | ---: | --- |
| “RSS Saved Queue is a private, deliberately small reading queue for people who save links from around the web and want to read them in their own RSS reader.” | 29 | Flag: >22, “deliberately small” marketing, unclear queue framing. Proposed rewrite in M3. |
| “It stores only the title, URL, tags, and queue state you provide; it never fetches saved pages or imports public feeds.” | 21 | Flag: unlisted storage/fetch/import claims and link/page inconsistency. Rewrite: “It stores the title, link, tags, and queue state you save.” Add separate tested no-fetch/import claims. |
| “Each browser receives a random device key stored locally.” | 9 | Flag: unlisted technical/privacy claim. Rewrite: “This browser keeps a device key for your queue.” Add a claim test. |
| “The server retains only its SHA-256 hash, so every API request and export is isolated to that device key.” | 19 | Flag: jargon and unlisted isolation claim. Rewrite: “The server stores a one-way hash of that key.” Link to a technical privacy detail and add a test. |
| “Treat it like a password.” | 5 | Clear imperative, but it needs the preceding key explanation. |
| “The app can create long random, revocable RSS links for feed readers; the reader state endpoint is `POST /reader/<feed-token>/items/<id>/read`.” | 18 | Flag: API jargon and unlisted RSS/revocation claim. Rewrite: “Create a private RSS link for your reader. You can revoke it later.” Move endpoint detail to API documentation. |
| “Open the app, save a page, then choose Connect reader to create a private RSS link.” | 16 | Flag: “page” versus “link” and misleading control label. Rewrite: “Save a link, then create a private RSS link for your reader.” |
| “The newly generated link is shown exactly once; paste it into your reader and revoke it from the app whenever needed.” | 21 | Flag: unlisted one-time/revocation claim. Rewrite: “Copy the private RSS link into your reader. You can revoke it later.” Add appropriate tests. |
| “To use the browser extension, load the repository's `extension/` directory as an unpacked Manifest V3 extension.” | 14 | Flag: setup jargon. Rewrite: “Load the `extension/` folder as an unpacked Chrome extension.” Link to browser instructions. |
| “In its options, paste the service URL and device key shown under Connect reader.” | 13 | Flag: vague “its” and outdated control label. Rewrite: “In the extension options, paste the service URL and device key from Create private RSS link.” |
| “Chrome then asks you to allow that exact service origin, so the extension can work with this hosted instance or a self-hosted one without gaining automatic access to every site.” | 30 | Flag: >22 and technical/jargon-heavy. Proposed rewrite in M3. |
| “It saves the active tab's title and URL, plus tags you enter.” | 11 | Flag: unlisted extension-data claim and URL/link inconsistency. Rewrite: “The extension saves the active tab’s title, link, and tags.” Add a claim test. |
| “Requires Node 22+ and Rust.” | 5 | Clear. |
| “Open `http://localhost:8080`.” | 1 | Clear. |
| “The default production database is `/data/rss-saved-queue.db`; mount `/data` when running the container to keep saved queues across replacements.” | 18 | Flag: deployment jargon in quick-start prose. Rewrite: “Mount `/data` in production to keep saved queues after container replacement.” |
| “The root Dockerfile builds the Svelte frontend and Rust service, runs as a non-root user, and listens on `PORT` (default `8080`).” | 21 | Flag: implementation detail, not first-use copy. Move to deployment documentation. |
| “Built hashed assets are served with immutable caching; API and private-feed responses are not stored.” | 15 | Flag: unlisted cache/privacy claim. Rewrite: “API and private-feed responses are not stored in browser caches.” Add header tests. |
| “There are no trackers, analytics, external fonts, or third-party scripts.” | 8 | Flag: unlisted privacy claim. Rewrite: retain only with an intercepted-request claim test. |
| “See `/privacy` and `/terms` in the running app.” | 7 | Clear. |

README headings: **“RSS Saved Queue”** (3), **“Use it”** (2), **“Run locally”** (2), **“Verify”** (1), and **“Privacy and legal”** (3). **“Use it”** and **“Verify”** are weak out of context; use **“Save and read links”** and **“Run checks”**.

## Claims and sandbox record

- `.factory/claims.json`: **missing**. Required listed claim tests: **none available to run**.
- Tagged claim tests: **none found**.
- `npm test`: **PASS**, 2/2, but no claim coverage.
- `?demo=1`: **FAIL** as a demo entry point; it operates the normal app and normal storage namespace.
- Offline/privacy claim exercise: **not verifiable as required**, because no isolated demo exists. The `?demo=1` request capture proves it opens a real session instead of a demo tenant.

## Structure and accessibility checks

- Root, Privacy, and Terms: one `<h1>`, `lang="en"`, `<main>`, metadata description, no console errors on ordinary cold load: **confirmed**.
- Root links crawled: home, Privacy, Terms, and skip anchor: **confirmed working**.
- Distinct visual identity: **confirmed**. The paper/ink ledger treatment is not a generic SaaS template.
- Per-route title pattern, canonical, OG/Twitter, favicon, robots, sitemap, demo route, designed 404, route focus, and route announcement: **failed** as documented in B5.
- Header/footer: Privacy and Terms appear on root and legal routes, but the required Demo navigation and **“Built by Param Factory”**/build identifier footer content are absent.

## Supplemental local checks

| Command | Result |
| --- | --- |
| `npm ci` | PASS |
| `npm test` | PASS — 2/2 |
| `npm run check` | PASS — 0 errors, 0 warnings |
| `npm run build` | PASS — 21.00 kB gzip JS; `dist/` produced |
| `cargo test --locked` | PASS — 6/6 |
| `npx playwright test --reporter=line` | PASS — 8/8, but it has no demo or `@claim:` coverage. |

## Verdict

**FAIL.** The product has five blocking findings: first-read ambiguity, absent demo sandbox, absent claims registry/tests, unlisted reliance claims, and missing required routes/metadata. It does not meet the threshold for PASS.
