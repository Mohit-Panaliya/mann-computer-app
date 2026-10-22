<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';

  export let onNavigate: (e: CustomEvent<string>) => void;

  let stats: any = null;
  let loading = true;

  const statCards = [
    { key: 'total_inward', label: 'Total Inward', icon: '📥', color: 'text-primary-600', bg: 'bg-primary-50', page: 'inward' },
    { key: 'pending_repair', label: 'In Repair', icon: '🔧', color: 'text-yellow-600', bg: 'bg-yellow-50', page: 'repairs' },
    { key: 'ready_for_delivery', label: 'Ready', icon: '✅', color: 'text-green-600', bg: 'bg-green-50', page: 'delivery' },
    { key: 'delivered', label: 'Delivered', icon: '🚚', color: 'text-indigo-600', bg: 'bg-indigo-50', page: 'delivery' },
    { key: 'total_bills', label: 'Total Bills', icon: '🧾', color: 'text-blue-600', bg: 'bg-blue-50', page: 'billing' },
    { key: 'pending_bills', label: 'Pending Bills', icon: '⏳', color: 'text-orange-600', bg: 'bg-orange-50', page: 'billing' },
    { key: 'today_revenue', label: "Today's Revenue", icon: '💰', color: 'text-emerald-600', bg: 'bg-emerald-50', page: 'billing' },
    { key: 'parts_in_stock', label: 'Parts Stock', icon: '📦', color: 'text-purple-600', bg: 'bg-purple-50', page: 'parts' },
  ];

  onMount(async () => {
    try {
      stats = await api.getDashboardStats();
    } catch (e) {
      console.error('Failed to load stats', e);
    } finally {
      loading = false;
    }
  });
</script>

<div>
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-gray-900">Dashboard</h1>
    <p class="text-gray-600 mt-1">Welcome back to Mann Computer</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="flex flex-col items-center gap-2 animate-pulse">
        <div class="w-12 h-12 rounded-full bg-primary-100 flex items-center justify-center text-primary-600">📊</div>
        <p class="text-gray-500 text-sm">Loading...</p>
      </div>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      {#each statCards as card}
        <div
          class="stat-card cursor-pointer hover:shadow-xl transition-shadow"
          on:click={() => onNavigate(new CustomEvent('navigate', { detail: card.page }))}
        >
          <div class="flex items-center justify-between mb-3">
            <div class="{card.color} w-10 h-10 rounded-xl flex items-center justify-center text-xl">{card.icon}</div>
            <span class="text-gray-400 text-xs">{card.label.toUpperCase()}</span>
          </div>
          <div>
            <div class="text-2xl font-bold text-gray-900 mt-1">
              {card.key === 'today_revenue' ? `₹${stats[card.key].toLocaleString('en-IN')}` : stats[card.key]}
            </div>
          </div>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="card">
        <h3 class="text-sm font-semibold text-gray-900 mb-4 flex items-center gap-2">
          <span class="text-primary-600">📈</span> Quick Actions
        </h3>
        <div class="grid grid-cols-2 gap-3">
          <button class="btn btn-primary text-sm w-full" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'inward' }))}>
            New Inward Slip
          </button>
          <button class="btn btn-outline text-sm w-full" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'repairs' }))}>
            View Repairs
          </button>
          <button class="btn btn-success text-sm w-full" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'billing' }))}>
            Create Bill
          </button>
          <button class="btn btn-outline text-sm w-full" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'delivery' }))}>
            Manage Delivery
          </button>
        </div>
      </div>

      <div class="card lg:col-span-2">
        <h3 class="text-sm font-semibold text-gray-900 mb-4 flex items-center gap-2">
          <span class="text-primary-600">📋</span> Recent Activity
        </h3>
        <div class="overflow-hidden">
          <table class="w-full text-sm">
            <thead>
              <tr class="table-header">
                <th class="text-left">Type</th>
                <th class="text-left">Reference</th>
                <th class="text-left">Customer</th>
                <th class="text-right">Status</th>
              </tr>
            </thead>
            <tbody>
              <tr class="hover:bg-gray-50 transition-colors">
                <td class="text-gray-700 font-medium">Inward</td>
                <td class="text-gray-600">INV-0042</td>
                <td class="text-gray-600">Ravi Patel</td>
                <td class="text-right"><span class="badge badge-repairing">In Repair</span></td>
              </tr>
              <tr class="hover:bg-gray-50 transition-colors">
                <td class="text-gray-700 font-medium">Bill</td>
                <td class="text-gray-600">BILL-0018</td>
                <td class="text-gray-600">Sunita Shah</td>
                <td class="text-right"><span class="badge badge-paid">Paid</span></td>
              </tr>
              <tr class="hover:bg-gray-50 transition-colors">
                <td class="text-gray-700 font-medium">Quote</td>
                <td class="text-gray-600">QTE-0031</td>
                <td class="text-gray-600">Amit Desai</td>
                <td class="text-right"><span class="badge badge-draft">Draft</span></td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  {/if}
</div>
