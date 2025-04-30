<script lang="ts">
import '../app.css';
import { onMount } from 'svelte';

onMount(() => {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const updateTheme = (e: MediaQueryListEvent | MediaQueryList) => {
    if (e.matches) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };

  mediaQuery.addEventListener('change', updateTheme);
  updateTheme(mediaQuery);

  return () => mediaQuery.removeEventListener('change', updateTheme);
});
</script>

<slot />