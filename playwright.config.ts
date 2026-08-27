import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  timeout: 30_000,
  use: { baseURL: 'http://127.0.0.1:4173', browserName: 'chromium', headless: true },
  webServer: {
    command: 'npm run build && PORT=4173 DATABASE_URL=sqlite:///tmp/rss-saved-queue-browser.db?mode=rwc STATIC_DIR=dist cargo run',
    url: 'http://127.0.0.1:4173/health',
    reuseExistingServer: false,
    timeout: 90_000
  }
});
