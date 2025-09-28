import { SvelteKitAuth } from '@auth/sveltekit';
import Keycloak from '@auth/sveltekit/providers/keycloak';
import {
	AUTH_KEYCLOAK_ID,
	AUTH_KEYCLOAK_ISSUER,
	AUTH_KEYCLOAK_SECRET,
	AUTH_SECRET
} from '$env/dynamic/private';

async function refreshAccessToken(token) {
    try {
        const url = `${AUTH_KEYCLOAK_ISSUER}/protocol/openid-connect/token`;

        const response = await fetch(url, {
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
            method: "POST",
            body: new URLSearchParams({
                client_id: AUTH_KEYCLOAK_ID,
                client_secret: AUTH_KEYCLOAK_SECRET,
                grant_type: "refresh_token",
                refresh_token: token.refreshToken,
            }),
        });

        const refreshedTokens = await response.json();

        if (!response.ok) {
            throw refreshedTokens;
        }

        return {
            ...token,
            accessToken: refreshedTokens.access_token,
            accessTokenExpires: Date.now() + refreshedTokens.expires_in * 1000,
            refreshToken: refreshedTokens.refresh_token ?? token.refreshToken, // Fall back to old refresh token
        };
    } catch (error) {
        console.error("Error refreshing access token", error);

        return {
            ...token,
            error: "RefreshAccessTokenError",
        };
    }
}

export const { handle, signIn, signOut } = SvelteKitAuth({
	providers: [
		Keycloak({
			clientId: AUTH_KEYCLOAK_ID,
			clientSecret: AUTH_KEYCLOAK_SECRET,
			issuer: AUTH_KEYCLOAK_ISSUER
		})
	],
	secret: AUTH_SECRET,
	trustHost: true,
	callbacks: {
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
		async session({ session, token }) {
			// Add property to session, like an access_token from a provider.
			session.accessToken = token.accessToken as string;
            session.error = token.error as string;
			return session;
		},
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