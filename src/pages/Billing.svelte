<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

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
      resetForm();
      await loadData();
    } catch (e) {
      console.error('Failed to create bill', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    formInwardId = '';
    formCustomerId = '';
    formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
    formPaymentMethod = '';
  }

  async function markPaid(b: any) {
    try {
      await api.updateBillPayment(b.id, 'paid', b.payment_method || 'Cash');
      await loadData();
    } catch (e) {
      console.error('Failed to mark paid', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  async function sendWhatsApp(b: any) {
    const itemsList = b.items.map((i: any) => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Bill from Mann Computer\nBill No: ${b.bill_no}\nDate: ${b.created_at}\n\n${itemsList}\n\nSubtotal: ${formatCurrency(b.subtotal)}\nTax: ${formatCurrency(b.tax)}\nTotal: ${formatCurrency(b.total_amount)}`;
    window.open(generateWhatsAppLink(b.customer_phone || '', msg), '_blank');
  }

  async function generatePdf(b: any) {
    try {
      const path = await api.generatePdf('bill', b.id);
      window.open(`file://${path}`, '_blank');
    } catch (e) {
      console.error('Failed to generate PDF', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Billing</h1>
      <p class="text-gray-600 mt-1">Generate invoices and track payments</p>
    </div>
    <button class="btn btn-primary" on:click={() => { showForm = true; }}>
      <span class="mr-2">+</span> New Bill
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left w-24">Bill No</th>
          <th class="text-left">Customer</th>
          <th class="text-center w-20">Items</th>
          <th class="text-right w-24">Total</th>
          <th class="text-center w-32">Payment</th>
          <th class="text-right w-40">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each bills as b}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-mono text-sm font-medium">{b.bill_no}</td>
            <td class="text-gray-700 font-medium">{b.customer_name}</td>
            <td class="text-center text-gray-600">{b.items.length}</td>
            <td class="text-right text-gray-700 font-medium">{formatCurrency(b.total_amount)}</td>
            <td class="text-center">
              <span class="badge {getStatusClass(b.payment_status)}">{getStatusLabel(b.payment_status)}</span>
            </td>
            <td class="text-right">
              <div class="flex items-center gap-1.5 justify-end">
                {#if b.payment_status === 'pending'}
                  <button class="btn btn-success btn-sm" on:click={() => markPaid(b)}>Mark Paid</button>
                {/if}
                <button class="btn btn-ghost btn-sm" on:click={() => generatePdf(b)} title="PDF">
                  <span class="text-lg">📄</span>
                </button>
                <button class="btn btn-ghost btn-sm text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(b)} title="WhatsApp">
                  <span class="text-lg">💬</span>
                </button>
              </div>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="text-center py-8 text-gray-500">No bills found.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-2xl">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">New Bill</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createBill} class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="label">Link to Inward Slip (optional)</label>
              <select class="input" bind:value={formInwardId}>
                <option value="">None</option>
                {#each slips as s}
                  <option value={s.id}>#{s.slip_no} - {s.customer_name}</option>
                {/each}
              </select>
            </div>
            <div class="space-y-2">
              <label class="label">Customer</label>
              <select class="input" bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}
                  <option value={c.id}>{c.name}</option>
                {/each}
              </select>
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
            <div class="space-y-1 mt-2">
              <div class="flex justify-between text-sm">
                <span class="text-gray-500">Subtotal</span>
                <span class="text-right font-medium">{formatCurrency(getSubtotal())}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-500">Tax (18%)</span>
                <span class="text-right font-medium">{formatCurrency(getTax())}</span>
              </div>
              <div class="flex justify-between text-sm border-t border-gray-200 pt-1">
                <span class="text-gray-900 font-bold">Total</span>
                <span class="text-right font-bold text-primary-600">{formatCurrency(getGrandTotal())}</span>
              </div>
            </div>
          </div>

          <div class="space-y-2">
            <label class="label">Payment Method</label>
            <select class="input" bind:value={formPaymentMethod}>
              <option value="">Select payment method...</option>
              <option value="Cash">Cash</option>
              <option value="UPI">UPI</option>
              <option value="Card">Card</option>
              <option value="Bank Transfer">Bank Transfer</option>
            </select>
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>
              {loading ? 'Creating...' : 'Create Bill'}
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
