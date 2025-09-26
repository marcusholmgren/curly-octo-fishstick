import { SvelteKitAuth } from '@auth/sveltekit';
import Keycloak from '@auth/sveltekit/providers/keycloak';
import {
	AUTH_KEYCLOAK_ID,
	AUTH_KEYCLOAK_ISSUER,
	AUTH_SECRET
} from '$env/dynamic/private';

export const { handle, signIn, signOut } = SvelteKitAuth({
	providers: [
		Keycloak({
			clientId: AUTH_KEYCLOAK_ID,
			issuer: AUTH_KEYCLOAK_ISSUER
		})
	],
	secret: AUTH_SECRET,
	trustHost: true,
	callbacks: {
		async jwt({ token, account }) {
			if (account) {
				token.accessToken = account.access_token;
			}
			return token;
		},
		async session({ session, token }) {
			// Add property to session, like an access_token from a provider.
			session.accessToken = token.accessToken as string;
			return session;
		}
	}
});