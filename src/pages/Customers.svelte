<script lang="ts">
  import { onMount } from 'svelte';
  import { api, Customer } from '../lib/api';

  let customers: Customer[] = [];
  let showForm = false;
  let editingId = '';

  let formName = '';
  let formPhone = '';
  let formAddress = '';

  onMount(async () => {
    customers = await api.getCustomers();
  });

  async function save() {
    if (editingId) {
      await api.updateCustomer(editingId, formName, formPhone, formAddress || null);
    } else {
      await api.createCustomer(formName, formPhone, formAddress || null);
    }
    showForm = false;
    editingId = '';
    formName = '';
    formPhone = '';
    formAddress = '';
    customers = await api.getCustomers();
  }

  function edit(c: Customer) {
    editingId = c.id;
    formName = c.name;
    formPhone = c.phone;
    formAddress = c.address || '';
    showForm = true;
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Customers</h1>
    <button class="btn btn-primary" on:click={() => { editingId = ''; showForm = true; }}>+ Add Customer</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Name</th><th>Phone</th><th>Address</th><th>Actions</th></tr></thead>
      <tbody>
        {#each customers as c}
          <tr>
            <td>{c.name}</td>
            <td>{c.phone}</td>
            <td>{c.address || '-'}</td>
            <td class="actions">
              <button class="btn btn-sm btn-outline" on:click={() => edit(c)}>Edit</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">{editingId ? 'Edit' : 'Add'} Customer</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Name</label><input class="input" bind:value={formName} /></div>
        <div class="form-group"><label class="form-label">Phone</label><input class="input" bind:value={formPhone} /></div>
        <div class="form-group"><label class="form-label">Address</label><textarea class="input" bind:value={formAddress} rows="2"></textarea></div>
        <button class="btn btn-primary" on:click={save}>Save</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
</style>
