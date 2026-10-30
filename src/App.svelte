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
    { id: 'inward', label: 'Inward Slips', icon: '📥' },
    { id: 'quotations', label: 'Quotations', icon: '📋' },
    { id: 'repairs', label: 'Repairs', icon: '🔧' },
    { id: 'billing', label: 'Billing', icon: '🧾' },
    { id: 'delivery', label: 'Delivery', icon: '🚚' },
    { id: 'parts', label: 'Parts', icon: '📦' },
    { id: 'cctv', label: 'CCTV Quotes', icon: '📷' },
    { id: 'laptop', label: 'Laptop Quotes', icon: '💻' },
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
</script>

<div class="app-layout">
  <!-- Mobile overlay -->
  {#if sidebarOpen}
    <div class="sidebar-overlay" on:click={() => sidebarOpen = false}></div>
  {/if}

  <!-- Sidebar -->
  <aside class="sidebar {sidebarOpen ? 'open' : ''}">
    <div class="sidebar-header">
      <div class="sidebar-logo">
        <div class="sidebar-logo-icon">MC</div>
        <div>
          <div class="sidebar-title">{shopName}</div>
          <div class="sidebar-subtitle">Computer Repair & Sales</div>
        </div>
      </div>
    </div>
    <nav class="sidebar-nav">
      {#each navItems as item}
        <button
          class="sidebar-nav-item {currentPage === item.id ? 'active' : ''}"
          on:click={() => navigate(item.id)}
        >
          <span class="icon">{item.icon}</span>
          <span>{item.label}</span>
        </button>
      {/each}
    </nav>
    <div class="sidebar-footer">
      <p class="sidebar-footer-text">Mann Computer v1.3.4</p>
    </div>
  </aside>

  <!-- Main content -->
  <div class="main-content">
    <!-- Mobile header -->
    <header class="mobile-header">
      <button class="mobile-menu-btn" on:click={() => sidebarOpen = true}>☰</button>
      <h1 class="mobile-header-title">{shopName}</h1>
      <div class="sidebar-logo-icon" style="width:2rem;height:2rem;font-size:0.75rem;">MC</div>
    </header>

    <div class="content-area">
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
  </div>
</div>
EOF
echo "Done"