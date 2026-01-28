<script lang="ts">
	import "../app.css";
	import logo from "$lib/assets/lgr_logo.png";
	import * as NavigationMenu from "$shadui/navigation-menu";
	import { IsMobile } from "$shadcn/hooks/is-mobile.svelte";
	import type { HTMLAttributes } from "svelte/elements";
	import { cn } from "$shadcn/utils";
	import { ModeWatcher } from "mode-watcher";
	import { Toaster } from "svelte-sonner";
	import LightSwitch from "$lib/components/light-switch.svelte";
	let { children, data } = $props();
	const isMobile = new IsMobile();
	const components: { title: string; href: string }[] = [
		{
			title: "Login",
			href: "/login",
		},
	];
	type ListItemProps = HTMLAttributes<HTMLAnchorElement> & {
		title: string;
		href: string;
		content: string;
	};
</script>

<ModeWatcher />

<Toaster />

<svelte:head><link rel="icon" href={logo} /></svelte:head>

{#snippet ListItem({
	title,
	content,
	href,
	class: className,
	...restProps
}: ListItemProps)}
	<li>
		<NavigationMenu.Link>
			{#snippet child()}
				<a
					{href}
					class={cn(
						"hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground block space-y-1 rounded p-3 leading-none no-underline transition-colors outline-none select-none",
						className,
					)}
					{...restProps}
				>
					<div class="text-sm leading-none font-medium">{title}</div>
					<!-- <p class="text-muted-foreground line-clamp-2 text-sm leading-snug"> -->
					<!-- 	{content} -->
					<!-- </p> -->
				</a>
			{/snippet}
		</NavigationMenu.Link>
	</li>
{/snippet}

<div class="flex flex-col">
	<div class="flex flex-row justify-between items-center p-12">
		<div class="flex flex-row items-center gap-2">
			<a href="/">
				<img src={logo} alt="Logo" class="size-10" />
			</a>
			<NavigationMenu.Root>
				<NavigationMenu.List class="flex-wrap">
					<NavigationMenu.Item>
						{#if data.session}
							{@render ListItem({
								title: "Logout",
								content: "Access your account",
								href: "/logout",
							})}
						{:else}
							{@render ListItem({
								title: "Login",
								content: "Access your account",
								href: "/login",
							})}
						{/if}
					</NavigationMenu.Item>
				</NavigationMenu.List>
			</NavigationMenu.Root>
		</div>
		<LightSwitch />
	</div>

	<div class="mx-20">
		{@render children()}
	</div>
</div>
