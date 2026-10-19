<script lang="ts">
  import { onMount } from 'svelte';
  import { api, Quotation, QuotationItem, Customer, InwardSlip } from '../lib/api';
  import { formatCurrency, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

  let quotes: Quotation[] = [];
  let customers: Customer[] = [];
  let slips: InwardSlip[] = [];
  let showForm = false;

  let formCustomerId = '';
  let formInwardId = '';
  let formItems: QuotationItem[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formNotes = '';

  onMount(async () => {
    [quotes, customers, slips] = await Promise.all([api.getQuotations(), api.getCustomers(), api.getInwardSlips()]);
  });

  function addItem() {
    formItems = [...formItems, { description: '', quantity: 1, unit_price: 0, total: 0 }];
  }

  function removeItem(i: number) {
    formItems = formItems.filter((_, idx) => idx !== i);
  }

  function updateItemTotal(i: number) {
    formItems[i].total = formItems[i].quantity * formItems[i].unit_price;
    formItems = [...formItems];
  }

  async function createQuote() {
    const items = formItems.filter(i => i.description && i.quantity > 0);
    const inward = formInwardId || null;
    await api.createQuotation(inward, formCustomerId, items, formNotes || null);
    showForm = false;
    formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
    formNotes = '';
    quotes = await api.getQuotations();
  }

  async function sendWhatsApp(q: Quotation) {
    const itemsList = q.items.map(i => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Quotation from Mann Computer\nQuote No: ${q.quotation_no}\n\n${itemsList}\n\nTotal: ${formatCurrency(q.total_amount)}`;
    const phone = slips.find(s => s.id === q.inward_id)?.customer_phone || '';
    window.open(generateWhatsAppLink(phone, msg), '_blank');
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Quotations</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Quotation</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Quote No</th><th>Customer</th><th>Items</th><th>Total</th><th>Status</th><th>Actions</th></tr></thead>
      <tbody>
        {#each quotes as q}
          <tr>
            <td>{q.quotation_no}</td>
            <td>{q.customer_name}</td>
            <td>{q.items.length}</td>
            <td>{formatCurrency(q.total_amount)}</td>
            <td><span class="badge {getStatusClass(q.status)}">{getStatusLabel(q.status)}</span></td>
            <td class="actions">
              <button class="btn btn-sm btn-outline" on:click={() => sendWhatsApp(q)}>WA Send</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">New Quotation</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Link to Inward (optional)</label>
          <select class="input" bind:value={formInwardId}>
            <option value="">None</option>
            {#each slips as s}<option value={s.id}>#{s.slip_no} - {s.customer_name}</option>{/each}
          </select>
        </div>
        <div class="form-group"><label class="form-label">Customer</label>
          <select class="input" bind:value={formCustomerId}>
            <option value="">Select...</option>
            {#each customers as c}<option value={c.id}>{c.name}</option>{/each}
          </select>
        </div>
        <div class="items-section">
          <label class="form-label">Items</label>
          {#each formItems as item, i}
            <div class="item-row">
              <input class="input" placeholder="Description" bind:value={item.description} />
              <input class="input" type="number" placeholder="Qty" bind:value={item.quantity} on:change={() => updateItemTotal(i)} />
              <input class="input" type="number" placeholder="Rate" bind:value={item.unit_price} on:change={() => updateItemTotal(i)} />
              <span class="item-total">{formatCurrency(item.total)}</span>
              <button class="btn btn-sm btn-danger" on:click={() => removeItem(i)}>X</button>
            </div>
          {/each}
          <button class="btn btn-sm btn-outline" on:click={addItem}>+ Add Item</button>
        </div>
        <div class="form-group"><label class="form-label">Notes</label><input class="input" bind:value={formNotes} /></div>
        <button class="btn btn-primary" on:click={createQuote}>Create Quotation</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
  .item-row { display: flex; gap: 8px; margin-bottom: 8px; align-items: center; }
  .item-row .input { flex: 1; }
  .item-total { min-width: 80px; text-align: right; font-weight: 600; }
  .items-section { margin: 12px 0; }
</style>
