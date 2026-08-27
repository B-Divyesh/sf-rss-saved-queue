import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('a private queue saves a page and has no serious populated-state accessibility violations', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('button', { name: 'Save a page' }).click();
  await page.getByLabel('Page title').fill('A private test article');
  await page.getByLabel('Page address').fill('https://example.com/private-test');
  await page.getByLabel(/Tags/).fill('research, later');
  await page.getByRole('button', { name: 'Save page' }).click();
  await expect(page.getByRole('heading', { name: 'A private test article' })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('a validation error remains accessible and explains recovery', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('button', { name: 'Save a page' }).click();
  await page.getByLabel('Page title').fill('Bad link');
  await page.getByLabel('Page address').fill('ftp://example.com/nope');
  await page.getByRole('button', { name: 'Save page' }).click();
  await expect(page.getByText(/complete http or https article URL/i)).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('mobile layout retains the primary private queue controls', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('button', { name: 'Save a page' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Export CSV' })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBeTruthy();
});

test('keyboard users can skip to the queue landmark', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByText('Skip to queue')).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('main')).toBeFocused();
});
