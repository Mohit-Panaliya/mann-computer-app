<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import Card from '$lib/components/ui/card.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import Badge from '$lib/components/ui/badge.svelte';
  import Button from '$lib/components/ui/button.svelte';

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
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-foreground">Dashboard</h1>
    <p class="text-muted-foreground mt-1">Welcome back to Mann Computer</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="flex flex-col items-center gap-2 animate-pulse">
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary">📊</div>
        <p class="text-muted-foreground text-sm">Loading...</p>
      </div>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      {#each statCards as card}
        <Card class="cursor-pointer hover:shadow-md transition-shadow" on:click={() => onNavigate(new CustomEvent('navigate', { detail: card.page }))}>
          <CardHeader>
            <CardTitle class="text-xs">{card.label}</CardTitle>
          </CardHeader>
          <CardContent>
            <div class="flex items-center justify-between mb-3">
              <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary">{card.icon}</div>
              <Badge variant="secondary">{card.label.toUpperCase()}</Badge>
            </div>
            <div class="text-2xl font-bold text-foreground mt-1">
              {card.key === 'today_revenue' ? `₹${stats[card.key].toLocaleString('en-IN')}` : stats[card.key]}
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <Card>
        <CardHeader>
          <CardTitle class="text-xs flex items-center gap-2">
            <span class="text-primary">📈</span> Quick Actions
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-2 gap-3">
            <Button variant="default" class="w-full text-sm" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'inward' }))}>
              <span class="mr-2">+</span> New Inward Slip
            </Button>
            <Button variant="outline" class="w-full text-sm" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'repairs' }))}>
              View Repairs
            </Button>
            <Button variant="default" class="w-full text-sm" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'billing' }))}>
              Create Bill
            </Button>
            <Button variant="outline" class="w-full text-sm" on:click={() => onNavigate(new CustomEvent('navigate', { detail: 'delivery' }))}>
              Manage Delivery
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card class="lg:col-span-2">
        <CardHeader>
          <CardTitle class="text-xs flex items-center gap-2">
            <span class="text-primary">📋</span> Recent Activity
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="overflow-hidden">
            <table class="w-full text-sm">
              <thead>
                <tr class="text-left border-b border-border">
                  <th class="text-xs font-semibold text-muted-foreground py-3">Type</th>
                  <th class="text-xs font-semibold text-muted-foreground py-3">Reference</th>
                  <th class="text-xs font-semibold text-muted-foreground py-3">Customer</th>
                  <th class="text-xs font-semibold text-muted-foreground py-3 text-right">Status</th>
                </tr>
              </thead>
              <tbody>
                <tr class="hover:bg-muted transition-colors border-b border-border">
                  <td class="text-sm font-medium text-foreground py-3">Inward</td>
                  <td class="text-sm text-muted-foreground py-3 font-mono">INV-0042</td>
                  <td class="text-sm text-muted-foreground py-3">Ravi Patel</td>
                  <td class="text-sm text-right py-3"><Badge variant="default" class="text-xs">In Repair</Badge></td>
                </tr>
                <tr class="hover:bg-muted transition-colors border-b border-border">
                  <td class="text-sm font-medium text-foreground py-3">Bill</td>
                  <td class="text-sm text-muted-foreground py-3 font-mono">BILL-0018</td>
                  <td class="text-sm text-muted-foreground py-3">Sunita Shah</td>
                  <td class="text-sm text-right py-3"><Badge variant="secondary" class="text-xs">Paid</Badge></td>
                </tr>
                <tr class="hover:bg-muted transition-colors border-b border-border">
                  <td class="text-sm font-medium text-foreground py-3">Quote</td>
                  <td class="text-sm text-muted-foreground py-3 font-mono">QTE-0031</td>
                  <td class="text-sm text-muted-foreground py-3">Amit Desai</td>
                  <td class="text-sm text-right py-3"><Badge variant="outline" class="text-xs">Draft</Badge></td>
                </tr>
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  {/if}
</div>
