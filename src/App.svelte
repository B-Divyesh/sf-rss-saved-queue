<script lang="ts">
  import { onMount } from 'svelte';
  import type { Item } from './lib';
  import { dateLabel, priorityLabel, snapshotKey } from './lib';

  let items: Item[] = [];
  let status: 'queue'|'read'|'archived' = 'queue';
  let query = '';
  let feedUrl = '';
  let loading = true;
  let importing = false;
  let notice = '';
  let error = '';
  let undo: Item | null = null;
  let showAdd = false;
  let dark = localStorage.getItem('rss-saved-queue:theme') === 'dark';
  let legalPage: 'privacy'|'terms'|null = location.pathname === '/privacy' ? 'privacy' : location.pathname === '/terms' ? 'terms' : null;

  $: visible = items.filter((item) => item.status === status && `${item.title} ${item.source}`.toLowerCase().includes(query.trim().toLowerCase()));
  $: counts = { queue: items.filter(x => x.status === 'queue').length, read: items.filter(x => x.status === 'read').length, archived: items.filter(x => x.status === 'archived').length };
  $: document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  $: localStorage.setItem('rss-saved-queue:theme', dark ? 'dark' : 'light');

  onMount(async () => { if (!legalPage) await load(); });
  async function load() {
    loading = true; error = '';
    try { const response = await fetch('/api/items'); if (!response.ok) throw new Error('The queue is unavailable.'); items = await response.json(); saveSnapshot(); }
    catch (e) { const local = localStorage.getItem(snapshotKey); if (local) { items = JSON.parse(local); notice = 'Showing your last saved snapshot while the queue reconnects.'; } else error = e instanceof Error ? e.message : 'The queue is unavailable.'; }
    finally { loading = false; }
  }
  function saveSnapshot() { localStorage.setItem(snapshotKey, JSON.stringify(items)); }
  async function request(path: string, options: RequestInit) { const response = await fetch(path, { headers: {'Content-Type':'application/json'}, ...options }); if (!response.ok) { const body = await response.json().catch(() => null); throw new Error(body?.error || 'Something went wrong.'); } return response.status === 204 ? null : response.json(); }
  async function importFeed() {
    importing = true; error = ''; notice = '';
    try { const result = await request('/api/feeds', {method:'POST', body:JSON.stringify({url:feedUrl})}); feedUrl = ''; showAdd = false; await load(); status = 'queue'; notice = `${result.added} ${result.added === 1 ? 'article' : 'articles'} added from ${result.feed_title}${result.duplicates ? `; ${result.duplicates} already saved.` : '.'}`; }
    catch (e) { error = e instanceof Error ? e.message : 'Could not add that feed.'; }
    finally { importing = false; }
  }
  async function update(item: Item, patch: Partial<Pick<Item,'status'|'priority'>>) {
    error = ''; const previous = {...item}; items = items.map(x => x.id === item.id ? {...x, ...patch} : x); saveSnapshot();
    try { await request(`/api/items/${item.id}`, {method:'PATCH', body:JSON.stringify(patch)}); }
    catch (e) { items = items.map(x => x.id === item.id ? previous : x); saveSnapshot(); error = e instanceof Error ? e.message : 'Could not update item.'; }
  }
  async function archive(item: Item) { undo = {...item}; await update(item, {status:'archived'}); notice = `Archived “${item.title}”.`; }
  async function undoArchive() { if (!undo) return; await update(undo, {status:undo.status}); notice = 'Archive undone.'; undo = null; }
  async function deleteItem(item: Item) { if (!confirm(`Remove “${item.title}” from your queue?`)) return; const previous = [...items]; items = items.filter(x => x.id !== item.id); saveSnapshot(); try { await request(`/api/items/${item.id}`, {method:'DELETE'}); notice = 'Item removed.'; } catch(e) { items = previous; saveSnapshot(); error = e instanceof Error ? e.message : 'Could not remove item.'; } }
  function exportCsv() { window.location.assign('/api/export.csv'); }
</script>

<svelte:head><title>RSS Saved Queue — read later, with intent</title></svelte:head>
<a class="skip-link" href="#main">Skip to queue</a>
<header class="masthead">
  <a class="wordmark" href="/" aria-label="RSS Saved Queue home"><span aria-hidden="true">↯</span> SAVED<br />QUEUE</a>
  <div class="top-actions">
    <button class="icon-button" aria-label={dark ? 'Use light theme' : 'Use dark theme'} onclick={() => dark = !dark}>{dark ? '☀' : '◐'}</button>
    <button class="button button-ink" onclick={() => showAdd = !showAdd} aria-expanded={showAdd}>Add a feed <span aria-hidden="true">↗</span></button>
  </div>
</header>

{#if showAdd}
  <section class="feed-sheet" aria-labelledby="feed-title">
    <div><p class="eyebrow">INBOX DOOR</p><h2 id="feed-title">Bring in a feed</h2><p>Paste the RSS or Atom feed address. New articles land in your queue; duplicates stay out.</p></div>
    <form onsubmit={(event) => { event.preventDefault(); importFeed(); }}>
      <label for="feed-url">Feed URL</label>
      <div class="feed-form"><input id="feed-url" type="url" bind:value={feedUrl} required placeholder="https://example.com/feed.xml" autocomplete="url" /><button class="button button-coral" disabled={importing}>{importing ? 'Adding…' : 'Add feed'}</button></div>
    </form>
  </section>
{/if}

<main id="main">
{#if legalPage}
  <article class="legal">
    <p class="eyebrow">THE SHORT VERSION</p>
    {#if legalPage === 'privacy'}
      <h1 id="page-title">Your reading<br /><em>stays yours.</em></h1>
      <h2>Privacy</h2><p>RSS Saved Queue stores the feed addresses and articles you import so it can maintain your reading queue. We do not sell data, run advertising trackers, or use third-party analytics.</p><p>Your browser keeps a small local snapshot of your queue to make temporary connection failures less disruptive. The server receives only the feed URL you choose to import and the resulting feed entries. Feed publishers may receive your server’s request when a feed is imported.</p><p>To remove a saved article, use its Remove button. For service questions, contact the site operator through Sociobot. This policy is effective 27 August 2026.</p>
    {:else}
      <h1 id="page-title">A calmer<br /><em>reading line.</em></h1>
      <h2>Terms of use</h2><p>RSS Saved Queue is a personal reading utility. You are responsible for the feed addresses you add and for respecting publishers’ terms and copyright. The app only retrieves publicly reachable RSS and Atom feeds.</p><p>The service is provided as-is, without a guarantee that every feed or publisher site will remain available. Do not use it to store sensitive personal information. We may limit requests that threaten reliability or security.</p><p>These terms are effective 27 August 2026. Continuing to use the service means you accept them.</p>
    {/if}
    <a class="button button-ink" href="/">Return to your queue</a>
  </article>
{:else}
  <section class="intro" aria-labelledby="page-title">
    <p class="eyebrow">A READING ROOM FOR THE INTERNET</p>
    <h1 id="page-title">Make a smaller<br /><em>later.</em></h1>
    <p class="lede">Collect the writing worth your attention, then give it a place in line. No algorithm, no endless scroll.</p>
  </section>
  <section class="queue-panel" aria-label="Saved reading queue">
    <div class="queue-toolbar">
      <div><p class="eyebrow">YOUR SHELF</p><h2>{counts.queue} {counts.queue === 1 ? 'piece' : 'pieces'} waiting</h2></div>
      <button class="text-button" onclick={exportCsv}>Export CSV</button>
    </div>
    <nav class="tabs" aria-label="Queue views">
      {#each [['queue','In queue',counts.queue], ['read','Read',counts.read], ['archived','Archive',counts.archived]] as tab}
        <button class:active={status === tab[0]} onclick={() => status = tab[0] as typeof status} aria-current={status === tab[0] ? 'page' : undefined}>{tab[1]} <span>{tab[2]}</span></button>
      {/each}
    </nav>
    <label class="search"><span class="sr-only">Search saved articles</span><span aria-hidden="true">⌕</span><input bind:value={query} placeholder="Search this shelf" type="search" /></label>
    <div class="live" aria-live="polite">{error || notice}</div>
    {#if undo}<div class="undo"><span>Moved to archive.</span><button onclick={undoArchive}>Undo</button></div>{/if}
    {#if loading}<div class="state"><p class="eyebrow">OPENING THE SHELF</p><h3>Loading your saved reading…</h3></div>
    {:else if error && items.length === 0}<div class="state"><p class="eyebrow">CONNECTION PAUSED</p><h3>We couldn’t open your queue.</h3><button class="button button-ink" onclick={load}>Try again</button></div>
    {:else if visible.length === 0}<div class="state empty"><p class="eyebrow">{query ? 'NO MATCHES' : 'SHELF CLEAR'}</p><h3>{query ? 'Nothing on this shelf matches that search.' : status === 'queue' ? 'Your next good read starts with a feed.' : `Nothing is ${status} yet.`}</h3><p>{query ? 'Try a title, publication, or a shorter phrase.' : status === 'queue' ? 'Add an RSS or Atom address and we’ll put its recent articles in a calm, sortable line.' : 'Move articles here from your queue when you are done with them.'}</p>{#if status === 'queue' && !query}<button class="button button-coral" onclick={() => showAdd = true}>Add your first feed</button>{/if}</div>
    {:else}<ol class="article-list">{#each visible as item (item.id)}<li class:done={item.status !== 'queue'}><article><div class="priority"><label for={`priority-${item.id}`} class="sr-only">Priority for {item.title}</label><select id={`priority-${item.id}`} value={item.priority} onchange={(event) => update(item, {priority:(event.currentTarget as HTMLSelectElement).value as Item['priority']})} aria-label={`Priority: ${priorityLabel(item.priority)}`}><option value="next">01</option><option value="soon">02</option><option value="later">03</option></select><span>{priorityLabel(item.priority)}</span></div><div class="article-copy"><p class="meta">{item.source} <span aria-hidden="true">·</span> {dateLabel(item.published_at)}</p><h3><a href={item.url} target="_blank" rel="noopener noreferrer">{item.title}<span class="external" aria-hidden="true"> ↗</span></a></h3>{#if item.summary}<p class="summary">{item.summary}</p>{/if}</div><div class="item-actions"><button class="round" onclick={() => update(item, {status:item.status === 'read' ? 'queue' : 'read'})} aria-label={item.status === 'read' ? `Mark ${item.title} unread` : `Mark ${item.title} read`}>{item.status === 'read' ? '↩' : '✓'}</button>{#if item.status === 'archived'}<button class="round" onclick={() => update(item, {status:'queue'})} aria-label={`Return ${item.title} to queue`}>↩</button>{:else}<button class="round" onclick={() => archive(item)} aria-label={`Archive ${item.title}`}>↓</button>{/if}<button class="round danger" onclick={() => deleteItem(item)} aria-label={`Remove ${item.title}`}>×</button></div></article></li>{/each}</ol>{/if}
  </section>
{/if}
</main>
<footer><p>RSS Saved Queue stores your feeds and reading list only to run your queue. <a href="/privacy">Privacy</a> <a href="/terms">Terms</a></p><p>Made for deliberate reading.</p></footer>
