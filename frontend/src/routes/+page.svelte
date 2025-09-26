<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>Contacts</title>
</svelte:head>

{#if !$page.data.session?.user}
	<div class="text-center">
		<h1 class="text-4xl font-bold">Welcome to Contacts App</h1>
		<p class="mt-4">Please sign in to manage your contacts.</p>
		<a
			href="/signin"
			class="mt-8 inline-block rounded-md bg-indigo-600 px-6 py-3 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500"
			>Sign In</a
		>
	</div>
{:else}
	<p>Welcome, {$page.data.session.user.name}. View your <a href="/contacts" class="text-indigo-600 hover:underline">contacts</a>.</p>
{/if}