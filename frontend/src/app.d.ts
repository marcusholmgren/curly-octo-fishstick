import type { CustomSession } from '$lib/types';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			auth: () => Promise<CustomSession | null>;
		}
		interface PageData {
			session: CustomSession | null;
		}
		// interface Platform {}
	}
}

export {};