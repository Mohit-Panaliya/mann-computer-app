<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatDate, generateWhatsAppLink } from '../lib/utils';

  let slips: any[] = [];
  let bills: any[] = [];
  let loading = false;

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [slips, bills] = await Promise.all([api.getInwardSlips(), api.getBills()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  async function deliver(s: any) {
    const bill = bills.find((b: any) => b.inward_id === s.id);
    if (!confirm(`Mark inward slip ${s.slip_no} as delivered?`)) return;
    try {
      await api.createDelivery(s.id, bill?.id || null, s.customer_id, null, s.customer_name, null);
      await loadData();
    } catch (e: any) {
      console.error('Failed to mark delivered', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function sendDeliveryWA(s: any) {
    const msg = `Dear ${s.customer_name},\nYour ${s.device_type} is ready for delivery.\nInward No: ${s.slip_no}\n\nPlease visit Mann Computer to collect your device.\n\nThank you!`;
    window.open(generateWhatsAppLink(s.customer_phone || '', msg), '_blank');
  }

  const readySlips = slips.filter((s: any) => s.status === 'ready' || s.status === 'completed');
  const deliveredSlips = slips.filter((s: any) => s.status === 'delivered');
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Delivery</h1>
      <p class="page-subtitle">Manage device delivery to customers</p>
    </div>
  </div>

  <div style="margin-bottom:1.5rem;">
    <h3 style="font-size:0.875rem; font-weight:600; color:var(--text); margin-bottom:0.75rem;">📦 Ready for Delivery</h3>
    <div class="card">
      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr><th>Slip No</th><th>Date</th><th>Customer</th><th>Device</th><th>Actions</th></tr>
          </thead>
          <tbody>
            {#each readySlips as s}
              <tr>
                <td class="font-medium">{s.slip_no}</td>
                <td>{formatDate(s.date)}</td>
                <td>{s.customer_name}</td>
                <td>{s.device_type}</td>
                <td>
                  <div style="display:flex; gap:0.5rem;">
                    <button class="btn btn-success btn-sm" on:click={() => deliver(s)}>Mark Delivered</button>
                    <button class="btn btn-ghost btn-sm" on:click={() => sendDeliveryWA(s)}>💬</button>
                  </div>
                </td>
              </tr>
            {:else}
              <tr><td colspan="5" class="empty-state">No items ready for delivery</td></tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <div>
    <h3 style="font-size:0.875rem; font-weight:600; color:var(--text); margin-bottom:0.75rem;">✅ Delivered</h3>
    <div class="card">
      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr><th>Slip No</th><th>Date</th><th>Customer</th><th>Device</th><th>Status</th></tr>
          </thead>
          <tbody>
            {#each deliveredSlips as s}
              <tr>
                <td class="font-medium">{s.slip_no}</td>
                <td>{formatDate(s.date)}</td>
                <td>{s.customer_name}</td>
                <td>{s.device_type}</td>
                <td><span class="badge badge-delivered">Delivered</span></td>
              </tr>
            {:else}
              <tr><td colspan="5" class="empty-state">No delivered items yet.</td></tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>
EOF
echo "Done"