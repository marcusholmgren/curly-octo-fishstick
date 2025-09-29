// frontend/src/routes/+layout.server.ts
// This file defines a server-side load function for the root layout.
// It retrieves the user's session data and makes it available to all pages.
// RELEVANT FILES: frontend/src/routes/+layout.svelte, frontend/src/auth.ts

import type { LayoutServerLoad } from './$types';

/**
 * Loads the session data on the server for the root layout.
 * This ensures that the session is available on all pages, and is fetched securely on the server.
 * @param {object} event - The SvelteKit event object.
 * @returns {Promise<object>} An object containing the session, or null if the user is not authenticated.
 */
export const load: LayoutServerLoad = async (event) => {
	return {
		session: await event.locals.auth()
	};
};