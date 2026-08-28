# Demo sandbox

- Canonical URL: <https://rss-saved-queue.sociobot.in/demo>
- Query alias: <https://rss-saved-queue.sociobot.in/?demo=1>
- Entry action: **Try it with sample data** on the first screen.
- Reset action: **Reset demo** in the persistent banner.
- Exit action: **Start for real** destroys the demo workspace before opening the real queue.

The demo provisions an in-memory backend workspace with a random 256-bit token.
It expires after 24 hours and cannot access SQLite accounts.
The browser keeps that token only in \`sessionStorage\` under \`demo:rss-saved-queue:workspace\`.
The sample RSS URL uses the separate \`demo:rss-saved-queue:feed\` key.
Demo requests use \`/api/demo/*\` and never send the real device key.

The workspace starts with three realistic links.
They cover next, soon, and later priorities plus queued and read states.
A sample private RSS link is ready on first load.
CSV export, save, state changes, feed creation, and revocation all run inside the workspace.

