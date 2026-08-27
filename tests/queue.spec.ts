import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('empty queue is usable and has no serious accessibility violations', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { name: /make a smaller/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /add your first feed/i })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact ?? ''))).toEqual([]);
});

test('mobile layout retains the primary queue controls', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('button', { name: /add a feed/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /export csv/i })).toBeVisible();
});
