<script lang="ts">
	import { onMount } from 'svelte';
	import Reveal from 'reveal.js';
	import Markdown from 'reveal.js/plugin/markdown/markdown';
	import Highlight from 'reveal.js/plugin/highlight/highlight';
	import Notes from 'reveal.js/plugin/notes/notes';
	import Zoom from 'reveal.js/plugin/zoom/zoom';

	import 'reveal.js/dist/reveal.css';
	import 'reveal.js/dist/theme/white.css';
	import './highlights.css';
	// latte-hightlights.css is a light theme just in case of contrast issues
	//import './latte-highlights.css';

	import Presentation from './Presentation.svelte';

	onMount(() => {
		const deck = new Reveal({
			autoAnimateEasing: 'ease',
			autoAnimateDuration: 1,
			hash: true,
			controls: true,
			progress: true
		});

		deck.initialize({
			plugins: [Markdown, Highlight, Notes, Zoom],

			// 16/9
			height: 700,
			width: 1244
		});
	});
</script>

<div class="reveal">
	<div class="slides">
		<Presentation />
	</div>
</div>
