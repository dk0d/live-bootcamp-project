<script lang="ts">
  import LoginForm from "$lib/components/login-form.svelte";

  let { data } = $props();
</script>

<LoginForm baseUrl={data.authUrl} />
