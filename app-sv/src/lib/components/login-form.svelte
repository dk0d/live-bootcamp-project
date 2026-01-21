<script lang="ts">
  import { Button } from "$shadui/button";
  import * as Card from "$shadui/card";
  import { Input } from "$shadui/input";
  import {
    FieldGroup,
    Field,
    FieldLabel,
    FieldDescription,
  } from "$shadui/field";

  let mode = $state<"login" | "signup">("login");
</script>

<Card.Root class="mx-auto w-full max-w-sm">
  <Card.Header>
    <Card.Title class="text-2xl">Login</Card.Title>
    <Card.Description
      >Enter your email below to login to your account</Card.Description
    >
  </Card.Header>
  <Card.Content>
    <form>
      <FieldGroup>
        <Field>
          <FieldLabel for="email">Email</FieldLabel>
          <Input
            id="email"
            type="email"
            name="email"
            placeholder="m@example.com"
            required
          />
        </Field>
        <Field>
          <div class="flex items-center">
            <FieldLabel for="password">Password</FieldLabel>
            <a href="##" class="ms-auto inline-block text-sm underline">
              Forgot your password?
            </a>
          </div>
          <Input id="password" name="password" type="password" required />
        </Field>

        {#if mode === "signup"}
          <Field>
            <div class="flex items-center">
              <FieldLabel for="confirm_password">Confirm Password</FieldLabel>
            </div>
            <Input
              id="confirm_password"
              name="confirm_password"
              type="password"
              required
            />
          </Field>
        {/if}

        <Field>
          <Button type="submit" class="w-full">Login</Button>
          {#if mode === "login"}
            <FieldDescription class="text-center">
              Don't have an account? <button
                class="underline hover:underline hover:cursor-pointer"
                onclick={() => {
                  mode = "signup";
                }}>Sign up</button
              >
            </FieldDescription>
          {:else}
            <FieldDescription class="text-center">
              Already have an account? <button
                class="underline hover:underline hover:cursor-pointer"
                onclick={() => {
                  mode = "login";
                }}>Log in</button
              >
            </FieldDescription>
          {/if}
        </Field>
      </FieldGroup>
    </form>
  </Card.Content>
</Card.Root>
