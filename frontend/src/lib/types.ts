// frontend/src/lib/types.ts
// This file defines custom TypeScript types and interfaces for the application.
// It extends the default session type from Auth.js to include a custom property.
// RELEVANT FILES: frontend/src/app.d.ts, frontend/src/auth.ts

import type { DefaultSession } from '@auth/sveltekit';

/**
 * Extends the default session to include the accessToken.
 * This makes the access token available on the session object throughout the app.
 * @interface CustomSession
 * @extends {DefaultSession}
 */
export interface CustomSession extends DefaultSession {
	/**
	 * The access token from the authentication provider.
	 * @type {string}
	 * @optional
	 */
	accessToken?: string;
}