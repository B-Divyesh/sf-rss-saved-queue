# RSS Saved Queue visual thesis

**Direction:** a quiet reading-room ledger. The interface treats saved articles as
physical slips on a shelf: a crisp serial number, publication line, generous bookish
title, and a deliberately small set of handling controls. This serves the product’s
purpose of making an overwhelming stream feel finite and chosen.

**Palette:** warm paper `#f5f0e7`, ink `#25231f`, library card coral `#a83222`,
bookmark ochre `#e2b94f`, and binding-cloth sage `#29483d`. Dark mode is charcoal
`#1d201e` with warm ink. These are tokens in `src/app.css`; the darker coral and
sage preserve 4.5:1-or-better contrast for errors and small ledger labels.

**Type:** self-hosted-free system typography: the platform UI sans makes controls
plain and quick, while Georgia provides a familiar editorial voice for reading titles.
No font, third-party script, tracking pixel, or external image is requested.

**Layout and interaction grammar:** a broad masthead leads to a single private
reading queue. Saving opens from the action that caused it; priority is an explicit
01/02/03 selector; the private-reader bridge is a distinct, quiet connection sheet.
A list row has compact, labelled-by-context handling controls. On a phone the row
becomes a two-column shelf label and story, while controls move under the story
rather than shrink below usable size.

**Motion:** a feed form may rise 6 px on entry over 180 ms; list state changes use
opacity only. Reduced-motion users receive effectively instant state changes. No
looping, flashing, or decorative image is used.

**Asset provenance:** no raster or generated assets are shipped. The hand-authored
wordmark and text glyphs are product UI, not external artwork.
