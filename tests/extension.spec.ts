import { test, expect } from '@playwright/test';
import { readFile } from 'node:fs/promises';

test('@claim:extension-save extension manifest and popup keep the save contract explicit', async () => {
  const manifest = JSON.parse(await readFile('extension/manifest.json', 'utf8'));
  const popup = await readFile('extension/popup.js', 'utf8');
  expect(manifest.manifest_version).toBe(3);
  expect(manifest.permissions).toEqual(expect.arrayContaining(['activeTab', 'storage']));
  expect(manifest.host_permissions).toBeUndefined();
  expect(manifest.optional_host_permissions).toEqual(expect.arrayContaining(['http://*/*', 'https://*/*']));
  expect(manifest.options_page).toBe('options.html');
  expect(popup).toContain('/api/items');
  expect(popup).toContain('Authorization');
  expect(popup).toContain('title: fields.title.value');
  const options = await readFile('extension/options.js', 'utf8');
  expect(options).toContain('chrome.permissions.request');
  expect(options).toContain('service URL');
});
