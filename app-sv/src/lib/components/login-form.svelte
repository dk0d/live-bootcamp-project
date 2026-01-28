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

  let email = $state<string>("");
  let password = $state<string>("");
  let confirmPassword = $state<string>("");
  // let errorMessage = $state<string | null>(null);

  // async function handleSubmit(event: Event) {
  //   event.preventDefault();
  //   errorMessage = null;
  //
  //   if (mode === "signup" && password !== confirmPassword) {
  //     errorMessage = "Passwords do not match.";
  //     return;
  //   }

  //   // Here you would typically send the data to your backend
  //   // For demonstration, we'll just log it to the console
  //   const response = await fetch(`${baseUrl}/${mode}`, {
  //     method: "POST",
  //     headers: {
  //       "Content-Type": "application/json",
  //     },
  //     body: JSON.stringify({
  //       method: "email_password",
  //       email,
  //       password,
  //       two_factor: "optional",
  //     }),
  //   });
  //
  //   console.log("Response:", response);
  //
  //   if (!response.ok) {
  //     const errorData = await response.json();
  //     errorMessage = errorData.message || "An error occurred.";
  //     toast.error(errorMessage!);
  //     return;
  //   }
  //
  //   console.log(response.headers);
  //
  //   toast.success(
  //     mode === "login" ? "Logged in successfully!" : "Signed up successfully!",
  //   );
  // }
</script>

<Card.Root class="mx-auto w-full max-w-sm">
  <Card.Header>
    <Card.Title class="text-2xl"
      >{mode === "login" ? "Login" : "Signup"}</Card.Title
    >
    <Card.Description
      >Enter your email below to login to your account</Card.Description
    >
  </Card.Header>
  <Card.Content>
    <form method="POST" action="/login?/{mode}">
      <input hidden name="method" value="email_password" />
      <input hidden name="two_factor" value="optional" />
      <FieldGroup>
        <Field>
          <FieldLabel for="email">Email</FieldLabel>
          <Input
            id="email"
            type="email"
            name="email"
            bind:value={email}
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
          <Input
            id="password"
            name="password"
            type="password"
            required
            bind:value={password}
          />
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
              bind:value={confirmPassword}
              required
            />
          </Field>
        {/if}

        <Field>
          {#if mode === "login"}
            <Button type="submit" class="w-full">Login</Button>
            <FieldDescription class="text-center">
              Don't have an account? <button
                class="underline hover:underline hover:cursor-pointer"
                onclick={() => {
                  mode = "signup";
                }}>Sign up</button
              >
            </FieldDescription>
          {:else}
            <Button type="submit" class="w-full">Signup</Button>
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
