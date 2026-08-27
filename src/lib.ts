export type Item = { id:number; title:string; url:string; tags:string[]; saved_at:string; status:'queue'|'read'|'archived'; priority:'next'|'soon'|'later' };
export type FeedToken = { id:number; label:string; created_at:string; revoked_at:string|null };
export const priorityLabel = (priority: Item['priority']) => ({next:'Read next', soon:'Read soon', later:'Read later'}[priority]);
export const dateLabel = (value: string) => { const date = new Date(value); return Number.isNaN(+date) ? 'No date' : new Intl.DateTimeFormat(undefined, { month:'short', day:'numeric', year: date.getFullYear() !== new Date().getFullYear() ? 'numeric' : undefined }).format(date); };
export const sessionKey = 'rss-saved-queue:device-key';
