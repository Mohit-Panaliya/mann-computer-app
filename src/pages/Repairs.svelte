<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { getStatusLabel, getStatusVariant } from '../lib/utils';
  import Card from '$lib/components/ui/card.svelte';
  import CardContent from '$lib/components/ui/card-content.svelte';
  import CardHeader from '$lib/components/ui/card-header.svelte';
  import CardTitle from '$lib/components/ui/card-title.svelte';
  import Badge from '$lib/components/ui/badge.svelte';
  import Button from '$lib/components/ui/button.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Select from '$lib/components/ui/select.svelte';
  import Dialog from '$lib/components/ui/dialog.svelte';
  import Textarea from '$lib/components/ui/textarea.svelte';

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
    if (!formInwardId) {
      alert('Please select an inward slip');
      return;
    }
    loading = true;
    try {
      await api.createRepair(formInwardId, null, formDiagnosis || null, formLaborCost, formTechnician || null);
      showForm = false;
      formDiagnosis = '';
      formLaborCost = 0;
      formTechnician = '';
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

  function getSlip(slipId: string) {
    return slips.find((s: any) => s.id === slipId);
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-foreground">Repairs</h1>
      <p class="text-muted-foreground mt-1">Track repair progress from start to delivery</p>
    </div>
    <Button variant="default" on:click={() => showForm = true}>
      <span class="mr-2">+</span> New Repair
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="w-24 py-3">Inward Slip</th>
              <th class="py-3">Diagnosis</th>
              <th class="w-24 py-3 text-right">Cost</th>
              <th class="w-32 py-3">Technician</th>
              <th class="w-32 py-3 text-center">Status</th>
              <th class="w-24 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each repairs as r}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td>
                  <div class="text-sm font-mono font-medium text-foreground">{getSlip(r.inward_id)?.slip_no || '-'}</div>
                  <div class="text-xs text-muted-foreground">{getSlip(r.inward_id)?.customer_name || '-'}</div>
                </td>
                <td class="py-3 text-sm text-muted-foreground">{r.diagnosis || 'Pending diagnosis'}</td>
                <td class="py-3 text-sm font-medium text-foreground text-right">₹{r.total_cost.toFixed(2)}</td>
                <td class="py-3 text-sm text-muted-foreground">{r.technician || '-'}</td>
                <td class="py-3 text-center"><Badge variant={getStatusVariant(r.status)} class="text-xs">{getStatusLabel(r.status)}</Badge></td>
                <td class="py-3 text-right">
                  <div class="flex items-center gap-2 justify-end">
                    {#if r.status === 'in_progress'}
                      <Button variant="secondary" size="sm" class="px-3" on:click={() => updateStatus(r, 'completed')}>Complete</Button>
                      <Button variant="default" size="sm" class="px-3" on:click={() => updateStatus(r, 'ready_for_delivery')}>Ready</Button>
                    {/if}
                  </div>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="6" class="py-8 text-center text-muted-foreground">No repairs found.</td>
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
          <h2 class="text-lg font-bold text-foreground">New Repair</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={createRepair} class="space-y-4">
          <div class="space-y-2">
            <Label>Inward Slip</Label>
            <Select bind:value={formInwardId}>
              <option value="">Select inward slip...</option>
              {#each slips.filter((s: any) => s.status === 'pending' || s.status === 'repairing') as s}
                <option value={s.id}>#{s.slip_no} - {s.customer_name} ({s.device_type})</option>
              {/each}
            </Select>
          </div>

          <div class="space-y-2">
            <Label>Diagnosis</Label>
            <Textarea bind:value={formDiagnosis} rows="3" placeholder="Describe the diagnosis..." />
          </div>

          <div class="space-y-2">
            <Label>Labor Cost (₹)</Label>
            <Input type="number" bind:value={formLaborCost} placeholder="0" />
          </div>

          <div class="space-y-2">
            <Label>Technician</Label>
            <Input bind:value={formTechnician} placeholder="Technician name" />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default" disabled={loading}>
              {loading ? 'Starting...' : 'Start Repair'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
