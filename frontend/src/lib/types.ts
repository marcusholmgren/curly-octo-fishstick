import type { DefaultSession } from '@auth/sveltekit';

export interface CustomSession extends DefaultSession {
	accessToken?: string;
}