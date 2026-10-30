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
    try { parts = await api.getParts(); } catch (e) { console.error('Failed to load parts', e); }
  }

  async function createPart() {
    if (!formName || !formSellingPrice) { alert('Please enter part name and selling price'); return; }
    try {
      await api.createPart(formName, formCategory || null, formDescription || null, formPurchasePrice, formSellingPrice, formStock, formCompatible || null);
      showForm = false;
      formName = ''; formCategory = ''; formDescription = '';
      formPurchasePrice = 0; formSellingPrice = 0; formStock = 0; formCompatible = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create part', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  async function sellPart() {
    if (!saleCustomerName) { alert('Please enter customer name'); return; }
    try {
      await api.createPartSale(salePartId, null, saleCustomerName, saleQty, salePrice);
      showSale = false;
      await loadData();
    } catch (e: any) {
      console.error('Failed to record sale', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function openSale(p: any) {
    salePartId = p.id; salePrice = p.selling_price; saleQty = 1; saleCustomerName = ''; showSale = true;
  }

  function getStockBadge(qty: number) {
    if (qty === 0) return 'badge-delivered';
    if (qty < 5) return 'badge-pending';
    return 'badge-ready';
  }

  function getStockLabel(qty: number) {
    if (qty === 0) return 'Out of Stock';
    if (qty < 5) return 'Low Stock';
    return 'In Stock';
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Parts Inventory</h1>
      <p class="page-subtitle">Manage spare parts stock and sales</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ Add Part</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Part Name</th><th>Category</th><th>Stock</th><th>Purchase</th><th>Selling</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each parts as p}
            <tr>
              <td class="font-medium">{p.name}</td>
              <td>{p.category || '-'}</td>
              <td>
                <span class="badge {getStockBadge(p.stock_quantity)}">{p.stock_quantity}</span>
                <div style="font-size:0.7rem; color:var(--text-muted); margin-top:0.125rem;">{getStockLabel(p.stock_quantity)}</div>
              </td>
              <td>{formatCurrency(p.purchase_price)}</td>
              <td class="font-medium">{formatCurrency(p.selling_price)}</td>
              <td><button class="btn btn-success btn-sm" on:click={() => openSale(p)}>Sell</button></td>
            </tr>
          {:else}
            <tr><td colspan="6" class="empty-state">No parts found. Add your first part.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">Add Part</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createPart}>
          <div class="form-group">
            <label class="form-label">Part Name</label>
            <input class="form-input" bind:value={formName} placeholder="e.g. HP Laptop Screen" />
          </div>
          <div class="form-group">
            <label class="form-label">Category</label>
            <input class="form-input" bind:value={formCategory} placeholder="e.g. Display, Battery" />
          </div>
          <div class="form-group">
            <label class="form-label">Description</label>
            <input class="form-input" bind:value={formDescription} placeholder="Part description..." />
          </div>
          <div style="display:grid; grid-template-columns:1fr 1fr; gap:0.75rem;">
            <div class="form-group">
              <label class="form-label">Purchase Price (₹)</label>
              <input class="form-input" type="number" bind:value={formPurchasePrice} placeholder="0" />
            </div>
            <div class="form-group">
              <label class="form-label">Selling Price (₹)</label>
              <input class="form-input" type="number" bind:value={formSellingPrice} placeholder="0" />
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">Stock Quantity</label>
            <input class="form-input" type="number" bind:value={formStock} placeholder="0" min="0" />
          </div>
          <div class="form-group">
            <label class="form-label">Compatible With</label>
            <input class="form-input" bind:value={formCompatible} placeholder="e.g. Dell XPS 15" />
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary">Add Part</button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showSale}
    <div class="modal-overlay" on:click|self={() => showSale = false}>
      <div class="modal" style="max-width:24rem;">
        <div class="modal-header">
          <h2 class="modal-title">Sell Part</h2>
          <button class="modal-close" on:click={() => showSale = false}>&times;</button>
        </div>
        <div class="form-group">
          <label class="form-label">Customer Name</label>
          <input class="form-input" bind:value={saleCustomerName} placeholder="Customer name" />
        </div>
        <div class="form-group">
          <label class="form-label">Quantity</label>
          <input class="form-input" type="number" bind:value={saleQty} min="1" />
        </div>
        <div class="form-group">
          <label class="form-label">Unit Price (₹)</label>
          <input class="form-input" type="number" bind:value={salePrice} />
        </div>
        <div style="padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem; margin-bottom:1rem;">
          <div style="display:flex; justify-content:space-between; font-size:0.875rem;">
            <span style="color:var(--text-secondary);">Total</span>
            <span style="font-size:1.125rem; font-weight:700; color:var(--primary);">{formatCurrency(saleQty * salePrice)}</span>
          </div>
        </div>
        <div style="display:flex; justify-content:flex-end; gap:0.75rem;">
          <button type="button" class="btn btn-secondary" on:click={() => showSale = false}>Cancel</button>
          <button class="btn btn-success" on:click={sellPart}>Record Sale</button>
        </div>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"