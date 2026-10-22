<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Button from '$lib/components/ui/button.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Textarea from '$lib/components/ui/textarea.svelte';
  import Dialog from '$lib/components/ui/dialog.svelte';

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
      editingId = '';
      formName = '';
      formPhone = '';
      formAddress = '';
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
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-foreground">Customers</h1>
      <p class="text-muted-foreground mt-1">Manage your customer database</p>
    </div>
    <Button variant="default" on:click={() => { editingId = ''; showForm = true; }}>
      <span class="mr-2">+</span> Add Customer
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="py-3">Name</th>
              <th class="w-40 py-3">Phone</th>
              <th class="py-3">Address</th>
              <th class="w-32 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each customers as c}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td class="py-3 text-sm font-medium text-foreground">{c.name}</td>
                <td class="py-3 text-sm text-muted-foreground font-mono">{c.phone}</td>
                <td class="py-3 text-sm text-muted-foreground truncate max-w-xs" title={c.address || '-'}>{c.address || '-'}</td>
                <td class="py-3 text-right">
                  <Button variant="ghost" size="sm" class="p-2 text-primary-600 hover:bg-primary-50" on:click={() => edit(c)} title="Edit">
                    <span class="text-lg">✏️</span>
                  </Button>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="4" class="py-8 text-center text-muted-foreground">No customers found.</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </CardContent>
  </Card>

  {#if showForm}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-md p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-foreground">{editingId ? 'Edit' : 'Add'} Customer</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={save} class="space-y-4">
          <div class="space-y-2">
            <Label>Customer Name</Label>
            <Input bind:value={formName} placeholder="Customer name" required />
          </div>
          <div class="space-y-2">
            <Label>Phone Number</Label>
            <Input type="tel" bind:value={formPhone} placeholder="e.g. 9876543210" required />
          </div>
          <div class="space-y-2">
            <Label>Address</Label>
            <Textarea bind:value={formAddress} rows="2" placeholder="Customer address..." />
          </div>
          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default" disabled={loading}>
              {loading ? 'Saving...' : 'Save Customer'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
