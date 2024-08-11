<script lang="ts">
	import NavBar from './lib/NavBar.svelte';

	let quote = '';
	let translation = '';
	async function fetchCountFor(id: string) {
		const response = await fetch(`/api/${id}/count`);
		const data = await response.json();
		return data;
	}
</script>

<NavBar></NavBar>

<div class="flex flex-col m-6 dark:text-neutral-100 text-neutral-900 gap-6">
	<div>
		<p class="m-1">
			JP2aaS - <i>"Jan Paweł 2 as a Service"</i> - JP2 content available as
			an API. We serve:
		</p>
		<ul class="ml-8 list-outside list-disc">
			{#await fetchCountFor('quotes') then val}
				<li>{val} Quotes</li>
			{/await}
			{#await fetchCountFor('images') then val}
				<li>{val} Images</li>
			{/await}
			<li>More content in the future</li>
		</ul>
	</div>

	<div class="flex flex-col gap-3">
		<div class="flex-row flex-1">
			<button
				id="get-quote-btn"
				class="bg-purple-600 text-neutral-100 hover:bg-purple-500 border border-transparent hover:border-purple-300 p-2 rounded-md"
				on:click={async () => {
					const response = await fetch('/api/quotes/random');
					console.log(response);
					const data = await response.json();
					console.log(data);
					console.log(translation);
					quote = data.quote;
					translation = `(${data.translation})`;
				}}
			>
				Random Popequote
			</button>
		</div>
		<div
			class="min-h-20 rounded-md shadow-sm pt-3 pb-5 pl-4 pr-4 dark:bg-neutral-800 border dark:border-neutral-700 border-neutral-200"
			id="quote-container"
		>
			<p class="text-lg italic dark:text-neutral-80">
				{quote}
			</p>
			<p class="italic text-sm dark:text-neutral-80">{translation}</p>
		</div>
	</div>
</div>
