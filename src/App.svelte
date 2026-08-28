<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { FeedToken, Item } from './lib';
  import { dateLabel, priorityLabel, sessionKey } from './lib';

  type Route = 'home' | 'demo' | 'privacy' | 'terms' | 'extension' | 'not-found';
  type ImportRow = { title: string; url: string; tags: string[]; status: Item['status']; priority: Item['priority'] };
  const demoSessionKey = 'demo:rss-saved-queue:workspace';
  const demoFeedKey = 'demo:rss-saved-queue:feed';
  const siteOrigin = 'https://rss-saved-queue.sociobot.in';

  let route: Route = routeFromLocation();
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
  let loading = false;
  let saving = false;
  let creatingToken = false;
  let notice = '';
  let error = '';
  let undo: Item | null = null;
  let showSave = false;
  let showConnect = false;
  let showImport = false;
  let importRows: ImportRow[] = [];
  let importProblem = '';
  let importing = false;
  let dark = localStorage.getItem('rss-saved-queue:theme') === 'dark';
  let routeAnnouncement = '';

  $: demoMode = route === 'demo';
  $: appRoute = route === 'home' || route === 'demo';
  $: visible = items.filter((item) => item.status === status && (item.title + ' ' + item.url + ' ' + item.tags.join(' ')).toLowerCase().includes(query.trim().toLowerCase()));
  $: counts = { queue: items.filter((item) => item.status === 'queue').length, read: items.filter((item) => item.status === 'read').length, archived: items.filter((item) => item.status === 'archived').length };
  $: document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  $: localStorage.setItem('rss-saved-queue:theme', dark ? 'dark' : 'light');
  $: extensionEndpoint = location.origin;
  $: applyMetadata(route);

  onMount(() => {
    const onPopState = () => void activateCurrentRoute(true);
    addEventListener('popstate', onPopState);
    if (route === 'demo' || (route === 'home' && localStorage.getItem(sessionKey))) void load();
    if (route === 'extension') void ensureSession();
    return () => removeEventListener('popstate', onPopState);
  });

  function routeFromLocation(): Route {
    if (location.pathname === '/demo' || new URLSearchParams(location.search).get('demo') === '1') return 'demo';
    if (location.pathname === '/') return 'home';
    if (location.pathname === '/privacy') return 'privacy';
    if (location.pathname === '/terms') return 'terms';
    if (location.pathname === '/extension-setup') return 'extension';
    return 'not-found';
  }

  function routeMeta(next: Route) {
    if (next === 'demo') return { title: 'Demo — RSS Saved Queue', description: 'Try a private RSS queue with three sample links. Demo changes never reach your real queue.', path: '/demo' };
    if (next === 'privacy') return { title: 'Privacy — RSS Saved Queue', description: 'See what RSS Saved Queue stores and how device keys protect each private queue.', path: '/privacy' };
    if (next === 'terms') return { title: 'Terms — RSS Saved Queue', description: 'Read the terms for using RSS Saved Queue.', path: '/terms' };
    if (next === 'extension') return { title: 'Extension setup — RSS Saved Queue', description: 'Install the RSS Saved Queue browser extension and connect it to this private queue.', path: '/extension-setup' };
    if (next === 'not-found') return { title: 'Page not found — RSS Saved Queue', description: 'This RSS Saved Queue page could not be found.', path: '/404' };
    return { title: 'RSS Saved Queue — save links to a private RSS queue', description: 'Save web links in a private queue, set their order, and read them in your RSS reader.', path: '/' };
  }

  function applyMetadata(next: Route) {
    const meta = routeMeta(next);
    document.title = meta.title;
    document.querySelector('meta[name="description"]')?.setAttribute('content', meta.description);
    document.querySelector('meta[property="og:title"]')?.setAttribute('content', meta.title);
    document.querySelector('meta[property="og:description"]')?.setAttribute('content', meta.description);
    document.querySelector('meta[name="twitter:title"]')?.setAttribute('content', meta.title);
    document.querySelector('meta[name="twitter:description"]')?.setAttribute('content', meta.description);
    document.querySelector('link[rel="canonical"]')?.setAttribute('href', siteOrigin + meta.path);
    document.querySelector('meta[property="og:url"]')?.setAttribute('content', siteOrigin + meta.path);
  }

  async function activateCurrentRoute(moveFocus: boolean) {
    route = routeFromLocation();
    items = [];
    tokens = [];
    status = 'queue';
    query = '';
    showSave = false;
    showConnect = false;
    showImport = false;
    error = '';
    notice = '';
    if (route === 'demo' || (route === 'home' && localStorage.getItem(sessionKey))) await load();
    if (route === 'extension') await ensureSession();
    await tick();
    const heading = document.querySelector<HTMLElement>('main h1');
    routeAnnouncement = heading?.textContent?.replace(/\s+/g, ' ').trim() || document.title;
    if (moveFocus) heading?.focus();
    scrollTo({ top: 0, behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' });
  }

  function navigate(event: MouseEvent, path: string) {
    if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    event.preventDefault();
    history.pushState({}, '', path);
    void activateCurrentRoute(true);
  }

  async function openSave() {
    showSave = true;
    await tick();
    document.getElementById('link-title')?.focus();
  }

  async function ensureSession() {
    if (route === 'demo') {
      deviceKey = sessionStorage.getItem(demoSessionKey) || '';
      createdFeedUrl = sessionStorage.getItem(demoFeedKey) || '';
      if (!deviceKey) {
        const response = await fetch('/api/demo/session', { method: 'POST' });
        if (!response.ok) throw new Error('We could not start the sample queue. Reset the demo to retry.');
        const session = await response.json();
        deviceKey = session.token;
        createdFeedUrl = location.origin + session.feed_path;
        sessionStorage.setItem(demoSessionKey, deviceKey);
        sessionStorage.setItem(demoFeedKey, createdFeedUrl);
      }
      return;
    }
    deviceKey = localStorage.getItem(sessionKey) || '';
    if (!deviceKey) {
      const response = await fetch('/api/session', { method: 'POST' });
      if (!response.ok) throw new Error('We could not prepare your private queue. Retry loading the queue.');
      deviceKey = (await response.json()).token;
      localStorage.setItem(sessionKey, deviceKey);
    }
  }

  async function request(path: string, options: RequestInit = {}) {
    const headers = new Headers(options.headers);
    const target = route === 'demo' ? path.replace(/^\/api/, '/api/demo') : path;
    if (route === 'demo') headers.set('X-Demo-Workspace', deviceKey);
    else headers.set('Authorization', 'Bearer ' + deviceKey);
    if (options.body) headers.set('Content-Type', 'application/json');
    let response: Response;
    try {
      response = await fetch(target, { ...options, headers });
    } catch {
      throw new Error('offline');
    }
    if (!response.ok) {
      const body = await response.json().catch(() => null);
      throw new Error(body?.error || 'The queue request failed. Retry it.');
    }
    return response.status === 204 ? null : response.json();
  }

  function recovery(caught: unknown, action: string) {
    if (caught instanceof Error && caught.message === 'offline') return `This action was not completed because you are offline. Reconnect, then ${action} again.`;
    return caught instanceof Error ? caught.message : `Could not ${action} this link. Check the fields and retry.`;
  }

  async function load(retryDemo = true) {
    loading = true;
    error = '';
    try {
      await ensureSession();
      items = await request('/api/items');
      tokens = await request('/api/feed-tokens');
    } catch (caught) {
      if (route === 'demo' && retryDemo && deviceKey) {
        sessionStorage.removeItem(demoSessionKey);
        sessionStorage.removeItem(demoFeedKey);
        deviceKey = '';
        loading = false;
        await load(false);
        return;
      }
      error = caught instanceof Error ? caught.message : 'The queue is unavailable. Retry loading the queue.';
    } finally {
      loading = false;
    }
  }

  async function resetDemo() {
    try {
      if (deviceKey) {
        const response = await fetch('/api/demo/session', { method: 'DELETE', headers: { 'X-Demo-Workspace': deviceKey } });
        if (!response.ok) throw new Error('offline');
      }
    } catch {
      error = 'This demo was not reset because you are offline. Reconnect, then reset it again.';
      return;
    }
    sessionStorage.removeItem(demoSessionKey);
    sessionStorage.removeItem(demoFeedKey);
    deviceKey = '';
    createdFeedUrl = '';
    items = [];
    tokens = [];
    showSave = false;
    showConnect = false;
    await load();
    notice = 'Demo reset to its three sample links.';
  }

  async function startForReal(event: MouseEvent) {
    event.preventDefault();
    if (deviceKey) void fetch('/api/demo/session', { method: 'DELETE', keepalive: true, headers: { 'X-Demo-Workspace': deviceKey } });
    sessionStorage.removeItem(demoSessionKey);
    sessionStorage.removeItem(demoFeedKey);
    history.pushState({}, '', '/');
    await activateCurrentRoute(true);
  }

  async function saveLink() {
    saving = true;
    error = '';
    notice = '';
    try {
      await ensureSession();
      const item = await request('/api/items', { method: 'POST', body: JSON.stringify({ title, url: pageUrl, tags: tags.split(',') }) });
      items = [item, ...items];
      title = '';
      pageUrl = '';
      tags = '';
      showSave = false;
      status = 'queue';
      notice = route === 'demo' ? 'Saved in this demo only.' : 'Saved to your private queue.';
    } catch (caught) {
      error = caught instanceof Error && caught.message === 'offline' ? 'This link was not saved because you are offline. Reconnect, then save it again.' : recovery(caught, 'save');
    } finally {
      saving = false;
    }
  }

  async function update(item: Item, patch: Partial<Pick<Item, 'status' | 'priority'>>) {
    error = '';
    const previous = { ...item };
    items = items.map((entry) => entry.id === item.id ? { ...entry, ...patch } : entry);
    try {
      await request('/api/items/' + item.id, { method: 'PATCH', body: JSON.stringify(patch) });
    } catch (caught) {
      items = items.map((entry) => entry.id === item.id ? previous : entry);
      error = caught instanceof Error && caught.message === 'offline' ? 'This change was not saved because you are offline. Reconnect, then make it again.' : recovery(caught, 'update');
    }
  }

  async function archive(item: Item) {
    undo = { ...item };
    await update(item, { status: 'archived' });
    notice = 'Archived “' + item.title + '”.';
  }

  async function undoArchive() {
    if (!undo) return;
    await update(undo, { status: undo.status });
    notice = 'Archive undone.';
    undo = null;
  }

  async function deleteItem(item: Item) {
    if (!confirm('Remove “' + item.title + '” from your queue?')) return;
    const previous = [...items];
    items = items.filter((entry) => entry.id !== item.id);
    try {
      await request('/api/items/' + item.id, { method: 'DELETE' });
      notice = 'Link removed.';
    } catch (caught) {
      items = previous;
      error = caught instanceof Error && caught.message === 'offline' ? 'This removal was not saved because you are offline. Reconnect, then remove it again.' : recovery(caught, 'remove');
    }
  }

  async function createToken() {
    creatingToken = true;
    error = '';
    createdFeedUrl = '';
    try {
      await ensureSession();
      const result = await request('/api/feed-tokens', { method: 'POST', body: JSON.stringify({ label: tokenLabel }) });
      createdFeedUrl = location.origin + result.feed_path;
      if (route === 'demo') sessionStorage.setItem(demoFeedKey, createdFeedUrl);
      tokens = await request('/api/feed-tokens');
      notice = 'Private RSS link created. Copy it now.';
    } catch (caught) {
      error = caught instanceof Error && caught.message === 'offline' ? 'This RSS link was not created because you are offline. Reconnect, then create it again.' : recovery(caught, 'create');
    } finally {
      creatingToken = false;
    }
  }

  async function revokeToken(token: FeedToken) {
    if (!confirm('Revoke the “' + token.label + '” RSS link? Readers using it will stop working.')) return;
    try {
      await request('/api/feed-tokens/' + token.id + '/revoke', { method: 'POST' });
      tokens = await request('/api/feed-tokens');
      createdFeedUrl = '';
      if (route === 'demo') sessionStorage.removeItem(demoFeedKey);
      notice = 'Private RSS link revoked.';
    } catch (caught) {
      error = caught instanceof Error && caught.message === 'offline' ? 'This RSS link was not revoked because you are offline. Reconnect, then revoke it again.' : recovery(caught, 'revoke');
    }
  }

  async function exportCsv() {
    try {
      await ensureSession();
      const path = route === 'demo' ? '/api/demo/export.csv' : '/api/export.csv';
      const headers = new Headers();
      if (route === 'demo') headers.set('X-Demo-Workspace', deviceKey);
      else headers.set('Authorization', 'Bearer ' + deviceKey);
      let response: Response;
      try { response = await fetch(path, { headers }); } catch { throw new Error('offline'); }
      if (!response.ok) throw new Error('Could not export your queue. Retry the export.');
      const objectUrl = URL.createObjectURL(await response.blob());
      const link = document.createElement('a');
      link.href = objectUrl;
      link.download = 'reading-queue.csv';
      link.click();
      URL.revokeObjectURL(objectUrl);
      notice = 'CSV export created.';
    } catch (caught) {
      error = caught instanceof Error && caught.message === 'offline' ? 'This CSV was not exported because you are offline. Reconnect, then export it again.' : recovery(caught, 'export');
    }
  }

  async function openConnect() {
    error = '';
    try {
      await ensureSession();
      showConnect = !showConnect;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : 'We could not prepare your private RSS link.';
    }
  }

  function parseCsv(text: string): ImportRow[] {
    const rows: string[][] = [];
    let row: string[] = [], value = '', quoted = false;
    for (let i = 0; i < text.length; i += 1) {
      const char = text[i];
      if (quoted && char === '"' && text[i + 1] === '"') { value += '"'; i += 1; }
      else if (char === '"') quoted = !quoted;
      else if (!quoted && char === ',') { row.push(value); value = ''; }
      else if (!quoted && (char === '\n' || char === '\r')) { if (char === '\r' && text[i + 1] === '\n') i += 1; row.push(value); if (row.some(Boolean)) rows.push(row); row = []; value = ''; }
      else value += char;
    }
    if (value || row.length) { row.push(value); rows.push(row); }
    const expected = 'title,url,tags,status,priority,saved_at';
    if (rows[0]?.join(',') !== expected) throw new Error('Use a CSV exported by RSS Saved Queue.');
    return rows.slice(1).map((entry, index) => {
      const [itemTitle, url, rawTags, itemStatus, priority] = entry;
      if (!itemTitle || !url || !['queue', 'read', 'archived'].includes(itemStatus) || !['next', 'soon', 'later'].includes(priority)) throw new Error(`Row ${index + 2} needs a title, web link, status, and priority.`);
      return { title: itemTitle, url, tags: rawTags ? rawTags.split(';').map((tag) => tag.trim()).filter(Boolean) : [], status: itemStatus as Item['status'], priority: priority as Item['priority'] };
    });
  }

  async function previewImport(event: Event) {
    importProblem = '';
    importRows = [];
    const file = (event.currentTarget as HTMLInputElement).files?.[0];
    if (!file) return;
    try { importRows = parseCsv(await file.text()); }
    catch (caught) { importProblem = caught instanceof Error ? caught.message : 'This CSV could not be read.'; }
  }

  async function importCsv() {
    importing = true;
    error = '';
    try {
      const result = await request('/api/import.csv', { method: 'POST', body: JSON.stringify({ items: importRows }) });
      await load();
      showImport = false;
      importRows = [];
      notice = `${result.added} links imported. ${result.duplicates} duplicates skipped.`;
    } catch (caught) {
      error = caught instanceof Error && caught.message === 'offline' ? 'This CSV was not imported because you are offline. Reconnect, then import it again.' : recovery(caught, 'import');
    } finally { importing = false; }
  }
</script>

<a class="skip-link" href="#main">Skip to main content</a>
{#if demoMode}
  <aside class="demo-banner" aria-label="Demo controls">
    <strong>Demo — sample data, nothing is saved</strong>
    <span>
      <button class="banner-action" onclick={resetDemo}>Reset demo</button>
      <a class="banner-action" href="/" onclick={startForReal}>Start for real</a>
    </span>
  </aside>
{/if}
<header class="masthead">
  <a class="wordmark" href="/" onclick={(event) => navigate(event, '/')} aria-label="RSS Saved Queue home"><span aria-hidden="true">↯</span> SAVED<br />QUEUE</a>
  <nav class="site-nav" aria-label="Main navigation">
    <a href="/demo" onclick={(event) => navigate(event, '/demo')}>Demo</a>
    <a href="/extension-setup" onclick={(event) => navigate(event, '/extension-setup')}>Extension</a>
    <a href="/privacy" onclick={(event) => navigate(event, '/privacy')}>Privacy</a>
  </nav>
  <div class="top-actions">
    <button class="theme-button" aria-label={dark ? 'Use light theme' : 'Use dark theme'} onclick={() => dark = !dark}><span aria-hidden="true">{dark ? '☀' : '◐'}</span>{dark ? 'Use light theme' : 'Use dark theme'}</button>
    {#if appRoute}<button class="button button-ink" onclick={openSave}>Save a link <span aria-hidden="true">↗</span></button>{/if}
  </div>
</header>

{#if showSave && appRoute}
  <section class="feed-sheet" aria-labelledby="save-title">
    <div><p class="eyebrow">ADD TO QUEUE</p><h2 id="save-title">Save a link</h2><p>Enter a title, link, and tags. The app does not fetch the link.</p></div>
    <form onsubmit={(event) => { event.preventDefault(); void saveLink(); }} aria-describedby={error ? 'save-error' : undefined}>
      <label for="link-title">Link title</label><input id="link-title" bind:value={title} required maxlength="300" autocomplete="off" />
      <label for="link-url">Web link</label><input id="link-url" type="url" bind:value={pageUrl} required placeholder="https://example.com/article" autocomplete="url" />
      <label for="link-tags">Tags <span class="optional">optional, comma separated, up to 12</span></label>
      <div class="feed-form"><input id="link-tags" bind:value={tags} autocomplete="off" /><button class="button button-coral" disabled={saving}>{saving ? 'Saving…' : 'Save link'}</button></div>
      {#if error}<p id="save-error" class="form-error" role="alert">{error}</p>{/if}
    </form>
  </section>
{/if}

<div class="sr-only" aria-live="polite" aria-atomic="true">{routeAnnouncement}</div>
<main id="main" tabindex="-1">
  {#if route === 'privacy'}
    <article class="legal">
      <p class="eyebrow">PRIVACY</p><h1 tabindex="-1">Your saved links stay private.</h1>
      <h2>Data we keep</h2><p>We keep the title, link, tags, priority, and queue state you save.</p><p>We do not fetch link contents or import public feeds.</p>
      <h2>Keys and requests</h2><p>Your browser keeps a device key. The server stores its one-way hash.</p><p>Do not share your device key or private RSS links.</p><p>We use no analytics, ads, external fonts, or third-party scripts.</p>
      <h2>Remove data</h2><p>Remove queue links and revoke RSS links before clearing your browser key.</p><p>This policy is effective 28 August 2026.</p>
      <a class="button button-ink" href="/" onclick={(event) => navigate(event, '/')}>Return to your queue</a>
    </article>
  {:else if route === 'terms'}
    <article class="legal">
      <p class="eyebrow">TERMS</p><h1 tabindex="-1">Use the queue for saved links.</h1>
      <h2>Your responsibility</h2><p>You control the links and tags you save.</p><p>Protect your device key and private RSS links like passwords.</p>
      <h2>Service limits</h2><p>The service is provided as-is.</p><p>We may limit requests that threaten reliability or security.</p><p>These terms are effective 28 August 2026.</p>
      <a class="button button-ink" href="/" onclick={(event) => navigate(event, '/')}>Return to your queue</a>
    </article>
  {:else if route === 'extension'}
    <article class="legal extension-guide">
      <p class="eyebrow">BROWSER EXTENSION</p><h1 tabindex="-1">Save the tab you are reading.</h1>
      <p>Download the package, then load it as an unpacked Chrome extension.</p>
      <p><a class="button button-coral" href="/extension.zip" download>Download extension package</a></p>
      <h2>Connect it to this queue</h2>
      <ol><li>Open Chrome’s extension page and turn on Developer mode.</li><li>Unzip the download and choose Load unpacked.</li><li>Open extension settings and paste these values.</li></ol>
      <label for="guide-endpoint">Service URL</label><input id="guide-endpoint" class="secret" readonly value={extensionEndpoint} />
      <label for="guide-device-key">Device key</label><input id="guide-device-key" class="secret" readonly value={deviceKey} />
      <p>The extension stores these settings locally. It saves the active tab’s title, link, and entered tags.</p>
      <a class="button button-ink" href="/" onclick={(event) => navigate(event, '/')}>Return to your queue</a>
    </article>
  {:else if route === 'not-found'}
    <section class="not-found">
      <p class="eyebrow">MISFILED LINK · 404</p><h1 tabindex="-1">This page is not in the queue.</h1>
      <p>The address may be wrong. Return home or open the sample queue.</p>
      <div class="hero-actions"><a class="button button-ink" href="/" onclick={(event) => navigate(event, '/')}>Return home</a><a class="text-link" href="/demo" onclick={(event) => navigate(event, '/demo')}>Open sample queue</a></div>
    </section>
  {:else}
    <section class:intro-compact={demoMode} class="intro" aria-labelledby="page-title">
      <p class="eyebrow">PRIVATE RSS QUEUE FOR SAVED LINKS</p>
      <h1 id="page-title" tabindex="-1">{demoMode ? 'Explore a sample private RSS queue.' : 'Save web links in a private RSS queue.'}</h1>
      <p class="lede">{demoMode ? 'Three sample links show priorities, queue states, export, and RSS.' : 'For people who save too many links and read in an RSS reader.'}</p>
      {#if !demoMode}
        <div class="hero-actions"><a class="button button-coral" href="/demo" onclick={(event) => navigate(event, '/demo')}>Try it with sample data</a><span>See three saved links and their RSS feed.</span><button class="text-link" onclick={openSave}>Save your first link</button></div>
        <ul class="plain-facts" aria-label="Product facts"><li><strong>Private:</strong> each browser has its own queue.</li><li><strong>No page fetching</strong> or tracking.</li><li><strong>Free.</strong> Internet connection required.</li></ul>
      {/if}
    </section>

    <section class="queue-panel" aria-labelledby="queue-title">
      <div class="queue-toolbar">
        <div><p class="eyebrow">YOUR SAVED LINKS</p><h2 id="queue-title">{counts.queue} {counts.queue === 1 ? 'link' : 'links'} in queue</h2></div>
        <div class="toolbar-actions"><button class="text-button" onclick={openConnect} aria-expanded={showConnect}>Create private RSS link</button><button class="text-button" onclick={() => showImport = !showImport} aria-expanded={showImport}>Import CSV</button><button class="text-button" onclick={exportCsv}>Export CSV</button></div>
      </div>
      {#if demoMode && createdFeedUrl}<div class="demo-outcome"><span><strong>Sample RSS is ready.</strong> It contains the two queued links.</span><a href={createdFeedUrl} target="_blank" rel="noopener">Preview sample RSS <span aria-hidden="true">↗</span></a></div>{/if}
      {#if showConnect}
        <section class="connect-sheet" aria-labelledby="connect-title">
          <div><p class="eyebrow">PRIVATE RSS LINK</p><h3 id="connect-title">Read this queue in your reader</h3><p>Create a private RSS link. Revoke it when a reader should lose access.</p></div>
          <form onsubmit={(event) => { event.preventDefault(); void createToken(); }}>
            <label for="token-label">Reader name</label><div class="feed-form"><input id="token-label" bind:value={tokenLabel} maxlength="80" required /><button class="button button-coral" disabled={creatingToken}>{creatingToken ? 'Creating…' : 'Create RSS feed link'}</button></div>
            {#if createdFeedUrl}<label for="feed-link">Private RSS link</label><input id="feed-link" class="secret" readonly value={createdFeedUrl} aria-describedby="feed-link-help" /><p id="feed-link-help">Copy this link into your reader. You can revoke it below.</p>{/if}
          </form>
          {#if !demoMode}<div class="extension-box"><h4>Browser extension</h4><p><a href="/extension-setup" onclick={(event) => navigate(event, '/extension-setup')}>Download the extension and follow the setup steps.</a></p><label for="extension-endpoint">Service URL</label><input id="extension-endpoint" readonly value={extensionEndpoint} /><label for="device-key">Device key</label><input id="device-key" class="secret" readonly value={deviceKey} /><p>The extension sends the active tab’s title and link, plus tags you enter.</p></div>{/if}
          <ul class="token-list" aria-label="Private RSS links">{#each tokens as token}<li><span><strong>{token.label}</strong><small>{token.revoked_at ? 'Revoked' : 'Active'}</small></span>{#if !token.revoked_at}<button class="text-button danger-text" onclick={() => revokeToken(token)}>Revoke RSS link</button>{/if}</li>{/each}</ul>
        </section>
      {/if}
      {#if showImport}
        <section class="import-sheet" aria-labelledby="import-title">
          <div><p class="eyebrow">IMPORT A PRIOR EXPORT</p><h3 id="import-title">Review CSV links before adding them.</h3><p>Only exported link details are read. The app does not fetch links.</p></div>
          <div><label for="csv-file">CSV file</label><input id="csv-file" type="file" accept="text/csv,.csv" onchange={previewImport} />
          {#if importProblem}<p class="form-error" role="alert">{importProblem}</p>{/if}
          {#if importRows.length}<p>{importRows.length} links are ready. Existing title-and-link matches will be skipped.</p><button class="button button-coral" onclick={importCsv} disabled={importing}>{importing ? 'Importing…' : 'Import CSV links'}</button>{/if}</div>
        </section>
      {/if}
      <nav class="tabs" aria-label="Queue views">
        {#each [['queue', 'In queue', counts.queue], ['read', 'Read', counts.read], ['archived', 'Archived', counts.archived]] as tab}
          <button class:active={status === tab[0]} onclick={() => status = tab[0] as Item['status']} aria-current={status === tab[0] ? 'page' : undefined}>{tab[1]} <span>{tab[2]}</span></button>
        {/each}
      </nav>
      <label class="search"><span class="sr-only">Search saved links</span><span aria-hidden="true">⌕</span><input bind:value={query} placeholder="Search saved links" type="search" /></label>
      <div class="live" aria-live="polite">{showSave ? '' : error || notice}</div>
      {#if undo}<div class="undo"><span>Moved to archive.</span><button onclick={undoArchive}>Undo archive</button></div>{/if}
      {#if loading}
        <div class="state"><p class="eyebrow">OPENING QUEUE</p><h3>Loading your private queue…</h3></div>
      {:else if error && items.length === 0}
        <div class="state"><p class="eyebrow">CONNECTION PAUSED</p><h3>We couldn’t open your queue.</h3><p>{error}</p><button class="button button-ink" onclick={() => load()}>Retry loading queue</button></div>
      {:else if visible.length === 0}
        <div class="state empty"><p class="eyebrow">{query ? 'NO MATCHES' : 'NO SAVED LINKS YET'}</p><h3>{query ? 'No saved links match that search.' : status === 'queue' ? 'Saved links will appear here.' : 'No links are ' + status + ' yet.'}</h3><p>{query ? 'Try a title, tag, or shorter phrase.' : status === 'queue' ? 'Save a link here or with the browser extension.' : 'Move links here when their queue state changes.'}</p>{#if status === 'queue' && !query}<button class="button button-coral" onclick={openSave}>Save your first link</button>{/if}</div>
      {:else}
        <ol class="article-list">{#each visible as item (item.id)}<li class:done={item.status !== 'queue'}><article><div class="priority"><label for={'priority-' + item.id} class="sr-only">Priority for {item.title}</label><select id={'priority-' + item.id} value={item.priority} onchange={(event) => update(item, { priority: (event.currentTarget as HTMLSelectElement).value as Item['priority'] })}><option value="next">01 — next</option><option value="soon">02 — soon</option><option value="later">03 — later</option></select><span>{priorityLabel(item.priority)}</span></div><div class="article-copy"><p class="meta">{item.tags.length ? item.tags.join(' · ') : 'Saved link'} <span aria-hidden="true">·</span> {dateLabel(item.saved_at)}</p><h3><a href={item.url} target="_blank" rel="noopener noreferrer">{item.title}<span class="external" aria-hidden="true"> ↗</span><span class="sr-only"> (opens in a new tab)</span></a></h3><p class="summary">{item.url}</p></div><div class="item-actions"><button class="round" onclick={() => update(item, { status: item.status === 'read' ? 'queue' : 'read' })} aria-label={item.status === 'read' ? 'Mark ' + item.title + ' unread' : 'Mark ' + item.title + ' read'}>{item.status === 'read' ? '↩' : '✓'}</button>{#if item.status === 'archived'}<button class="round" onclick={() => update(item, { status: 'queue' })} aria-label={'Return ' + item.title + ' to queue'}>↩</button>{:else}<button class="round" onclick={() => archive(item)} aria-label={'Archive ' + item.title}>↓</button>{/if}<button class="round danger" onclick={() => deleteItem(item)} aria-label={'Remove ' + item.title}>×</button></div></article></li>{/each}</ol>
      {/if}
    </section>

    {#if route === 'home'}
      <section class="explainer" aria-labelledby="how-title">
        <p class="eyebrow">THREE STEPS</p><h2 id="how-title">How the queue works</h2>
        <ol><li><strong>Save a link.</strong><span>Enter a title, link, and optional tags.</span></li><li><strong>Choose its place.</strong><span>Set each link to next, soon, or later.</span></li><li><strong>Read the private RSS feed.</strong><span>Create a private RSS link for your reader.</span></li></ol>
      </section>
      <section class="privacy-note" aria-labelledby="boundary-title">
        <p class="eyebrow">CLEAR BOUNDARIES</p><h2 id="boundary-title">What the queue does not do</h2>
        <p>It does not fetch link contents or import public feeds.</p><p>It uses no analytics, ads, external fonts, or third-party scripts.</p>
      </section>
    {/if}
  {/if}
</main>
<footer>
  <p>A private RSS queue for saved links.</p>
  <nav aria-label="Legal links"><a href="/extension-setup" onclick={(event) => navigate(event, '/extension-setup')}>Extension</a><a href="/privacy" onclick={(event) => navigate(event, '/privacy')}>Privacy</a><a href="/terms" onclick={(event) => navigate(event, '/terms')}>Terms</a></nav>
  <p>Built by Param Factory · build {__BUILD_SHA__.slice(0, 8)}</p>
</footer>
