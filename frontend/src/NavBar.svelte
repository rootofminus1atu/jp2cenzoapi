<script lang="ts">
	import NavLink from './NavLink.svelte';
</script>

<nav
	class="flex items-center p-2 pl-4 dark:bg-neutral-800 bg-neutral-200 gap-4"
>
	<img src="images/papaspin.gif" alt="papaspin" class="w-10" />
	<h1 class="dark:text-neutral-100 text-neutral-900 text-lg">Jp2CenzoApi</h1>
	<div
		class="gap-4 flex flex-row-reverse flex-1 dark:text-neutral-100 text-neutral-900"
	>
		<NavLink href="docs.html">Api docs</NavLink>
		<NavLink href="index.html">Home</NavLink>
	</div>
</nav>
