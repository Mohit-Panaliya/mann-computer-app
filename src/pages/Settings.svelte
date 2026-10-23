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

  let settings: any = {};
  let loading = false;
  let saving = false;

  let shopName = '';
  let shopAddress = '';
  let shopPhone = '';
  let shopEmail = '';
  let taxRate = 18;
  let jurisdiction = '';

  onMount(loadSettings);

  async function loadSettings() {
    loading = true;
    try {
      settings = await api.getSettings();
      shopName = settings.shop_name || '';
      shopAddress = settings.shop_address || '';
      shopPhone = settings.shop_phone || '';
      shopEmail = settings.shop_email || '';
      taxRate = parseFloat(settings.tax_rate || '18');
      jurisdiction = settings.jurisdiction || '';
    } catch (e) {
      console.error('Failed to load settings', e);
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    saving = true;
    try {
      await api.updateSetting('shop_name', shopName);
      await api.updateSetting('shop_address', shopAddress);
      await api.updateSetting('shop_phone', shopPhone);
      await api.updateSetting('shop_email', shopEmail);
      await api.updateSetting('tax_rate', taxRate.toString());
      await api.updateSetting('jurisdiction', jurisdiction);
      alert('Settings saved successfully!');
    } catch (e: any) {
      console.error('Failed to save settings', e);
      alert('Failed to save settings: ' + (e?.message || String(e)));
    } finally {
      saving = false;
    }
  }

  async function backupDatabase() {
    try {
      const path = await api.backupDatabase();
      alert('Database backed up to: ' + path);
    } catch (e: any) {
      alert('Backup failed: ' + (e?.message || String(e)));
    }
  }

  async function exportSettings() {
    try {
      const json = await api.exportSettings();
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'mann_computer_settings.json';
      a.click();
      URL.revokeObjectURL(url);
    } catch (e: any) {
      alert('Export failed: ' + (e?.message || String(e)));
    }
  }

  async function importSettings() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'application/json';
    input.onchange = async (e: any) => {
      const file = e.target.files[0];
      if (!file) return;
      const text = await file.text();
      try {
        await api.importSettings(text);
        alert('Settings imported successfully!');
        await loadSettings();
      } catch (e: any) {
        alert('Import failed: ' + (e?.message || String(e)));
      }
    };
    input.click();
  }
</script>

<div>
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-foreground">Settings</h1>
    <p class="text-muted-foreground mt-1">Manage shop configuration and data backup</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="flex flex-col items-center gap-2 animate-pulse">
        <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary">⚙️</div>
        <p class="text-muted-foreground text-sm">Loading settings...</p>
      </div>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Shop Information</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="space-y-2">
            <Label>Shop Name</Label>
            <Input bind:value={shopName} placeholder="Mann Computer" />
          </div>
          <div class="space-y-2">
            <Label>Shop Address</Label>
            <Textarea bind:value={shopAddress} rows="3" placeholder="Full address..." />
          </div>
          <div class="space-y-2">
            <Label>Phone Number</Label>
            <Input type="tel" bind:value={shopPhone} placeholder="9876543210" />
          </div>
          <div class="space-y-2">
            <Label>Email</Label>
            <Input type="email" bind:value={shopEmail} placeholder="email@example.com" />
          </div>
          <div class="space-y-2">
            <Label>Tax Rate (%)</Label>
            <Input type="number" bind:value={taxRate} placeholder="18" />
          </div>
          <div class="space-y-2">
            <Label>Jurisdiction</Label>
            <Input bind:value={jurisdiction} placeholder="Subject to Anand Jurisdiction." />
          </div>
          <Button variant="default" class="w-full" on:click={saveSettings} disabled={saving}>
            {saving ? 'Saving...' : 'Save Settings'}
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle class="text-sm">Data Management</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="space-y-2">
            <Label>Backup Database</Label>
            <p class="text-xs text-muted-foreground mb-2">Create a backup copy of your database</p>
            <Button variant="outline" class="w-full" on:click={backupDatabase}>
              <span class="mr-2">💾</span> Create Backup
            </Button>
          </div>

          <div class="space-y-2">
            <Label>Export Settings</Label>
            <p class="text-xs text-muted-foreground mb-2">Export shop settings as JSON file</p>
            <Button variant="outline" class="w-full" on:click={exportSettings}>
              <span class="mr-2">📤</span> Export Settings
            </Button>
          </div>

          <div class="space-y-2">
            <Label>Import Settings</Label>
            <p class="text-xs text-muted-foreground mb-2">Import settings from a JSON file</p>
            <Button variant="outline" class="w-full" on:click={importSettings}>
              <span class="mr-2">📥</span> Import Settings
            </Button>
          </div>

          <div class="space-y-2">
            <Label>Database Info</Label>
            <div class="text-xs text-muted-foreground space-y-1">
              <p>Database: SQLite (local file)</p>
              <p>Location: App Data Directory</p>
              <p>Tables: customers, inward_slips, quotations, repairs, bills, parts, part_sales, cctv_quotes, laptop_quotes, delivery_records, whatsapp_messages, settings</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  {/if}
</div>
