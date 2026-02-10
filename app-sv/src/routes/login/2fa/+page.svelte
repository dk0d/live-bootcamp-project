<script lang="ts">
  import { Button } from "$shadui/button";
  import * as Card from "$shadui/card";
  import * as InputOTP from "$shadui/input-otp";
  import SuperDebug from "sveltekit-superforms";

  let { data } = $props();
</script>

<Card.Root class="mx-auto w-full max-w-sm">
  <Card.Header>
    <Card.Title class="text-2xl">2FA Authentication</Card.Title>
    <Card.Description
      >Please enter the code from your authenticator app.</Card.Description
    >
  </Card.Header>
  <Card.Content class="flex flex-col gap-8">
    {#if data.email}
      <div>{data.email}</div>
      <SuperDebug {data} />
      <form method="POST" action="">
        <input hidden name="method" value="email_password" />
        <input hidden name="two_factor" value="optional" />
        <input hidden name="email" value={data.email} />
        <input hidden name="id" value={data.id} />
        <div class="flex justify-center w-full">
          <InputOTP.Root maxlength={6} pattern="\d" name="code">
            {#snippet children({ cells })}
              <InputOTP.Group>
                {#each cells.slice(0, 6) as cell (cell)}
                  <InputOTP.Slot {cell} class="size-11 text-xl" />
                {/each}
              </InputOTP.Group>
            {/snippet}
          </InputOTP.Root>
        </div>
        <Button type="submit">Submit</Button>
      </form>
    {:else if data.error?.details.includes("Expired")}
      <div class="text-red-500">
        Your 2FA code has expired. Please try again.
      </div>
      <Button href={"/login"}>Try Logging in Again</Button>
    {:else if data.error}
      <div class="text-red-500">Invalid 2FA code. Please try again.</div>
      <Button href={"/login"}>Try Logging in Again</Button>
    {/if}
  </Card.Content>
</Card.Root>
