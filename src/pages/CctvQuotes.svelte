<script lang="ts">
  import { onMount } from 'svelte';
  import { api, CctvQuote, QuotationItem } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let quotes: CctvQuote[] = [];
  let showForm = false;

  let formName = '';
  let formPhone = '';
  let formItems: QuotationItem[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formNotes = '';

  onMount(async () => {
    quotes = await api.getCctvQuotes();
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
    await api.createCctvQuote(formName, formPhone, items, formNotes || null);
    showForm = false;
    formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
    quotes = await api.getCctvQuotes();
  }

  async function sendWhatsApp(q: CctvQuote) {
    const itemsList = q.items.map(i => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `CCTV Quote from Mann Computer\nQuote No: ${q.quote_no}\n\n${itemsList}\n\nTotal: ${formatCurrency(q.total_amount)}`;
    window.open(generateWhatsAppLink(q.customer_phone || '', msg), '_blank');
  }

  const commonItems = [
    'Hikvision 2MP Bullet Camera',
    'Hikvision 2MP Dome Camera',
    'Dahua 2MP Bullet Camera',
    'Dahua 2MP Dome Camera',
    '4CH DVR',
    '8CH DVR',
    '16CH DVR',
    '1TB HDD',
    '2TB HDD',
    'Power Supply 12V 2A',
    'BNC Connector',
    'RG59 Cable (per meter)',
    'Installation Charges',
  ];
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">CCTV Quotes</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Quote</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Quote No</th><th>Customer</th><th>Phone</th><th>Items</th><th>Total</th><th>Actions</th></tr></thead>
      <tbody>
        {#each quotes as q}
          <tr>
            <td>{q.quote_no}</td>
            <td>{q.customer_name}</td>
            <td>{q.customer_phone}</td>
            <td>{q.items.length}</td>
            <td>{formatCurrency(q.total_amount)}</td>
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
          <h2 class="modal-title">New CCTV Quote</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Customer Name</label><input class="input" bind:value={formName} /></div>
        <div class="form-group"><label class="form-label">Phone</label><input class="input" bind:value={formPhone} /></div>
        <div class="items-section">
          <label class="form-label">Items</label>
          <div class="quick-items">
            {#each commonItems as item}
              <button class="quick-item" on:click={() => { formItems = [...formItems, { description: item, quantity: 1, unit_price: 0, total: 0 }]; }}>{item}</button>
            {/each}
          </div>
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
        <button class="btn btn-primary" on:click={createQuote}>Create Quote</button>
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
  .quick-items { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 12px; }
  .quick-item { padding: 4px 8px; font-size: 11px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg); cursor: pointer; }
  .quick-item:hover { background: #e8f0fe; }
</style>
