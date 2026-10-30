<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { openExternal, formatCurrency, generateWhatsAppLink } from '../lib/utils';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Button from '$lib/components/ui/button.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Textarea from '$lib/components/ui/textarea.svelte';
  import Dialog from '$lib/components/ui/dialog.svelte';

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
      formSpecs = '';
      formPrice = 0;
      formNotes = '';
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
    openExternal(generateWhatsAppLink(q.customer_phone || '', msg));
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-foreground">Laptop Quotes</h1>
      <p class="text-muted-foreground mt-1">Create quotations for laptop sales</p>
    </div>
    <Button variant="default" on:click={() => showForm = true}>
      <span class="mr-2">+</span> New Quote
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto -mx-4 sm:mx-0">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="w-24 py-3">Quote No</th>
              <th class="py-3">Customer</th>
              <th class="py-3">Phone</th>
              <th class="py-3">Specifications</th>
              <th class="w-24 py-3 text-right">Price</th>
              <th class="w-32 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each quotes as q}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td class="py-3 text-sm font-mono font-medium text-foreground">{q.quote_no}</td>
                <td class="py-3 text-sm font-medium text-foreground">{q.customer_name}</td>
                <td class="py-3 text-sm text-muted-foreground">{q.customer_phone}</td>
                <td class="py-3 text-sm text-muted-foreground truncate max-w-xs" title={q.specs}>{q.specs}</td>
                <td class="py-3 text-sm font-medium text-foreground text-right">{formatCurrency(q.price)}</td>
                <td class="py-3 text-right">
                  <Button variant="ghost" size="sm" class="p-2 text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(q)} title="Send WhatsApp">
                    <span class="text-lg">💬</span>
                  </Button>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" class="py-8 text-center text-muted-foreground">No laptop quotes found.</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>

  {#if showForm}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-lg p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-foreground">New Laptop Quote</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={createQuote} class="space-y-4">
          <div class="space-y-2">
            <Label>Customer Name</Label>
            <Input bind:value={formName} placeholder="Customer name" required />
          </div>
          <div class="space-y-2">
            <Label>Phone Number</Label>
            <Input type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" required />
          </div>
          <div class="space-y-2">
            <Label>Specifications</Label>
            <Textarea bind:value={formSpecs} rows="4" placeholder="e.g. Intel i5 12th Gen, 16GB RAM, 512GB SSD, 15.6 inch FHD Display" required />
          </div>
          <div class="space-y-2">
            <Label>Price (₹)</Label>
            <Input type="number" bind:value={formPrice} placeholder="0" required />
          </div>
          <div class="space-y-2">
            <Label>Notes</Label>
            <Input bind:value={formNotes} placeholder="Additional notes..." />
          </div>
          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default" disabled={loading}>
              {loading ? 'Creating...' : 'Create Quote'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
