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

<div class="flex h-screen overflow-hidden bg-gray-50">
  <nav class="w-64 bg-white border-r border-gray-200 flex flex-col z-20 shadow-sm">
    <div class="p-5 border-b border-gray-200">
      <div class="flex items-center gap-2">
        <div class="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center text-white font-bold">MC</div>
        <h1 class="text-sm font-bold text-gray-900 tracking-tight">{shopName}</h1>
      </div>
      <p class="text-[10px] text-gray-500 mt-1">Computer Repair &amp; Sales</p>
    </div>
    <div class="flex-1 overflow-y-auto py-3">
      {#each navItems as item}
        <button
          class="sidebar-item"
          class:sidebar-item-active={currentPage === item.id}
          on:click={() => currentPage = item.id}
        >
          <span class="text-lg">{item.icon}</span>
          <span>{item.label}</span>
        </button>
      {/each}
    </div>
    <div class="p-4 border-t border-gray-200 bg-gray-50">
      <div class="text-[10px] text-gray-500 uppercase tracking-wider font-semibold mb-2">SETTINGS</div>
      <button class="sidebar-item text-gray-600 hover:bg-gray-100 hover:text-gray-900">
        <span class="text-lg">⚙️</span>
        <span>Settings</span>
      </button>
    </div>
  </nav>

  <main class="flex-1 overflow-y-auto bg-gray-50">
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
