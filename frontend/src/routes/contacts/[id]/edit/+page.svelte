<!--
// /frontend/src/routes/contacts/[id]/edit/+page.svelte
// This file contains the page for editing an existing contact.
// This file exists to provide a page for editing an existing contact.
// RELEVANT FILES:
// - /frontend/src/lib/components/ContactForm.svelte
// - /frontend/src/routes/contacts/[id]/edit/+page.ts
-->
<script lang="ts">
	import ContactForm from '$lib/components/ContactForm.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Edit Contact</title>
</svelte:head>

<div class="px-4 sm:px-6 lg:px-8">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<h1 class="text-base font-semibold text-gray-900 dark:text-white">Edit Contact</h1>
			<p class="mt-2 text-sm text-gray-700 dark:text-gray-300">Edit the contact details.</p>
		</div>
	</div>
	<div class="mt-8 flow-root">
		<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
				<ContactForm contact={data.contact} method="PUT" />
			</div>
		</div>
	</div>
</div>
