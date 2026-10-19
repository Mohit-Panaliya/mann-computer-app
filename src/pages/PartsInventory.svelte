<script lang="ts">
  import { onMount } from 'svelte';
  import { api, Part } from '../lib/api';
  import { formatCurrency } from '../lib/utils';

  let parts: Part[] = [];
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

  onMount(async () => {
    parts = await api.getParts();
  });

  async function createPart() {
    await api.createPart(formName, formCategory || null, formDescription || null, formPurchasePrice, formSellingPrice, formStock, formCompatible || null);
    showForm = false;
    formName = '';
    formCategory = '';
    formDescription = '';
    formPurchasePrice = 0;
    formSellingPrice = 0;
    formStock = 0;
    formCompatible = '';
    parts = await api.getParts();
  }

  async function sellPart() {
    await api.createPartSale(salePartId, null, saleCustomerName || null, saleQty, salePrice);
    showSale = false;
    parts = await api.getParts();
  }

  function openSale(p: Part) {
    salePartId = p.id;
    salePrice = p.selling_price;
    saleQty = 1;
    saleCustomerName = '';
    showSale = true;
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Parts Inventory</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ Add Part</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Name</th><th>Category</th><th>Stock</th><th>Purchase Price</th><th>Selling Price</th><th>Actions</th></tr></thead>
      <tbody>
        {#each parts as p}
          <tr>
            <td>{p.name}</td>
            <td>{p.category || '-'}</td>
            <td class:low-stock={p.stock_quantity < 5}>{p.stock_quantity}</td>
            <td>{formatCurrency(p.purchase_price)}</td>
            <td>{formatCurrency(p.selling_price)}</td>
            <td class="actions">
              <button class="btn btn-sm btn-primary" on:click={() => openSale(p)}>Sell</button>
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
          <h2 class="modal-title">Add Part</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Name</label><input class="input" bind:value={formName} /></div>
        <div class="form-group"><label class="form-label">Category</label><input class="input" bind:value={formCategory} /></div>
        <div class="form-group"><label class="form-label">Description</label><input class="input" bind:value={formDescription} /></div>
        <div class="grid-2">
          <div class="form-group"><label class="form-label">Purchase Price</label><input class="input" type="number" bind:value={formPurchasePrice} /></div>
          <div class="form-group"><label class="form-label">Selling Price</label><input class="input" type="number" bind:value={formSellingPrice} /></div>
        </div>
        <div class="form-group"><label class="form-label">Stock Quantity</label><input class="input" type="number" bind:value={formStock} /></div>
        <div class="form-group"><label class="form-label">Compatible With</label><input class="input" bind:value={formCompatible} /></div>
        <button class="btn btn-primary" on:click={createPart}>Add Part</button>
      </div>
    </div>
  {/if}

  {#if showSale}
    <div class="modal-overlay" on:click|self={() => showSale = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">Sell Part</h2>
          <button class="close-btn" on:click={() => showSale = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Customer Name</label><input class="input" bind:value={saleCustomerName} /></div>
        <div class="form-group"><label class="form-label">Quantity</label><input class="input" type="number" bind:value={saleQty} min="1" /></div>
        <div class="form-group"><label class="form-label">Unit Price</label><input class="input" type="number" bind:value={salePrice} /></div>
        <p>Total: <strong>{formatCurrency(saleQty * salePrice)}</strong></p>
        <button class="btn btn-primary" on:click={sellPart}>Record Sale</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
  .low-stock { color: #ea4335; font-weight: 700; }
</style>
