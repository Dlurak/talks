<script>
	import Code from './code.svelte';
	import Notes from './notes.svelte';
	import Slide from './slide.svelte';
	import { samples } from '$lib/const/samples';
</script>

<Slide>
	<h1>Einen CLI mit Rust erstellen</h1>
</Slide>

<Slide>
	<h2>Vorstellungsrunde</h2>
	<ul>
		<li>Wie möchtet ihr Angesprochen werden?</li>
		<li>Wieviel könnt ihr schon programmieren?</li>
	</ul>
</Slide>

<Slide>
	<h2>Rust Installieren</h2>

	<div class="w-full grid grid-cols-2">
		<div>
			<h3>Linux {`<3`} und Mac</h3>
			<pre>https://www.rust-lang.org/tools/install</pre>
			<Code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</Code>
		</div>
		<div>
			<h3>Windows</h3>
			<p>
				Nach <i>rustup installation suchen</i> suchen
			</p>
		</div>
	</div>

	<Notes>
		- Rustup aber *nur* cargo reicht auch

		<h2>Linux und Macos</h2>
		<p>Oder halt mit pkg manager</p>

		<h2>Windows</h2>
		<p>Auf so eine wiki seite von rustup und dann eine exe downloaden</p>
	</Notes>
</Slide>

<Slide>
	<Slide animate>
		<Code lines>{samples.helloWorld}</Code>
	</Slide>
	<Slide animate>
		<Code lines="1">{samples.helloWorld}</Code>
		<ul>
			<li><i>main</i> Funktionen wird ausgeführt</li>
			<li><i>fn</i> für Funktionen</li>
			<li>Geschweifte Klammern</li>
		</ul>
	</Slide>
	<Slide animate>
		<Code lines="2">{samples.helloWorld}</Code>
		<ul>
			<li><i>let</i> für Variabelen</li>
			<li>Unveränderbar außer man erlaubt veränderungen</li>
			<li>Der type wird meistens erkannt</li>
		</ul>
	</Slide>
	<Slide animate>
		<Code lines="2-3">
			{samples.helloWorld
				.replace('let', 'let mut')
				.replace('David";', 'David";\n   name = "Serafin";')}
		</Code>
		<ul>
			<li>Unveränderbar außer man erlaubt veränderungen</li>
			<li>Mit <i>mut</i></li>
		</ul>
	</Slide>
</Slide>

<Slide>
	<h2>Struct, Enums und Traits</h2>

	<Notes>
		<ul>
			<li>Fragen wer structs und enums kennt</li>
			<li>examples/enumsStructsTraits.rs</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Cargo und dependency-management</h2>

	<Notes>
		<ul>
			<li>cargo new name</li>
			<li>cargo add -F derive</li>
		</ul>
	</Notes>
</Slide>
