<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatCurrency } from '../lib/utils';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Badge from '$lib/components/ui/badge.svelte';
  import Button from '$lib/components/ui/button.svelte';

  let analytics: any = null;
  let loading = true;

  onMount(async () => {
    try {
      analytics = await api.getAnalytics();
    } catch (e) {
      console.error('Failed to load analytics', e);
    } finally {
      loading = false;
    }
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
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-foreground">Analytics</h1>
    <p class="text-muted-foreground mt-1">Business insights and performance metrics</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="flex flex-col items-center gap-2 animate-pulse">
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary">📊</div>
        <p class="text-muted-foreground text-sm">Loading analytics...</p>
      </div>
    </div>
  {:else if analytics}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <Card>
        <CardContent class="p-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">Total Revenue</span>
            <span class="text-lg">💰</span>
          </div>
          <div class="text-2xl font-bold text-foreground">{formatCurrency(analytics.total_revenue)}</div>
          <div class="text-xs text-muted-foreground mt-1">From {analytics.paid_bills} paid bills</div>
        </CardContent>
      </Card>
      <Card>
        <CardContent class="p-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">Today's Revenue</span>
            <span class="text-lg">📈</span>
          </div>
          <div class="text-2xl font-bold text-foreground">{formatCurrency(analytics.today_revenue)}</div>
          <div class="text-xs text-muted-foreground mt-1">Week: {formatCurrency(analytics.week_revenue)}</div>
        </CardContent>
      </Card>
      <Card>
        <CardContent class="p-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">Total Inward</span>
            <span class="text-lg">📥</span>
          </div>
          <div class="text-2xl font-bold text-foreground">{analytics.total_inward}</div>
          <div class="text-xs text-muted-foreground mt-1">{analytics.delivered} delivered</div>
        </CardContent>
      </Card>
      <Card>
        <CardContent class="p-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase">Total Customers</span>
            <span class="text-lg">👥</span>
          </div>
          <div class="text-2xl font-bold text-foreground">{analytics.total_customers}</div>
          <div class="text-xs text-muted-foreground mt-1">{analytics.total_repairs} repairs, {analytics.completed_repairs} completed</div>
        </CardContent>
      </Card>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Revenue - Last 7 Days</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            {#each analytics.revenue_by_day as day}
              <div class="flex items-center gap-3">
                <span class="text-xs text-muted-foreground w-20">{day.date.slice(5)}</span>
                <div class="flex-1 bg-muted rounded-full h-6 overflow-hidden">
                  <div class="bg-blue-500 h-full rounded-full transition-all" style="width: {getBarWidth(day.amount, maxRevenue())}"></div>
                </div>
                <span class="text-xs font-medium text-foreground w-20 text-right">{formatCurrency(day.amount)}</span>
              </div>
            {/each}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Status Distribution</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            {#each analytics.status_distribution as item}
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <span class="text-sm font-medium text-foreground capitalize">{item.status}</span>
                </div>
                <div class="flex items-center gap-3">
                  <div class="w-32 bg-muted rounded-full h-2 overflow-hidden">
                    <div class="bg-blue-500 h-full rounded-full" style="width: {item.count / analytics.total_inward * 100}%"></div>
                  </div>
                  <span class="text-sm font-bold text-foreground w-8 text-right">{item.count}</span>
                </div>
              </div>
            {/each}
          </div>
        </CardContent>
      </Card>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Payment Status</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            {#each analytics.payment_distribution as item}
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground capitalize">{item.status}</span>
                <Badge variant={item.status === 'paid' ? 'secondary' : 'outline'} class="text-xs">{item.count}</Badge>
              </div>
            {/each}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Device Types</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            {#each analytics.device_type_distribution as item}
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground capitalize">{item.device_type}</span>
                <Badge variant="outline" class="text-xs">{item.count}</Badge>
              </div>
            {/each}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Quotations</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-sm text-muted-foreground">Total Quotations</span>
              <Badge variant="outline" class="text-xs">{analytics.total_quotations}</Badge>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-muted-foreground">CCTV Quotes</span>
              <Badge variant="outline" class="text-xs">{analytics.total_cctv_quotes}</Badge>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-muted-foreground">Laptop Quotes</span>
              <Badge variant="outline" class="text-xs">{analytics.total_laptop_quotes}</Badge>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-muted-foreground">Parts Sold</span>
              <Badge variant="outline" class="text-xs">{analytics.parts_sold}</Badge>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  {:else}
    <div class="text-center py-12 text-muted-foreground">Failed to load analytics data.</div>
  {/if}
</div>
