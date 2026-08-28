# RSS Saved Queue visual thesis

**Direction:** a quiet reading-room ledger. The interface treats saved articles as
physical slips on a shelf: a crisp serial number, publication line, generous bookish
title, and a deliberately small set of handling controls. This serves the product’s
purpose of making an overwhelming stream feel finite and chosen.

**Palette:** warm paper `#f5f0e7`, ink `#25231f`, library card coral `#a83222`,
bookmark ochre `#e2b94f`, and binding-cloth sage `#29483d`. Dark mode is charcoal
`#1d201e` with warm ink. These are tokens in `src/app.css`; the darker coral and
sage preserve 4.5:1-or-better contrast for errors and small ledger labels.
The ochre save sheet is deliberately a light card in both treatments, so its
own ink `#25231f`, muted copy `#3f3b34`, and sage label `#29483d` are scoped in
`src/contrast.css`; it never inherits low-contrast dark-paper tokens.

**Type:** self-hosted-free system typography: the platform UI sans makes controls
plain and quick, while Georgia provides a familiar editorial voice for reading titles.
No font, third-party script, tracking pixel, or external image is requested.

**Layout and interaction grammar:** a broad masthead leads from a plain job statement
into the live queue. The primary coral action opens an isolated sample; a quieter
underlined action opens the real save form. Priority remains an explicit 01/02/03
selector, and the private RSS bridge remains a separate connection sheet. Explanatory
steps use ruled ledger columns instead of generic cards. On a phone, every queue row
becomes one column and every action remains at least 44 px.

**Motion:** a feed form may rise 6 px on entry over 180 ms; list state changes use
opacity only. Reduced-motion users receive effectively instant state changes. No
looping, flashing, or decorative image is used.

**Asset provenance:** all artwork is original and repository-owned. `favicon.svg`
and `og-card.svg` were hand-authored for this product from its ledger rules, coral
bookmark mark, and typography. Their PNG/ICO derivatives were rendered locally with
Chromium/ImageMagick on 28 August 2026. No generated imagery, stock art, external
font, or third-party visual asset is shipped.
