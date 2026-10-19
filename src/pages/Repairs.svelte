<script lang="ts">
  import { onMount } from 'svelte';
  import { api, Repair, InwardSlip } from '../lib/api';
  import { getStatusClass, getStatusLabel } from '../lib/utils';

  let repairs: Repair[] = [];
  let slips: InwardSlip[] = [];
  let showForm = false;

  let formInwardId = '';
  let formDiagnosis = '';
  let formLaborCost = 0;
  let formTechnician = '';

  onMount(async () => {
    [repairs, slips] = await Promise.all([api.getRepairs(), api.getInwardSlips()]);
  });

  async function createRepair() {
    await api.createRepair(formInwardId, null, formDiagnosis || null, formLaborCost, formTechnician || null);
    showForm = false;
    repairs = await api.getRepairs();
  }

  async function updateStatus(r: Repair, status: string) {
    await api.updateRepair(r.id, status, r.diagnosis, r.parts_used, r.labor_cost, r.total_cost, status === 'completed' || status === 'ready_for_delivery' ? new Date().toISOString().slice(0, 10) : null);
    repairs = await api.getRepairs();
  }
</script>

<div>
  <div class="page-header">
    <h1 class="page-title">Repairs</h1>
    <button class="btn btn-primary" on:click={() => showForm = true}>+ New Repair</button>
  </div>

  <div class="card">
    <table>
      <thead><tr><th>Inward</th><th>Diagnosis</th><th>Cost</th><th>Technician</th><th>Status</th><th>Actions</th></tr></thead>
      <tbody>
        {#each repairs as r}
          <tr>
            <td>{slips.find(s => s.id === r.inward_id)?.slip_no || '-'}</td>
            <td>{r.diagnosis || '-'}</td>
            <td>{r.total_cost.toFixed(2)}</td>
            <td>{r.technician || '-'}</td>
            <td><span class="badge {getStatusClass(r.status)}">{getStatusLabel(r.status)}</span></td>
            <td class="actions">
              {#if r.status === 'in_progress'}
                <button class="btn btn-sm btn-success" on:click={() => updateStatus(r, 'completed')}>Complete</button>
                <button class="btn btn-sm btn-primary" on:click={() => updateStatus(r, 'ready_for_delivery')}>Ready</button>
              {/if}
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
          <h2 class="modal-title">New Repair</h2>
          <button class="close-btn" on:click={() => showForm = false}>&times;</button>
        </div>
        <div class="form-group"><label class="form-label">Inward Slip</label>
          <select class="input" bind:value={formInwardId}>
            <option value="">Select...</option>
            {#each slips.filter(s => s.status === 'pending' || s.status === 'repairing') as s}
              <option value={s.id}>#{s.slip_no} - {s.customer_name}</option>
            {/each}
          </select>
        </div>
        <div class="form-group"><label class="form-label">Diagnosis</label><textarea class="input" bind:value={formDiagnosis} rows="3"></textarea></div>
        <div class="form-group"><label class="form-label">Labor Cost</label><input class="input" type="number" bind:value={formLaborCost} /></div>
        <div class="form-group"><label class="form-label">Technician</label><input class="input" bind:value={formTechnician} /></div>
        <button class="btn btn-primary" on:click={createRepair}>Start Repair</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .actions { display: flex; gap: 4px; }
</style>
