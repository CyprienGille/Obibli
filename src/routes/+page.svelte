<script>
	import img_library from '$lib/assets/Stray_library.jpeg';
	import img_search from '$lib/assets/search.svg';
</script>

<main class="relative h-screen overflow-hidden bg-orange-800">
	<img src={img_library} alt="A dusty library" class="absolute z-0 h-full w-full object-cover" />
	<div class="absolute z-0 h-full w-full bg-black opacity-25"></div>
	<div class="relative z-10 flex h-full w-full flex-col items-center justify-center">
		<div class="">
			<button
				class="h-full rounded-md bg-blue-500 px-4 py-4 text-4xl font-bold text-white transition duration-300 ease-in hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:ring-offset-blue-200"
			>
				Ajouter
			</button>
		</div>
		<div>
			<img src={img_search} alt="A search icon" class="size-32" />
		</div>
		<div>
			<button
				class="h-full rounded-md bg-blue-500 px-4 py-4 text-4xl font-bold text-white transition duration-300 ease-in hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:ring-offset-blue-200"
			>
				Parcourir
			</button>
		</div>
	</div>
</main>
