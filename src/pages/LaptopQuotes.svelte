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
    try {
      quotes = await api.getLaptopQuotes();
    } catch (e) {
      console.error('Failed to load laptop quotes', e);
    } finally {
      loading = false;
    }
  });

  async function createQuote() {
    if (!formName || !formPhone) {
      alert('Please enter customer name and phone');
      return;
    }
    if (!formSpecs) {
      alert('Please enter specifications');
      return;
    }
    if (!formPrice || formPrice <= 0) {
      alert('Please enter a valid price');
      return;
    }

    loading = true;
    try {
      await api.createLaptopQuote(formName, formPhone, formSpecs, formPrice, formNotes || null);
      showForm = false;
      resetForm();
      loading = false;
      quotes = await api.getLaptopQuotes();
    } catch (e) {
      console.error('Failed to create laptop quote', e);
      alert('Failed to create quote');
      loading = false;
    }
  }

  function resetForm() {
    formName = '';
    formPhone = '';
    formSpecs = '';
    formPrice = 0;
    formNotes = '';
  }

  async function sendWhatsApp(q: any) {
    const msg = `Laptop Quote from Mann Computer\nQuote No: ${q.quote_no}\nSpecs: ${q.specs}\nPrice: ${formatCurrency(q.price)}`;
    window.open(generateWhatsAppLink(q.customer_phone || '', msg), '_blank');
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Laptop Quotes</h1>
      <p class="text-gray-600 mt-1">Create quotations for laptop sales</p>
    </div>
    <button class="btn btn-primary" on:click={() => { showForm = true; }}>
      <span class="mr-2">+</span> New Quote
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left w-24">Quote No</th>
          <th class="text-left">Customer</th>
          <th class="text-left">Phone</th>
          <th class="text-left">Specifications</th>
          <th class="text-right w-24">Price</th>
          <th class="text-right w-32">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each quotes as q}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-mono text-sm font-medium">{q.quote_no}</td>
            <td class="text-gray-700 font-medium">{q.customer_name}</td>
            <td class="text-gray-600">{q.customer_phone}</td>
            <td class="text-gray-600 truncate max-w-xs" title={q.specs}>{q.specs}</td>
            <td class="text-right text-gray-700 font-medium">{formatCurrency(q.price)}</td>
            <td class="text-right">
              <button class="btn btn-ghost btn-sm text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(q)} title="Send WhatsApp">
                <span class="text-lg">💬</span>
              </button>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="text-center py-8 text-gray-500">No laptop quotes found.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-lg">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">New Laptop Quote</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createQuote} class="space-y-4">
          <div class="space-y-2">
            <label class="label">Customer Name</label>
            <input class="input" bind:value={formName} placeholder="Customer name" required />
          </div>

          <div class="space-y-2">
            <label class="label">Phone Number</label>
            <input class="input" type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" required />
          </div>

          <div class="space-y-2">
            <label class="label">Specifications</label>
            <textarea class="input" bind:value={formSpecs} rows="4" placeholder='e.g. Intel i5 12th Gen, 16GB RAM, 512GB SSD, 15.6" FHD Display' required></textarea>
          </div>

          <div class="space-y-2">
            <label class="label">Price (₹)</label>
            <input class="input" type="number" bind:value={formPrice} placeholder="0" required />
          </div>

          <div class="space-y-2">
            <label class="label">Notes</label>
            <input class="input" bind:value={formNotes} placeholder="Additional notes..." />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>
              {loading ? 'Creating...' : 'Create Quote'}
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
