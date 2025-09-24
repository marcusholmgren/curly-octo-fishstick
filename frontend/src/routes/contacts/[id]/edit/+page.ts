// /frontend/src/routes/contacts/[id]/edit/+page.ts
// This file contains the load function for the edit contact page.
// This file exists to fetch the contact data from the API.
// RELEVANT FILES:
// - /frontend/src/routes/contacts/[id]/edit/+page.svelte
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
	const { id } = params;
	const response = await fetch(`/api/contacts/${id}`);
	const contact = await response.json();
	return {
		contact
	};
};
