<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';
  import { openExternal, formatDate, generateWhatsAppLink } from '../lib/utils';
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
    } catch (e: any) {
      console.error('Failed to load slip', e);
      alert('Failed to load slip: ' + (e?.message || String(e)));
    }
  }

  async function updateStatus(s: any, status: string) {
    if (!confirm(`Mark this inward slip as ${status}?`)) return;
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
    openExternal(generateWhatsAppLink(s.customer_phone || '', msg));
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

  function getStatusVariant(status: string): string {
    const variants: Record<string, string> = {
      pending: 'outline', repairing: 'default', ready: 'secondary', delivered: 'outline',
      completed: 'secondary', in_progress: 'default', draft: 'outline', sent: 'default',
      approved: 'secondary', rejected: 'destructive', paid: 'secondary',
    };
    return variants[status] || 'outline';
  }
</script>

<div>
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold text-foreground">Inward Slips</h1>
      <p class="text-muted-foreground mt-1">Record new device receipts and track repair status</p>
    </div>
    <Button variant="default" on:click={() => showForm = true}>
      <span class="mr-2">+</span> New Inward Slip
    </Button>
  </div>

  <Card>
    <CardContent class="p-0">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
              <th class="w-24 py-3">Slip No</th>
              <th class="py-3">Date</th>
              <th class="py-3">Customer</th>
              <th class="w-32 py-3">Device</th>
              <th class="py-3">Problem</th>
              <th class="w-32 py-3 text-center">Status</th>
              <th class="w-56 py-3 text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each slips as s}
              <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                <td class="py-3 text-sm font-mono font-medium text-foreground">{s.slip_no}</td>
                <td class="py-3 text-sm text-muted-foreground">{formatDate(s.date)}</td>
                <td class="py-3 text-sm font-medium text-foreground">{s.customer_name}</td>
                <td class="py-3 text-sm text-muted-foreground capitalize">{s.device_type}</td>
                <td class="py-3 text-sm text-muted-foreground truncate max-w-xs" title={s.problem_description || 'N/A'}>{s.problem_description || 'N/A'}</td>
                <td class="py-3 text-center"><Badge variant={getStatusVariant(s.status)} class="text-xs">{getStatusLabel(s.status)}</Badge></td>
                <td class="py-3 text-right">
                  <div class="flex items-center gap-2 justify-end">
                    <Button variant="ghost" size="sm" class="p-2" on:click={() => viewSlip(s)} title="View Details">
                      <span class="text-lg">👁️</span>
                    </Button>
                    {#if s.status === 'pending'}
                      <Button variant="default" size="sm" class="px-3" on:click={() => updateStatus(s, 'repairing')}>Start Repair</Button>
                    {:else if s.status === 'repairing'}
                      <Button variant="secondary" size="sm" class="px-3" on:click={() => updateStatus(s, 'ready')}>Mark Ready</Button>
                    {:else if s.status === 'ready'}
                      <Button variant="secondary" size="sm" class="px-3" on:click={() => updateStatus(s, 'delivered')}>Deliver</Button>
                    {/if}
                    <Button variant="ghost" size="sm" class="p-2 text-green-600 hover:bg-green-50" on:click={() => sendWhatsApp(s)} title="WhatsApp">
                      <span class="text-lg">💬</span>
                    </Button>
                  </div>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="7" class="py-8 text-center text-muted-foreground">No inward slips found. Create your first slip.</td>
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
          <h2 class="text-lg font-bold text-foreground">New Inward Slip</h2>
          <button on:click={() => showForm = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <form on:submit|preventDefault={createSlip} class="space-y-4">
          <div class="space-y-2">
            <Label>Customer</Label>
            <label class="flex items-center gap-2 cursor-pointer p-2 bg-muted rounded-lg hover:bg-muted/50">
              <input type="checkbox" bind:checked={newCustomer} class="rounded text-primary focus:ring-primary" />
              <span class="text-sm font-medium">New Customer</span>
            </label>
          </div>

          {#if newCustomer}
            <div class="space-y-2">
              <Label>Customer Name</Label>
              <Input bind:value={formCustomerName} placeholder="Enter customer name" />
            </div>
            <div class="space-y-2">
              <Label>Phone Number</Label>
              <Input type="tel" bind:value={formCustomerPhone} placeholder="e.g. 9876543210" />
            </div>
          {:else}
            <div class="space-y-2">
              <Label>Select Customer</Label>
              <Select bind:value={formCustomerId}>
                <option value="">Select customer...</option>
                {#each customers as c}<option value={c.id}>{c.name} - {c.phone}</option>{/each}
              </Select>
            </div>
          {/if}

          <div class="space-y-2">
            <Label>Device Type</Label>
            <Select bind:value={formDeviceType}>
              <option>Laptop</option><option>Desktop</option><option>CCTV Camera</option><option>Printer</option><option>Other</option>
            </Select>
          </div>

          <div class="space-y-2">
            <Label>Problem Description</Label>
            <Textarea bind:value={formProblem} rows="3" placeholder="Describe the issue..." />
          </div>

          <div class="space-y-2">
            <Label>Remarks</Label>
            <Input bind:value={formRemarks} placeholder="Additional notes" />
          </div>

          <div class="flex justify-end gap-3 pt-4">
            <Button variant="outline" on:click={() => showForm = false}>Cancel</Button>
            <Button variant="default" disabled={loading}>
              {loading ? 'Creating...' : 'Create Slip'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  {/if}

  {#if showDetail && selectedSlip}
    <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" >
      <div class="bg-background border border-input rounded-xl shadow-xl w-full max-w-lg p-6" role="dialog" aria-modal="true">
        <div class="flex justify-between items-start mb-6">
          <div>
            <h2 class="text-lg font-bold text-foreground">Inward Slip {selectedSlip.slip_no}</h2>
            <p class="text-muted-foreground text-sm">{selectedSlip.date}</p>
          </div>
          <button on:click={() => showDetail = false} class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
            <span class="sr-only">Close</span>
            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
          </button>
        </div>

        <div class="grid grid-cols-2 gap-4 mb-6">
          <div class="p-3 bg-muted rounded-lg">
            <p class="text-xs font-semibold text-muted-foreground uppercase">Customer</p>
            <p class="text-sm font-medium text-foreground">{selectedSlip.customer_name}</p>
            <p class="text-xs text-muted-foreground">{selectedSlip.customer_phone}</p>
          </div>
          <div class="p-3 bg-muted rounded-lg">
            <p class="text-xs font-semibold text-muted-foreground uppercase">Device</p>
            <p class="text-sm font-medium text-foreground capitalize">{selectedSlip.device_type}</p>
          </div>
          <div class="p-3 bg-muted rounded-lg">
            <p class="text-xs font-semibold text-muted-foreground uppercase">Problem</p>
            <p class="text-sm text-muted-foreground">{selectedSlip.problem_description || 'N/A'}</p>
          </div>
          <div class="p-3 bg-muted rounded-lg">
            <p class="text-xs font-semibold text-muted-foreground uppercase">Status</p>
            <Badge variant={getStatusVariant(selectedSlip.status)}>{getStatusLabel(selectedSlip.status)}</Badge>
          </div>
        </div>

        {#if selectedSlip.remarks}
          <div class="mb-6">
            <p class="text-xs font-semibold text-muted-foreground uppercase mb-1">Remarks</p>
            <p class="text-sm text-foreground">{selectedSlip.remarks}</p>
          </div>
        {/if}

        <div class="flex gap-3 justify-end pt-4 border-t border-border">
          <Button variant="outline" on:click={() => sendWhatsApp(selectedSlip)}>Send WhatsApp</Button>
          <Button variant="default" on:click={() => updateStatus(selectedSlip, selectedSlip.status === 'ready' ? 'delivered' : 'repairing')}>Update Status</Button>
        </div>
      </div>
    </div>
  {/if}
</div>
