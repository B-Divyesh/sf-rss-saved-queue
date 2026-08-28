import { test, expect } from '@playwright/test';
import { readFile } from 'node:fs/promises';

declare global {
  interface Window {
    __extensionRequest?: { url: string; method: string; headers: Record<string, string>; body: string };
    __permissionRequest?: string;
    __savedSettings?: { endpoint: string; deviceKey: string };
  }
}

test('@claim:extension-save the extension requests one service origin and saves the active tab fields', async ({ page, baseURL }) => {
  const manifest = JSON.parse(await readFile('extension/manifest.json', 'utf8'));
  expect(manifest.manifest_version).toBe(3);
  expect(manifest.permissions).toEqual(expect.arrayContaining(['activeTab', 'storage']));
  expect(manifest.host_permissions).toBeUndefined();
  expect(manifest.optional_host_permissions).toEqual(expect.arrayContaining(['http://*/*', 'https://*/*']));

  await page.addInitScript((origin) => {
    const extensionChrome = {
      tabs: { query: async () => [{ title: 'Active tab title', url: 'https://example.com/active-tab' }] },
      storage: {
        local: {
          get: async (defaults: Record<string, string>) => ({ ...defaults, endpoint: origin, deviceKey: 'fixture-device-key' }),
          set: async (values: { endpoint: string; deviceKey: string }) => { window.__savedSettings = values; }
        }
      },
      permissions: {
        contains: async () => false,
        request: async ({ origins }: { origins: string[] }) => {
          window.__permissionRequest = origins[0];
          return true;
        }
      }
    };
    Object.defineProperty(window, 'chrome', { value: extensionChrome, configurable: true });
    const nativeFetch = window.fetch.bind(window);
    window.fetch = async (input, init = {}) => {
      const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url;
      if (url.endsWith('/api/items')) {
        window.__extensionRequest = {
          url,
          method: init.method || 'GET',
          headers: Object.fromEntries(new Headers(init.headers).entries()),
          body: String(init.body || '')
        };
        return new Response(JSON.stringify({ id: 41 }), { status: 201, headers: { 'Content-Type': 'application/json' } });
      }
      return nativeFetch(input, init);
    };
  }, baseURL!);

  await page.goto('/extension/popup.html');
  await expect(page.getByLabel('Link title')).toHaveValue('Active tab title');
  await expect(page.getByLabel('Web link')).toHaveValue('https://example.com/active-tab');
  await page.getByLabel('Tags').fill('research, later');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByRole('status')).toHaveText('Saved to your private queue.');

  const savedRequest = await page.evaluate(() => window.__extensionRequest);
  expect(savedRequest).toEqual({
    url: baseURL + '/api/items',
    method: 'POST',
    headers: { authorization: 'Bearer fixture-device-key', 'content-type': 'application/json' },
    body: JSON.stringify({ title: 'Active tab title', url: 'https://example.com/active-tab', tags: ['research', ' later'] })
  });

  await page.goto('/extension/options.html');
  await page.getByLabel('RSS Saved Queue address').fill('https://self-host.example:8443/');
  await page.getByLabel('Device key').fill('new-device-key');
  await page.getByRole('button', { name: 'Save settings' }).click();
  await expect(page.getByRole('status')).toHaveText('Settings and this site permission are saved locally.');
  expect(await page.evaluate(() => window.__permissionRequest)).toBe('https://self-host.example:8443/*');
  expect(await page.evaluate(() => window.__savedSettings)).toEqual({ endpoint: 'https://self-host.example:8443', deviceKey: 'new-device-key' });

  await page.goto('/extension-setup');
  await expect(page.getByRole('link', { name: 'Download extension package' })).toHaveAttribute('href', '/extension.zip');
  await expect(page.getByLabel('Device key')).not.toHaveValue('');
  const packageResponse = await page.request.get('/extension.zip');
  expect(packageResponse.status()).toBe(200);
  expect(packageResponse.headers()['content-type']).toContain('application/zip');
});
