<script lang="ts">
	interface Contact {
		id: number;
		first_name: string;
		last_name: string;
		email: string;
		phone_number: string;
	}

	let contacts: Contact[] = [];

	async function getContacts() {
		try {
			const response = await fetch('/api/contacts');
			if (response.ok) {
				contacts = await response.json();
			} else {
				console.error('Failed to fetch contacts', response.status, response.statusText);
			}
		} catch (error) {
			console.error('An error occurred while fetching contacts:', error);
		}
	}

	getContacts();
</script>

<div class="px-4 sm:px-6 lg:px-8">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<h1 class="text-base font-semibold text-gray-900 dark:text-white">Contacts</h1>
			<p class="mt-2 text-sm text-gray-700 dark:text-gray-300">
				A list of all the contacts in your account.
			</p>
		</div>
		<div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
			<button
				type="button"
				class="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:bg-indigo-500 dark:hover:bg-indigo-400 dark:focus-visible:outline-indigo-500"
			>
				Add contact
			</button>
		</div>
	</div>
	<div class="mt-8 flow-root">
		<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
				<table class="relative min-w-full divide-y divide-gray-300 dark:divide-white/15">
					<thead>
						<tr>
							<th
								scope="col"
								class="sticky top-0 z-10 border-b border-gray-300 bg-white/75 py-3.5 pr-3 pl-4 text-left text-sm font-semibold text-gray-900 backdrop-blur-sm backdrop-filter sm:pl-6 lg:pl-8 dark:border-white/15 dark:bg-gray-900/75 dark:text-white"
							>
								First Name
							</th>
							<th
								scope="col"
								class="sticky top-0 z-10 hidden border-b border-gray-300 bg-white/75 px-3 py-3.5 text-left text-sm font-semibold text-gray-900 backdrop-blur-sm backdrop-filter sm:table-cell dark:border-white/15 dark:bg-gray-900/75 dark:text-white"
							>
								Last Name
							</th>
							<th
								scope="col"
								class="sticky top-0 z-10 hidden border-b border-gray-300 bg-white/75 px-3 py-3.5 text-left text-sm font-semibold text-gray-900 backdrop-blur-sm backdrop-filter lg:table-cell dark:border-white/15 dark:bg-gray-900/75 dark:text-white"
							>
								Email
							</th>
							<th
								scope="col"
								class="sticky top-0 z-10 border-b border-gray-300 bg-white/75 px-3 py-3.5 text-left text-sm font-semibold text-gray-900 backdrop-blur-sm backdrop-filter dark:border-white/15 dark:bg-gray-900/75 dark:text-white"
							>
								Phone Number
							</th>
							<th
								scope="col"
								class="sticky top-0 z-10 border-b border-gray-300 bg-white/75 py-3.5 pr-4 pl-3 backdrop-blur-sm backdrop-filter sm:pr-6 lg:pr-8 dark:border-white/15 dark:bg-gray-900/75"
							>
								<span class="sr-only">Edit</span>
							</th>
						</tr>
					</thead>
					<tbody>
						{#each contacts as contact (contact.id)}
							<tr>
								<td
									class="border-b border-gray-200 py-4 pr-3 pl-4 text-sm font-medium whitespace-nowrap text-gray-900 sm:pl-6 lg:pl-8 dark:border-white/10 dark:bg-gray-900 dark:text-white"
								>
									{contact.first_name}
								</td>
								<td
									class="hidden border-b border-gray-200 px-3 py-4 text-sm whitespace-nowrap text-gray-500 sm:table-cell dark:border-white/10 dark:bg-gray-900 dark:text-gray-400"
								>
									{contact.last_name}
								</td>
								<td
									class="hidden border-b border-gray-200 px-3 py-4 text-sm whitespace-nowrap text-gray-500 lg:table-cell dark:border-white/10 dark:bg-gray-900 dark:text-gray-400"
								>
									{contact.email}
								</td>
								<td
									class="border-b border-gray-200 px-3 py-4 text-sm whitespace-nowrap text-gray-500 dark:border-white/10 dark:bg-gray-900 dark:text-gray-400"
								>
									{contact.phone_number}
								</td>
								<td
									class="border-b border-gray-200 py-4 pr-4 pl-3 text-right text-sm font-medium whitespace-nowrap sm:pr-8 lg:pr-8 dark:border-white/10 dark:bg-gray-900"
								>
									<a
										href="#"
										class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300"
										>Edit<span class="sr-only">, {contact.first_name} {contact.last_name}</span></a
									>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>
