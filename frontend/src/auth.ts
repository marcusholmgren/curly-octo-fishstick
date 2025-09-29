// frontend/src/auth.ts
// This file configures authentication for the SvelteKit application using Auth.js.
// It sets up the Keycloak provider and handles token refresh logic.
// RELEVANT FILES: frontend/src/hooks.server.ts, frontend/src/routes/+layout.server.ts

import { SvelteKitAuth } from '@auth/sveltekit';
import Keycloak from '@auth/sveltekit/providers/keycloak';
import {
	AUTH_KEYCLOAK_ID,
	AUTH_KEYCLOAK_ISSUER,
	AUTH_KEYCLOAK_SECRET,
	AUTH_SECRET
} from '$env/static/private';

/**
 * Refreshes the access token using the refresh token.
 * @param {object} token - The token object containing the refreshToken.
 * @property {string} token.refreshToken - The refresh token from the provider.
 * @returns {Promise<object>} The updated token object with the new access token, or an error object.
 */
async function refreshAccessToken(token: { refreshToken: string }): Promise<object> {
	try {
		const url = `${AUTH_KEYCLOAK_ISSUER}/protocol/openid-connect/token`;

		const response = await fetch(url, {
			headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
			method: 'POST',
			body: new URLSearchParams({
				client_id: AUTH_KEYCLOAK_ID,
				client_secret: AUTH_KEYCLOAK_SECRET,
				grant_type: 'refresh_token',
				refresh_token: token.refreshToken
			})
		});

		const refreshedTokens = await response.json();

		if (!response.ok) {
			throw refreshedTokens;
		}

		return {
			...token,
			accessToken: refreshedTokens.access_token,
			accessTokenExpires: Date.now() + refreshedTokens.expires_in * 1000,
			refreshToken: refreshedTokens.refresh_token ?? token.refreshToken // Fall back to old refresh token
		};
	} catch (error) {
		console.error('Error refreshing access token', error);

		return {
			...token,
			error: 'RefreshAccessTokenError'
		};
	}
}

/**
 * The main configuration for SvelteKitAuth.
 * It initializes the Keycloak provider and defines callbacks for handling JWTs, sessions, and redirects.
 */
export const { handle, signIn, signOut } = SvelteKitAuth({
	providers: [
		Keycloak({
			clientId: AUTH_KEYCLOAK_ID,
			clientSecret: AUTH_KEYCLOAK_SECRET,
			issuer: AUTH_KEYCLOAK_ISSUER
		})
	],
	secret: AUTH_SECRET,
	useSecureCookies: false,
	trustHost: true,
	callbacks: {
		/**
		 * This callback is called whenever a JSON Web Token is created (i.e. at sign in)
		 * or updated (i.e. whenever a session is accessed in the client).
		 * @param {object} params - The parameters for the callback.
		 * @param {object} params.token - The token from the provider.
		 * @param {object} params.account - The account from the provider.
		 * @returns {Promise<object>} The token to be stored.
		 */
		async jwt({ token, account }) {
			// Initial sign in
			if (account) {
				token.accessToken = account.access_token;
				token.refreshToken = account.refresh_token;
				token.accessTokenExpires = account.expires_at * 1000;
				return token;
			}

			// Return previous token if the access token has not expired yet
			if (Date.now() < token.accessTokenExpires) {
				return token;
			}

			// Access token has expired, try to update it
			return refreshAccessToken(token);
		},
		/**
		 * This callback is called whenever a session is checked.
		 * By default, only a subset of the token is returned for security reasons.
		 * We are exposing the access token and error to the client session.
		 * @param {object} params - The parameters for the callback.
		 * @param {object} params.session - The session object.
		 * @param {object} params.token - The token from the JWT callback.
		 * @returns {Promise<object>} The session object to be returned to the client.
		 */
		async session({ session, token }) {
			// Add property to session, like an access_token from a provider.
			session.accessToken = token.accessToken as string;
			session.error = token.error as string;
			return session;
		},
		/**
		 * This callback is called anytime the user is redirected to a callback URL (e.g. after signing in).
		 * @param {object} params - The parameters for the callback.
		 * @param {string} params.url - The URL the user is being redirected to.
		 * @param {string} params.baseUrl - The base URL of the application.
		 * @returns {Promise<string>} The URL to redirect to.
		 */
		async redirect({ url, baseUrl }) {
			// After sign in, if the callback URL is the sign-in page, redirect to /contacts
			if (url.startsWith(`${baseUrl}/signin`)) {
				return `${baseUrl}/contacts`;
			}
			// Allows relative callback URLs
			if (url.startsWith('/')) return `${baseUrl}${url}`;
			// Allows callback URLs on the same origin
			else if (new URL(url).origin === baseUrl) return url;
			return baseUrl;
		}
	}
});