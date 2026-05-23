// frontend/src/routes/contacts/+layout.server.ts
// This file defines the layout server load function for contacts.
// It verifies that the user is authenticated, redirecting them if they are not.
// RELEVANT FILES: frontend/src/routes/contacts/+page.server.ts, frontend/src/routes/+layout.server.ts, frontend/src/auth.ts

import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
	const session = await event.locals.auth();
	if (!session?.user) {
		throw redirect(303, '/signin');
	}
	return {};
};
