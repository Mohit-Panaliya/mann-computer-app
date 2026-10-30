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
  import Analytics from './pages/Analytics.svelte';
  import Settings from './pages/Settings.svelte';
  import { api } from './lib/api';

  type Page = 'dashboard' | 'inward' | 'quotations' | 'repairs' | 'billing' | 'delivery' | 'parts' | 'cctv' | 'laptop' | 'customers' | 'analytics' | 'settings';

  let currentPage: Page = 'dashboard';
  let shopName = 'Mann Computer';
  let sidebarOpen = false;

  const navItems: { id: Page; label: string; icon: string }[] = [
    { id: 'dashboard', label: 'Dashboard', icon: '📊' },
    { id: 'inward', label: 'Inward', icon: '📥' },
    { id: 'quotations', label: 'Quotes', icon: '📋' },
    { id: 'repairs', label: 'Repairs', icon: '🔧' },
    { id: 'billing', label: 'Billing', icon: '🧾' },
    { id: 'delivery', label: 'Delivery', icon: '🚚' },
    { id: 'parts', label: 'Parts', icon: '📦' },
    { id: 'cctv', label: 'CCTV', icon: '📷' },
    { id: 'laptop', label: 'Laptop', icon: '💻' },
    { id: 'customers', label: 'Customers', icon: '👥' },
    { id: 'analytics', label: 'Analytics', icon: '📈' },
    { id: 'settings', label: 'Settings', icon: '⚙️' },
  ];

  onMount(async () => {
    try {
      const settings = await api.getSettings();
      shopName = settings.shop_name || 'Mann Computer';
    } catch (e) {}
  });

  function navigate(page: Page) {
    currentPage = page;
    sidebarOpen = false;
  }

  function closeSidebar() {
    sidebarOpen = false;
  }
</script>

<div class="flex h-screen overflow-hidden bg-background">
  <!-- Mobile overlay backdrop -->
  {#if sidebarOpen}
    <div class="fixed inset-0 z-40 bg-black/60 md:hidden" on:click={closeSidebar} role="button" tabindex="0" on:keydown={(e) => e.key === 'Escape' && closeSidebar()}></div>
  {/if}

  <!-- Sidebar -->
  <aside class="fixed md:static inset-y-0 left-0 z-50 w-72 max-w-[85vw] bg-card border-r border-border flex flex-col transform transition-transform duration-300 ease-in-out md:translate-x-0 {sidebarOpen ? 'translate-x-0' : '-translate-x-full'}">
    <!-- Sidebar header -->
    <div class="flex items-center justify-between p-4 border-b border-border">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-primary flex items-center justify-center text-white font-bold text-sm shadow-sm">MC</div>
        <div>
          <h1 class="text-sm font-bold text-foreground tracking-tight">{shopName}</h1>
          <p class="text-[10px] text-muted-foreground uppercase tracking-wider">Computer Repair & Sales</p>
        </div>
      </div>
      <button class="md:hidden p-2 text-muted-foreground hover:text-foreground hover:bg-muted rounded-lg" on:click={closeSidebar}>
        <span class="text-xl">✕</span>
      </button>
    </div>

    <!-- Nav items -->
    <nav class="flex-1 overflow-y-auto py-2 px-2">
      {#each navItems as item}
        <button
          class="flex items-center gap-3 w-full px-3 py-3 rounded-lg text-sm font-medium transition-colors mb-0.5 {currentPage === item.id ? 'bg-primary text-primary-foreground shadow-sm' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
          on:click={() => navigate(item.id)}
        >
          <span class="text-lg">{item.icon}</span>
          <span>{item.label}</span>
        </button>
      {/each}
    </nav>

    <!-- Footer -->
    <div class="p-3 border-t border-border bg-muted/30">
      <p class="text-[10px] text-muted-foreground text-center">Mann Computer v1.3.3</p>
    </div>
  </aside>

  <!-- Main content area -->
  <div class="flex-1 flex flex-col overflow-hidden min-w-0">
    <!-- Mobile header -->
    <header class="md:hidden flex items-center gap-3 px-4 py-3 border-b border-border bg-card sticky top-0 z-30">
      <button on:click={() => sidebarOpen = true} class="p-2 -ml-2 text-foreground hover:bg-muted rounded-lg">
        <span class="text-xl">☰</span>
      </button>
      <h1 class="text-base font-bold text-foreground flex-1 truncate">{shopName}</h1>
      <div class="w-8 h-8 rounded-lg bg-primary flex items-center justify-center text-white font-bold text-xs">MC</div>
    </header>

    <main class="flex-1 overflow-y-auto bg-background">
      <div class="p-3 sm:p-4 md:p-6">
        {#if currentPage === 'dashboard'}
          <Dashboard on:navigate={(e) => navigate(e.detail)} />
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
        {:else if currentPage === 'analytics'}
          <Analytics />
        {:else if currentPage === 'settings'}
          <Settings />
        {/if}
      </div>
    </main>
  </div>
</div>
EOF
echo "Done"