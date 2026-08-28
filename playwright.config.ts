import { defineConfig } from '@playwright/test';
import { rmSync } from 'node:fs';

const databasePath = `/tmp/rss-saved-queue-browser-${process.pid}-${Date.now()}.db`;
rmSync(databasePath, { force: true });
process.env.RSS_TEST_DATABASE_PATH = databasePath;

export default defineConfig({
  testDir: './tests',
  timeout: 30_000,
  workers: 1,
  use: { baseURL: 'http://127.0.0.1:4173', browserName: 'chromium', headless: true },
  webServer: {
    command: 'npm run build && PORT=4173 DATABASE_URL=sqlite://' + databasePath + '?mode=rwc STATIC_DIR=dist cargo run',
    url: 'http://127.0.0.1:4173/health',
    reuseExistingServer: false,
    timeout: 300_000
  }
});
