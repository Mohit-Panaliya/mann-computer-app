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

<div class="layout">
  <nav class="sidebar">
    <div class="sidebar-header">
      <div class="sidebar-title">{shopName}</div>
    </div>
    <div class="sidebar-nav">
      {#each navItems as item}
        <div
          class="nav-item"
          class:active={currentPage === item.id}
          on:click={() => currentPage = item.id}
        >
          <span>{item.icon}</span>
          <span>{item.label}</span>
        </div>
      {/each}
    </div>
  </nav>

  <main class="main-content">
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
  </main>
</div>

<style>
  .layout {
    display: flex;
    height: 100vh;
  }
</style>
