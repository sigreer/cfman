<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import SettingsForm from '$lib/components/settings-form.svelte';
  import { goto } from '$app/navigation';

  interface CloudflareZone {
    id: string;
    name: string;
    status: string;
    paused: boolean;
    type: string;
    development_mode: number;
    name_servers: string[];
    original_name_servers?: string[];
    original_registrar?: string;
    original_dnshost?: string;
    modified_on: string;
    created_on: string;
    activated_on: string;
    meta: {
      step: number;
      custom_certificate_quota: number;
      page_rule_quota: number;
      phishing_detected: boolean;
    };
    owner: {
      id?: string;
      type: string;
      email?: string;
    };
    account: {
      id: string;
      name: string;
    };
    tenant?: {
      id?: string;
      name?: string;
    };
    tenant_unit?: {
      id?: string;
    };
    permissions: string[];
    plan: {
      id: string;
      name: string;
      price: number;
      currency: string;
      frequency: string;
      is_subscribed: boolean;
      can_subscribe: boolean;
      legacy_id: string;
      legacy_discount: boolean;
      externally_managed: boolean;
    };
    visitors?: number;
  }

  interface CloudflareAccount {
    id: string;
    name: string;
    account_type: string;
    created_on?: string;
  }

  interface CloudflareDnsRecord {
    id: string;
    zone_id: string;
    record_type: string;
    name: string;
    content: string;
    proxied: boolean;
    ttl: number;
  }

  let hasCredentials = false;
  let accounts: CloudflareAccount[] = [];
  let selectedAccount: CloudflareAccount | null = null;
  let zones: CloudflareZone[] = [];
  let loading = false;
  let error: string | null = null;
  let checkingForUpdates = false;
  let lastUpdateTime: string | null = null;

  onMount(async () => {
    try {
      const result = await invoke('check_credentials');
      hasCredentials = result as boolean;
      if (hasCredentials) {
        accounts = await invoke('fetch_accounts');
      }
    } catch (error) {
      console.error('Failed to check credentials:', error);
    }
  });

  async function handleAccountSelected(accountId: string) {
    try {
      loading = true;
      error = null;
      selectedAccount = accounts.find(a => a.id === accountId) || null;
      if (selectedAccount) {
        console.log('Fetching zones for account:', selectedAccount.name);
        // First try to get cached zones
        zones = await invoke('get_cached_zones', { accountId }) as CloudflareZone[];
        lastUpdateTime = new Date().toLocaleTimeString();
        
        // Then check for updates in the background
        checkingForUpdates = true;
        const updatedZones = await invoke('fetch_zones', { accountId }) as CloudflareZone[];
        checkingForUpdates = false;
        
        if (JSON.stringify(zones) !== JSON.stringify(updatedZones)) {
          zones = updatedZones;
          lastUpdateTime = new Date().toLocaleTimeString();
        }
      }
    } catch (error) {
      console.error('Failed to fetch zones:', error);
      error = error instanceof Error ? error.message : String(error);
    } finally {
      loading = false;
    }
  }

  function handleZoneSelected(zone: CloudflareZone) {
    goto(`/zone/${zone.id}`);
  }

  function handleBack() {
    if (selectedAccount) {
      selectedAccount = null;
      zones = [];
      lastUpdateTime = null;
    } else if (hasCredentials) {
      hasCredentials = false;
      accounts = [];
    }
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }
</script>

<div class="container mx-auto p-4">
  <div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">Cloudflare DNS Manager</h1>
    {#if hasCredentials}
      <button
        class="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200"
        on:click={handleBack}
      >
        ← Back to {selectedAccount ? 'Accounts' : 'Credentials'}
      </button>
    {/if}
  </div>
  
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      {error}
    </div>
  {/if}
  
  {#if !hasCredentials}
    <SettingsForm on:accountsLoaded={(event) => accounts = event.detail} />
  {:else if accounts.length > 0 && !selectedAccount}
    <div class="space-y-4">
      <h2 class="text-lg font-semibold">Select an Account</h2>
      {#each accounts as account}
        <button
          class="w-full text-left p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-800"
          on:click={() => handleAccountSelected(account.id)}
        >
          {account.name}
        </button>
      {/each}
    </div>
  {:else if selectedAccount}
    <div class="space-y-4">
      <div class="flex justify-between items-center">
        <h2 class="text-lg font-semibold">Zones for {selectedAccount.name}</h2>
        {#if lastUpdateTime}
          <span class="text-sm text-gray-500">Last updated: {lastUpdateTime}</span>
        {/if}
      </div>
      
      {#if checkingForUpdates}
        <div class="text-sm text-gray-500 mb-4">
          Checking for updates...
        </div>
      {/if}
      
      {#if loading && zones.length === 0}
        <div class="flex justify-center items-center h-64">
          <span class="text-gray-500">Loading zones...</span>
        </div>
      {:else if zones.length === 0}
        <div class="flex justify-center items-center h-64">
          <span class="text-gray-500">No zones found for this account</span>
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead class="bg-gray-50 dark:bg-gray-800">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Domain</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Status</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Created</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Plan</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Visitors</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
              {#each zones as zone}
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800">
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-gray-100">
                    {zone.name}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                      {zone.status === 'active' ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' : 
                       zone.status === 'pending' ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200' : 
                       'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'}">
                      {zone.status}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    {formatDate(zone.created_on)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    {zone.plan.name}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    {zone.visitors?.toLocaleString() ?? '-'}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <button
                      class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300"
                      on:click={() => handleZoneSelected(zone)}
                    >
                      View Details
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}
</div>
