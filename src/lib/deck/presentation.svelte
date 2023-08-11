<script lang="ts">
	import AppScreenshot from '$lib/presentation/appScreenshot.svelte';
	import Notes from './notes.svelte';
	import Slide from './slide.svelte';
</script>

<Slide>
	<h1>Openstreetmap und Overpass</h1>
</Slide>

<Slide animate>
	<h2>Openstreetmap</h2>
</Slide>

<Slide animate>
	<h2>Openstreetmap</h2>
	<ul>
		<li>Openstreetmap ist eine freie Karte</li>
		<li>Jeder kann sie bearbeiten</li>
		<li>Die vollständigkeit der Daten kann sehr unterschiedlich sein</li>
	</ul>
</Slide>

<Slide>
	<div class="flex flex-wrap justify-evenly items-center">
		<img src="detailedMap.png" alt="Eine sehr detaillierte Karte" width="45%" />
		<img src="unDetailedMap.png" alt="Eine sehr ungenaue Karte" width="45%" />
	</div>
</Slide>

<Slide>
	<div class="flex gap-2 items-center justify-evenly h-[500px]">
		{#each ['OSMApp', 'OrganicMaps', 'OSMAnd', 'Komoot'] as name}
			<AppScreenshot {name} src="/screenshots/{name}.PNG" />
		{/each}
	</div>
</Slide>
