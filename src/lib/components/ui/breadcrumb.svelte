<script lang="ts">
  import { cn } from "$lib/utils";
  import { ChevronRight } from "lucide-svelte";
  let className: string | undefined = undefined;
  export { className as class };
</script>

<div class={cn("flex items-center space-x-1 text-sm text-muted-foreground", className)} {...$$restProps}>
  <slot />
</div> 