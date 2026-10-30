<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';

  let repairs: any[] = [];
  let slips: any[] = [];
  let showForm = false;
  let loading = false;

  let formInwardId = '';
  let formDiagnosis = '';
  let formLaborCost = 0;
  let formTechnician = '';

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [repairs, slips] = await Promise.all([api.getRepairs(), api.getInwardSlips()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  async function createRepair() {
    if (!formInwardId) { alert('Please select an inward slip'); return; }
    loading = true;
    try {
      await api.createRepair(formInwardId, null, formDiagnosis || null, formLaborCost, formTechnician || null);
      showForm = false;
      formDiagnosis = ''; formLaborCost = 0; formTechnician = '';
      await loadData();
    } catch (e: any) {
      console.error('Failed to create repair', e);
      alert('Failed: ' + (e?.message || String(e)));
    } finally {
      loading = false;
    }
  }

  async function updateStatus(r: any, status: string) {
    try {
      await api.updateRepair(r.id, status, r.diagnosis, r.parts_used, r.labor_cost, r.total_cost, status === 'completed' || status === 'ready_for_delivery' ? new Date().toISOString().slice(0, 10) : null);
      await loadData();
    } catch (e: any) {
      console.error('Failed to update repair', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function getSlip(slipId: string) { return slips.find((s: any) => s.id === slipId); }

  function getStatusLabel(status: string): string {
    const labels: Record<string, string> = { pending: 'Pending', repairing: 'Repairing', ready: 'Ready', delivered: 'Delivered', completed: 'Completed', in_progress: 'In Progress', draft: 'Draft', sent: 'Sent', approved: 'Approved', rejected: 'Rejected', paid: 'Paid' };
    return labels[status] || status;
  }

  function getStatusBadge(status: string): string {
    const classes: Record<string, string> = { pending: 'badge-pending', repairing: 'badge-repairing', ready: 'badge-ready', delivered: 'badge-delivered', completed: 'badge-ready', in_progress: 'badge-repairing', draft: 'badge-draft', sent: 'badge-repairing', approved: 'badge-ready', rejected: 'badge-delivered', paid: 'badge-paid' };
    return classes[status] || 'badge-draft';
  }
</script>

<div>
  <div class="page-header">
    <div>
      <h1 class="page-title">Repairs</h1>
      <p class="page-subtitle">Track repair progress from start to delivery</p>
    </div>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Repair</button>
  </div>

  <div class="card">
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr><th>Inward Slip</th><th>Diagnosis</th><th>Cost</th><th>Technician</th><th>Status</th><th>Actions</th></tr>
        </thead>
        <tbody>
          {#each repairs as r}
            <tr>
              <td>
                <div class="font-medium">{getSlip(r.inward_id)?.slip_no || '-'}</div>
                <div style="font-size:0.75rem; color:var(--text-muted);">{getSlip(r.inward_id)?.customer_name || '-'}</div>
              </td>
              <td>{r.diagnosis || 'Pending diagnosis'}</td>
              <td class="font-medium">₹{r.total_cost.toFixed(2)}</td>
              <td>{r.technician || '-'}</td>
              <td><span class="badge {getStatusBadge(r.status)}">{getStatusLabel(r.status)}</span></td>
              <td>
                {#if r.status === 'in_progress'}
                  <div style="display:flex; gap:0.5rem;">
                    <button class="btn btn-primary btn-sm" on:click={() => updateStatus(r, 'completed')}>Complete</button>
                    <button class="btn btn-success btn-sm" on:click={() => updateStatus(r, 'ready_for_delivery')}>Ready</button>
                  </div>
                {/if}
              </td>
            </tr>
          {:else}
            <tr><td colspan="6" class="empty-state">No repairs found.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">New Repair</h2>
          <button class="modal-close" on:click={() => showForm = false}>&times;</button>
        </div>
        <form on:submit|preventDefault={createRepair}>
          <div class="form-group">
            <label class="form-label">Inward Slip</label>
            <select class="form-select" bind:value={formInwardId}>
              <option value="">Select inward slip...</option>
              {#each slips.filter((s: any) => s.status === 'pending' || s.status === 'repairing') as s}
                <option value={s.id}>#{s.slip_no} - {s.customer_name} ({s.device_type})</option>
              {/each}
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Diagnosis</label>
            <textarea class="form-textarea" bind:value={formDiagnosis} placeholder="Describe the diagnosis..."></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">Labor Cost (₹)</label>
            <input class="form-input" type="number" bind:value={formLaborCost} placeholder="0" />
          </div>
          <div class="form-group">
            <label class="form-label">Technician</label>
            <input class="form-input" bind:value={formTechnician} placeholder="Technician name" />
          </div>
          <div style="display:flex; justify-content:flex-end; gap:0.75rem; padding-top:1rem;">
            <button type="button" class="btn btn-secondary" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-primary" disabled={loading}>{loading ? 'Starting...' : 'Start Repair'}</button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"