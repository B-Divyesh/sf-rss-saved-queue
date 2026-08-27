<script lang="ts">
  import { onMount } from 'svelte';
  import type { FeedToken, Item } from './lib';
  import { dateLabel, priorityLabel, sessionKey } from './lib';

  let items: Item[] = [];
  let tokens: FeedToken[] = [];
  let status: Item['status'] = 'queue';
  let query = '';
  let title = '';
  let pageUrl = '';
  let tags = '';
  let tokenLabel = 'My feed reader';
  let deviceKey = '';
  let createdFeedUrl = '';
  let loading = true;
  let saving = false;
  let creatingToken = false;
  let notice = '';
  let error = '';
  let undo: Item | null = null;
  let showSave = false;
  let showConnect = false;
  let dark = localStorage.getItem('rss-saved-queue:theme') === 'dark';
  let legalPage: 'privacy'|'terms'|null = location.pathname === '/privacy' ? 'privacy' : location.pathname === '/terms' ? 'terms' : null;

  $: visible = items.filter((item) => item.status === status && `${item.title} ${item.url} ${item.tags.join(' ')}`.toLowerCase().includes(query.trim().toLowerCase()));
  $: counts = { queue: items.filter(x => x.status === 'queue').length, read: items.filter(x => x.status === 'read').length, archived: items.filter(x => x.status === 'archived').length };
  $: document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  $: localStorage.setItem('rss-saved-queue:theme', dark ? 'dark' : 'light');
  $: extensionEndpoint = location.origin;

  onMount(async () => { if (!legalPage) { await ensureSession(); await load(); } });
  async function ensureSession() {
    deviceKey = localStorage.getItem(sessionKey) || '';
    if (!deviceKey) {
      const response = await fetch('/api/session', { method: 'POST' });
      if (!response.ok) throw new Error('We could not prepare your private queue. Try again.');
      deviceKey = (await response.json()).token;
      localStorage.setItem(sessionKey, deviceKey);
    }
  }
  async function request(path: string, options: RequestInit = {}) {
    const headers = new Headers(options.headers);
    headers.set('Authorization', `Bearer ${deviceKey}`);
    if (options.body) headers.set('Content-Type', 'application/json');
    const response = await fetch(path, { ...options, headers });
    if (!response.ok) { const body = await response.json().catch(() => null); throw new Error(body?.error || 'Something went wrong.'); }
    return response.status === 204 ? null : response.json();
  }
  async function load() {
    loading = true; error = '';
    try { items = await request('/api/items'); tokens = await request('/api/feed-tokens'); }
    catch (e) { error = e instanceof Error ? e.message : 'The queue is unavailable.'; }
    finally { loading = false; }
  }
  async function savePage() {
    saving = true; error = ''; notice = '';
    try {
      const item = await request('/api/items', { method:'POST', body:JSON.stringify({ title, url: pageUrl, tags: tags.split(',') }) });
      items = [item, ...items]; title = ''; pageUrl = ''; tags = ''; showSave = false; status = 'queue'; notice = 'Saved to your private queue.';
    } catch (e) { error = e instanceof Error ? e.message : 'Could not save this page.'; }
    finally { saving = false; }
  }
  async function update(item: Item, patch: Partial<Pick<Item,'status'|'priority'>>) {
    error = ''; const previous = {...item}; items = items.map(x => x.id === item.id ? {...x, ...patch} : x);
    try { await request(`/api/items/${item.id}`, {method:'PATCH', body:JSON.stringify(patch)}); }
    catch (e) { items = items.map(x => x.id === item.id ? previous : x); error = e instanceof Error ? e.message : 'Could not update item.'; }
  }
  async function archive(item: Item) { undo = {...item}; await update(item, {status:'archived'}); notice = `Archived “${item.title}”.`; }
  async function undoArchive() { if (!undo) return; await update(undo, {status:undo.status}); notice = 'Archive undone.'; undo = null; }
  async function deleteItem(item: Item) { if (!confirm(`Remove “${item.title}” from your queue?`)) return; const previous = [...items]; items = items.filter(x => x.id !== item.id); try { await request(`/api/items/${item.id}`, {method:'DELETE'}); notice = 'Item removed.'; } catch(e) { items = previous; error = e instanceof Error ? e.message : 'Could not remove item.'; } }
  async function createToken() { creatingToken = true; error = ''; createdFeedUrl = ''; try { const result = await request('/api/feed-tokens', { method:'POST', body: JSON.stringify({label: tokenLabel}) }); createdFeedUrl = `${location.origin}${result.feed_path}`; tokens = await request('/api/feed-tokens'); notice = 'Private feed link created. Copy it now; it is shown only once.'; } catch (e) { error = e instanceof Error ? e.message : 'Could not create a private feed link.'; } finally { creatingToken = false; } }
  async function revokeToken(token: FeedToken) { if (!confirm(`Revoke the “${token.label}” feed link? Readers using it will stop working.`)) return; try { await request(`/api/feed-tokens/${token.id}/revoke`, {method:'POST'}); tokens = await request('/api/feed-tokens'); notice = 'Private feed link revoked.'; } catch (e) { error = e instanceof Error ? e.message : 'Could not revoke that link.'; } }
  async function exportCsv() {
    try {
      const response = await fetch('/api/export.csv', { headers: { Authorization: `Bearer ${deviceKey}` } });
      if (!response.ok) throw new Error('Could not export your queue.');
      const objectUrl = URL.createObjectURL(await response.blob()); const link = document.createElement('a');
      link.href = objectUrl; link.download = 'reading-queue.csv'; link.click(); URL.revokeObjectURL(objectUrl);
    } catch (e) { error = e instanceof Error ? e.message : 'Could not export your queue.'; }
  }
</script>

<svelte:head><title>RSS Saved Queue — private reading, in a smaller line</title></svelte:head>
<a class="skip-link" href="#main">Skip to queue</a>
<header class="masthead">
  <a class="wordmark" href="/" aria-label="RSS Saved Queue home"><span aria-hidden="true">↯</span> SAVED<br />QUEUE</a>
  <div class="top-actions">
    <button class="icon-button" aria-label={dark ? 'Use light theme' : 'Use dark theme'} onclick={() => dark = !dark}>{dark ? '☀' : '◐'}</button>
    {#if !legalPage}<button class="button button-ink" onclick={() => showSave = !showSave} aria-expanded={showSave}>Save a page <span aria-hidden="true">↗</span></button>{/if}
  </div>
</header>

{#if showSave}
  <section class="feed-sheet" aria-labelledby="save-title">
    <div><p class="eyebrow">INBOX DOOR</p><h2 id="save-title">Save what matters</h2><p>Only the title, address, and tags you enter are kept. We never fetch or copy the page.</p></div>
    <form onsubmit={(event) => { event.preventDefault(); savePage(); }}>
      <label for="page-title">Page title</label><input id="page-title" bind:value={title} required maxlength="300" placeholder="A good read for later" />
      <label for="page-url">Page address</label><input id="page-url" type="url" bind:value={pageUrl} required placeholder="https://example.com/article" />
      <label for="page-tags">Tags <span class="optional">optional, comma separated</span></label><div class="feed-form"><input id="page-tags" bind:value={tags} placeholder="design, long read" /><button class="button button-coral" disabled={saving}>{saving ? 'Saving…' : 'Save page'}</button></div>
    </form>
  </section>
{/if}

<main id="main" tabindex="-1">
{#if legalPage}
  <article class="legal">
    <p class="eyebrow">THE SHORT VERSION</p>
    {#if legalPage === 'privacy'}
      <h1 id="page-title">Your reading<br /><em>stays yours.</em></h1>
      <h2>Privacy</h2><p>RSS Saved Queue keeps only the title, web address, tags, and queue state that you save. It does not fetch the pages you save, import feeds, run advertising trackers, or send analytics.</p><p>Your browser stores a random device key so it can open your private queue. The server stores only a one-way hash of that key. Do not share the key or a private feed URL. Feed URLs are revocable and are stored as hashes, too.</p><p>To remove saved data, remove its entries and revoke any feed links. This policy is effective 27 August 2026.</p>
    {:else}
      <h1 id="page-title">A calmer<br /><em>reading line.</em></h1>
      <h2>Terms of use</h2><p>RSS Saved Queue is a personal reading utility. You are responsible for links and tags you save and for safeguarding your private device key and feed links. Do not use it for sensitive information.</p><p>The service is provided as-is. We may limit requests that threaten reliability or security. These terms are effective 27 August 2026.</p>
    {/if}
    <a class="button button-ink" href="/">Return to your queue</a>
  </article>
{:else}
  <section class="intro" aria-labelledby="page-title">
    <p class="eyebrow">A PRIVATE READING ROOM FOR THE INTERNET</p>
    <h1 id="page-title">Make a smaller<br /><em>later.</em></h1>
    <p class="lede">Save the links worth your attention, give them a place in line, then read them in the feed reader you already use.</p>
  </section>
  <section class="queue-panel" aria-label="Saved reading queue">
    <div class="queue-toolbar">
      <div><p class="eyebrow">YOUR PRIVATE SHELF</p><h2>{counts.queue} {counts.queue === 1 ? 'piece' : 'pieces'} waiting</h2></div>
      <div class="toolbar-actions"><button class="text-button" onclick={() => showConnect = !showConnect} aria-expanded={showConnect}>Connect reader</button><button class="text-button" onclick={exportCsv}>Export CSV</button></div>
    </div>
    {#if showConnect}
      <section class="connect-sheet" aria-labelledby="connect-title"><div><p class="eyebrow">PRIVATE BRIDGE</p><h3 id="connect-title">Send your queue to a reader</h3><p>Create a revocable RSS link for a feed reader, or configure the included browser extension with this device’s key. Keep both private.</p></div><form onsubmit={(event) => { event.preventDefault(); createToken(); }}><label for="token-label">Reader name</label><div class="feed-form"><input id="token-label" bind:value={tokenLabel} maxlength="80" required /><button class="button button-coral" disabled={creatingToken}>{creatingToken ? 'Creating…' : 'Create feed link'}</button></div>{#if createdFeedUrl}<label for="feed-link">New private RSS link — shown once</label><input id="feed-link" class="secret" readonly value={createdFeedUrl} aria-describedby="feed-link-help" /><p id="feed-link-help">Copy this link into your reader now. You can revoke it below.</p>{/if}</form><div class="extension-box"><h4>Browser extension</h4><p>Load <code>extension/</code> as an unpacked extension, then set:</p><label for="extension-endpoint">Service URL</label><input id="extension-endpoint" readonly value={extensionEndpoint} /><label for="device-key">Device key</label><input id="device-key" class="secret" readonly value={deviceKey} /><p>The extension sends only the active page’s title, URL, and tags you enter.</p></div><ul class="token-list" aria-label="Private feed links">{#each tokens as token}<li><span><strong>{token.label}</strong><small>{token.revoked_at ? 'Revoked' : 'Active'}</small></span>{#if !token.revoked_at}<button class="text-button danger-text" onclick={() => revokeToken(token)}>Revoke</button>{/if}</li>{/each}</ul></section>
    {/if}
    <nav class="tabs" aria-label="Queue views">
      {#each [['queue','In queue',counts.queue], ['read','Read',counts.read], ['archived','Archive',counts.archived]] as tab}
        <button class:active={status === tab[0]} onclick={() => status = tab[0] as Item['status']} aria-current={status === tab[0] ? 'page' : undefined}>{tab[1]} <span>{tab[2]}</span></button>
      {/each}
    </nav>
    <label class="search"><span class="sr-only">Search saved articles</span><span aria-hidden="true">⌕</span><input bind:value={query} placeholder="Search this shelf" type="search" /></label>
    <div class="live" aria-live="polite">{error || notice}</div>
    {#if undo}<div class="undo"><span>Moved to archive.</span><button onclick={undoArchive}>Undo</button></div>{/if}
    {#if loading}<div class="state"><p class="eyebrow">OPENING THE SHELF</p><h3>Loading your private queue…</h3></div>
    {:else if error && items.length === 0}<div class="state"><p class="eyebrow">CONNECTION PAUSED</p><h3>We couldn’t open your queue.</h3><button class="button button-ink" onclick={load}>Try again</button></div>
    {:else if visible.length === 0}<div class="state empty"><p class="eyebrow">{query ? 'NO MATCHES' : 'SHELF CLEAR'}</p><h3>{query ? 'Nothing on this shelf matches that search.' : status === 'queue' ? 'Your next good read starts with a saved page.' : `Nothing is ${status} yet.`}</h3><p>{query ? 'Try a title, tag, or a shorter phrase.' : status === 'queue' ? 'Save a page yourself or use the included extension. Your queue remains private to this device key.' : 'Move articles here from your queue when you are done with them.'}</p>{#if status === 'queue' && !query}<button class="button button-coral" onclick={() => showSave = true}>Save your first page</button>{/if}</div>
    {:else}<ol class="article-list">{#each visible as item (item.id)}<li class:done={item.status !== 'queue'}><article><div class="priority"><label for={`priority-${item.id}`} class="sr-only">Priority for {item.title}</label><select id={`priority-${item.id}`} value={item.priority} onchange={(event) => update(item, {priority:(event.currentTarget as HTMLSelectElement).value as Item['priority']})} aria-label={`Priority: ${priorityLabel(item.priority)}`}><option value="next">01</option><option value="soon">02</option><option value="later">03</option></select><span>{priorityLabel(item.priority)}</span></div><div class="article-copy"><p class="meta">{item.tags.length ? item.tags.join(' · ') : 'Saved link'} <span aria-hidden="true">·</span> {dateLabel(item.saved_at)}</p><h3><a href={item.url} target="_blank" rel="noopener noreferrer">{item.title}<span class="external" aria-hidden="true"> ↗</span></a></h3><p class="summary">{item.url}</p></div><div class="item-actions"><button class="round" onclick={() => update(item, {status:item.status === 'read' ? 'queue' : 'read'})} aria-label={item.status === 'read' ? `Mark ${item.title} unread` : `Mark ${item.title} read`}>{item.status === 'read' ? '↩' : '✓'}</button>{#if item.status === 'archived'}<button class="round" onclick={() => update(item, {status:'queue'})} aria-label={`Return ${item.title} to queue`}>↩</button>{:else}<button class="round" onclick={() => archive(item)} aria-label={`Archive ${item.title}`}>↓</button>{/if}<button class="round danger" onclick={() => deleteItem(item)} aria-label={`Remove ${item.title}`}>×</button></div></article></li>{/each}</ol>{/if}
  </section>
{/if}
</main>
<footer><p>RSS Saved Queue stores only the link details you provide to run your private queue. <a href="/privacy">Privacy</a> <a href="/terms">Terms</a></p><p>Made for deliberate reading.</p></footer>
