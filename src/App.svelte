<script lang="ts">
  import { onMount } from 'svelte';
  import Dashboard from './pages/Dashboard.svelte';
  import InwardSlips from './pages/InwardSlips.svelte';
  import Quotations from './pages/Quotations.svelte';
  import Repairs from './pages/Repairs.svelte';
  import Billing from './pages/Billing.svelte';
  import Delivery from './pages/Delivery.svelte';
  import PartsInventory from './pages/PartsInventory.svelte';
  import CctvQuotes from './pages/CctvQuotes.svelte';
  import LaptopQuotes from './pages/LaptopQuotes.svelte';
  import Customers from './pages/Customers.svelte';
  import { api } from './lib/api';
  import Button from '$lib/components/ui/button.svelte';
  import Badge from '$lib/components/ui/badge.svelte';

  type Page = 'dashboard' | 'inward' | 'quotations' | 'repairs' | 'billing' | 'delivery' | 'parts' | 'cctv' | 'laptop' | 'customers';

  let currentPage: Page = 'dashboard';
  let shopName = 'Mann Computer';

  const navItems: { id: Page; label: string; icon: string }[] = [
    { id: 'dashboard', label: 'Dashboard', icon: '📊' },
    { id: 'inward', label: 'Inward Slips', icon: '📥' },
    { id: 'quotations', label: 'Quotations', icon: '📋' },
    { id: 'repairs', label: 'Repairs', icon: '🔧' },
    { id: 'billing', label: 'Billing', icon: '🧾' },
    { id: 'delivery', label: 'Delivery', icon: '🚚' },
    { id: 'parts', label: 'Parts Inventory', icon: '📦' },
    { id: 'cctv', label: 'CCTV Quotes', icon: '📷' },
    { id: 'laptop', label: 'Laptop Quotes', icon: '💻' },
    { id: 'customers', label: 'Customers', icon: '👥' },
  ];

  onMount(async () => {
    try {
      const settings = await api.getSettings();
      shopName = settings.shop_name || 'Mann Computer';
    } catch (e) {}
  });
</script>

<div class="flex h-screen overflow-hidden bg-background">
  <nav class="w-64 bg-border border-r flex flex-col z-20 shadow-sm">
    <div class="p-5 border-b">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-lg bg-primary flex items-center justify-center text-white font-bold">MC</div>
        <h1 class="text-sm font-bold text-foreground tracking-tight">{shopName}</h1>
      </div>
      <p class="text-[10px] text-muted-foreground uppercase tracking-wider font-medium mt-1">Computer Repair &amp; Sales</p>
    </div>
    <div class="flex-1 overflow-y-auto py-3">
      {#each navItems as item}
        <Button variant={currentPage === item.id ? 'default' : 'ghost'} size="sm" class="w-full justify-start" on:click={() => currentPage = item.id}>
          <span class="mr-3 text-lg">{item.icon}</span>
          <span>{item.label}</span>
        </Button>
      {/each}
    </div>
    <div class="p-4 border-t bg-muted">
      <div class="text-[10px] text-muted-foreground uppercase tracking-wider font-medium mb-2">Settings</div>
      <Button variant="ghost" size="sm" class="w-full justify-start">
        <span class="mr-3 text-lg">⚙️</span>
        <span>Settings</span>
      </Button>
    </div>
  </nav>

  <main class="flex-1 overflow-y-auto bg-background">
    <div class="p-6">
      {#if currentPage === 'dashboard'}
        <Dashboard on:navigate={(e) => currentPage = e.detail} />
      {:else if currentPage === 'inward'}
        <InwardSlips />
      {:else if currentPage === 'quotations'}
        <Quotations />
      {:else if currentPage === 'repairs'}
        <Repairs />
      {:else if currentPage === 'billing'}
        <Billing />
      {:else if currentPage === 'delivery'}
        <Delivery />
      {:else if currentPage === 'parts'}
        <PartsInventory />
      {:else if currentPage === 'cctv'}
        <CctvQuotes />
      {:else if currentPage === 'laptop'}
        <LaptopQuotes />
      {:else if currentPage === 'customers'}
        <Customers />
      {/if}
    </div>
  </main>
</div>
