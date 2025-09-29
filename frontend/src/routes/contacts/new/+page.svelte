<!--
// frontend/src/routes/contacts/new/+page.svelte
// This page provides the user interface for adding a new contact.
// It uses the reusable ContactForm component to render the form fields.
// RELEVANT FILES: frontend/src/lib/components/ContactForm.svelte
-->
<script lang="ts">
	import ContactForm from '$lib/components/ContactForm.svelte';
</script>

<svelte:head>
	<title>New Contact</title>
</svelte:head>

<div class="px-4 sm:px-6 lg:px-8">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<h1 class="text-base font-semibold text-gray-900 dark:text-white">New Contact</h1>
			<p class="mt-2 text-sm text-gray-700 dark:text-gray-300">
				Add a new contact to your account.
			</p>
		</div>
	</div>
	<div class="mt-8 flow-root">
		<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
				<ContactForm />
			</div>
		</div>
	</div>
</div>
