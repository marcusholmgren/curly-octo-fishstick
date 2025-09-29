<!--
// frontend/src/routes/+page.svelte
// This is the main landing page of the application.
// It displays a welcome message and a sign-in button for unauthenticated users,
// or a welcome message and a link to the contacts page for authenticated users.
// RELEVANT FILES: frontend/src/routes/+layout.svelte, frontend/src/routes/signin/+page.svelte
-->
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
	<p>
		Welcome, {$page.data.session.user.name}. View your <a
			href="/contacts"
			class="text-indigo-600 hover:underline">contacts</a
		>.
	</p>
{/if}