import { describe, expect, it } from 'vitest';
import { dateLabel, priorityLabel } from './lib';

describe('queue presentation helpers', () => {
  it('gives priorities meaningful names', () => {
    expect(priorityLabel('next')).toBe('Read next');
    expect(priorityLabel('later')).toBe('Read later');
  });
  it('handles an invalid saved date safely', () => {
    expect(dateLabel('not-a-date')).toBe('No date');
  });
});
