<!--
// frontend/src/routes/signin/+page.svelte
// This file creates the sign-in page for the application.
// It provides a button for users to initiate the Keycloak authentication process.
// RELEVANT FILES: frontend/src/auth.ts, frontend/src/hooks.server.ts
-->
<div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
	<div class="sm:mx-auto sm:w-full sm:max-w-sm">
		<h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
			Sign in to your account
		</h2>
	</div>

	<div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
		<form class="space-y-6" action="/auth/signin/keycloak" method="POST">
			<button
				type="submit"
				class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			>
				Sign in with Keycloak
			</button>
		</form>
	</div>
</div>