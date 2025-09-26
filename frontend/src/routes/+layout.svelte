<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
</script>

<header class="bg-gray-100 p-4">
	<div class="container mx-auto flex justify-between items-center">
		<h1 class="text-xl font-bold">
			<a href="/">Contacts App</a>
		</h1>
		<div>
			{#if $page.data.session?.user}
				<div class="flex items-center">
					<span class="mr-4">Hi, {$page.data.session.user.name}</span>
					<form action="/signout" method="POST">
						<button
							class="rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500"
							>Sign out</button
						>
					</form>
				</div>
			{/if}
		</div>
	</div>
</header>

<main class="container mx-auto p-4">
	<slot />
</main>