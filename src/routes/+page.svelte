<script>
	import logo from '$lib/assets/obibli.svg';
	import { invoke } from '@tauri-apps/api/tauri';

	let promise = invoke('get_empty');
	let unsaved_promise = invoke('get_false');
	let names_promise = invoke('get_empty');
	let genres_promise = invoke('get_empty');
	let locations_promise = invoke('get_empty');

	let nav_id = 0; //0 for home, -1 for browse, 1 for add
	let details_id = -1;
	let query = '';

	let new_media = {
		title: '',
		author: '',
		genre: '',
		copies: 0,
		year: 0,
		location: '',
		notes: ''
	};

	function load_data() {
		invoke('load_library_file');
		promise = invoke('get_all_medias');
		unsaved_promise = invoke('get_unsaved_status');
	}

	function save_data() {
		invoke('save_lib_to_file');
		unsaved_promise = invoke('get_unsaved_status');
	}

	function go_to_browse() {
		promise = invoke('get_all_medias');
		nav_id = -1;
	}
	function go_to_add() {
		nav_id = 1;
	}
	function go_home() {
		nav_id = 0;
	}

	/**
	 * @param {number} id
	 */
	function display_details(id) {
		if (details_id == id) {
			details_id = -1;
		} else {
			details_id = id;
		}
	}

	/**
	 * @param {string} by
	 */
	function get_sorted(by) {
		promise = invoke('get_sorted_medias', { by });
	}

	function add_media() {
		invoke('add_media', {
			title: new_media.title,
			author: new_media.author,
			genre: new_media.genre,
			copies: new_media.copies,
			year: new_media.year,
			location: new_media.location,
			notes: new_media.notes
		});
		unsaved_promise = invoke('get_unsaved_status');
		go_home();
		new_media.title = '';
		new_media.author = '';
		new_media.genre = '';
		new_media.copies = 0;
		new_media.year = 0;
		new_media.location = '';
		new_media.notes = '';
	}

	/**
	 * @param {string} query
	 */
	function get_search_results(query) {
		promise = invoke('get_filtered_medias', { query });
	}

	function reset_search() {
		query = '';
		promise = invoke('get_all_medias');
	}

	function get_filtered_names() {
		names_promise = invoke('get_filtered_names', { substring: new_media.author });
	}
	function get_filtered_genres() {
		genres_promise = invoke('get_filtered_genres', { substring: new_media.genre });
	}
	function get_filtered_locations() {
		locations_promise = invoke('get_filtered_locations', { substring: new_media.location });
	}
</script>

<main class="h-full w-full">
	<div class="mb-2 mt-1 flex w-full justify-evenly">
		<button
			class="rounded-md bg-blue-300 p-2 text-xl text-slate-800 hover:bg-blue-500"
			type="button"
			on:click={load_data}
		>
			Charger
		</button>

		{#await unsaved_promise then unsaved}
			{#if unsaved}
				<button
					class="rounded-md bg-red-700 p-2 text-xl text-slate-100 hover:bg-red-500"
					type="button"
					on:click={save_data}
				>
					Sauvegarder
				</button>
			{:else}
				<button
					class="rounded-md bg-green-400 p-2 text-xl text-slate-900 hover:bg-green-500"
					type="button"
					on:click={save_data}
				>
					Sauvegarder
				</button>
			{/if}
		{/await}
	</div>
	<div class="h-full w-full">
		{#await promise}
			<div class="flex h-full w-full items-center justify-center">
				<div class="rounded bg-slate-900 p-3 text-3xl text-slate-100">Chargement...</div>
			</div>
		{:then lib}
			<div>
				{#if nav_id != 0}
					<div class="justify-normal text-center">
						<button
							class="mb-2 mr-4 rounded bg-slate-100 py-2 text-5xl"
							type="button"
							on:click={go_home}>🏠</button
						>
					</div>
				{/if}

				{#if nav_id == 0}
					<div class="flex h-full items-center justify-evenly">
						<button
							class="h-full w-full rounded-xl bg-blue-500 px-4 py-4 text-4xl font-bold text-white hover:bg-blue-600"
							type="button"
							on:click={go_to_add}
						>
							Ajouter
						</button>
						<img class="mx-1 h-20 w-auto" src={logo} alt="the obibli logo" />
						<button
							class="h-full w-full rounded-xl bg-blue-500 px-4 py-4 text-4xl font-bold text-white hover:bg-blue-600"
							type="button"
							on:click={go_to_browse}
						>
							Parcourir
						</button>
					</div>
				{:else if nav_id == 1}
					<!-- Adding UI -->
					<div class="flex justify-evenly">
						<div class="">
							<div class="text-lg font-bold text-slate-900">Titre</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="text"
								name="title"
								id="title"
								bind:value={new_media.title}
							/>
						</div>
						<div>
							<div class="text-lg font-bold text-slate-900">Auteur</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="text"
								list="names"
								id="author"
								bind:value={new_media.author}
								on:input={get_filtered_names}
								on:focus={get_filtered_names}
							/>
							<datalist id="names">
								{#await names_promise}
									<option value=""></option>
								{:then names}
									{#each names as name}
										<option value={name}>{name}</option>
									{/each}
								{/await}
							</datalist>
						</div>
						<div class="">
							<div class="text-lg font-bold text-slate-900">Année</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="number"
								id="year"
								bind:value={new_media.year}
							/>
						</div>
						<div class="">
							<div class="text-lg font-bold text-slate-900">Copies</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="number"
								id="copies"
								bind:value={new_media.copies}
							/>
						</div>
					</div>
					<div class="flex justify-evenly">
						<div>
							<div class="text-lg font-bold text-slate-900">Genre</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="text"
								list="genres"
								id="genre"
								bind:value={new_media.genre}
								on:input={get_filtered_genres}
								on:focus={get_filtered_genres}
							/>
							<datalist id="genres">
								{#await genres_promise}
									<option value=""></option>
								{:then genres}
									{#each genres as genre}
										<option value={genre}>{genre}</option>
									{/each}
								{/await}
							</datalist>
						</div>
						<div>
							<div class="text-lg font-bold text-slate-900">Emplacement</div>
							<input
								class="rounded-md border-2 border-slate-400 px-3 py-2"
								type="text"
								list="locations"
								id="location"
								bind:value={new_media.location}
								on:input={get_filtered_locations}
								on:focus={get_filtered_locations}
							/>
							<datalist id="locations">
								{#await locations_promise}
									<option value=""></option>
								{:then locations}
									{#each locations as location}
										<option value={location}>{location}</option>
									{/each}
								{/await}
							</datalist>
						</div>
					</div>
					<div class="flex justify-evenly">
						<div>
							<div class="text-lg font-bold text-slate-900">Notes</div>
							<textarea
								class="rounded-md border-2 border-slate-400 p-2"
								name="notes"
								id="notes"
								cols="40"
								rows="4"
								bind:value={new_media.notes}
							></textarea>
						</div>
					</div>
					<div class="flex w-full justify-center">
						<button
							class="mt-3 rounded-md bg-green-400 p-3 text-2xl hover:bg-green-500"
							type="button"
							on:click={add_media}>Ajouter</button
						>
					</div>
				{:else if nav_id == -1}
					<!-- Browsing UI -->
					<div class="mb-2 flex justify-center">
						<input class="rounded-xl border-2 px-6 py-2" type="text" bind:value={query} />
						<button class="text-2xl" on:click={() => get_search_results(query)}>🔎</button>
						<button class="text-2xl" on:click={reset_search}>❌</button>
					</div>
					<div class="mx-2 text-slate-800">
						<div class="flex font-semibold">
							<div class="w-1/4">
								<button class="underline" on:click={() => get_sorted('title')}> Titre </button>
							</div>
							<div class="w-1/12">
								<button class="underline" on:click={() => get_sorted('year')}> Année </button>
							</div>
							<div class="w-1/4">
								<button class="underline" on:click={() => get_sorted('author')}> Auteur </button>
							</div>
							<div class="w-1/6">Genre</div>
							<div class="w-1/4">Emplacement</div>
						</div>
						{#each lib as media}
							<div class="rounded border-2">
								<div class="flex">
									<div class="w-1/4">
										<button
											class="ml-1 text-blue-500 underline"
											on:click={() => display_details(media.id)}
										>
											{media.title}
										</button>
									</div>
									<div class="w-1/12">{media.year}</div>
									<div class="w-1/4">
										{media.author}
									</div>
									<div class="w-1/6">
										{media.genre}
									</div>
									<div class="w-1/4">
										{media.location}
									</div>
								</div>
								{#if details_id == media.id}
									<div class="flex font-semibold">
										<div class="w-1/4">Copies</div>
										<div class="w-3/4">Notes</div>
									</div>
									<div class="mb-4 flex">
										<div class="w-1/4">{media.copies}</div>
										<div class="w-3/4 whitespace-pre-line">{media.notes}</div>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/await}
	</div>
</main>
