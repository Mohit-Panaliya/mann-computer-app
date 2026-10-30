<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let quotes: any[] = [];
  let showForm = false;
  let loading = false;

  let formName = '';
  let formPhone = '';
  let formSpecs = '';
  let formPrice = 0;
  let formNotes = '';

  onMount(async () => {
    loading = true;
    try { quotes = await api.getLaptopQuotes(); } catch (e) { console.error('Failed to load laptop quotes', e); }
    finally { loading = false; }
  });

  async function createQuote() {
    if (!formName || !formPhone) { alert('Please enter customer name and phone'); return; }
    if (!formSpecs) { alert('Please enter specifications'); return; }
    if (!formPrice || formPrice <= 0) { alert('Please enter a valid price'); return; }

    loading = true;
    try {
      await api.createLaptopQuote(formName, formPhone, formSpecs, formPrice, formNotes || null);
      showForm = false;
      formSpecs = ''; formPrice = 0; formNotes = '';
      quotes = await api.getLaptopQuotes();
    } catch (e: any) {
      console.error('Failed to create laptop quote', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function sendWhatsApp(q: any) {
    const msg = `Laptop Quote from Mann Computer\nQuote No: ${q.quote_no}\nSpecs: ${q.specs}\nPrice: ${formatCurrency(q.price)}`;
    window.open(generateWhatsAppLink(q.customer_phone || '', msg), '_blank');
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Laptop Quotes</h1>
      <p class="page-subtitle">Create quotations for laptop sales</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Quote</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Quote No</th><th>Customer</th><th>Phone</th><th>Specifications</th><th>Price</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each quotes as q}
            <tr>
              <td class="font-medium">{q.quote_no}</td>
              <td>{q.customer_name}</td>
              <td>{q.customer_phone}</td>
              <td style="max-width:200px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">{q.specs}</td>
              <td class="font-medium">{formatCurrency(q.price)}</td>
              <td><button class="btn btn-ghost btn-sm" on:click={() => sendWhatsApp(q)}>💬</button></td>
            </tr>
          {:else}
            <tr><td colspan="6" class="empty-state">No laptop quotes found.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">New Laptop Quote</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createQuote}>
          <div class="form-group">
            <label class="form-label">Customer Name</label>
            <input class="form-input" bind:value={formName} placeholder="Customer name" />
          </div>
          <div class="form-group">
            <label class="form-label">Phone Number</label>
            <input class="form-input" type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" />
          </div>
          <div class="form-group">
            <label class="form-label">Specifications</label>
            <textarea class="form-textarea" bind:value={formSpecs} placeholder="e.g. Intel i5 12th Gen, 16GB RAM, 512GB SSD, 15.6 inch FHD Display"></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">Price (₹)</label>
            <input class="form-input" type="number" bind:value={formPrice} placeholder="0" />
          </div>
          <div class="form-group">
            <label class="form-label">Notes</label>
            <input class="form-input" bind:value={formNotes} placeholder="Additional notes..." />
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Creating...' : 'Create Quote'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"