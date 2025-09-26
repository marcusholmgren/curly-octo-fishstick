import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const session = await locals.auth();
	const response = await fetch('http://localhost:8000/contacts', {
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