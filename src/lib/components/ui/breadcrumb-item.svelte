<script lang="ts">
  import { cn } from "$lib/utils";
  import { ChevronRight } from "lucide-svelte";
  let className: string | undefined = undefined;
  export { className as class };
  export let href: string | undefined = undefined;
  export let isLast = false;
</script>

<div class={cn("flex items-center", className)}>
  <a
    href={href}
    class={cn(
      "hover:text-foreground",
      !href && "cursor-default",
      isLast && "text-foreground"
    )}
  >
    <slot />
  </a>
  {#if !isLast}
    <ChevronRight class="h-4 w-4 mx-1" />
  {/if}
</div> 