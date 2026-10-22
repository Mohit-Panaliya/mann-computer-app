<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';

  let customers: any[] = [];
  let showForm = false;
  let editingId = '';
  let loading = false;

  let formName = '';
  let formPhone = '';
  let formAddress = '';

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      customers = await api.getCustomers();
    } catch (e) {
      console.error('Failed to load customers', e);
    } finally {
      loading = false;
    }
  }

  async function save() {
    if (!formName || !formPhone) {
      alert('Please enter name and phone');
      return;
    }
    loading = true;
    try {
      if (editingId) {
        await api.updateCustomer(editingId, formName, formPhone, formAddress || null);
      } else {
        await api.createCustomer(formName, formPhone, formAddress || null);
      }
      showForm = false;
      resetForm();
      await loadData();
    } catch (e) {
      console.error('Failed to save customer', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    editingId = '';
    formName = '';
    formPhone = '';
    formAddress = '';
  }

  function edit(c: any) {
    editingId = c.id;
    formName = c.name;
    formPhone = c.phone;
    formAddress = c.address || '';
    showForm = true;
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Customers</h1>
      <p class="text-gray-600 mt-1">Manage your customer database</p>
    </div>
    <button class="btn btn-primary" on:click={() => { resetForm(); showForm = true; }}>
      <span class="mr-2">+</span> Add Customer
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left">Name</th>
          <th class="text-left w-40">Phone</th>
          <th class="text-left">Address</th>
          <th class="text-right w-32">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each customers as c}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-medium">{c.name}</td>
            <td class="text-gray-600 font-mono text-sm">{c.phone}</td>
            <td class="text-gray-600 truncate max-w-xs" title={c.address || '-'}>{c.address || '-'}</td>
            <td class="text-right">
              <button class="btn btn-ghost btn-sm text-primary-600 hover:bg-primary-50" on:click={() => edit(c)} title="Edit">
                <span class="text-lg">✏️</span>
              </button>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="4" class="text-center py-8 text-gray-500">No customers found.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">{editingId ? 'Edit' : 'Add'} Customer</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={save} class="space-y-4">
          <div class="space-y-2">
            <label class="label">Customer Name</label>
            <input class="input" bind:value={formName} placeholder="Customer name" required />
          </div>

          <div class="space-y-2">
            <label class="label">Phone Number</label>
            <input class="input" type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" required />
          </div>

          <div class="space-y-2">
            <label class="label">Address</label>
            <textarea class="input" bind:value={formAddress} rows="2" placeholder="Customer address..."></textarea>
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>
              {loading ? 'Saving...' : 'Save Customer'}
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
