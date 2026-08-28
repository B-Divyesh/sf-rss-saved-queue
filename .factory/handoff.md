# Perfection loop round 1 handoff — PASS

## Scope

Repaired released candidate \`41085519b2b806c7ac568857f9f00325d8fc16e7\` against every blocking and major finding in \`.factory/review-1.md\`.

Executable repair commit: \`c5ed65d9d36a43cf3d5e25b3eecd0eea99300dad\`.

## Review acceptance

- B1: the first screen now says **“Save web links in a private RSS queue.”**
  It names the audience, leads with **Try it with sample data**, explains the click, and keeps **Save your first link** secondary.
- B2: \`/demo\` and \`/?demo=1\` provision random, in-memory demo tenants.
  They seed three links, three priorities, two queue states, and a working RSS preview.
  The persistent banner includes **Reset demo** and **Start for real**.
  Demo requests use only \`/api/demo/*\`; real SQLite accounts are unreachable.
- B3/B4: \`.factory/claims.json\` registers eight public claims.
  Each claim has one tagged test and an exact command.
  CSV content, RSS creation/revocation, no-fetch behavior, key hashing/isolation, same-origin privacy, extension fields, free access, and demo isolation are observable.
- B5: real routes now cover \`/demo\`, \`/privacy\`, \`/terms\`, and styled 404 recovery.
  Titles, descriptions, canonical, Open Graph, Twitter, favicon, apple-touch icon, robots, sitemap, and social art are present.
  History navigation focuses the new \`h1\` and updates a polite route announcement.
- M1–M4: the landing page includes three facts, the live queue, three workflow steps, and clear privacy boundaries.
  Queue/link terminology is consistent.
  README sentences were shortened.
  RSS controls now name their result.
- Supplemental product defects: private feeds use \`no-store\`; HSTS is present.
  All visible controls and links meet 44 px touch sizing, including extension pages.
  Rate limits use the first \`X-Forwarded-For\` hop and return \`Retry-After\`.

The quiet reading-room ledger identity remains intact.
The responsive design uses the same paper, ink, coral, ochre, and sage palette.
The social preview and icons are original, hand-authored derivatives of that system.

## Clean-clone claim evidence

A new clone at \`/tmp/rss-claims-0cmS8C\` ran every \`test\` command from \`.factory/claims.json\`.
All eight commands passed independently:

- \`@claim:demo-isolation\`
- \`@claim:saved-metadata-no-fetch\`
- \`@claim:csv-export\`
- \`@claim:rss-feed-revocation\`
- \`@claim:device-isolation\`
- \`@claim:same-origin-privacy\`
- \`@claim:extension-save\`
- \`@claim:free-access\`

## Full clean-clone verification

A separate clean clone at \`/tmp/rss-full-cp5T3b\` passed:

\`\`\`text
npm ci                                             PASS — 86 packages; 0 vulnerabilities
npm test                                           PASS — 2/2
npm run check                                      PASS — 0 errors; 0 warnings
npm run build                                      PASS — JS 61.60 kB / 23.06 kB gzip
                                                       CSS 12.35 kB / 3.40 kB gzip
cargo fmt --check                                  PASS
cargo test --locked                                PASS — 8/8
cargo clippy --locked --all-targets --all-features -- -D warnings
                                                   PASS
cargo build --release --locked                     PASS
npm run test:browser -- --reporter=line            PASS — 23/23
git status --short                                 PASS — clean
\`\`\`

Browser coverage includes light and dark themes, invalid forms, demo reset, direct \`?demo=1\`, CSV, RSS revocation, route history, focus, announcements, links, 404, privacy, offline recovery, and 390 px layout.
Playwright axe reported zero serious or critical issues on demo, populated, error, privacy, terms, and 404 states.

## Runtime and performance evidence

The release binary started with only \`PORT=4181\`.
It created default storage, logged configuration origin without secrets, and returned \`{"status":"ok","build":"dev"}\`.

Local \`verify-url.sh\` result:

\`\`\`text
HTTP 200; 519 ms
title present; lang=en; one h1; main present
0 missing image alternatives; 0 unlabeled buttons; 0 console/page errors
\`\`\`

Local Lighthouse mobile:

\`\`\`text
Performance 99; Accessibility 100; Best Practices 100; SEO 100
LCP 1.5 s; TBT 90 ms; CLS 0; transfer 98 KiB
\`\`\`

A 50-request forwarded-IP burst returned 43 authorization responses and 7 rate-limit responses.
The final \`429\` included \`Retry-After: 1\`.
A sample private RSS response returned \`Cache-Control: no-store\`.
HTML and RSS responses include CSP, HSTS, nosniff, same-origin referrer, framing, and permissions headers.

Docker and Podman were unavailable locally.
The factory ACR build completed successfully, which exercised the root multi-stage Dockerfile.

## Deployment evidence

Factory deployment command:

\`\`\`sh
/opt/fleet/lib/deploy-container.sh rss-saved-queue /work/repo Dockerfile 8080
\`\`\`

Result:

\`\`\`text
ACR run chg3: succeeded after 5m12s
Image: sociobotregistry.azurecr.io/sf-rss-saved-queue:c5ed65d9d36a
Container app: sf-rss-saved-queue
HTTPS: https://rss-saved-queue.sociobot.in → 200
Health build: c5ed65d9d36a43cf3d5e25b3eecd0eea99300dad
Designed unknown route: 404
\`\`\`

Live \`verify-url.sh\` passed in 540 ms with zero console errors.
The live 390 px demo had no overflow or undersized controls.
Its request log was same-origin only, and it created no real device key.
Live axe found zero serious or critical violations.
Live Lighthouse scored 100 performance, 100 accessibility, 100 best practices, and 100 SEO.
Live LCP was 1.4 s, TBT 50 ms, CLS 0, and total transfer 97 KiB.

## Run and verify

\`\`\`sh
npm ci
npm run build
DATABASE_URL='sqlite://rss-saved-queue.db?mode=rwc' STATIC_DIR=dist cargo run
npm run test:browser
\`\`\`

Demo documentation is in \`.factory/demo.md\`.
Copy evidence is in \`.factory/copy-audit.md\`.
Claim commands are in \`.factory/claims.json\`.

## Known gaps

No blocking or product defect is known.
