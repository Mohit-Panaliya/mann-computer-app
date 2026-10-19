<script lang="ts">
  import { onMount } from 'svelte';
  import { api, InwardSlip, Customer } from '../lib/api';
  import { formatDate, getStatusClass, getStatusLabel, generateWhatsAppLink } from '../lib/utils';

  let slips: InwardSlip[] = [];
  let customers: Customer[] = [];
  let showForm = false;
  let showDetail = false;
  let selectedSlip: InwardSlip | null = null;

  let formCustomerId = '';
  let formDeviceType = 'Laptop';
  let formProblem = '';
  let formRemarks = '';
  let formCustomerName = '';
  let formCustomerPhone = '';
  let newCustomer = false;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    [slips, customers] = await Promise.all([api.getInwardSlips(), api.getCustomers()]);
  }

  async function createSlip() {
    let customerId = formCustomerId;
    if (newCustomer) {
      const c = await api.createCustomer(formCustomerName, formCustomerPhone);
      customerId = c.id;
    }
    if (!customerId) return;
    await api.createInwardSlip(customerId, formDeviceType, formProblem || null, null, formRemarks || null);
    showForm = false;
    formProblem = '';
    formRemarks = '';
    await loadData();
  }

  async function viewSlip(s: InwardSlip) {
    selectedSlip = await api.getInwardSlip(s.id);
    showDetail = true;
  }

  async function updateStatus(s: InwardSlip, status: string) {
    await api.updateInwardSlipStatus(s.id, status);
    await loadData();
  }

  async function sendWhatsApp(s: InwardSlip) {
    const msg = `Dear ${s.customer_name},\nYour ${s.device_type} has been received at Mann Computer.\nInward No: ${s.slip_no}\nDate: ${s.date}\nProblem: ${s.problem_description || 'N/A'}\n\nThank you!`;
    window.open(generateWhatsAppLink(s.customer_phone || '', msg), '_blank');
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Inward Slips</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Inward</button>
  </div>

  <div class="card">
    <table>
      <thead>
        <tr>
          <th>Slip No</th><th>Date</th><th>Customer</th><th>Device</th><th>Problem</th><th>Status</th><th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each slips as s}
          <tr>
            <td>{s.slip_no}</td>
            <td>{formatDate(s.date)}</td>
            <td>{s.customer_name}</td>
            <td>{s.device_type}</td>
            <td>{s.problem_description || '-'}</td>
            <td><span class="badge {getStatusClass(s.status)}">{getStatusLabel(s.status)}</span></td>
            <td class="actions">
              <button class="btn btn-sm btn-outline" on:click={() => viewSlip(s)}>View</button>
              {#if s.status === 'pending'}
                <button class="btn btn-sm btn-primary" on:click={() => updateStatus(s, 'repairing')}>Start Repair</button>
              {:else if s.status === 'repairing'}
                <button class="btn btn-sm btn-success" on:click={() => updateStatus(s, 'ready')}>Ready</button>
              {:else if s.status === 'ready'}
                <button class="btn btn-sm btn-success" on:click={() => updateStatus(s, 'delivered')}>Deliver</button>
              {/if}
              <button class="btn btn-sm btn-outline" on:click={() => sendWhatsApp(s)}>WA</button>
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
          <h2 class="modal-title">New Inward Slip</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group">
          <label class="form-label">Customer</label>
          <label><input type="checkbox" bind:checked={newCustomer} /> New Customer</label>
        </div>
        {#if newCustomer}
          <div class="form-group"><label class="form-label">Name</label><input class="input" bind:value={formCustomerName} /></div>
          <div class="form-group"><label class="form-label">Phone</label><input class="input" bind:value={formCustomerPhone} /></div>
        {:else}
          <div class="form-group"><label class="form-label">Select Customer</label>
            <select class="input" bind:value={formCustomerId}>
              <option value="">Select...</option>
              {#each customers as c}<option value={c.id}>{c.name} - {c.phone}</option>{/each}
            </select>
          </div>
        {/if}
        <div class="form-group"><label class="form-label">Device Type</label>
          <select class="input" bind:value={formDeviceType}>
            <option>Laptop</option><option>Desktop</option><option>CCTV</option><option>Printer</option><option>Other</option>
          </select>
        </div>
        <div class="form-group"><label class="form-label">Problem Description</label><textarea class="input" bind:value={formProblem} rows="3"></textarea></div>
        <div class="form-group"><label class="form-label">Remarks</label><input class="input" bind:value={formRemarks} /></div>
        <button class="btn btn-primary" on:click={createSlip}>Create Slip</button>
      </div>
    </div>
  {/if}

  {#if showDetail && selectedSlip}
    <div class="modal-overlay" on:click|self={() => showDetail = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">Inward Slip {selectedSlip.slip_no}</h2>
          <button class="close-btn" on:click={() => showDetail = false}>&times;</button>
        </div>
        <p><strong>Customer:</strong> {selectedSlip.customer_name} ({selectedSlip.customer_phone})</p>
        <p><strong>Date:</strong> {selectedSlip.date}</p>
        <p><strong>Device:</strong> {selectedSlip.device_type}</p>
        <p><strong>Problem:</strong> {selectedSlip.problem_description || 'N/A'}</p>
        <p><strong>Status:</strong> {getStatusLabel(selectedSlip.status)}</p>
        <p><strong>Remarks:</strong> {selectedSlip.remarks || 'N/A'}</p>
        <button class="btn btn-primary" on:click={() => sendWhatsApp(selectedSlip)}>Send via WhatsApp</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; flex-wrap: wrap; }
  .card { overflow-x: auto; }
</style>
