<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency, generateWhatsAppLink } from '../lib/utils';

  let quotes: any[] = [];
  let showForm = false;
  let loading = false;

  let formName = '';
  let formPhone = '';
  let formItems: any[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formNotes = '';

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

  onMount(async () => {
    loading = true;
    try {
      quotes = await api.getCctvQuotes();
    } catch (e) {
      console.error('Failed to load CCTV quotes', e);
    } finally {
      loading = false;
    }
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

  function getGrandTotal() {
    return formItems.reduce((sum, i) => sum + i.total, 0);
  }

  async function createQuote() {
    const validItems = formItems.filter(i => i.description && i.quantity > 0 && i.unit_price >= 0);
    if (validItems.length === 0) {
      alert('Please add at least one item');
      return;
    }
    if (!formName || !formPhone) {
      alert('Please enter customer name and phone');
      return;
    }

    loading = true;
    try {
      await api.createCctvQuote(formName, formPhone, validItems, formNotes || null);
      showForm = false;
      resetForm();
      loading = false;
      quotes = await api.getCctvQuotes();
    } catch (e) {
      console.error('Failed to create CCTV quote', e);
      alert('Failed: ' + (e?.message || String(e)));
      loading = false;
    }
  }

  function resetForm() {
    formName = '';
    formPhone = '';
    formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
    formNotes = '';
  }

  async function sendWhatsApp(q: any) {
    const itemsList = q.items.map((i: any) => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `CCTV Quote from Mann Computer\nQuote No: ${q.quote_no}\n\n${itemsList}\n\nTotal: ${formatCurrency(q.total_amount)}`;
    window.open(generateWhatsAppLink(q.customer_phone || '', msg), '_blank');
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">CCTV Quotes</h1>
      <p class="text-gray-600 mt-1">Create quotations for CCTV camera systems</p>
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
          <th class="text-left w-32">Phone</th>
          <th class="text-center w-20">Items</th>
          <th class="text-right w-24">Total</th>
          <th class="text-right w-32">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each quotes as q}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-mono text-sm font-medium">{q.quote_no}</td>
            <td class="text-gray-700 font-medium">{q.customer_name}</td>
            <td class="text-gray-600">{q.customer_phone}</td>
            <td class="text-center text-gray-600">{q.items.length}</td>
            <td class="text-right text-gray-700 font-medium">{formatCurrency(q.total_amount)}</td>
            <td class="text-right">
              <button class="btn btn-ghost btn-sm text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(q)} title="Send WhatsApp">
                <span class="text-lg">💬</span>
              </button>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="text-center py-8 text-gray-500">No CCTV quotes found.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-2xl">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">New CCTV Quote</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createQuote} class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="label">Customer Name</label>
              <input class="input" bind:value={formName} placeholder="Customer name" required />
            </div>
            <div class="space-y-2">
              <label class="label">Phone Number</label>
              <input class="input" type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" required />
            </div>
          </div>

          <div class="space-y-2">
            <label class="label">Quick Add Items</label>
            <div class="flex flex-wrap gap-2 mb-2">
              {#each commonItems as item}
                <button type="button" class="px-3 py-1.5 text-xs border border-gray-200 rounded-lg bg-white hover:bg-primary-50 hover:text-primary-600 transition-colors" on:click={() => {
                  formItems = [...formItems, { description: item, quantity: 1, unit_price: 0, total: 0 }];
                }}>
                  {item}
                </button>
              {/each}
            </div>
          </div>

          <div class="space-y-2">
            <label class="label">Items</label>
            <div class="space-y-2">
              {#each formItems as item, i}
                <div class="flex gap-2 items-start">
                  <input class="input flex-1" placeholder="Description" bind:value={item.description} />
                  <input class="input w-20" type="number" placeholder="Qty" bind:value={item.quantity} on:change={() => updateItemTotal(i)} />
                  <input class="input w-24" type="number" placeholder="Rate" bind:value={item.unit_price} on:change={() => updateItemTotal(i)} />
                  <span class="w-20 text-right font-medium text-gray-700">{formatCurrency(item.total)}</span>
                  <button class="btn btn-danger btn-sm p-1.5" on:click={() => removeItem(i)}>×</button>
                </div>
              {/each}
              <button type="button" class="btn btn-outline btn-sm text-gray-600" on:click={addItem}>+ Add Item</button>
            </div>
            <div class="mt-2 text-right">
              <div class="text-lg font-bold text-gray-900">Grand Total: <span class="text-primary-600">{formatCurrency(getGrandTotal())}</span></div>
            </div>
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
