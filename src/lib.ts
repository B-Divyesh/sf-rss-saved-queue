export type Item = { id:number; title:string; url:string; source:string; summary:string; published_at:string|null; saved_at:string; status:'queue'|'read'|'archived'; priority:'next'|'soon'|'later' };
export const priorityLabel = (priority: Item['priority']) => ({next:'Read next', soon:'Read soon', later:'Read later'}[priority]);
export const dateLabel = (value: string | null) => { if (!value) return 'No date'; const date = new Date(value); return Number.isNaN(+date) ? 'No date' : new Intl.DateTimeFormat(undefined, { month:'short', day:'numeric', year: date.getFullYear() !== new Date().getFullYear() ? 'numeric' : undefined }).format(date); };
export const snapshotKey = 'rss-saved-queue:snapshot';
