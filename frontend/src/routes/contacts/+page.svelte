<!--
// frontend/src/routes/contacts/+page.svelte
// This page displays the list of contacts.
// It uses the ContactsTable component, which handles fetching and rendering the contacts.
// RELEVANT FILES: frontend/src/lib/components/ContactsTable.svelte, frontend/src/routes/+layout.server.ts
-->
<script lang="ts">
	import ContactsTable from '$lib/components/ContactsTable.svelte';
</script>

<svelte:head>
	<title>Contacts</title>
</svelte:head>

<ContactsTable />