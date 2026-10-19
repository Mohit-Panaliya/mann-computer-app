<script lang="ts">
  import { onMount } from 'svelte';
  import { api, DashboardStats } from '../lib/api';

  export let onNavigate: (e: CustomEvent<string>) => void;

  let stats: DashboardStats | null = null;

  onMount(async () => {
    try {
      stats = await api.getDashboardStats();
    } catch (e) {
      console.error('Failed to load stats', e);
    }
  });

  const cards = [
    { key: 'total_inward', label: 'Total Inward', color: '#1a73e8', page: 'inward' },
    { key: 'pending_repair', label: 'In Repair', color: '#fbbc04', page: 'repairs' },
    { key: 'ready_for_delivery', label: 'Ready', color: '#34a853', page: 'delivery' },
    { key: 'delivered', label: 'Delivered', color: '#5f6368', page: 'delivery' },
    { key: 'total_bills', label: 'Total Bills', color: '#1a73e8', page: 'billing' },
    { key: 'pending_bills', label: 'Pending Bills', color: '#ea4335', page: 'billing' },
    { key: 'today_revenue', label: "Today's Revenue", color: '#34a853', page: 'billing' },
    { key: 'parts_in_stock', label: 'Parts in Stock', color: '#5f6368', page: 'parts' },
  ];
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Dashboard</h1>
  </div>

  {#if stats}
    <div class="stats-grid">
      {#each cards as card}
        <div class="stat-card" on:click={() => onNavigate(new CustomEvent('navigate', { detail: card.page }))}>
          <div class="stat-label">{card.label}</div>
          <div class="stat-value" style="color: {card.color}">
            {card.key === 'today_revenue' ? `₹${stats[card.key].toLocaleString('en-IN')}` : stats[card.key]}
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <p>Loading...</p>
  {/if}
</div>
