<script lang="ts">
	export let href: string;

	export let selected = false;
</script>

<a
	class={selected
		? 'dark:text-purple-300 text-purple-600 font-semibold'
		: 'font-semibold'}
	{href}
>
	<slot></slot>
</a>
