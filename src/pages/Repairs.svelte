<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { getStatusClass, getStatusLabel } from '../lib/utils';

  let repairs: any[] = [];
  let slips: any[] = [];
  let showForm = false;
  let loading = false;

  let formInwardId = '';
  let formDiagnosis = '';
  let formLaborCost = 0;
  let formTechnician = '';
  let formPartsUsed = '';

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
    if (!formInwardId) {
      alert('Please select an inward slip');
      return;
    }
    loading = true;
    try {
      await api.createRepair(formInwardId, null, formDiagnosis || null, formLaborCost, formTechnician || null);
      showForm = false;
      resetForm();
      await loadData();
    } catch (e) {
      console.error('Failed to create repair', e);
      alert('Failed to start repair');
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    formInwardId = '';
    formDiagnosis = '';
    formLaborCost = 0;
    formTechnician = '';
    formPartsUsed = '';
  }

  async function updateStatus(r: any, status: string) {
    const today = new Date().toISOString().slice(0, 10);
    try {
      await api.updateRepair(r.id, status, r.diagnosis, r.parts_used, r.labor_cost, r.total_cost, status === 'completed' || status === 'ready_for_delivery' ? today : null);
      await loadData();
    } catch (e) {
      console.error('Failed to update repair', e);
      alert('Failed to update status');
    }
  }

  function getSlip(slipId: string) {
    return slips.find((s: any) => s.id === slipId);
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Repairs</h1>
      <p class="text-gray-600 mt-1">Track repair progress from start to delivery</p>
    </div>
    <button class="btn btn-primary" on:click={() => { showForm = true; }}>
      <span class="mr-2">+</span> New Repair
    </button>
  </div>

  <div class="card overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="table-header">
          <th class="text-left w-24">Inward Slip</th>
          <th class="text-left">Diagnosis</th>
          <th class="text-right w-24">Cost</th>
          <th class="text-left w-32">Technician</th>
          <th class="text-center w-32">Status</th>
          <th class="text-right w-32">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each repairs as r}
          <tr class="hover:bg-gray-50 transition-colors border-b border-gray-50 last:border-0">
            <td>
              <div class="text-gray-700 font-mono text-sm font-medium">{getSlip(r.inward_id)?.slip_no || '-'}</div>
              <div class="text-xs text-gray-500">{getSlip(r.inward_id)?.customer_name || '-'}</div>
            </td>
            <td class="text-gray-700">{r.diagnosis || 'Pending diagnosis'}</td>
            <td class="text-right text-gray-700 font-medium">₹{r.total_cost.toFixed(2)}</td>
            <td class="text-gray-600">{r.technician || '-'}</td>
            <td class="text-center">
              <span class="badge {getStatusClass(r.status)}">{getStatusLabel(r.status)}</span>
            </td>
            <td class="text-right">
              <div class="flex items-center gap-1.5 justify-end">
                {#if r.status === 'in_progress'}
                  <button class="btn btn-primary btn-sm" on:click={() => updateStatus(r, 'completed')}>Complete</button>
                  <button class="btn btn-success btn-sm" on:click={() => updateStatus(r, 'ready_for_delivery')}>Ready</button>
                {/if}
              </div>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="text-center py-8 text-gray-500">No repairs found.</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  {#if showForm}
    <div class="modal-overlay" on:click|self={() => showForm = false}>
      <div class="modal max-w-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="text-lg font-bold text-gray-900">New Repair</h2>
          <button class="text-gray-400 hover:text-gray-600 text-2xl" on:click={() => showForm = false}>&times;</button>
        </div>
        
        <form on:submit|preventDefault={createRepair} class="space-y-4">
          <div class="space-y-2">
            <label class="label">Inward Slip</label>
            <select class="input" bind:value={formInwardId}>
              <option value="">Select inward slip...</option>
              {#each slips.filter((s: any) => s.status === 'pending' || s.status === 'repairing') as s}
                <option value={s.id}>#{s.slip_no} - {s.customer_name} ({s.device_type})</option>
              {/each}
            </select>
          </div>

          <div class="space-y-2">
            <label class="label">Diagnosis</label>
            <textarea class="input" bind:value={formDiagnosis} rows="3" placeholder="Describe the diagnosis..."></textarea>
          </div>

          <div class="space-y-2">
            <label class="label">Labor Cost (₹)</label>
            <input class="input" type="number" bind:value={formLaborCost} placeholder="0" />
          </div>

          <div class="space-y-2">
            <label class="label">Technician</label>
            <input class="input" bind:value={formTechnician} placeholder="Technician name" />
          </div>

          <div class="space-y-2">
            <label class="label">Parts Used</label>
            <input class="input" bind:value={formPartsUsed} placeholder="e.g. Screen, Battery" />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <button type="button" class="btn btn-outline" on:click={() => showForm = false}>Cancel</button>
            <button type="submit" class="btn btn-success" disabled={loading}>
              {loading ? 'Starting...' : 'Start Repair'}
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
