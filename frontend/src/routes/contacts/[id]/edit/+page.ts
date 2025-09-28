// /frontend/src/routes/contacts/[id]/edit/+page.ts
// This file contains the load function for the edit contact page.
// This file exists to fetch the contact data from the API.
// RELEVANT FILES:
// - /frontend/src/routes/contacts/[id]/edit/+page.svelte
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	const { id } = params;
	const { session } = await parent();

	if (!session?.accessToken) {
		// Or redirect to login
		return { contact: null };
	}

	const response = await fetch(`/api/contacts/${id}`, {
		headers: {
			Authorization: `Bearer ${session.accessToken}`
		}
	});

	if (response.ok) {
		const contact = await response.json();
		return {
			contact
		};
	}

	// Handle error case
	return {
		contact: null
	};
};
