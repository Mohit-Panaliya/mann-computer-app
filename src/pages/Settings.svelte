<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../lib/api';

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
  <div class="page-header">
    <div>
      <h1 class="page-title">Settings</h1>
      <p class="page-subtitle">Manage shop configuration and data backup</p>
    </div>
  </div>

  {#if loading}
    <div class="empty-state">
      <div class="empty-state-icon">⚙️</div>
      <p>Loading settings...</p>
    </div>
  {:else}
    <div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(300px, 1fr)); gap:1.5rem;">
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Shop Information</h3>
        </div>
        <div class="form-group">
          <label class="form-label">Shop Name</label>
          <input class="form-input" bind:value={shopName} placeholder="Mann Computer" />
        </div>
        <div class="form-group">
          <label class="form-label">Shop Address</label>
          <textarea class="form-textarea" bind:value={shopAddress} placeholder="Full address..."></textarea>
        </div>
        <div class="form-group">
          <label class="form-label">Phone Number</label>
          <input class="form-input" type="tel" bind:value={shopPhone} placeholder="9876543210" />
        </div>
        <div class="form-group">
          <label class="form-label">Email</label>
          <input class="form-input" type="email" bind:value={shopEmail} placeholder="email@example.com" />
        </div>
        <div class="form-group">
          <label class="form-label">Tax Rate (%)</label>
          <input class="form-input" type="number" bind:value={taxRate} placeholder="18" />
        </div>
        <div class="form-group">
          <label class="form-label">Jurisdiction</label>
          <input class="form-input" bind:value={jurisdiction} placeholder="Subject to Anand Jurisdiction." />
        </div>
        <button class="btn btn-primary w-full" on:click={saveSettings} disabled={saving}>
          {saving ? 'Saving...' : 'Save Settings'}
        </button>
      </div>

      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Data Management</h3>
        </div>
        <div class="form-group">
          <label class="form-label">Backup Database</label>
          <p style="font-size:0.75rem; color:var(--text-muted); margin-bottom:0.5rem;">Create a backup copy of your database</p>
          <button class="btn btn-secondary w-full" on:click={backupDatabase}>💾 Create Backup</button>
        </div>
        <div class="form-group">
          <label class="form-label">Export Settings</label>
          <p style="font-size:0.75rem; color:var(--text-muted); margin-bottom:0.5rem;">Export shop settings as JSON file</p>
          <button class="btn btn-secondary w-full" on:click={exportSettings}>📤 Export Settings</button>
        </div>
        <div class="form-group">
          <label class="form-label">Import Settings</label>
          <p style="font-size:0.75rem; color:var(--text-muted); margin-bottom:0.5rem;">Import settings from a JSON file</p>
          <button class="btn btn-secondary w-full" on:click={importSettings}>📥 Import Settings</button>
        </div>
        <div class="form-group">
          <label class="form-label">Database Info</label>
          <div style="font-size:0.75rem; color:var(--text-muted); line-height:1.6;">
            <p>Database: SQLite (local file)</p>
            <p>Location: App Data Directory</p>
            <p>Tables: customers, inward_slips, quotations, repairs, bills, parts, part_sales, cctv_quotes, laptop_quotes, delivery_records, whatsapp_messages, settings</p>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
EOF
echo "Done"