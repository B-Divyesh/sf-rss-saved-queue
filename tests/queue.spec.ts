import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('a private queue saves a link and has no serious populated-state accessibility violations', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('A private test article');
  await page.getByLabel('Web link').fill('https://example.com/private-test');
  await page.getByLabel(/Tags/).fill('research, later');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByRole('heading', { name: 'A private test article' })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('a validation error remains accessible and explains recovery', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('Bad link');
  await page.getByLabel('Web link').fill('ftp://example.com/nope');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByText(/complete http or https web link/i)).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('dark save sheet has no serious accessibility violations', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Use dark theme' }).click();
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('Bad link');
  await page.getByLabel('Web link').fill('ftp://example.com/nope');
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByText(/complete http or https web link/i)).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('over-limit tags are explained instead of being silently discarded', async ({ page }) => {
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Save a link' }).click();
  await page.getByLabel('Link title').fill('Too many tags');
  await page.getByLabel('Web link').fill('https://example.com/tags');
  await page.getByLabel(/Tags/).fill(Array.from({ length: 13 }, (_, index) => `tag-${index + 1}`).join(','));
  await page.getByRole('button', { name: 'Save link' }).click();
  await expect(page.getByText('Use up to 12 tags per saved link.')).toBeVisible();
});

test('a first-load connection failure gives an offline recovery state', async ({ page }) => {
  await page.addInitScript(() => localStorage.setItem('rss-saved-queue:device-key', 'offline-device'));
  await page.route('**/api/items', (route) => route.abort());
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'We couldn’t open your queue.' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Retry loading queue' })).toBeVisible();
});

test('mobile layout retains the primary private queue controls', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/demo');
  await expect(page.getByRole('button', { name: 'Save a link' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Export CSV' })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBeTruthy();
});

test('keyboard users can skip to the queue landmark', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByText('Skip to main content')).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('main')).toBeFocused();
});
