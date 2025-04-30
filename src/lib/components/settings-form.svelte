<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { createForm } from "svelte-forms-lib";
  import { createEventDispatcher } from "svelte";

  interface FormValues {
    email: string;
    apiToken: string;
  }

  interface CloudflareAccount {
    id: string;
    name: string;
  }

  let accounts: CloudflareAccount[] = [];
  let loading = false;
  let error: string | null = null;

  const { form, handleSubmit } = createForm({
    initialValues: {
      email: "",
      apiToken: "",
    },
    onSubmit: async (values: FormValues) => {
      error = null;
      try {
        console.log("Submitting credentials...", values);
        loading = true;
        accounts = await invoke<CloudflareAccount[]>("save_credentials", { 
          email: values.email, 
          apiToken: values.apiToken 
        });
        console.log("Received accounts:", accounts);
        dispatch("accountsLoaded", accounts);
      } catch (err) {
        console.error("Failed to save credentials:", err);
        error = err instanceof Error ? err.message : String(err);
      } finally {
        loading = false;
      }
    },
  });

  async function selectAccount(accountId: string) {
    error = null;
    try {
      console.log("Fetching zones for account:", accountId);
      loading = true;
      const zones = await invoke("fetch_zones", { accountId });
      console.log("Received zones:", zones);
      dispatch("accountSelected", { accountId, zones });
    } catch (err) {
      console.error("Failed to fetch zones:", err);
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  }

  const dispatch = createEventDispatcher();
</script>

<Card.Root class="w-[350px] mx-auto mt-20">
  <Card.Header>
    <Card.Title>Cloudflare Credentials</Card.Title>
    <Card.Description>
      Enter your Cloudflare API token and email to get started.
    </Card.Description>
  </Card.Header>
  <Card.Content>
    {#if error}
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
        {error}
      </div>
    {/if}
    
    {#if accounts.length === 0}
      <form on:submit|preventDefault={handleSubmit} class="space-y-4">
        <div class="space-y-2">
          <label for="email" class="text-sm font-medium">Email</label>
          <Input
            id="email"
            type="email"
            bind:value={$form.email}
            placeholder="your@email.com"
            required
            disabled={loading}
          />
        </div>
        <div class="space-y-2">
          <label for="apiToken" class="text-sm font-medium">API Token</label>
          <Input
            id="apiToken"
            type="password"
            bind:value={$form.apiToken}
            placeholder="Your Cloudflare API token"
            required
            disabled={loading}
          />
        </div>
        <Button type="submit" class="w-full" disabled={loading}>
          {loading ? "Saving..." : "Save Credentials"}
        </Button>
      </form>
    {:else}
      <div class="space-y-4">
        <h3 class="text-sm font-medium">Select an Account</h3>
        {#each accounts as account}
          <Button
            variant="outline"
            class="w-full justify-start"
            on:click={() => selectAccount(account.id)}
            disabled={loading}
          >
            {account.name}
          </Button>
        {/each}
      </div>
    {/if}
  </Card.Content>
</Card.Root> 