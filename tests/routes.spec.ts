import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('real routes set titles, canonical metadata, focus, announcements, and history', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('RSS Saved Queue — save links to a private RSS queue');
  await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', 'https://rss-saved-queue.sociobot.in/');
  await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', 'https://rss-saved-queue.sociobot.in/og-card.png');

  await page.getByRole('link', { name: 'Privacy', exact: true }).first().click();
  await expect(page).toHaveURL('/privacy');
  await expect(page).toHaveTitle('Privacy — RSS Saved Queue');
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await expect(page.locator('[aria-live="polite"]').first()).toContainText('Your saved links stay private.');

  await page.goBack();
  await expect(page).toHaveURL('/');
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await page.goForward();
  await expect(page).toHaveURL('/privacy');
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
});

test('demo deep links, discovery files, icons, and the designed 404 route work', async ({ page, request }) => {
  await page.goto('/demo');
  await expect(page).toHaveTitle('Demo — RSS Saved Queue');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.goto('/?demo=1');
  await expect(page).toHaveTitle('Demo — RSS Saved Queue');
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();

  for (const path of ['/robots.txt', '/sitemap.xml', '/favicon.ico', '/favicon.svg', '/apple-touch-icon.png', '/og-card.png']) {
    expect((await request.get(path)).status(), path).toBe(200);
  }

  const response = await page.goto('/not-a-route');
  expect(response?.status()).toBe(404);
  await expect(page).toHaveTitle('Page not found — RSS Saved Queue');
  await expect(page.getByRole('heading', { name: 'This page is not in the queue.' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Return home' })).toBeVisible();
});

test('all internal landing links resolve without dead ends', async ({ page, request }) => {
  await page.goto('/');
  const hrefs = await page.locator('a[href]').evaluateAll((links) => [...new Set(links.map((link) => (link as HTMLAnchorElement).getAttribute('href')).filter((href): href is string => Boolean(href && href.startsWith('/'))))]);
  for (const href of hrefs) {
    const response = await request.get(href);
    expect(response.status(), href).toBe(200);
  }
});

test('mobile controls and links meet touch sizing without horizontal overflow', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/demo');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBeTruthy();
  const undersized = await page.locator('button:visible, a[href]:visible, input:visible, select:visible').evaluateAll((elements) =>
    elements.map((element) => {
      const box = element.getBoundingClientRect();
      return { label: element.getAttribute('aria-label') || element.textContent?.trim() || element.tagName, width: box.width, height: box.height };
    }).filter((box) => box.width < 44 || box.height < 44)
  );
  expect(undersized).toEqual([]);
});

for (const route of ['/demo', '/privacy', '/terms', '/not-a-route']) {
  test('accessibility has no serious or critical findings on ' + route, async ({ page }) => {
    await page.goto(route);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((violation) => ['critical', 'serious'].includes(violation.impact || ''))).toEqual([]);
  });
}
