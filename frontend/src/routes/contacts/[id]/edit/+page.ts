// frontend/src/routes/contacts/[id]/edit/+page.ts
// This file contains the load function for the edit contact page.
// It is responsible for fetching the specific contact's data from the API before the page is rendered.
// RELEVANT FILES: frontend/src/routes/contacts/[id]/edit/+page.svelte, frontend/src/lib/components/ContactForm.svelte
import type { PageLoad } from './$types';

/**
 * Fetches the data for a single contact from the API.
 * This function runs on the client or server before the page component is rendered.
 * It uses the contact ID from the URL parameters to make the API request.
 * @param {object} context - The SvelteKit load context.
 * @param {object} context.params - The route parameters, containing the contact `id`.
 * @param {function} context.fetch - The SvelteKit fetch function.
 * @param {function} context.parent - A function to get data from parent layouts.
 * @returns {Promise<object>} An object containing the contact data, or null if not found or on error.
 */
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
