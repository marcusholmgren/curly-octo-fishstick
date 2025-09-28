import type { Session } from '@auth/core/types';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			auth: () => Promise<Session | null>;
		}
		interface PageData {
			session: Session | null;
		}
		// interface Platform {}
	}
}

declare module '@auth/core/types' {
	interface Session {
		accessToken?: string;
		error?: string;
	}
}

export {};