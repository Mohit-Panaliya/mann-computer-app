import { openUrl } from '@tauri-apps/plugin-opener';

/**
 * Open an external URL (WhatsApp, PDF, etc.) in a platform-aware way.
 * Uses the Tauri opener plugin which works on desktop AND Android/iOS.
 * Falls back to window.open if the plugin is unavailable (e.g. browser dev).
 */
export async function openExternal(url: string): Promise<void> {
  try {
    await openUrl(url);
  } catch (e) {
    try {
      window.open(url, '_blank');
    } catch (err) {
      console.error('Failed to open URL', url, err);
    }
  }
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-IN', {
    style: 'currency',
    currency: 'INR',
    minimumFractionDigits: 2,
  }).format(amount);
}

export function formatDate(dateStr: string): string {
  if (!dateStr) return '';
  const d = new Date(dateStr);
  return d.toLocaleDateString('en-IN', { day: '2-digit', month: '2-digit', year: 'numeric' });
}

export function formatDateTime(dateStr: string): string {
  if (!dateStr) return '';
  const d = new Date(dateStr);
  return d.toLocaleDateString('en-IN', { day: '2-digit', month: '2-digit', year: 'numeric' }) + ' ' +
    d.toLocaleTimeString('en-IN', { hour: '2-digit', minute: '2-digit' });
}

export function getStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    pending: 'Pending',
    repairing: 'Repairing',
    ready: 'Ready',
    delivered: 'Delivered',
    completed: 'Completed',
    in_progress: 'In Progress',
    draft: 'Draft',
    sent: 'Sent',
    approved: 'Approved',
    rejected: 'Rejected',
    paid: 'Paid',
  };
  return labels[status] || status;
}

export function getStatusClass(status: string): string {
  const classes: Record<string, string> = {
    pending: 'badge-pending',
    repairing: 'badge-repairing',
    ready: 'badge-ready',
    delivered: 'badge-delivered',
    completed: 'badge-ready',
    in_progress: 'badge-repairing',
    draft: 'badge-draft',
    sent: 'badge-repairing',
    approved: 'badge-ready',
    rejected: 'badge-delivered',
    paid: 'badge-paid',
  };
  return classes[status] || 'badge-draft';
}

export function generateWhatsAppLink(phone: string, message: string): string {
  const cleaned = phone.replace(/\D/g, '');
  const withCountry = cleaned.startsWith('91') ? cleaned : `91${cleaned}`;
  return `https://wa.me/${withCountry}?text=${encodeURIComponent(message)}`;
}

export function getStatusVariant(status: string): string {
  const variants: Record<string, string> = {
    pending: 'outline',
    repairing: 'default',
    ready: 'secondary',
    delivered: 'outline',
    completed: 'secondary',
    in_progress: 'default',
    draft: 'outline',
    sent: 'default',
    approved: 'secondary',
    rejected: 'destructive',
    paid: 'secondary',
  };
  return variants[status] || 'outline';
}
