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

  let slips: any[] = [];
  let bills: any[] = [];
  let loading = false;

  onMount(loadData);

  async function loadData() {
    loading = true;
    try {
      [slips, bills] = await Promise.all([api.getInwardSlips(), api.getBills()]);
    } catch (e) {
      console.error('Failed to load data', e);
    } finally {
      loading = false;
    }
  }

  async function deliver(s: any) {
    const bill = bills.find((b: any) => b.inward_id === s.id);
    if (!confirm(`Mark inward slip ${s.slip_no} as delivered?`)) return;
    try {
      await api.createDelivery(s.id, bill?.id || null, s.customer_id, null, s.customer_name, null);
      await loadData();
    } catch (e: any) {
      console.error('Failed to mark delivered', e);
      alert('Failed: ' + (e?.message || String(e)));
    }
  }

  function sendDeliveryWA(s: any) {
    const msg = `Dear ${s.customer_name},\nYour ${s.device_type} is ready for delivery.\nInward No: ${s.slip_no}\n\nPlease visit Mann Computer to collect your device.\n\nThank you!`;
    openExternal(generateWhatsAppLink(s.customer_phone || '', msg));
  }

  const readySlips = slips.filter((s: any) => s.status === 'ready' || s.status === 'completed');
  const deliveredSlips = slips.filter((s: any) => s.status === 'delivered');
</script>

<div>
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-foreground">Delivery</h1>
    <p class="text-muted-foreground mt-1">Manage device delivery to customers</p>
  </div>

  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-semibold text-foreground mb-3 flex items-center gap-2">
        <span class="text-yellow-600">📦</span> Ready for Delivery
      </h3>
      <Card>
        <CardContent class="p-0">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
                <th class="w-24 py-3">Slip No</th>
                <th class="w-40 py-3">Date</th>
                <th class="py-3">Customer</th>
                <th class="w-32 py-3">Device</th>
                <th class="w-56 py-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each readySlips as s}
                <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                  <td class="py-3 text-sm font-mono font-medium text-foreground">{s.slip_no}</td>
                  <td class="py-3 text-sm text-muted-foreground">{formatDate(s.date)}</td>
                  <td class="py-3 text-sm font-medium text-foreground">{s.customer_name}</td>
                  <td class="py-3 text-sm text-muted-foreground capitalize">{s.device_type}</td>
                  <td class="py-3 text-right">
                    <div class="flex items-center gap-2">
                      <Button variant="secondary" size="sm" class="w-full" on:click={() => deliver(s)}>Mark Delivered</Button>
                      <Button variant="ghost" size="sm" class="p-2 text-green-600 hover:bg-green-50" on:click={() => sendDeliveryWA(s)} title="Notify WhatsApp">
                        <span class="text-lg">💬</span>
                      </Button>
                    </div>
                  </td>
                </tr>
              {:else}
                <tr>
                  <td colspan="5" class="py-8 text-center text-muted-foreground">No items ready for delivery</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </CardContent>
      </Card>
    </div>

    <div>
      <h3 class="text-sm font-semibold text-foreground mb-3 flex items-center gap-2">
        <span class="text-green-600">✅</span> Delivered
      </h3>
      <Card>
        <CardContent class="p-0">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-border text-left text-xs font-semibold text-muted-foreground">
                <th class="w-24 py-3">Slip No</th>
                <th class="w-40 py-3">Date</th>
                <th class="py-3">Customer</th>
                <th class="w-32 py-3">Device</th>
                <th class="w-32 py-3 text-right">Status</th>
              </tr>
            </thead>
            <tbody>
              {#each deliveredSlips as s}
                <tr class="hover:bg-muted transition-colors border-b border-border last:border-0">
                  <td class="py-3 text-sm font-mono font-medium text-foreground">{s.slip_no}</td>
                  <td class="py-3 text-sm text-muted-foreground">{formatDate(s.date)}</td>
                  <td class="py-3 text-sm font-medium text-foreground">{s.customer_name}</td>
                  <td class="py-3 text-sm text-muted-foreground capitalize">{s.device_type}</td>
                  <td class="py-3 text-right"><Badge variant="secondary" class="text-xs">Delivered</Badge></td>
                </tr>
              {/each}
            </tbody>
          </table>
        </CardContent>
      </Card>
    </div>
  </div>
</div>
