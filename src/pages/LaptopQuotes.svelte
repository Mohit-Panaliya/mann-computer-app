<script lang="ts">
  import { onMount } from 'svelte';
  import { api, LaptopQuote } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let quotes: LaptopQuote[] = [];
  let showForm = false;

  let formName = '';
  let formPhone = '';
  let formSpecs = '';
  let formPrice = 0;
  let formNotes = '';

  onMount(async () => {
    quotes = await api.getLaptopQuotes();
  });

  async function createQuote() {
    await api.createLaptopQuote(formName, formPhone, formSpecs, formPrice, formNotes || null);
    showForm = false;
    formSpecs = '';
    formPrice = 0;
    formNotes = '';
    quotes = await api.getLaptopQuotes();
  }

  async function sendWhatsApp(q: LaptopQuote) {
    const msg = `Laptop Quote from Mann Computer\nQuote No: ${q.quote_no}\nSpecs: ${q.specs}\nPrice: ${formatCurrency(q.price)}`;
    window.open(generateWhatsAppLink(q.customer_phone || '', msg), '_blank');
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Laptop Quotes</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Quote</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Quote No</th><th>Customer</th><th>Specs</th><th>Price</th><th>Actions</th></tr></thead>
      <tbody>
        {#each quotes as q}
          <tr>
            <td>{q.quote_no}</td>
            <td>{q.customer_name}</td>
            <td>{q.specs}</td>
            <td>{formatCurrency(q.price)}</td>
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
          <h2 class="modal-title">New Laptop Quote</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Customer Name</label><input class="input" bind:value={formName} /></div>
        <div class="form-group"><label class="form-label">Phone</label><input class="input" bind:value={formPhone} /></div>
        <div class="form-group"><label class="form-label">Specifications</label><textarea class="input" bind:value={formSpecs} rows="4" placeholder="e.g. Intel i5 12th Gen, 16GB RAM, 512GB SSD, 15.6&quot; FHD Display"></textarea></div>
        <div class="form-group"><label class="form-label">Price</label><input class="input" type="number" bind:value={formPrice} /></div>
        <div class="form-group"><label class="form-label">Notes</label><input class="input" bind:value={formNotes} /></div>
        <button class="btn btn-primary" on:click={createQuote}>Create Quote</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
</style>
