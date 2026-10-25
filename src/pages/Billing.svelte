<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { openExternal, formatCurrency, generateWhatsAppLink } from '../lib/utils';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Badge from '$lib/components/ui/badge.svelte';
  import Button from '$lib/components/ui/button.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Select from '$lib/components/ui/select.svelte';
  import Dialog from '$lib/components/ui/dialog.svelte';
  import Textarea from '$lib/components/ui/textarea.svelte';

  let bills: any[] = [];
  let customers: any[] = [];
  let slips: any[] = [];
  let showForm = false;
  let loading = false;

  let formCustomerId = '';
  let formInwardId = '';
  let formItems: any[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formPaymentMethod = '';

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [bills, customers, slips] = await Promise.all([api.getBills(), api.getCustomers(), api.getInwardSlips()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

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

  function getSubtotal() {
    return formItems.reduce((sum, i) => sum + i.total, 0);
  }

  function getTax() {
    return getSubtotal() * 0.18;
  }

  function getGrandTotal() {
    return getSubtotal() + getTax();
  }

  async function createBill() {
    const validItems = formItems.filter(i => i.description && i.quantity > 0 && i.unit_price >= 0);
    if (validItems.length === 0) {
      alert('Please add at least one item');
      return;
    }
    if (!formCustomerId) {
      alert('Please select a customer');
      return;
    }
    if (!formPaymentMethod) {
      alert('Please select payment method');
      return;
    }

    loading = true;
    try {
      await api.createBill(formInwardId || null, null, formCustomerId, validItems, formPaymentMethod);
      showForm = false;
      formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
      formPaymentMethod = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create bill', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function markPaid(b: any) {
    try {
      await api.updateBillPayment(b.id, 'paid', b.payment_method || 'Cash');
      await loadData();
    } catch (e: any) {
      console.error('Failed to mark paid', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  async function sendWhatsApp(b: any) {
    const itemsList = b.items.map((i: any) => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Bill from Mann Computer\nBill No: ${b.bill_no}\nDate: ${b.created_at}\n\n${itemsList}\n\nSubtotal: ${formatCurrency(b.subtotal)}\nTax: ${formatCurrency(b.tax)}\nTotal: ${formatCurrency(b.total_amount)}`;
    openExternal(generateWhatsAppLink(b.customer_phone || '', msg));
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-foreground">Billing</h1>
      <p class="text-muted-foreground mt-1">Generate invoices and track payments</p>
    </div>
    <Button variant="default" on:click={() => showForm = true}>
      <span class="mr-2">+</span> New Bill
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="w-24 py-3">Bill No</th>
              <th class="py-3">Customer</th>
              <th class="w-20 py-3 text-center">Items</th>
              <th class="w-24 py-3 text-right">Total</th>
              <th class="w-32 py-3 text-center">Payment</th>
              <th class="w-40 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each bills as b}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td class="py-3 text-sm font-mono font-medium text-foreground">{b.bill_no}</td>
                <td class="py-3 text-sm font-medium text-foreground">{b.customer_name}</td>
                <td class="py-3 text-sm text-muted-foreground text-center">{b.items.length}</td>
                <td class="py-3 text-sm font-medium text-foreground text-right">{formatCurrency(b.total_amount)}</td>
                <td class="py-3 text-center"><Badge variant={b.payment_status === 'paid' ? 'secondary' : 'outline'} class="text-xs">{b.payment_status}</Badge></td>
                <td class="py-3 text-right">
                  <div class="flex items-center gap-2 justify-end">
                    {#if b.payment_status === 'pending'}
                      <Button variant="secondary" size="sm" class="px-3" on:click={() => markPaid(b)}>Mark Paid</Button>
                    {/if}
                    <Button variant="ghost" size="sm" class="p-2" on:click={() => openExternal(`file://${b.pdf_path || ''}`)} title="PDF">
                      <span class="text-lg">📄</span>
                    </Button>
                    <Button variant="ghost" size="sm" class="p-2 text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(b)} title="WhatsApp">
                      <span class="text-lg">💬</span>
                    </Button>
                  </div>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" class="py-8 text-center text-muted-foreground">No bills found.</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>

  {#if showForm}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-2xl p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-foreground">New Bill</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={createBill} class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label>Link to Inward Slip (optional)</Label>
              <Select bind:value={formInwardId}>
                <option value="">None</option>
                {#each slips as s}<option value={s.id}>#{s.slip_no} - {s.customer_name}</option>{/each}
              </Select>
            </div>
            <div class="space-y-2">
              <Label>Customer</Label>
              <Select bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}<option value={c.id}>{c.name}</option>{/each}
              </Select>
            </div>
          </div>

          <div class="space-y-2">
            <Label>Items</Label>
            <div class="space-y-2">
              {#each formItems as item, i}
                <div class="flex gap-2 items-start">
                  <Input class="flex-1" placeholder="Description" bind:value={item.description} />
                  <Input type="number" class="w-20" placeholder="Qty" bind:value={item.quantity} on:change={() => updateItemTotal(i)} />
                  <Input type="number" class="w-24" placeholder="Rate" bind:value={item.unit_price} on:change={() => updateItemTotal(i)} />
                  <span class="w-20 text-right font-medium text-foreground">{formatCurrency(item.total)}</span>
                  <Button variant="destructive" size="sm" class="p-1.5" on:click={() => removeItem(i)}>×</Button>
                </div>
              {/each}
              <Button variant="outline" size="sm" class="text-gray-600" on:click={addItem}>+ Add Item</Button>
            </div>
            <div class="space-y-1 mt-2">
              <div class="flex justify-between text-sm">
                <span class="text-muted-foreground">Subtotal</span>
                <span class="text-right font-medium">{formatCurrency(getSubtotal())}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-muted-foreground">Tax (18%)</span>
                <span class="text-right font-medium">{formatCurrency(getTax())}</span>
              </div>
              <div class="flex justify-between text-sm border-t border-border pt-1">
                <span class="text-foreground font-bold">Total</span>
                <span class="text-right font-bold text-primary">{formatCurrency(getGrandTotal())}</span>
              </div>
            </div>
          </div>

          <div class="space-y-2">
            <Label>Payment Method</Label>
            <Select bind:value={formPaymentMethod}>
              <option value="">Select payment method...</option>
              <option>Cash</option><option>UPI</option><option>Card</option><option>Bank Transfer</option>
            </Select>
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default" disabled={loading}>
              {loading ? 'Creating...' : 'Create Bill'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
