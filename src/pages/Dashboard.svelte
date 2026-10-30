<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';

  export let onNavigate: (e: CustomEvent<string>) => void;

  let stats: any = null;
  let loading = true;

  const statCards = [
    { key: 'total_inward', label: 'Total Inward', icon: '📥', page: 'inward' },
    { key: 'pending_repair', label: 'In Repair', icon: '🔧', page: 'repairs' },
    { key: 'ready_for_delivery', label: 'Ready', icon: '✅', page: 'delivery' },
    { key: 'delivered', label: 'Delivered', icon: '🚚', page: 'delivery' },
    { key: 'total_bills', label: 'Total Bills', icon: '🧾', page: 'billing' },
    { key: 'pending_bills', label: 'Pending Bills', icon: '⏳', page: 'billing' },
    { key: 'today_revenue', label: "Today's Revenue", icon: '💰', page: 'billing' },
    { key: 'parts_in_stock', label: 'Parts Stock', icon: '📦', page: 'parts' },
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
  <div class="page-header">
    <div>
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">Welcome back to Mann Computer</p>
    </div>
  </div>

  {#if loading}
    <div class="empty-state">
      <div class="empty-state-icon">📊</div>
      <p>Loading...</p>
    </div>
  {:else}
    <div class="stats-grid">
      {#each statCards as card}
        <div class="stat-card" on:click={() => onNavigate(new CustomEvent('navigate', { detail: card.page }))}>
          <div class="stat-label">{card.label}</div>
          <div class="stat-value">{card.key === 'today_revenue' ? '₹' + stats[card.key].toLocaleString('en-IN') : stats[card.key]}</div>
          <div style="margin-top:0.5rem; font-size:1.25rem;">{card.icon}</div>
        </div>
      {/each}
    </div>

    <div class="card">
      <div class="card-header">
        <h3 class="card-title">Quick Actions</h3>
      </div>
      <div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(140px, 1fr)); gap:0.75rem;">
        <button class="btn btn-primary" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'inward' }))}>+ New Inward</button>
        <button class="btn btn-secondary" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'repairs' }))}>View Repairs</button>
        <button class="btn btn-primary" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'billing' }))}>Create Bill</button>
        <button class="btn btn-secondary" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'delivery' }))}>Manage Delivery</button>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"