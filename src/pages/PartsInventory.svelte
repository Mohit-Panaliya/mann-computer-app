<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency } from '../lib/utils';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Badge from '$lib/components/ui/badge.svelte';
  import Button from '$lib/components/ui/button.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Dialog from '$lib/components/ui/dialog.svelte';

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
      formName = '';
      formCategory = '';
      formDescription = '';
      formPurchasePrice = 0;
      formSellingPrice = 0;
      formStock = 0;
      formCompatible = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create part', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
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
    } catch (e: any) {
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
    if (qty === 0) return 'bg-destructive text-destructive-foreground';
    if (qty < 5) return 'bg-yellow-100 text-yellow-800';
    return 'bg-green-100 text-green-800';
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
      <h1 class="text-2xl font-bold text-foreground">Parts Inventory</h1>
      <p class="text-muted-foreground mt-1">Manage spare parts stock and sales</p>
    </div>
    <Button variant="default" on:click={() => showForm = true}>
      <span class="mr-2">+</span> Add Part
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="py-3">Part Name</th>
              <th class="w-32 py-3">Category</th>
              <th class="w-24 py-3 text-center">Stock</th>
              <th class="w-24 py-3 text-right">Purchase Price</th>
              <th class="w-24 py-3 text-right">Selling Price</th>
              <th class="w-32 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each parts as p}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td class="py-3 text-sm font-medium text-foreground">{p.name}</td>
                <td class="py-3 text-sm text-muted-foreground">{p.category || '-'}</td>
                <td class="py-3 text-center">
                    <span class={`${getStockBadge(p.stock_quantity)} text-xs font-bold`}>{p.stock_quantity}</span>
                  <div class="text-[10px] text-muted-foreground">{getStockLabel(p.stock_quantity)}</div>
                </td>
                <td class="py-3 text-sm text-muted-foreground text-right">{formatCurrency(p.purchase_price)}</td>
                <td class="py-3 text-sm font-medium text-foreground text-right">{formatCurrency(p.selling_price)}</td>
                <td class="py-3 text-right">
                  <Button variant="secondary" size="sm" class="px-3" on:click={() => openSale(p)}>Sell</Button>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" class="py-8 text-center text-muted-foreground">No parts found. Add your first part.</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>

  {#if showForm}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-md p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-foreground">Add Part</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={createPart} class="space-y-4">
          <div class="space-y-2">
            <Label>Part Name</Label>
            <Input bind:value={formName} placeholder="e.g. HP Laptop Screen" required />
          </div>
          <div class="space-y-2">
            <Label>Category</Label>
            <Input bind:value={formCategory} placeholder="e.g. Display, Battery" />
          </div>
          <div class="space-y-2">
            <Label>Description</Label>
            <Input bind:value={formDescription} placeholder="Part description..." />
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label>Purchase Price (₹)</Label>
              <Input type="number" bind:value={formPurchasePrice} placeholder="0" />
            </div>
            <div class="space-y-2">
              <Label>Selling Price (₹)</Label>
              <Input type="number" bind:value={formSellingPrice} placeholder="0" required />
            </div>
          </div>
          <div class="space-y-2">
            <Label>Stock Quantity</Label>
            <Input type="number" bind:value={formStock} placeholder="0" min="0" />
          </div>
          <div class="space-y-2">
            <Label>Compatible With</Label>
            <Input bind:value={formCompatible} placeholder="e.g. Dell XPS 15" />
          </div>
          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default">Add Part</Button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showSale}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-sm p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-foreground">Sell Part</h2>
          <button on:click={() => showSale = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <div class="space-y-4">
          <div class="space-y-2">
            <Label>Customer Name</Label>
            <Input bind:value={saleCustomerName} placeholder="Customer name" required />
          </div>
          <div class="space-y-2">
            <Label>Quantity</Label>
            <Input type="number" bind:value={saleQty} min="1" required />
          </div>
          <div class="space-y-2">
            <Label>Unit Price (₹)</Label>
            <Input type="number" bind:value={salePrice} required />
          </div>
          <div class="pt-2 border-t border-border">
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">Total</span>
              <span class="text-lg font-bold text-primary">{formatCurrency(saleQty * salePrice)}</span>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-4">
          <Button variant="outline" on:click={() => showSale = false}>Cancel</Button>
          <Button variant="default" on:click={sellPart}>Record Sale</Button>
        </div>
      </div>
    </div>
  {/if}
</div>
