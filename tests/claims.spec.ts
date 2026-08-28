import { test, expect } from '@playwright/test';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';

test('@claim:demo-isolation sample mode uses only its ephemeral namespace and resets', async ({ page }) => {
  const realKey = 'real-device-key-must-not-change';
  const requests: Array<{ path: string; authorization?: string }> = [];
  page.on('request', (request) => requests.push({ path: new URL(request.url()).pathname, authorization: request.headers().authorization }));
  await page.goto('/');
  await page.evaluate(([key, value]) => localStorage.setItem(key, value), ['rss-saved-queue:device-key', realKey]);
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL('/demo');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.getByRole('heading', { name: '2 links in queue' })).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('rss-saved-queue:device-key'))).toBe(realKey);
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual(expect.arrayContaining(['demo:rss-saved-queue:workspace', 'demo:rss-saved-queue:feed']));
  expect(requests.some(({ path }) => path === '/api/session' || path === '/api/items')).toBeFalsy();
  expect(requests.filter(({ path }) => path.startsWith('/api/demo/')).every(({ authorization }) => authorization === undefined)).toBeTruthy();

  const sample = await page.evaluate(async () => {
    const key = sessionStorage.getItem('demo:rss-saved-queue:workspace') || '';
    const response = await fetch('/api/demo/items', { headers: { 'X-Demo-Workspace': key } });
    return response.json();
  });
  expect(sample).toEqual(expect.arrayContaining([
    expect.objectContaining({ priority: 'next', status: 'queue' }),
    expect.objectContaining({ priority: 'soon', status: 'queue' }),
    expect.objectContaining({ priority: 'later', status: 'read' })
  ]));
  const sampleFeed = await page.evaluate(() => sessionStorage.getItem('demo:rss-saved-queue:feed') || '');
  const feedResponse = await page.request.get(sampleFeed);
  expect((await feedResponse.text()).match(/<item>/g)).toHaveLength(2);

  await page.getByRole('button', { name: 'Mark A field guide to calmer web typography read' }).click();
  await expect(page.getByRole('heading', { name: '1 link in queue' })).toBeVisible();
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.getByRole('heading', { name: '2 links in queue' })).toBeVisible();
  const resetWorkspace = await page.evaluate(() => sessionStorage.getItem('demo:rss-saved-queue:workspace'));
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL('/');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toHaveCount(0);
  expect(await page.evaluate(() => sessionStorage.getItem('demo:rss-saved-queue:workspace'))).toBeNull();
  expect(await page.evaluate(() => localStorage.getItem('rss-saved-queue:device-key'))).toBe(realKey);
  await expect.poll(async () => (await page.request.get('/api/demo/items', { headers: { 'X-Demo-Workspace': resetWorkspace || '' } })).status()).toBe(401);
});

test('@claim:saved-metadata-no-fetch stores entered fields and queue choices without fetching or importing', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('A test link kept as metadata');
  await page.getByLabel('Web link').fill('https://no-fetch.invalid/private-article');
  await page.getByLabel(/Tags/).fill('research, later');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByRole('heading', { name: 'A test link kept as metadata' })).toBeVisible();
  expect(requests.some((url) => url.startsWith('https://no-fetch.invalid'))).toBeFalsy();
  await page.getByLabel('Priority for A test link kept as metadata').selectOption('later');
  await page.getByRole('button', { name: 'Mark A test link kept as metadata read' }).click();
  const saved = await page.evaluate(async () => {
    const key = sessionStorage.getItem('demo:rss-saved-queue:workspace') || '';
    const response = await fetch('/api/demo/items', { headers: { 'X-Demo-Workspace': key } });
    return response.json();
  });
  expect(saved).toEqual(expect.arrayContaining([
    expect.objectContaining({
      title: 'A test link kept as metadata',
      url: 'https://no-fetch.invalid/private-article',
      tags: ['research', 'later'],
      status: 'read',
      priority: 'later'
    })
  ]));
  expect((await page.request.post('/api/feeds', { data: { url: 'https://example.com/feed.xml' } })).status()).toBe(404);
});

test('@claim:csv-export exports one CSV row per saved link', async ({ page }) => {
  await page.goto('/demo');
  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export CSV' }).click();
  const download = await downloadPromise;
  const stream = await download.createReadStream();
  const chunks: Buffer[] = [];
  for await (const chunk of stream) chunks.push(Buffer.from(chunk));
  const csv = Buffer.concat(chunks).toString('utf8');
  const rows = csv.trim().split('\n');
  expect(rows[0]).toBe('title,url,tags,status,priority,saved_at');
  expect(rows).toHaveLength(4);
  expect(csv).toContain('"A field guide to calmer web typography"');
  expect(csv).toContain('"read","later"');
});

test('@claim:rss-feed-revocation creates a usable RSS link and revokes it', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Create private RSS link' }).click();
  const sampleFeedUrl = await page.locator('#feed-link').inputValue();
  await page.getByLabel('Reader name').fill('Claim test reader');
  await page.getByRole('button', { name: 'Create RSS feed link' }).click();
  await expect(page.locator('#feed-link')).not.toHaveValue(sampleFeedUrl);
  const feedUrl = await page.locator('#feed-link').inputValue();
  const liveFeed = await page.request.get(feedUrl);
  expect(liveFeed.status()).toBe(200);
  expect(liveFeed.headers()['content-type']).toContain('application/rss+xml');
  expect(liveFeed.headers()['cache-control']).toBe('no-store');
  const xml = await liveFeed.text();
  expect(xml).toContain('A field guide to calmer web typography');
  expect(xml).not.toContain('Keep up with the web using private RSS');
  expect(xml.match(/<item>/g)).toHaveLength(2);
  for (const value of xml.matchAll(/<pubDate>([^<]+)<\/pubDate>/g)) expect(Number.isNaN(Date.parse(value[1]))).toBeFalsy();

  page.on('dialog', (dialog) => dialog.accept());
  await page.getByRole('button', { name: 'Revoke RSS link' }).last().click();
  await expect(page.getByText('Private RSS link revoked.')).toBeVisible();
  expect((await page.request.get(feedUrl)).status()).toBe(404);
});

test('@claim:device-isolation one device cannot read or change another queue', async ({ request }) => {
  const alice = await request.post('/api/session');
  const bob = await request.post('/api/session');
  const aliceKey = (await alice.json()).token;
  const bobKey = (await bob.json()).token;
  const saved = await request.post('/api/items', {
    headers: { Authorization: 'Bearer ' + aliceKey },
    data: { title: 'Alice only', url: 'https://example.com/alice', tags: ['private'] }
  });
  const item = await saved.json();
  const bobList = await request.get('/api/items', { headers: { Authorization: 'Bearer ' + bobKey } });
  expect(await bobList.json()).toEqual([]);
  const bobUpdate = await request.patch('/api/items/' + item.id, {
    headers: { Authorization: 'Bearer ' + bobKey },
    data: { status: 'read' }
  });
  expect(bobUpdate.status()).toBe(404);
  const database = await readFile('/tmp/rss-saved-queue-browser.db');
  expect(database.includes(Buffer.from(aliceKey))).toBeFalsy();
  expect(database.includes(Buffer.from(bobKey))).toBeFalsy();
  expect(database.includes(Buffer.from(createHash('sha256').update(aliceKey).digest('hex')))).toBeTruthy();
});

test('@claim:same-origin-privacy the complete sample flow makes only same-origin requests', async ({ page, baseURL }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Create private RSS link' }).click();
  await page.getByRole('button', { name: 'Create RSS feed link' }).click();
  await page.getByRole('button', { name: 'Export CSV' }).click();
  await expect(page.getByText('CSV export created.')).toBeVisible();
  expect([...origins]).toEqual([new URL(baseURL!).origin]);
});

test('@claim:free-access the sample and core controls require no account or payment', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('Free queue proof');
  await page.getByLabel('Web link').fill('https://example.com/free-proof');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByRole('heading', { name: 'Free queue proof' })).toBeVisible();
  await page.getByRole('button', { name: 'Create private RSS link' }).click();
  await page.getByRole('button', { name: 'Create RSS feed link' }).click();
  await expect(page.getByText('Private RSS link created. Copy it now.')).toBeVisible();
  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Export CSV' }).click();
  await downloadPromise;
  expect(requests.some((url) => /checkout|billing|login|oauth/i.test(url))).toBeFalsy();
});
