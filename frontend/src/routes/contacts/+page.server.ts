// frontend/src/routes/contacts/+page.server.ts
// This file defines the page server load function for the contacts list.
// It fetches all the contacts from the backend API using the authenticated session token.
// RELEVANT FILES: frontend/src/routes/contacts/+page.svelte, frontend/src/lib/components/ContactsTable.svelte, frontend/src/routes/contacts/+layout.server.ts

import { CONTACTS_API } from '$env/static/private';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, fetch }) => {
	const session = await locals.auth();
	const response = await fetch(`${CONTACTS_API}/contacts`, {
		headers: {
			Authorization: `Bearer ${session?.accessToken}`
		}
	});

	if (response.ok) {
		const contacts = await response.json();
		return { contacts };
	} else {
		return { contacts: [] };
	}
};