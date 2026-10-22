<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency } from '../lib/utils';

  let parts: any[] = [];
  let showForm = false;
  let showSale = false;
  let salePartId = '';
  let saleQty = 1;
  let salePrice = 0;
  let saleCustomerName = '';

  let formName = '';
  let formCategory = '';
  let formDescription = '';
  let formPurchasePrice = 0;
  let formSellingPrice = 0;
  let formStock = 0;
  let formCompatible = '';

  onMount(loadData);

  async function loadData() {
    try {
      parts = await api.getParts();
    } catch (e) {
      console.error('Failed to load parts', e);
    }
  }

  async function createPart() {
    if (!formName || !formSellingPrice) {
      alert('Please enter part name and selling price');
      return;
    }
    try {
      await api.createPart(formName, formCategory || null, formDescription || null, formPurchasePrice, formSellingPrice, formStock, formCompatible || null);
      showForm = false;
      resetForm();
      await loadData();
    } catch (e) {
      console.error('Failed to create part', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function resetForm() {
    formName = '';
    formCategory = '';
    formDescription = '';
    formPurchasePrice = 0;
    formSellingPrice = 0;
    formStock = 0;
    formCompatible = '';
  }

  async function sellPart() {
    if (!saleCustomerName) {
      alert('Please enter customer name');
      return;
    }
    try {
      await api.createPartSale(salePartId, null, saleCustomerName, saleQty, salePrice);
      showSale = false;
      await loadData();
    } catch (e) {
      console.error('Failed to record sale', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function openSale(p: any) {
    salePartId = p.id;
    salePrice = p.selling_price;
    saleQty = 1;
    saleCustomerName = '';
    showSale = true;
  }

  function getStockBadge(qty: number) {
    if (qty === 0) return 'badge bg-red-100 text-red-800';
    if (qty < 5) return 'badge bg-yellow-100 text-yellow-800';
    return 'badge bg-green-100 text-green-800';
  }

  function getStockLabel(qty: number) {
    if (qty === 0) return 'Out of Stock';
    if (qty < 5) return 'Low Stock';
    return 'In Stock';
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Parts Inventory</h1>
      <p class="text-gray-600 mt-1">Manage spare parts stock and sales</p>
    </div>
    <button class="btn btn-primary" on:click={() => { showForm = true; }}>
      <span class="mr-2">+</span> Add Part
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left">Part Name</th>
          <th class="text-left w-32">Category</th>
          <th class="text-center w-24">Stock</th>
          <th class="text-right w-24">Purchase Price</th>
          <th class="text-right w-24">Selling Price</th>
          <th class="text-right w-32">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each parts as p}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-medium">{p.name}</td>
            <td class="text-gray-600">{p.category || '-'}</td>
            <td class="text-center">
              <span class="{getStockBadge(p.stock_quantity)}">{p.stock_quantity}</span>
              <div class="text-[10px] text-gray-500 mt-0.5">{getStockLabel(p.stock_quantity)}</div>
            </td>
            <td class="text-right text-gray-600">{formatCurrency(p.purchase_price)}</td>
            <td class="text-right text-gray-700 font-medium">{formatCurrency(p.selling_price)}</td>
            <td class="text-right">
              <button class="btn btn-success btn-sm" on:click={() => openSale(p)}>Sell</button>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="text-center py-8 text-gray-500">No parts found. Add your first part.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">Add Part</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createPart} class="space-y-4">
          <div class="space-y-2">
            <label class="label">Part Name</label>
            <input class="input" bind:value={formName} placeholder="e.g. HP Laptop Screen" required />
          </div>

          <div class="space-y-2">
            <label class="label">Category</label>
            <input class="input" bind:value={formCategory} placeholder="e.g. Display, Battery" />
          </div>

          <div class="space-y-2">
            <label class="label">Description</label>
            <input class="input" bind:value={formDescription} placeholder="Part description..." />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="label">Purchase Price (₹)</label>
              <input class="input" type="number" bind:value={formPurchasePrice} placeholder="0" />
            </div>
            <div class="space-y-2">
              <label class="label">Selling Price (₹)</label>
              <input class="input" type="number" bind:value={formSellingPrice} placeholder="0" required />
            </div>
          </div>

          <div class="space-y-2">
            <label class="label">Stock Quantity</label>
            <input class="input" type="number" bind:value={formStock} placeholder="0" min="0" />
          </div>

          <div class="space-y-2">
            <label class="label">Compatible With</label>
            <input class="input" bind:value={formCompatible} placeholder="e.g. Dell XPS 15" />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary">Add Part</button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showSale}
    <div class="modal-overlay" on:click|self={() => showSale = false}>
      <div class="modal max-w-sm">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">Sell Part</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showSale = false}>&times;</button>
        </div>
        
        <div class="space-y-4">
          <div class="space-y-2">
            <label class="label">Customer Name</label>
            <input class="input" bind:value={saleCustomerName} placeholder="Customer name" required />
          </div>
          <div class="space-y-2">
            <label class="label">Quantity</label>
            <input class="input" type="number" bind:value={saleQty} min="1" required />
          </div>
          <div class="space-y-2">
            <label class="label">Unit Price (₹)</label>
            <input class="input" type="number" bind:value={salePrice} required />
          </div>
          <div class="pt-2 border-t border-gray-100">
            <div class="flex justify-between text-sm">
              <span class="text-gray-600">Total</span>
              <span class="text-lg font-bold text-primary-600">{formatCurrency(saleQty * salePrice)}</span>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-4">
          <button type="button" class="btn btn-outline" on:click={() => showSale = false}>Cancel</button>
          <button class="btn btn-success" on:click={sellPart}>Record Sale</button>
        </div>
      </div>
    </div>
  {/if}
</div>
