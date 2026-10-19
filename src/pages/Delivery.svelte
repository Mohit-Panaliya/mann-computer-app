<script lang="ts">
  import { onMount } from 'svelte';
  import { api, InwardSlip, Bill } from '../lib/api';
  import { formatDate, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

  let slips: InwardSlip[] = [];
  let bills: Bill[] = [];

  onMount(async () => {
    [slips, bills] = await Promise.all([api.getInwardSlips(), api.getBills()]);
  });

  async function deliver(s: InwardSlip) {
    const bill = bills.find(b => b.inward_id === s.id);
    await api.createDelivery(s.id, bill?.id || null, s.customer_id, null, s.customer_name, null);
    slips = await api.getInwardSlips();
  }

  async function sendDeliveryWA(s: InwardSlip) {
    const msg = `Dear ${s.customer_name},\nYour ${s.device_type} is ready for delivery.\nInward No: ${s.slip_no}\n\nPlease visit Mann Computer to collect your device.\n\nThank you!`;
    window.open(generateWhatsAppLink(s.customer_phone || '', msg), '_blank');
  }

  const readySlips = slips.filter(s => s.status === 'ready' || s.status === 'completed');
  const deliveredSlips = slips.filter(s => s.status === 'delivered');
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Delivery</h1>
  </div>

  <h3>Ready for Delivery</h3>
  <div class="card">
    <table>
      <thead><tr><th>Slip No</th><th>Date</th><th>Customer</th><th>Device</th><th>Actions</th></tr></thead>
      <tbody>
        {#each readySlips as s}
          <tr>
            <td>{s.slip_no}</td>
            <td>{formatDate(s.date)}</td>
            <td>{s.customer_name}</td>
            <td>{s.device_type}</td>
            <td class="actions">
              <button class="btn btn-sm btn-success" on:click={() => deliver(s)}>Mark Delivered</button>
              <button class="btn btn-sm btn-outline" on:click={() => sendDeliveryWA(s)}>WA Notify</button>
            </td>
          </tr>
        {:else}
          <tr><td colspan="5">No items ready for delivery</td></tr>
        {/each}
      </tbody>
    </table>
  </div>

  <h3>Delivered</h3>
  <div class="card">
    <table>
      <thead><tr><th>Slip No</th><th>Date</th><th>Customer</th><th>Device</th><th>Status</th></tr></thead>
      <tbody>
        {#each deliveredSlips as s}
          <tr>
            <td>{s.slip_no}</td>
            <td>{formatDate(s.date)}</td>
            <td>{s.customer_name}</td>
            <td>{s.device_type}</td>
            <td><span class="badge badge-delivered">Delivered</span></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .actions { display: flex; gap: 4px; }
  h3 { margin: 16px 0 8px; font-size: 16px; }
</style>
