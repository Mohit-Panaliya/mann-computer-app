<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatDate, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

  let slips: any[] = [];
  let customers: any[] = [];
  let showForm = false;
  let showDetail = false;
  let selectedSlip: any = null;
  let loading = false;

  let formCustomerId = '';
  let formDeviceType = 'Laptop';
  let formProblem = '';
  let formRemarks = '';
  let formCustomerName = '';
  let formCustomerPhone = '';
  let newCustomer = false;

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [slips, customers] = await Promise.all([api.getInwardSlips(), api.getCustomers()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  async function createSlip() {
    if (newCustomer && (!formCustomerName || !formCustomerPhone)) {
      alert('Please enter customer name and phone');
      return;
    }
    if (!newCustomer && !formCustomerId) {
      alert('Please select a customer');
      return;
    }

    loading = true;
    try {
      let customerId = formCustomerId;
      if (newCustomer) {
        const c = await api.createCustomer(formCustomerName, formCustomerPhone);
        customerId = c.id;
      }
      await api.createInwardSlip(customerId, formDeviceType, formProblem || null, null, formRemarks || null);
      showForm = false;
      resetForm();
      await loadData();
    } catch (e: any) {
      console.error('Failed to create slip', e);
      alert('Failed to create inward slip: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function viewSlip(s: any) {
    try {
      selectedSlip = await api.getInwardSlip(s.id);
      showDetail = true;
    } catch (e) {
      console.error('Failed to load slip', e);
    }
  }

  async function updateStatus(s: any, status: string) {
    if (!confirm(`Mark this inward slip as ${getStatusLabel(status)}?`)) return;
    try {
      await api.updateInwardSlipStatus(s.id, status);
      await loadData();
    } catch (e: any) {
      console.error('Failed to update status', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function sendWhatsApp(s: any) {
    const msg = `Dear ${s.customer_name},\nYour ${s.device_type} has been received at Mann Computer.\nInward No: ${s.slip_no}\nDate: ${s.date}\nProblem: ${s.problem_description || 'N/A'}\n\nThank you!`;
    window.open(generateWhatsAppLink(s.customer_phone || '', msg), '_blank');
  }

  function resetForm() {
    formCustomerId = '';
    formDeviceType = 'Laptop';
    formProblem = '';
    formRemarks = '';
    formCustomerName = '';
    formCustomerPhone = '';
    newCustomer = false;
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Inward Slips</h1>
      <p class="text-gray-600 mt-1">Record new device receipts and track repair status</p>
    </div>
    <button class="btn btn-primary" on:click={() => { showForm = true; }}>
      <span class="mr-2">+</span> New Inward Slip
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left w-20">Slip No</th>
          <th class="text-left">Date</th>
          <th class="text-left">Customer</th>
          <th class="text-left w-32">Device</th>
          <th class="text-left">Problem</th>
          <th class="text-center w-32">Status</th>
          <th class="text-right w-40">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each slips as s}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td class="text-gray-700 font-mono text-sm font-medium">{s.slip_no}</td>
            <td class="text-gray-600">{formatDate(s.date)}</td>
            <td class="text-gray-700 font-medium">{s.customer_name}</td>
            <td class="text-gray-600 capitalize">{s.device_type}</td>
            <td class="text-gray-600 truncate max-w-xs" title={s.problem_description || 'N/A'}>{s.problem_description || 'N/A'}</td>
            <td class="text-center">
              <span class="badge {getStatusClass(s.status)}">{getStatusLabel(s.status)}</span>
            </td>
            <td class="text-right">
              <div class="flex items-center gap-1.5 justify-end">
                <button class="btn btn-ghost btn-sm" on:click={() => viewSlip(s)} title="View Details">
                  <span class="text-lg">👁️</span>
                </button>
                {#if s.status === 'pending'}
                  <button class="btn btn-primary btn-sm" on:click={() => updateStatus(s, 'repairing')}>Start Repair</button>
                {:else if s.status === 'repairing'}
                  <button class="btn btn-success btn-sm" on:click={() => updateStatus(s, 'ready')}>Mark Ready</button>
                {:else if s.status === 'ready'}
                  <button class="btn btn-success btn-sm" on:click={() => updateStatus(s, 'delivered')}>Deliver</button>
                {/if}
                <button class="btn btn-ghost btn-sm text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(s)} title="WhatsApp">
                  <span class="text-lg">💬</span>
                </button>
              </div>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="7" class="text-center py-8 text-gray-500">No inward slips found. Create your first slip.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">New Inward Slip</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createSlip} class="space-y-4">
          <div class="space-y-2">
            <label class="label">Customer</label>
            <label class="flex items-center gap-2 cursor-pointer p-2 bg-gray-50 rounded-lg hover:bg-gray-100">
              <input type="checkbox" bind:checked={newCustomer} class="rounded text-primary-600 focus:ring-primary-500" />
              <span class="text-sm font-medium">New Customer</span>
            </label>
          </div>

          {#if newCustomer}
            <div class="space-y-2">
              <label class="label">Customer Name</label>
              <input class="input" bind:value={formCustomerName} placeholder="Enter customer name" />
            </div>
            <div class="space-y-2">
              <label class="label">Phone Number</label>
              <input class="input" type="tel" bind:value={formCustomerPhone} placeholder="e.g. 9876543210" />
            </div>
          {:else}
            <div class="space-y-2">
              <label class="label">Select Customer</label>
              <select class="input" bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}
                  <option value={c.id}>{c.name} - {c.phone}</option>
                {/each}
              </select>
            </div>
          {/if}

          <div class="space-y-2">
            <label class="label">Device Type</label>
            <select class="input" bind:value={formDeviceType}>
              <option value="Laptop">Laptop</option>
              <option value="Desktop">Desktop</option>
              <option value="CCTV">CCTV Camera</option>
              <option value="Printer">Printer</option>
              <option value="Other">Other</option>
            </select>
          </div>

          <div class="space-y-2">
            <label class="label">Problem Description</label>
            <textarea class="input" bind:value={formProblem} rows="3" placeholder="Describe the issue..."></textarea>
          </div>

          <div class="space-y-2">
            <label class="label">Remarks</label>
            <input class="input" bind:value={formRemarks} placeholder="Additional notes" />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>
              {loading ? 'Creating...' : 'Create Slip'}
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showDetail && selectedSlip}
    <div class="modal-overlay" on:click|self={() => showDetail = false}>
      <div class="modal max-w-lg">
        <div class="flex justify-between items-start mb-6">
          <div>
            <h2 class="text-lg font-bold text-gray-900">Inward Slip {selectedSlip.slip_no}</h2>
            <p class="text-gray-600 text-sm">{selectedSlip.date}</p>
          </div>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showDetail = false}>&times;</button>
        </div>
        
        <div class="grid grid-cols-2 gap-4 mb-6">
          <div class="p-3 bg-gray-50 rounded-xl">
            <p class="text-xs font-semibold text-gray-500 uppercase">Customer</p>
            <p class="text-gray-900 font-medium">{selectedSlip.customer_name}</p>
            <p class="text-xs text-gray-600">{selectedSlip.customer_phone}</p>
          </div>
          <div class="p-3 bg-gray-50 rounded-xl">
            <p class="text-xs font-semibold text-gray-500 uppercase">Device</p>
            <p class="text-gray-900 font-medium capitalize">{selectedSlip.device_type}</p>
          </div>
          <div class="p-3 bg-gray-50 rounded-xl">
            <p class="text-xs font-semibold text-gray-500 uppercase">Problem</p>
            <p class="text-gray-900 text-sm">{selectedSlip.problem_description || 'N/A'}</p>
          </div>
          <div class="p-3 bg-gray-50 rounded-xl">
            <p class="text-xs font-semibold text-gray-500 uppercase">Status</p>
            <span class="badge {getStatusClass(selectedSlip.status)}">{getStatusLabel(selectedSlip.status)}</span>
          </div>
        </div>

        {#if selectedSlip.remarks}
          <div class="mb-6">
            <p class="text-xs font-semibold text-gray-500 uppercase mb-1">Remarks</p>
            <p class="text-gray-700 text-sm">{selectedSlip.remarks}</p>
          </div>
        {/if}

        <div class="flex gap-3 justify-end pt-4 border-t border-gray-100">
          <button class="btn btn-outline" on:click={() => sendWhatsApp(selectedSlip)}>
            <span class="mr-2">💬</span> Send WhatsApp
          </button>
          <button class="btn btn-success" on:click={() => updateStatus(selectedSlip, selectedSlip.status === 'ready' ? 'delivered' : 'repairing')}>
            Update Status
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* No extra styles needed - using Tailwind */
</style>
