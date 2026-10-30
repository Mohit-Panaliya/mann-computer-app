<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency } from '../lib/utils';

  let analytics: any = null;
  let loading = true;

  onMount(async () => {
    try { analytics = await api.getAnalytics(); } catch (e) { console.error('Failed to load analytics', e); }
    finally { loading = false; }
  });

  function getBarWidth(value: number, max: number): string {
    if (max === 0) return '0%';
    return `${Math.max(5, (value / max) * 100)}%`;
  }

  function maxRevenue(): number {
    if (!analytics?.revenue_by_day) return 1;
    return Math.max(...analytics.revenue_by_day.map((d: any) => d.amount), 1);
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Analytics</h1>
      <p class="page-subtitle">Business insights and performance metrics</p>
    </div>
  </div>

  {#if loading}
    <div class="empty-state">
      <div class="empty-state-icon">📊</div>
      <p>Loading analytics...</p>
    </div>
  {:else if analytics}
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">Total Revenue</div>
        <div class="stat-value">{formatCurrency(analytics.total_revenue)}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Today's Revenue</div>
        <div class="stat-value">{formatCurrency(analytics.today_revenue)}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Total Inward</div>
        <div class="stat-value">{analytics.total_inward}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Total Customers</div>
        <div class="stat-value">{analytics.total_customers}</div>
      </div>
    </div>

    <div class="card" style="margin-bottom:1.5rem;">
      <div class="card-header">
        <h3 class="card-title">Revenue - Last 7 Days</h3>
      </div>
      <div style="display:flex; flex-direction:column; gap:0.75rem;">
        {#each analytics.revenue_by_day as day}
          <div style="display:flex; align-items:center; gap:0.75rem;">
            <span style="font-size:0.75rem; color:var(--text-muted); width:3rem;">{day.date.slice(5)}</span>
            <div style="flex:1; background:var(--surface-alt); border-radius:9999px; height:1.5rem; overflow:hidden;">
              <div style="background:var(--primary); height:100%; border-radius:9999px; transition:width 0.3s; width:{getBarWidth(day.amount, maxRevenue())};"></div>
            </div>
            <span style="font-size:0.75rem; font-weight:600; width:5rem; text-align:right;">{formatCurrency(day.amount)}</span>
          </div>
        {/each}
      </div>
    </div>

    <div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(250px, 1fr)); gap:1.5rem;">
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Status Distribution</h3>
        </div>
        {#each analytics.status_distribution as item}
          <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
            <span style="font-size:0.875rem; text-transform:capitalize;">{item.status}</span>
            <div style="display:flex; align-items:center; gap:0.75rem;">
              <div style="width:6rem; background:var(--surface-alt); border-radius:9999px; height:0.5rem; overflow:hidden;">
                <div style="background:var(--primary); height:100%; border-radius:9999px; width:{item.count / analytics.total_inward * 100}%;"></div>
              </div>
              <span style="font-size:0.875rem; font-weight:700; width:2rem; text-align:right;">{item.count}</span>
            </div>
          </div>
        {/each}
      </div>

      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Payment Status</h3>
        </div>
        {#each analytics.payment_distribution as item}
          <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
            <span style="font-size:0.875rem; text-transform:capitalize;">{item.status}</span>
            <span class="badge {item.status === 'paid' ? 'badge-ready' : 'badge-pending'}">{item.count}</span>
          </div>
        {/each}
      </div>

      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Device Types</h3>
        </div>
        {#each analytics.device_type_distribution as item}
          <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
            <span style="font-size:0.875rem; text-transform:capitalize;">{item.device_type}</span>
            <span class="badge badge-draft">{item.count}</span>
          </div>
        {/each}
      </div>

      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Quotations</h3>
        </div>
        <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
          <span style="font-size:0.875rem; color:var(--text-secondary);">Total Quotations</span>
          <span class="badge badge-draft">{analytics.total_quotations}</span>
        </div>
        <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
          <span style="font-size:0.875rem; color:var(--text-secondary);">CCTV Quotes</span>
          <span class="badge badge-draft">{analytics.total_cctv_quotes}</span>
        </div>
        <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
          <span style="font-size:0.875rem; color:var(--text-secondary);">Laptop Quotes</span>
          <span class="badge badge-draft">{analytics.total_laptop_quotes}</span>
        </div>
        <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:0.75rem;">
          <span style="font-size:0.875rem; color:var(--text-secondary);">Parts Sold</span>
          <span class="badge badge-draft">{analytics.parts_sold}</span>
        </div>
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <div class="empty-state-icon">📊</div>
      <p>Failed to load analytics data.</p>
    </div>
  {/if}
</div>
EOF
echo "Done"