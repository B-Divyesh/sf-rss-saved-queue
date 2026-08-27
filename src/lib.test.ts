import { describe, expect, it } from 'vitest';
import { dateLabel, priorityLabel } from './lib';

describe('queue presentation helpers', () => {
  it('gives priorities meaningful names', () => {
    expect(priorityLabel('next')).toBe('Read next');
    expect(priorityLabel('later')).toBe('Read later');
  });
  it('does not throw for absent dates', () => {
    expect(dateLabel(null)).toBe('No date');
  });
});
