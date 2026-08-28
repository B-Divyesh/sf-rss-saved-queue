# Verification handoff — FAIL

**Verdict: FAIL — do not release `770122766ee02b191147dd74d6f55572c9000353`.**

Fresh independent QA on 2026-08-28 verified that
<https://rss-saved-queue.sociobot.in> serves the candidate executable source:
live `/health` build `e271f860a7c038b2` equals the candidate Dockerfile source
digest. The private queue, account isolation, revocable feed token, reader-read
endpoint, persistence, cache/security headers, responsive/keyboard flow, and
clean local gates work. `npm ci`, unit tests, Svelte check, Vite build,
Playwright (5), Rust tests (3), fmt, strict clippy, and locked release build
all passed. Lighthouse mobile scored 97 performance and 100 accessibility.

Release is blocked by a live axe serious violation: the dark-theme save sheet
renders text at 1.12:1–1.44:1 against its ochre background. This fails the
non-negotiable contrast and zero-serious/critical-axe gates. Further defects:
RSS emits non-conformant RFC 3339 `pubDate`; the extension advertises arbitrary
self-host endpoints while its manifest permits only the hosted origin; public
session creation is unrate-limited; and a 13th tag is silently discarded.

See `.factory/verification-2.md` for exact commands, deployment proof, full
evidence, defects by severity, and remediation. Docker/Podman was unavailable,
so the exact container-image build was not exercised locally; the locked
release binary was built and run.
