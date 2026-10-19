<script lang="ts">
  import { onMount } from 'svelte';
  import { api, Bill, BillItem, Customer, InwardSlip } from '../lib/api';
  import { formatCurrency, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

  let bills: Bill[] = [];
  let customers: Customer[] = [];
  let slips: InwardSlip[] = [];
  let showForm = false;

  let formCustomerId = '';
  let formInwardId = '';
  let formItems: BillItem[] = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
  let formPaymentMethod = '';

  onMount(async () => {
    [bills, customers, slips] = await Promise.all([api.getBills(), api.getCustomers(), api.getInwardSlips()]);
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

  async function createBill() {
    const items = formItems.filter(i => i.description && i.quantity > 0);
    await api.createBill(formInwardId || null, null, formCustomerId, items, formPaymentMethod || null);
    showForm = false;
    formItems = [{ description: '', quantity: 1, unit_price: 0, total: 0 }];
    bills = await api.getBills();
  }

  async function markPaid(b: Bill) {
    await api.updateBillPayment(b.id, 'paid', b.paymentMethod || 'Cash');
    bills = await api.getBills();
  }

  async function sendWhatsApp(b: Bill) {
    const itemsList = b.items.map(i => `${i.description} x${i.quantity} = ${formatCurrency(i.total)}`).join('\n');
    const msg = `Bill from Mann Computer\nBill No: ${b.bill_no}\nDate: ${b.created_at}\n\n${itemsList}\n\nSubtotal: ${formatCurrency(b.subtotal)}\nTax: ${formatCurrency(b.tax)}\nTotal: ${formatCurrency(b.total_amount)}`;
    window.open(generateWhatsAppLink(b.customer_phone || '', msg), '_blank');
  }

  async function generatePdf(b: Bill) {
    const path = await api.generatePdf('bill', b.id);
    window.open(`file://${path}`, '_blank');
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Billing</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Bill</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Bill No</th><th>Customer</th><th>Items</th><th>Total</th><th>Payment</th><th>Actions</th></tr></thead>
      <tbody>
        {#each bills as b}
          <tr>
            <td>{b.bill_no}</td>
            <td>{b.customer_name}</td>
            <td>{b.items.length}</td>
            <td>{formatCurrency(b.total_amount)}</td>
            <td><span class="badge {getStatusClass(b.payment_status)}">{getStatusLabel(b.payment_status)}</span></td>
            <td class="actions">
              {#if b.payment_status === 'pending'}
                <button class="btn btn-sm btn-success" on:click={() => markPaid(b)}>Mark Paid</button>
              {/if}
              <button class="btn btn-sm btn-outline" on:click={() => generatePdf(b)}>PDF</button>
              <button class="btn btn-sm btn-outline" on:click={() => sendWhatsApp(b)}>WA Send</button>
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
          <h2 class="modal-title">New Bill</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Link to Inward (optional)</label>
          <select class="input" bind:value={formInwardId}>
            <option value="">None</option>
            {#each slips as s}<option value={s.id}>#{s.slip_no} - {s.customer_name}</option>{/each}
          </select>
        </div>
        <div class="form-group"><label class="form-label">Customer</label>
          <select class="input" bind:value={formCustomerId}>
            <option value="">Select...</option>
            {#each customers as c}<option value={c.id}>{c.name}</option>{/each}
          </select>
        </div>
        <div class="items-section">
          <label class="form-label">Items</label>
          {#each formItems as item, i}
            <div class="item-row">
              <input class="input" placeholder="Description" bind:value={item.description} />
              <input class="input" type="number" placeholder="Qty" bind:value={item.quantity} on:change={() => updateItemTotal(i)} />
              <input class="input" type="number" placeholder="Rate" bind:value={item.unit_price} on:change={() => updateItemTotal(i)} />
              <span class="item-total">{formatCurrency(item.total)}</span>
              <button class="btn btn-sm btn-danger" on:click={() => removeItem(i)}>X</button>
            </div>
          {/each}
          <button class="btn btn-sm btn-outline" on:click={addItem}>+ Add Item</button>
        </div>
        <div class="form-group"><label class="form-label">Payment Method</label>
          <select class="input" bind:value={formPaymentMethod}>
            <option value="">Select...</option>
            <option>Cash</option><option>UPI</option><option>Card</option><option>Bank Transfer</option>
          </select>
        </div>
        <button class="btn btn-primary" on:click={createBill}>Create Bill</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
  .item-row { display: flex; gap: 8px; margin-bottom: 8px; align-items: center; }
  .item-row .input { flex: 1; }
  .item-total { min-width: 80px; text-align: right; font-weight: 600; }
  .items-section { margin: 12px 0; }
</style>
