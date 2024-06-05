<script>
	import Home from '$lib/Home.svelte';
	import img_library from '$lib/assets/Stray_library.jpeg';
	import { invoke } from '@tauri-apps/api/tauri';

	let loaded_data = false;

	let promise = invoke('get_empty');

	function add_test() {
		invoke('add_media', {
			title: 'Les Misérables',
			people: [{ name: 'Victor Hugo', role: 'Author' }]
		});
	}

	function load_data() {
		promise = invoke('load_library_file');
		loaded_data = true;
	}
</script>

<main class="relative h-screen overflow-hidden bg-orange-800">
	<img src={img_library} alt="A dusty library" class="absolute z-0 h-full w-full object-cover" />
	<div class="absolute z-0 h-full w-full bg-black opacity-25" />
	<div class="absolute z-20 flex w-full justify-evenly">
		<button
			class="rounded-md bg-blue-300 p-2 text-lg text-slate-800 hover:bg-blue-500"
			type="button"
			on:click={load_data}
		>
			Charger
		</button>
		<button
			class="rounded-md bg-blue-300 p-2 text-lg text-slate-800 hover:bg-blue-500"
			type="button"
		>
			Sauvegarder
		</button>
	</div>
	<div class="z-10">
		{#if loaded_data}
			{#await promise}
				<div class="absolute flex h-full w-full items-center justify-center">
					<div class="rounded bg-slate-900 p-3 text-3xl text-slate-100">Chargement...</div>
				</div>
			{:then lib}
				<Home {lib} />
			{/await}
		{:else}
			<div class="absolute flex h-full w-full items-center justify-center">
				<div class="rounded bg-slate-900 p-3 text-3xl text-slate-100">
					⚠️ Chargez une Bibliothèque ⚠️
				</div>
			</div>
		{/if}
	</div>
</main>
