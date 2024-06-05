<script>
	export let list;
</script>

<main>
	{#each list as media}
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
</main>
