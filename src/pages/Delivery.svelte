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
    } catch (e) {
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
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-900">Delivery</h1>
    <p class="text-gray-600 mt-1">Manage device delivery to customers</p>
  </div>

  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-semibold text-gray-900 mb-3 flex items-center gap-2">
        <span class="text-yellow-600">📦</span> Ready for Delivery
      </h3>
      <div class="card">
        <table class="w-full text-sm">
          <thead>
            <tr class="table-header">
              <th class="text-left w-24">Slip No</th>
              <th class="text-left w-40">Date</th>
              <th class="text-left">Customer</th>
              <th class="text-left w-32">Device</th>
              <th class="text-right w-48">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each readySlips as s}
              <tr class="hover:bg-yellow-50 transition-colors border-b border-yellow-50 last:border-0">
                <td class="text-gray-700 font-mono text-sm font-medium">{s.slip_no}</td>
                <td class="text-gray-600">{formatDate(s.date)}</td>
                <td class="text-gray-700 font-medium">{s.customer_name}</td>
                <td class="text-gray-600 capitalize">{s.device_type}</td>
                <td class="text-right">
                  <div class="flex items-center gap-2">
                    <button class="btn btn-success btn-sm w-full" on:click={() => deliver(s)}>Mark Delivered</button>
                    <button class="btn btn-ghost btn-sm text-green-600 hover:bg-green-50 p-1.5" on:click={() => sendDeliveryWA(s)} title="Notify WhatsApp">
                      <span class="text-lg">💬</span>
                    </button>
                  </div>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="5" class="text-center py-8 text-gray-500">No items ready for delivery</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <div>
      <h3 class="text-sm font-semibold text-gray-900 mb-3 flex items-center gap-2">
        <span class="text-green-600">✅</span> Delivered
      </h3>
      <div class="card">
        <table class="w-full text-sm">
          <thead>
            <tr class="table-header">
              <th class="text-left w-24">Slip No</th>
              <th class="text-left w-40">Date</th>
              <th class="text-left">Customer</th>
              <th class="text-left w-32">Device</th>
              <th class="text-right w-32">Status</th>
            </tr>
          </thead>
          <tbody>
            {#each deliveredSlips as s}
              <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
                <td class="text-gray-700 font-mono text-sm font-medium">{s.slip_no}</td>
                <td class="text-gray-600">{formatDate(s.date)}</td>
                <td class="text-gray-700 font-medium">{s.customer_name}</td>
                <td class="text-gray-600 capitalize">{s.device_type}</td>
                <td class="text-right"><span class="badge badge-delivered">Delivered</span></td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>
