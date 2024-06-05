<script>
	export let lib;
	import img_search from '$lib/assets/search.svg';

	let nav_id = 1; //-1 for browse, 1 for add

	function go_to_browse() {
		nav_id = -1;
	}
	function go_to_add() {
		nav_id = 1;
	}
	function go_home() {
		nav_id = 0;
	}
</script>

<main>
	{#if nav_id != 0}
		<div class="absolute z-20 justify-normal text-left">
			<button class="ml-4 mt-12 rounded bg-slate-100 py-2 text-5xl" type="button" on:click={go_home}
				>🔙</button
			>
		</div>
	{/if}

	{#if nav_id == 0}
		<div class="absolute z-10 flex h-full w-full flex-col items-center justify-center">
			<div>
				<button
					class="h-full rounded-md bg-blue-500 px-4 py-4 text-4xl font-bold text-white transition duration-300 ease-in hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:ring-offset-blue-200"
					type="button"
					on:click={go_to_add}
				>
					Ajouter
				</button>
			</div>
			<div>
				<button class="h-full px-4 py-4" type="button">
					<img src={img_search} alt="A search icon" class="size-24" />
				</button>
			</div>
			<div>
				<button
					class="h-full rounded-md bg-blue-500 px-4 py-4 text-4xl font-bold text-white transition duration-300 ease-in hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:ring-offset-blue-200"
					type="button"
					on:click={go_to_browse}
				>
					Parcourir
				</button>
			</div>
		</div>
	{:else if nav_id == 1}
		<div class="absolute z-10 flex h-full w-full items-center justify-center">
			<div>
				<div class="text-lg font-bold text-slate-100">Titre</div>
				<input
					class="rounded-md border-2 border-slate-400 px-3 py-2"
					type="text"
					name="title"
					id="title"
				/>
			</div>
		</div>
	{:else if nav_id == -1}
		<div class="text-slate-50">
			{#each lib as media}
				<div class="flex">
					<div>{media.title}</div>
					<div>
						{#each media.people as person}
							<div>{person.name}</div>
							<div>{person.role}</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</main>
