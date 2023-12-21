<script lang="ts">
	import { onMount } from 'svelte';
	import Reveal from 'reveal.js';
	import Markdown from 'reveal.js/plugin/markdown/markdown';
	import Highlight from 'reveal.js/plugin/highlight/highlight';
	import Notes from 'reveal.js/plugin/notes/notes';

	import 'reveal.js/dist/reveal.css';
	import 'reveal.js/dist/theme/black.css';
	import 'reveal.js/plugin/highlight/zenburn.css';

	import Presentation from './presentation.svelte';

	onMount(() => {
		const deck = new Reveal({
			autoAnimateEasing: 'ease',
			autoAnimateDuration: 1,
			hash: true,
			controls: true,
			progress: true
		});

		deck.initialize({
			plugins: [Markdown, Highlight, Notes],

			height: 700,
			// 16/9
			width: 1244
		});

		// deck.configure({ showNotes: 'separate-page' });
	});
</script>

<div class="reveal">
	<div class="slides">
		<Presentation />
	</div>
</div>
