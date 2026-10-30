<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let quotes: any[] = [];
  let customers: any[] = [];
  let slips: any[] = [];
  let showForm = false;
  let loading = false;

  let formCustomerId = '';
  let formInwardId = '';
  let formItems: any[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formNotes = '';

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [quotes, customers, slips] = await Promise.all([api.getQuotations(), api.getCustomers(), api.getInwardSlips()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  function addItem() { formItems = [...formItems, { description: '', quantity: 1, unit_price: 0, total: 0 }]; }
  function removeItem(i: number) { formItems = formItems.filter((_, idx) => idx !== i); }
  function updateItemTotal(i: number) { formItems[i].total = formItems[i].quantity * formItems[i].unit_price; formItems = [...formItems]; }
  function getGrandTotal() { return formItems.reduce((sum, i) => sum + i.total, 0); }

  async function createQuote() {
    const validItems = formItems.filter(i => i.description && i.quantity > 0);
    if (validItems.length === 0) { alert('Please add at least one item'); return; }
    if (!formCustomerId) { alert('Please select a customer'); return; }

    loading = true;
    try {
      await api.createQuotation(formInwardId || null, formCustomerId, validItems, formNotes || null);
      showForm = false;
      formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
      formNotes = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create quotation', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function sendWhatsApp(q: any) {
    const itemsList = q.items.map((i: any) => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Quotation from Mann Computer\nQuote No: ${q.quotation_no}\n\n${itemsList}\n\nTotal: ${formatCurrency(q.total_amount)}`;
    const phone = slips.find((s: any) => s.id === q.inward_id)?.customer_phone || '';
    window.open(generateWhatsAppLink(phone, msg), '_blank');
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Quotations</h1>
      <p class="page-subtitle">Create and manage price quotations</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Quotation</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Quote No</th><th>Customer</th><th>Items</th><th>Total</th><th>Status</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each quotes as q}
            <tr>
              <td class="font-medium">{q.quotation_no}</td>
              <td>{q.customer_name}</td>
              <td>{q.items.length}</td>
              <td class="font-medium">{formatCurrency(q.total_amount)}</td>
              <td><span class="badge badge-draft">{q.status}</span></td>
              <td>
                <button class="btn btn-ghost btn-sm" on:click={() => sendWhatsApp(q)}>💬</button>
              </td>
            </tr>
          {:else}
            <tr><td colspan="6" class="empty-state">No quotations found.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal" style="max-width:42rem;">
        <div class="modal-header">
          <h2 class="modal-title">New Quotation</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createQuote}>
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
          <div style="text-align:right; margin-bottom:1rem; font-size:1.125rem; font-weight:700;">Grand Total: <span style="color:var(--primary);">{formatCurrency(getGrandTotal())}</span></div>
          <div class="form-group">
            <label class="form-label">Notes</label>
            <input class="form-input" bind:value={formNotes} placeholder="Additional notes..." />
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Creating...' : 'Create Quotation'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"