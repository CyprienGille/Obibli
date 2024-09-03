<script>
	import logo from '$lib/assets/obibli.svg';
	import { invoke } from '@tauri-apps/api/tauri';

	let promise = invoke('get_empty');
	let unsaved_promise = invoke('get_false');
	let names_promise = invoke('get_empty');
	let genres_promise = invoke('get_empty');
	let locations_promise = invoke('get_empty');

	let nav_id = -1; //0 for home, -1 for browse, 1 for add
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

<main class="h-dvh w-dvw">
	<div class="mb-2 mt-1 flex h-12 w-full justify-evenly">
		<button class="variant-filled-success btn text-xl" type="button" on:click={load_data}>
			Charger
		</button>

		{#await unsaved_promise then unsaved}
			{#if unsaved}
				<button class="variant-filled-error btn text-xl" type="button" on:click={save_data}>
					Sauvegarder
				</button>
			{:else}
				<button class="variant-filled-success btn text-xl" type="button" on:click={save_data}>
					Sauvegarder
				</button>
			{/if}
		{/await}
	</div>
	<div class="h-[calc(100dvh-3rem)] w-full">
		{#await promise}
			<div class="flex h-full w-full items-center justify-center">
				<div class="variant-filled-surface rounded-lg p-3 text-3xl">Chargement...</div>
			</div>
		{:then lib}
			<div class="h-full w-full">
				{#if nav_id != 0}
					<div class="justify-normal text-center">
						<button class="btn text-5xl" type="button" on:click={go_home}>🏠</button>
					</div>
				{/if}

				{#if nav_id == 0}
					<div class="flex items-center justify-evenly">
						<button
							class="variant-filled-secondary btn mx-3 w-full rounded-xl px-4 py-4 text-4xl font-bold"
							type="button"
							on:click={go_to_add}
						>
							Ajouter
						</button>
						<img class="mx-1 h-20 w-auto" src={logo} alt="the obibli logo" />
						<button
							class="variant-filled-secondary btn mx-3 w-full rounded-xl px-4 py-4 text-4xl font-bold"
							type="button"
							on:click={go_to_browse}
						>
							Parcourir
						</button>
					</div>
				{:else if nav_id == 1}
					<!-- Adding UI -->
					<div class="logo-cloud mx-1 grid-cols-1 gap-1 md:grid-cols-2 lg:grid-cols-3">
						<div class="logo-item">
							<label class="label text-lg font-bold">
								<span>Titre</span>
								<input
									class="input"
									type="text"
									name="title"
									id="title"
									bind:value={new_media.title}
								/>
							</label>
						</div>
						<div class="logo-item">
							<label class="label text-lg font-bold">
								<span> Auteur </span>
								<input
									class="input"
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
							</label>
						</div>
						<div class="logo-item">
							<label class="label text-lg font-bold">
								<span>Année</span>
								<input class="input" type="number" id="year" bind:value={new_media.year} />
							</label>
						</div>
						<div class="logo-item">
							<label class="label text-lg font-bold">
								<span> Copies </span>
								<input class="input" type="number" id="copies" bind:value={new_media.copies} />
							</label>
						</div>

						<div class="logo-item">
							<label class="label text-lg font-bold">
								<span> Genre </span>
								<input
									class="input"
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
							</label>
						</div>
						<div class="logo-item">
							<label class="text-lg font-bold">
								<span> Emplacement </span>
								<input
									class="input"
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
							</label>
						</div>

						<div class="logo-item col-span-1 md:col-span-2 lg:col-span-3">
							<label class="label text-lg font-bold">
								<span> Notes </span>
								<textarea
									class="textarea"
									name="notes"
									id="notes"
									cols="40"
									rows="4"
									bind:value={new_media.notes}
								></textarea>
							</label>
						</div>
					</div>
					<div class="flex w-full justify-center">
						<button
							class="variant-filled-secondary btn mt-3 text-2xl"
							type="button"
							on:click={add_media}>Ajouter</button
						>
					</div>
				{:else if nav_id == -1}
					<!-- Browsing UI -->
					<div class="mb-2 flex justify-center">
						<input class="input w-80" type="text" bind:value={query} />
						<button class="btn text-2xl" on:click={() => get_search_results(query)}>🔎</button>
						<button class="btn text-2xl" on:click={reset_search}>❌</button>
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
