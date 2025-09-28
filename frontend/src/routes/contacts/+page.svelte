<script lang="ts">
	import ContactsTable from '$lib/components/ContactsTable.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Contacts</title>
</svelte:head>

<ContactsTable contacts={data.contacts} />