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
    try { customers = await api.getCustomers(); } catch (e) { console.error('Failed to load customers', e); }
    finally { loading = false; }
  }

  async function save() {
    if (!formName || !formPhone) { alert('Please enter name and phone'); return; }
    loading = true;
    try {
      if (editingId) {
        await api.updateCustomer(editingId, formName, formPhone, formAddress || null);
      } else {
        await api.createCustomer(formName, formPhone, formAddress || null);
      }
      showForm = false;
      editingId = ''; formName = ''; formPhone = ''; formAddress = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to save customer', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
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
  <div class="page-header">
    <div>
      <h1 class="page-title">Customers</h1>
      <p class="page-subtitle">Manage your customer database</p>
    </div>
    <button class="btn btn-primary" on:click={() => { editingId = ''; showForm = true; }}>+ Add Customer</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Name</th><th>Phone</th><th>Address</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each customers as c}
            <tr>
              <td class="font-medium">{c.name}</td>
              <td>{c.phone}</td>
              <td>{c.address || '-'}</td>
              <td><button class="btn btn-ghost btn-sm" on:click={() => edit(c)}>Edit</button></td>
            </tr>
          {:else}
            <tr><td colspan="4" class="empty-state">No customers found.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">{editingId ? 'Edit' : 'Add'} Customer</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={save}>
          <div class="form-group">
            <label class="form-label">Customer Name</label>
            <input class="form-input" bind:value={formName} placeholder="Customer name" />
          </div>
          <div class="form-group">
            <label class="form-label">Phone Number</label>
            <input class="form-input" type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" />
          </div>
          <div class="form-group">
            <label class="form-label">Address</label>
            <textarea class="form-textarea" bind:value={formAddress} placeholder="Customer address..."></textarea>
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Saving...' : 'Save Customer'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"