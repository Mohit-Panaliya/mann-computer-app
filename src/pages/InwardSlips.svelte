<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { formatDate, generateWhatsAppLink } from '../lib/utils';

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
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function viewSlip(s: any) {
    try {
      selectedSlip = await api.getInwardSlip(s.id);
      showDetail = true;
    } catch (e: any) {
      console.error('Failed to load slip', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  async function updateStatus(s: any, status: string) {
    if (!confirm(`Mark as ${status}?`)) return;
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

  function getStatusLabel(status: string): string {
    const labels: Record<string, string> = {
      pending: 'Pending', repairing: 'Repairing', ready: 'Ready', delivered: 'Delivered',
      completed: 'Completed', in_progress: 'In Progress', draft: 'Draft', sent: 'Sent',
      approved: 'Approved', rejected: 'Rejected', paid: 'Paid',
    };
    return labels[status] || status;
  }

  function getStatusBadge(status: string): string {
    const classes: Record<string, string> = {
      pending: 'badge-pending', repairing: 'badge-repairing', ready: 'badge-ready',
      delivered: 'badge-delivered', completed: 'badge-ready', in_progress: 'badge-repairing',
      draft: 'badge-draft', sent: 'badge-repairing', approved: 'badge-ready',
      rejected: 'badge-delivered', paid: 'badge-paid',
    };
    return classes[status] || 'badge-draft';
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Inward Slips</h1>
      <p class="page-subtitle">Record new device receipts and track repair status</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Inward</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Slip No</th>
            <th>Date</th>
            <th>Customer</th>
            <th>Device</th>
            <th>Problem</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each slips as s}
            <tr>
              <td class="font-medium">{s.slip_no}</td>
              <td>{formatDate(s.date)}</td>
              <td>{s.customer_name}</td>
              <td>{s.device_type}</td>
              <td style="max-width:200px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">{s.problem_description || '-'}</td>
              <td><span class="badge {getStatusBadge(s.status)}">{getStatusLabel(s.status)}</span></td>
              <td>
                <div style="display:flex; gap:0.5rem; flex-wrap:wrap;">
                  <button class="btn btn-ghost btn-sm" on:click={() => viewSlip(s)}>View</button>
                  {#if s.status === 'pending'}
                    <button class="btn btn-primary btn-sm" on:click={() => updateStatus(s, 'repairing')}>Start Repair</button>
                  {:else if s.status === 'repairing'}
                    <button class="btn btn-success btn-sm" on:click={() => updateStatus(s, 'ready')}>Ready</button>
                  {:else if s.status === 'ready'}
                    <button class="btn btn-success btn-sm" on:click={() => updateStatus(s, 'delivered')}>Deliver</button>
                  {/if}
                  <button class="btn btn-ghost btn-sm" on:click={() => sendWhatsApp(s)} title="WhatsApp">💬</button>
                </div>
              </td>
            </tr>
          {:else}
            <tr>
              <td colspan="7" class="empty-state">No inward slips found. Create your first slip.</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">New Inward Slip</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createSlip}>
          <div class="form-group">
            <label class="form-label">Customer</label>
            <label style="display:flex; align-items:center; gap:0.5rem; cursor:pointer; padding:0.5rem; background:var(--surface-alt); border-radius:0.5rem;">
              <input type="checkbox" bind:checked={newCustomer} />
              <span style="font-size:0.875rem;">New Customer</span>
            </label>
          </div>
          {#if newCustomer}
            <div class="form-group">
              <label class="form-label">Customer Name</label>
              <input class="form-input" bind:value={formCustomerName} placeholder="Enter name" />
            </div>
            <div class="form-group">
              <label class="form-label">Phone Number</label>
              <input class="form-input" type="tel" bind:value={formCustomerPhone} placeholder="e.g. 9876543210" />
            </div>
          {:else}
            <div class="form-group">
              <label class="form-label">Select Customer</label>
              <select class="form-select" bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}
                  <option value={c.id}>{c.name} - {c.phone}</option>
                {/each}
              </select>
            </div>
          {/if}
          <div class="form-group">
            <label class="form-label">Device Type</label>
            <select class="form-select" bind:value={formDeviceType}>
              <option>Laptop</option><option>Desktop</option><option>CCTV Camera</option><option>Printer</option><option>Other</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Problem Description</label>
            <textarea class="form-textarea" bind:value={formProblem} placeholder="Describe the issue..."></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">Remarks</label>
            <input class="form-input" bind:value={formRemarks} placeholder="Additional notes" />
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Creating...' : 'Create Slip'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showDetail && selectedSlip}
    <div class="modal-overlay" on:click|self={() => showDetail = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">Inward Slip {selectedSlip.slip_no}</h2>
          <button class="modal-close" on:click={() => showDetail = false}>&times;</button>
        </div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:0.75rem; margin-bottom:1rem;">
          <div style="padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem;">
            <div style="font-size:0.75rem; color:var(--text-muted); text-transform:uppercase; font-weight:500;">Customer</div>
            <div style="font-weight:600; font-size:0.875rem;">{selectedSlip.customer_name}</div>
            <div style="font-size:0.75rem; color:var(--text-secondary);">{selectedSlip.customer_phone}</div>
          </div>
          <div style="padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem;">
            <div style="font-size:0.75rem; color:var(--text-muted); text-transform:uppercase; font-weight:500;">Device</div>
            <div style="font-weight:600; font-size:0.875rem;">{selectedSlip.device_type}</div>
          </div>
          <div style="padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem;">
            <div style="font-size:0.75rem; color:var(--text-muted); text-transform:uppercase; font-weight:500;">Problem</div>
            <div style="font-size:0.875rem;">{selectedSlip.problem_description || 'N/A'}</div>
          </div>
          <div style="padding:0.75rem; background:var(--surface-alt); border-radius:0.5rem;">
            <div style="font-size:0.75rem; color:var(--text-muted); text-transform:uppercase; font-weight:500;">Status</div>
            <span class="badge {getStatusBadge(selectedSlip.status)}">{getStatusLabel(selectedSlip.status)}</span>
          </div>
        </div>
        {#if selectedSlip.remarks}
          <div style="margin-bottom:1rem;">
            <div style="font-size:0.75rem; color:var(--text-muted); text-transform:uppercase; font-weight:500; margin-bottom:0.25rem;">Remarks</div>
            <div style="font-size:0.875rem;">{selectedSlip.remarks}</div>
          </div>
        {/if}
        <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem; border-top:1px solid var(--border);">
          <button class="btn btn-secondary btn-sm" on:click={() => sendWhatsApp(selectedSlip)}>💬 WhatsApp</button>
          <button class="btn btn-primary btn-sm" on:click={() => updateStatus(selectedSlip, selectedSlip.status === 'ready' ? 'delivered' : 'repairing')}>Update Status</button>
        </div>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"