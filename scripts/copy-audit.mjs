import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '..');
const inventoryPath = resolve(root, '.factory/copy-inventory.json');
const auditPath = resolve(root, '.factory/copy-audit.md');
const sourcePath = resolve(root, 'src/App.svelte');
const readmePath = resolve(root, 'README.md');
const mode = process.argv[2] || '--check';
const banned = ['leverage', 'seamless', 'effortless', 'robust', 'powerful', 'intuitive', 'reimagine', 'supercharge', 'delightful', 'journey', 'ecosystem', 'ai-powered'];

const normalize = (value) => value
  .replace(/```[\s\S]*?```/g, ' ')
  .replace(/<(https?:\/\/[^>]+)>/g, '$1')
  .replace(/`([^`]+)`/g, '$1')
  .replace(/\[[^\]]+\]\([^)]*\)/g, (match) => match.slice(1, match.indexOf(']')))
  .replace(/\*\*/g, '')
  .replace(/<[^>]*>/g, ' ')
  .replace(/\s+/g, ' ')
  .trim();
const words = (value) => normalize(value).split(/\s+/).filter(Boolean).length;
const quoteTable = (rows) => rows.map(({ text, result }) => `| ${text.replaceAll('|', '\\|')} | ${words(text)} | ${result} |`).join('\n');

const inventory = JSON.parse(await readFile(inventoryPath, 'utf8'));
const source = normalize(await readFile(sourcePath, 'utf8'));
const readme = normalize(await readFile(readmePath, 'utf8'));
const problems = [];
for (const entry of inventory.landing) if (!source.includes(normalize(entry.text))) problems.push(`Landing inventory text is absent from src/App.svelte: ${entry.text}`);
for (const entry of inventory.readme) if (!readme.includes(normalize(entry.text))) problems.push(`README inventory text is absent from README.md: ${entry.text}`);
for (const entry of [...inventory.landing, ...inventory.readme]) {
  if (words(entry.text) > 22) problems.push(`Sentence exceeds 22 words: ${entry.text}`);
  const foundBanned = banned.filter((term) => new RegExp(`\\b${term.replace('-', '\\-')}\\b`, 'i').test(entry.text));
  if (foundBanned.length) problems.push(`Banned copy term ${foundBanned.join(', ')}: ${entry.text}`);
}

const output = `# Copy audit\n\nGenerated from \`.factory/copy-inventory.json\` on 28 August 2026. The checked inventory covers every visitor-facing sentence on the cold landing screen and every prose sentence in \`README.md\`. Word counts use whitespace-separated words. Hyphenated terms, URLs, and code paths count as one word. No audited sentence exceeds 22 words. No banned marketing word appears.\n\n## Landing page sentences\n\n| Sentence | Words | Result |\n| --- | ---: | --- |\n${quoteTable(inventory.landing)}\n\n## README sentences\n\n| Sentence | Words | Result |\n| --- | ---: | --- |\n${quoteTable(inventory.readme)}\n\n## Headings and controls\n\n${inventory.headings_and_controls}\n\n## Terminology table\n\n| Concept | One term |\n| --- | --- |\n${inventory.terms.map(([concept, term]) => `| ${concept} | ${term} |`).join('\n')}\n\n## Inventory check\n\nRun \`npm run audit:copy\` to compare this generated audit with the current landing source and README. The command fails if an inventoried sentence is missing, violates the word or banned-word rules, or if this file is stale. \`npm test\` runs this check before unit tests.\n`;

if (problems.length) {
  console.error(problems.join('\n'));
  process.exit(1);
}
if (mode === '--print') {
  process.stdout.write(output);
} else if (mode === '--write') {
  await writeFile(auditPath, output);
  console.log('Updated .factory/copy-audit.md');
} else if (mode === '--check') {
  const existing = await readFile(auditPath, 'utf8');
  if (existing !== output) {
    console.error('Copy audit is stale. Run: node scripts/copy-audit.mjs --write');
    process.exit(1);
  }
  console.log('Copy audit matches the current landing source and README.');
} else {
  console.error('Usage: node scripts/copy-audit.mjs [--check|--print|--write]');
  process.exit(2);
}
