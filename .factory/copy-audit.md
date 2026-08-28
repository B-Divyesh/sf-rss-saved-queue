# Copy audit

Generated from `.factory/copy-inventory.json` on 28 August 2026. The checked inventory covers every visitor-facing sentence on the cold landing screen and every prose sentence in `README.md`. Word counts use whitespace-separated words. Hyphenated terms, URLs, and code paths count as one word. No audited sentence exceeds 22 words. No banned marketing word appears.

## Landing page sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Save web links in a private RSS queue. | 8 | Pass |
| For people who save too many links and read in an RSS reader. | 13 | Pass |
| See three saved links and their RSS feed. | 8 | Pass |
| Private: each browser has its own queue. | 7 | device-isolation |
| No page fetching or tracking. | 5 | saved-metadata-no-fetch; same-origin-privacy |
| Free. | 1 | free-access |
| Internet connection required. | 3 | internet-connection |
| Saved links will appear here. | 5 | Pass |
| Save a link here or with the browser extension. | 9 | extension-save |
| Enter a title, link, and optional tags. | 7 | saved-metadata-no-fetch |
| Set each link to next, soon, or later. | 8 | saved-metadata-no-fetch |
| Create a private RSS link for your reader. | 8 | rss-feed-revocation |
| It does not fetch link contents or import public feeds. | 10 | saved-metadata-no-fetch |
| It uses no analytics, ads, external fonts, or third-party scripts. | 10 | same-origin-privacy |
| A private RSS queue for saved links. | 7 | Pass |

## README sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Save web links in a private queue, then read them in your RSS reader. | 14 | Pass |
| It is for people who save too many links and want a smaller reading list. | 15 | Pass |
| The queue is free to use and requires no account or payment. | 12 | free-access |
| Open the sample queue. | 4 | Pass |
| It starts with three links, varied priorities, two queue states, and a working RSS preview. | 15 | demo-isolation; rss-feed-revocation |
| Demo changes stay in a separate memory-only workspace and never touch your real queue. | 14 | demo-isolation |
| Use Reset demo for a fresh sample. | 7 | demo-isolation |
| Use Start for real to discard the demo. | 8 | demo-isolation |
| Save a title, link, and optional tags. | 7 | saved-metadata-no-fetch |
| Set its priority and queue state. | 6 | saved-metadata-no-fetch |
| Create a private RSS link for queued links and revoke it later. | 12 | rss-feed-revocation |
| Export CSV writes one row for every saved link. | 9 | csv-export |
| Import CSV restores exported links and skips duplicates. | 8 | csv-import |
| Each browser keeps a device key for its queue. | 9 | device-isolation |
| The server stores a one-way hash of that key. | 9 | device-isolation |
| A second device key cannot open or change the first queue. | 11 | device-isolation |
| The queue stores entered link metadata. | 6 | saved-metadata-no-fetch |
| It does not fetch link contents or import public feeds. | 10 | saved-metadata-no-fetch |
| Open the extension setup page to download the package and copy your device key. | 14 | extension-save |
| In Chrome’s Extensions page, turn on Developer mode. | 8 | extension-save |
| Choose Load unpacked and select the unzipped folder. | 8 | extension-save |
| Chrome then asks permission for this site’s address. | 8 | extension-save |
| The extension saves the active tab title, link, and entered tags. | 11 | extension-save |
| Install Node 22+ and stable Rust. | 6 | Pass (developer setup) |
| Open http://localhost:8080. | 2 | Pass (developer setup) |
| Mount /data in production. | 4 | Pass (developer setup) |
| Run each listed claim command from a clean checkout. | 9 | Pass |
| Build the root Dockerfile with BUILD_SHA set to the source commit. | 11 | Pass (developer setup) |
| Run it on PORT, which defaults to 8080. | 8 | Pass (developer setup) |
| Mount persistent storage at /data. | 5 | Pass (developer setup) |
| The health endpoint is /health. | 5 | Pass (developer setup) |
| The app uses no analytics, ads, external fonts, or third-party scripts. | 11 | same-origin-privacy |
| Read the live privacy policy and terms. | 7 | Pass |
| MIT licensed. | 2 | Pass |
| See LICENSE. | 2 | Pass |

## Headings and controls

The headline names the job in eight words. The primary action is **Try it with sample data**. Its adjacent sentence explains the result. **Save your first link** is secondary. The collection is always **queue**. Entries are always **links**. RSS output is always a **private RSS link**. The sample environment is always **demo**. The browser credential is always **device key**. Result-naming controls include **Save a link**, **Create private RSS link**, **Create RSS feed link**, **Import CSV**, **Export CSV**, **Reset demo**, **Start for real**, and **Retry loading queue**.

## Terminology table

| Concept | One term |
| --- | --- |
| Collection | queue |
| Saved entry | link |
| Syndication output | private RSS link |
| Browser credential | device key |
| Sample environment | demo |

## Inventory check

Run `npm run audit:copy` to compare this generated audit with the current landing source and README. The command fails if an inventoried sentence is missing, violates the word or banned-word rules, or if this file is stale. `npm test` runs this check before unit tests.
