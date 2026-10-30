<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let bills: any[] = [];
  let customers: any[] = [];
  let slips: any[] = [];
  let showForm = false;
  let loading = false;

  let formCustomerId = '';
  let formInwardId = '';
  let formItems: any[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formPaymentMethod = '';

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [bills, customers, slips] = await Promise.all([api.getBills(), api.getCustomers(), api.getInwardSlips()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  function addItem() { formItems = [...formItems, { description: '', quantity: 1, unit_price: 0, total: 0 }]; }
  function removeItem(i: number) { formItems = formItems.filter((_, idx) => idx !== i); }
  function updateItemTotal(i: number) { formItems[i].total = formItems[i].quantity * formItems[i].unit_price; formItems = [...formItems]; }
  function getSubtotal() { return formItems.reduce((sum, i) => sum + i.total, 0); }
  function getTax() { return getSubtotal() * 0.18; }
  function getGrandTotal() { return getSubtotal() + getTax(); }

  async function createBill() {
    const validItems = formItems.filter(i => i.description && i.quantity > 0);
    if (validItems.length === 0) { alert('Please add at least one item'); return; }
    if (!formCustomerId) { alert('Please select a customer'); return; }
    if (!formPaymentMethod) { alert('Please select payment method'); return; }

    loading = true;
    try {
      await api.createBill(formInwardId || null, null, formCustomerId, validItems, formPaymentMethod);
      showForm = false;
      formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
      formPaymentMethod = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create bill', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function markPaid(b: any) {
    try {
      await api.updateBillPayment(b.id, 'paid', b.payment_method || 'Cash');
      await loadData();
    } catch (e: any) {
      console.error('Failed to mark paid', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  async function sendWhatsApp(b: any) {
    const itemsList = b.items.map((i: any) => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Bill from Mann Computer\nBill No: ${b.bill_no}\nDate: ${b.created_at}\n\n${itemsList}\n\nSubtotal: ${formatCurrency(b.subtotal)}\nTax: ${formatCurrency(b.tax)}\nTotal: ${formatCurrency(b.total_amount)}`;
    window.open(generateWhatsAppLink(b.customer_phone || '', msg), '_blank');
  }

  function getStatusBadge(status: string): string {
    return status === 'paid' ? 'badge-paid' : 'badge-pending';
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Billing</h1>
      <p class="page-subtitle">Generate invoices and track payments</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Bill</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Bill No</th><th>Customer</th><th>Items</th><th>Total</th><th>Payment</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each bills as b}
            <tr>
              <td class="font-medium">{b.bill_no}</td>
              <td>{b.customer_name}</td>
              <td>{b.items.length}</td>
              <td class="font-medium">{formatCurrency(b.total_amount)}</td>
              <td><span class="badge {getStatusBadge(b.payment_status)}">{b.payment_status}</span></td>
              <td>
                <div style="display:flex; gap:0.5rem; flex-wrap:wrap;">
                  {#if b.payment_status === 'pending'}
                    <button class="btn btn-success btn-sm" on:click={() => markPaid(b)}>Mark Paid</button>
                  {/if}
                  <button class="btn btn-ghost btn-sm" on:click={() => sendWhatsApp(b)}>💬</button>
                </div>
              </td>
            </tr>
          {:else}
            <tr><td colspan="6" class="empty-state">No bills found.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal" style="max-width:42rem;">
        <div class="modal-header">
          <h2 class="modal-title">New Bill</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createBill}>
          <div style="display:grid; grid-template-columns:1fr 1fr; gap:0.75rem; margin-bottom:1rem;">
            <div class="form-group" style="margin-bottom:0;">
              <label class="form-label">Link to Inward Slip</label>
              <select class="form-select" bind:value={formInwardId}>
                <option value="">None</option>
                {#each slips as s}<option value={s.id}>#{s.slip_no} - {s.customer_name}</option>{/each}
              </select>
            </div>
            <div class="form-group" style="margin-bottom:0;">
              <label class="form-label">Customer</label>
              <select class="form-select" bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}<option value={c.id}>{c.name}</option>{/each}
              </select>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">Items</label>
            {#each formItems as item, i}
              <div style="display:flex; gap:0.5rem; margin-bottom:0.5rem; align-items:center;">
                <input class="form-input" style="flex:1;" placeholder="Description" bind:value={item.description} />
                <input class="form-input" style="width:4rem;" type="number" placeholder="Qty" bind:value={item.quantity} on:change={() => updateItemTotal(i)} />
                <input class="form-input" style="width:5rem;" type="number" placeholder="Rate" bind:value={item.unit_price} on:change={() => updateItemTotal(i)} />
                <span style="width:5rem; text-align:right; font-weight:600; font-size:0.875rem;">{formatCurrency(item.total)}</span>
                <button type="button" class="btn btn-danger btn-sm" style="padding:0.25rem 0.5rem;" on:click={() => removeItem(i)}>×</button>
              </div>
            {/each}
            <button type="button" class="btn btn-secondary btn-sm" on:click={addItem}>+ Add Item</button>
          </div>
          <div style="margin-bottom:1rem; padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem;">
            <div style="display:flex; justify-content:space-between; font-size:0.875rem; margin-bottom:0.25rem;">
              <span style="color:var(--text-secondary);">Subtotal</span>
              <span style="font-weight:600;">{formatCurrency(getSubtotal())}</span>
            </div>
            <div style="display:flex; justify-content:space-between; font-size:0.875rem; margin-bottom:0.25rem;">
              <span style="color:var(--text-secondary);">Tax (18%)</span>
              <span style="font-weight:600;">{formatCurrency(getTax())}</span>
            </div>
            <div style="display:flex; justify-content:space-between; font-size:1rem; font-weight:700; padding-top:0.5rem; border-top:1px solid var(--border);">
              <span>Total</span>
              <span style="color:var(--primary);">{formatCurrency(getGrandTotal())}</span>
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">Payment Method</label>
            <select class="form-select" bind:value={formPaymentMethod}>
              <option value="">Select payment method...</option>
              <option>Cash</option><option>UPI</option><option>Card</option><option>Bank Transfer</option>
            </select>
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Creating...' : 'Create Bill'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"