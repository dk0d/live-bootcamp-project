<script lang="ts">
  import SunIcon from "@lucide/svelte/icons/sun";
  import MoonIcon from "@lucide/svelte/icons/moon";

  import { toggleMode } from "mode-watcher";
  import { Button } from "$shadui/button";
</script>

<Button onclick={toggleMode} variant="outline" size="icon">
  <SunIcon
    class="size-[1.2rem] rotate-0 scale-100 transition-all! dark:-rotate-90 dark:scale-0"
  />
  <MoonIcon
    class="absolute size-[1.2rem] rotate-90 scale-0 transition-all! dark:rotate-0 dark:scale-100"
  />
  <span class="sr-only">Toggle theme</span>
</Button>
