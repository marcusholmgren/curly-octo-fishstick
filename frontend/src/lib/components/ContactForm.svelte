<!--
// /frontend/src/lib/components/ContactForm.svelte
// This file contains the contact form component.
// This file exists to provide a reusable form for adding and editing contacts.
// RELEVANT FILES:
// - /frontend/src/routes/contacts/new/+page.svelte
// - /frontend/src/routes/contacts/[id]/edit/+page.svelte
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';

	export let contact: {
		id?: number;
		first_name: string;
		last_name: string;
		email: string;
		phone_number: string;
	} = {
		first_name: '',
		last_name: '',
		email: '',
		phone_number: ''
	};

	export let method: 'POST' | 'PUT' = 'POST';

	async function handleSubmit(event: Event) {
		const target = event.target as HTMLFormElement;
		const formData = new FormData(target);
		const formValues = Object.fromEntries(formData.entries());

		const url = contact.id ? `${base}/api/contacts/${contact.id}` : `${base}/api/contacts`;

		try {
			const response = await fetch(url, {
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(formValues)
			});

			if (response.ok) {
				goto(`${base}/`);
			} else {
				console.error('Failed to save contact', response.status, response.statusText);
			}
		} catch (error) {
			console.error('An error occurred while saving the contact:', error);
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<div class="space-y-12">
		<div class="border-b border-gray-900/10 pb-12 dark:border-white/10">
			<div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
				<div class="sm:col-span-3">
					<label
						for="first-name"
						class="block text-sm leading-6 font-medium text-gray-900 dark:text-white"
						>First name</label
					>
					<div class="mt-2">
						<input
							type="text"
							name="first_name"
							id="first-name"
							autocomplete="given-name"
							class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-gray-300 ring-inset placeholder:text-gray-400 focus:ring-2 focus:ring-indigo-600 focus:ring-inset sm:text-sm sm:leading-6 dark:bg-white/5 dark:text-white dark:ring-white/10 dark:focus:ring-indigo-500"
							bind:value={contact.first_name}
						/>
					</div>
				</div>

				<div class="sm:col-span-3">
					<label
						for="last-name"
						class="block text-sm leading-6 font-medium text-gray-900 dark:text-white"
						>Last name</label
					>
					<div class="mt-2">
						<input
							type="text"
							name="last_name"
							id="last-name"
							autocomplete="family-name"
							class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-gray-300 ring-inset placeholder:text-gray-400 focus:ring-2 focus:ring-indigo-600 focus:ring-inset sm:text-sm sm:leading-6 dark:bg-white/5 dark:text-white dark:ring-white/10 dark:focus:ring-indigo-500"
							bind:value={contact.last_name}
						/>
					</div>
				</div>

				<div class="sm:col-span-4">
					<label
						for="email"
						class="block text-sm leading-6 font-medium text-gray-900 dark:text-white"
						>Email address</label
					>
					<div class="mt-2">
						<input
							id="email"
							name="email"
							type="email"
							autocomplete="email"
							class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-gray-300 ring-inset placeholder:text-gray-400 focus:ring-2 focus:ring-indigo-600 focus:ring-inset sm:text-sm sm:leading-6 dark:bg-white/5 dark:text-white dark:ring-white/10 dark:focus:ring-indigo-500"
							bind:value={contact.email}
						/>
					</div>
				</div>

				<div class="sm:col-span-4">
					<label
						for="phone-number"
						class="block text-sm leading-6 font-medium text-gray-900 dark:text-white"
						>Phone number</label
					>
					<div class="mt-2">
						<input
							type="text"
							name="phone_number"
							id="phone-number"
							autocomplete="tel"
							class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-gray-300 ring-inset placeholder:text-gray-400 focus:ring-2 focus:ring-indigo-600 focus:ring-inset sm:text-sm sm:leading-6 dark:bg-white/5 dark:text-white dark:ring-white/10 dark:focus:ring-indigo-500"
							bind:value={contact.phone_number}
						/>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="mt-6 flex items-center justify-end gap-x-6">
		<a href="{base}/" class="text-sm leading-6 font-semibold text-gray-900 dark:text-white"
			>Cancel</a
		>
		<button
			type="submit"
			class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:bg-indigo-500 dark:hover:bg-indigo-400 dark:focus-visible:outline-indigo-500"
			>Save</button
		>
	</div>
</form>
