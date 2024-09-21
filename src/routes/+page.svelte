<script lang="ts">
	import logo from '$lib/assets/obibli.svg';
	import edit from '$lib/assets/edit.svg';
	import trash from '$lib/assets/trash.svg';
	import { invoke } from '@tauri-apps/api/tauri';

	let promise: Promise<Array<Media>> = invoke('get_empty');
	let unsavedPromise: Promise<boolean> = invoke('get_false');
	let namesPromise: Promise<Array<string>> = invoke('get_empty');
	let genresPromise: Promise<Array<string>> = invoke('get_empty');
	let locationsPromise: Promise<Array<string>> = invoke('get_empty');

	let navId = -1; //-1 for home, -3 for browse, -2 for add
	let detailsId = -1;
	let query = '';

	let newMedia: Media = {
		title: '',
		author: '',
		genre: '',
		copies: 0,
		year: 0,
		location: '',
		notes: ''
	};

	let editedMedia: Media = {
		title: '',
		author: '',
		genre: '',
		copies: 0,
		year: 0,
		location: '',
		notes: ''
	};

	function loadData() {
		invoke('load_library_file');
		promise = invoke('get_all_medias');
		unsavedPromise = invoke('get_unsaved_status');
	}

	function saveData() {
		invoke('save_lib_to_file');
		unsavedPromise = invoke('get_unsaved_status');
	}

	function goToBrowse() {
		promise = invoke('get_all_medias');
		navId = -3;
	}
	function goToAdd() {
		navId = -2;
	}
	function goHome() {
		navId = -1;
	}

	function goToEdit(media: Media) {
		editedMedia = media;
		navId = 1;
	}

	function displayDetails(id: number | undefined) {
		if (id === undefined) {
			return;
		}
		if (detailsId == id) {
			detailsId = -1;
		} else {
			detailsId = id;
		}
	}

	function getSorted(by: string) {
		promise = invoke('get_sorted_medias', { by });
	}

	function addMedia() {
		invoke('add_media', {
			title: newMedia.title,
			author: newMedia.author,
			genre: newMedia.genre,
			copies: newMedia.copies,
			year: newMedia.year,
			location: newMedia.location,
			notes: newMedia.notes
		});
		unsavedPromise = invoke('get_unsaved_status');
		goHome();
		newMedia.title = '';
		newMedia.author = '';
		newMedia.genre = '';
		newMedia.copies = 0;
		newMedia.year = 0;
		newMedia.location = '';
		newMedia.notes = '';
	}

	function submitEdit() {
		invoke('edit_media', {
			id: editedMedia.id,
			changedTitle: editedMedia.title,
			changedAuthor: editedMedia.author,
			changedGenre: editedMedia.genre,
			changedCopies: editedMedia.copies,
			changedYear: editedMedia.year,
			changedLocation: editedMedia.location,
			changedNotes: editedMedia.notes
		});
		unsavedPromise = invoke('get_unsaved_status');
		goToBrowse();
	}

	function removeMedia() {
		invoke('remove_media', { id: detailsId });
		unsavedPromise = invoke('get_unsaved_status');
		promise = invoke('get_all_medias');
	}

	function getSearchResults() {
		promise = invoke('get_filtered_medias', { query });
	}

	function resetSearch() {
		query = '';
		promise = invoke('get_all_medias');
	}

	function getFilteredNames() {
		namesPromise = invoke('get_filtered_names', { substring: newMedia.author });
	}
	function getFilteredGenres() {
		genresPromise = invoke('get_filtered_genres', { substring: newMedia.genre });
	}
	function getFilteredLocations() {
		locationsPromise = invoke('get_filtered_locations', { substring: newMedia.location });
	}

	function handleEnterKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			getSearchResults();
		}
	}
</script>

<main class="h-dvh w-dvw">
	<div class="mb-2 mt-1 flex h-12 w-full justify-evenly">
		<button class="variant-filled-success btn text-xl" type="button" on:click={loadData}>
			Charger
		</button>

		{#await unsavedPromise then unsaved}
			{#if unsaved}
				<button class="variant-filled-error btn text-xl" type="button" on:click={saveData}>
					Sauvegarder
				</button>
			{:else}
				<button class="variant-filled-success btn text-xl" type="button" on:click={saveData}>
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
				{#if navId != -1}
					<div class="justify-normal text-center">
						<button class="btn text-5xl" type="button" on:click={goHome}>🏠</button>
					</div>
				{/if}
				{#if navId == -1}
					<div class="flex items-center justify-evenly">
						<button
							class="variant-filled-secondary btn mx-3 w-full rounded-xl px-4 py-4 text-4xl font-bold"
							type="button"
							on:click={goToAdd}
						>
							Ajouter
						</button>
						<img class="mx-1 h-20 w-auto" src={logo} alt="the obibli logo" />
						<button
							class="variant-filled-secondary btn mx-3 w-full rounded-xl px-4 py-4 text-4xl font-bold"
							type="button"
							on:click={goToBrowse}
						>
							Parcourir
						</button>
					</div>
				{:else if navId == -2}
					<!-- Adding UI -->
					<div class="logo-cloud mx-1 grid-cols-1 gap-1 md:grid-cols-2 lg:grid-cols-3">
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span>Titre</span>
								<input
									class="input font-normal"
									type="text"
									name="title"
									id="title"
									bind:value={newMedia.title}
								/>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Auteur </span>
								<input
									class="input font-normal"
									type="text"
									list="names"
									id="author"
									bind:value={newMedia.author}
									on:input={getFilteredNames}
									on:focus={getFilteredNames}
								/>
								<datalist id="names">
									{#await namesPromise}
										<option value=""></option>
									{:then names}
										{#each names as name}
											<option value={name}>{name}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span>Année</span>
								<input
									class="input font-normal"
									type="number"
									id="year"
									bind:value={newMedia.year}
								/>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Copies </span>
								<input
									class="input font-normal"
									type="number"
									id="copies"
									bind:value={newMedia.copies}
								/>
							</label>
						</div>

						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Genre </span>
								<input
									class="input font-normal"
									type="text"
									list="genres"
									id="genre"
									bind:value={newMedia.genre}
									on:input={getFilteredGenres}
									on:focus={getFilteredGenres}
								/>
								<datalist id="genres">
									{#await genresPromise}
										<option value=""></option>
									{:then genres}
										{#each genres as genre}
											<option value={genre}>{genre}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="text-lg font-bold">
								<span> Emplacement </span>
								<input
									class="input font-normal"
									type="text"
									list="locations"
									id="location"
									bind:value={newMedia.location}
									on:input={getFilteredLocations}
									on:focus={getFilteredLocations}
								/>
								<datalist id="locations">
									{#await locationsPromise}
										<option value=""></option>
									{:then locations}
										{#each locations as location}
											<option value={location}>{location}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>

						<div class="logo-item col-span-1 -my-4 md:col-span-2 lg:col-span-3">
							<label class="label text-lg font-bold">
								<span> Notes </span>
								<textarea
									class="textarea font-normal"
									name="notes"
									id="notes"
									cols="40"
									rows="3"
									bind:value={newMedia.notes}
								></textarea>
							</label>
						</div>
					</div>
					<div class="flex w-full justify-center">
						<button
							class="variant-filled-secondary btn mt-3 text-2xl"
							type="button"
							on:click={addMedia}>Ajouter</button
						>
					</div>
				{:else if navId == -3}
					<!-- Browsing UI -->
					<div class="mb-2 flex justify-center">
						<input
							class="input w-80"
							type="text"
							id="search"
							bind:value={query}
							on:keydown={handleEnterKeyDown}
						/>
						<button class="btn text-2xl" on:click={getSearchResults}>🔎</button>
						<button class="btn text-2xl" on:click={resetSearch}>❌</button>
					</div>
					<div class="table-container">
						<table class="table table-interactive table-compact">
							<thead>
								<tr>
									<th>
										<button class="underline" on:click={() => getSorted('title')}> Titre </button>
									</th>
									<th>
										<button class="underline" on:click={() => getSorted('year')}> Année </button>
									</th>
									<th>
										<button class="underline" on:click={() => getSorted('author')}> Auteur </button>
									</th>
									<th>Genre</th>
									<th>Emplacement</th>
								</tr>
							</thead>
							<tbody>
								{#each lib as media}
									<tr on:click={() => displayDetails(media.id)}>
										<td>
											{media.title}
										</td>
										<td>
											{media.year}
										</td>
										<td>
											{media.author}
										</td>
										<td>
											{media.genre}
										</td>
										<td>
											{media.location}
										</td>
									</tr>
									{#if detailsId == media.id}
										<tr>
											<td>
												<div class="font-bold">Copies</div>
												<div class="">{media.copies}</div>
											</td>
											<td colspan="2">
												<div class="font-bold">Notes</div>
												<div class="whitespace-pre-line">{media.notes}</div>
											</td>
											<td>
												<button class="btn" on:click={() => goToEdit(media)}>
													<img src={edit} alt="An editing icon" />
												</button>
											</td>
											<td>
												<button class="btn" on:click={removeMedia}>
													<img src={trash} alt="An trashbin icon" />
												</button>
											</td>
										</tr>
									{/if}
								{/each}
							</tbody>
						</table>
					</div>
				{:else if navId == 1}
					<!-- Editing UI -->
					<div class="logo-cloud mx-1 grid-cols-1 gap-1 md:grid-cols-2 lg:grid-cols-3">
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span>Titre</span>
								<input
									class="input font-normal"
									type="text"
									name="title"
									id="title"
									bind:value={editedMedia.title}
								/>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Auteur </span>
								<input
									class="input font-normal"
									type="text"
									list="names"
									id="author"
									bind:value={editedMedia.author}
									on:input={getFilteredNames}
									on:focus={getFilteredNames}
								/>
								<datalist id="names">
									{#await namesPromise}
										<option value=""></option>
									{:then names}
										{#each names as name}
											<option value={name}>{name}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span>Année</span>
								<input
									class="input font-normal"
									type="number"
									id="year"
									bind:value={editedMedia.year}
								/>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Copies </span>
								<input
									class="input font-normal"
									type="number"
									id="copies"
									bind:value={editedMedia.copies}
								/>
							</label>
						</div>

						<div class="logo-item -my-4">
							<label class="label text-lg font-bold">
								<span> Genre </span>
								<input
									class="input font-normal"
									type="text"
									list="genres"
									id="genre"
									bind:value={editedMedia.genre}
									on:input={getFilteredGenres}
									on:focus={getFilteredGenres}
								/>
								<datalist id="genres">
									{#await genresPromise}
										<option value=""></option>
									{:then genres}
										{#each genres as genre}
											<option value={genre}>{genre}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>
						<div class="logo-item -my-4">
							<label class="text-lg font-bold">
								<span> Emplacement </span>
								<input
									class="input font-normal"
									type="text"
									list="locations"
									id="location"
									bind:value={editedMedia.location}
									on:input={getFilteredLocations}
									on:focus={getFilteredLocations}
								/>
								<datalist id="locations">
									{#await locationsPromise}
										<option value=""></option>
									{:then locations}
										{#each locations as location}
											<option value={location}>{location}</option>
										{/each}
									{/await}
								</datalist>
							</label>
						</div>
						<div class="logo-item col-span-1 -my-4 md:col-span-2 lg:col-span-3">
							<label class="label text-lg font-bold">
								<span> Notes </span>
								<textarea
									class="textarea font-normal"
									name="notes"
									id="notes"
									cols="40"
									rows="3"
									bind:value={editedMedia.notes}
								></textarea>
							</label>
						</div>
					</div>
					<div class="flex w-full justify-center">
						<button
							class="variant-filled-secondary btn mt-3 text-2xl"
							type="button"
							on:click={submitEdit}>Valider</button
						>
					</div>
				{/if}
			</div>
		{/await}
	</div>
</main>
