<script lang="ts">
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import Breadcrumb from "$lib/components/ui/breadcrumb.svelte";
  import BreadcrumbItem from "$lib/components/ui/breadcrumb-item.svelte";
  import * as Card from "$lib/components/ui/card";
  import { onMount } from "svelte";

  interface Zone {
    id: string;
    name: string;
    status: string;
    plan: string;
    visitors: number;
  }

  let zone: Zone | null = null;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    try {
      const zoneId = $page.params.id;
      // First get the accounts to find which account this zone belongs to
      const accounts = await invoke<{id: string, name: string}[]>("fetch_accounts");
      
      // For each account, try to find the zone
      for (const account of accounts) {
        const zones = await invoke<Zone[]>("fetch_zones", { accountId: account.id });
        const foundZone = zones.find(z => z.id === zoneId);
        if (foundZone) {
          zone = foundZone;
          break;
        }
      }
      
      if (!zone) {
        error = "Zone not found";
      }
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  });

  function handleBack() {
    goto("/");
  }
</script>

<div class="container mx-auto p-4">
  <div class="flex justify-between items-center mb-4">
    <Breadcrumb>
      <BreadcrumbItem href="/">Accounts</BreadcrumbItem>
      <BreadcrumbItem isLast>{zone?.name || "Loading..."}</BreadcrumbItem>
    </Breadcrumb>
    <button
      class="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200"
      on:click={handleBack}
    >
      ← Back
    </button>
  </div>

  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      {error}
    </div>
  {:else if loading}
    <div class="flex justify-center items-center h-64">
      <span class="text-gray-500">Loading zone information...</span>
    </div>
  {:else if zone}
    <div class="grid grid-cols-4 gap-4">
      <!-- Sidebar -->
      <div class="col-span-1 space-y-2">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h2 class="text-lg font-semibold mb-4">Menu</h2>
          <nav class="space-y-2">
            <a href="/zone/{zone.id}" class="block px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
              Overview
            </a>
            <a href="/zone/{zone.id}/dns" class="block px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
              Add/Edit DNS Records
            </a>
            <a href="/zone/{zone.id}/bulk-dns" class="block px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
              Bulk Add DNS Records
            </a>
          </nav>
        </div>
      </div>

      <!-- Main Content -->
      <div class="col-span-3">
        <Card.Root class="mb-4">
          <Card.Header>
            <Card.Title>Zone Overview</Card.Title>
          </Card.Header>
          <Card.Content>
            <div class="grid grid-cols-3 gap-4">
              <div>
                <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Status</h3>
                <p class="text-lg font-semibold">{zone.status}</p>
              </div>
              <div>
                <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Unique Visitors</h3>
                <p class="text-lg font-semibold">{zone.visitors.toLocaleString()}</p>
              </div>
              <div>
                <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">Plan</h3>
                <p class="text-lg font-semibold">{zone.plan}</p>
              </div>
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    </div>
  {/if}
</div> 