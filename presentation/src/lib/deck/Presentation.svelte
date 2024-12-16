<script>
	import Code from './components/Code.svelte';
	import Notes from './components/Notes.svelte';
	import Slide from './components/Slide.svelte';
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
		<li>Habt ihr eine CLI Idee?</li>
	</ul>
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
			{samples.helloWorldMut}
		</Code>
		<ul>
			<li>Unveränderbar außer man erlaubt veränderungen</li>
			<li>Mit <i>mut</i></li>
		</ul>
	</Slide>
	<Slide animate>
		<Code>
			{samples.helloWorldMut}
		</Code>
		<b>Eigene Übung (ﾉ◕ヮ◕)ﾉ</b>
		<Notes>
			Namen und Alter ändern auf deinen <br/>
			<b>Für Expternten</b>: Erst namen und alter ausprinten und dann die vars verändern um das
			alter um eins zu erhöhen
		</Notes>
	</Slide>
	<Slide>
		<Code lines={false}>{samples.init}</Code>
		<Notes>Cargo schauen wir uns später nochmal genauer an</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>If-Anweisungen</h2>
	</Slide>
	<Slide animate>
		<h2>If-Anweisungen</h2>

		<Code id="ifs">
			{samples.ifs}
		</Code>
	</Slide>
	<Slide animate>
		<h2>If-Anweisungen</h2>

		<Code id="assignments">
			{samples.ifAssignments}
		</Code>

		<Notes>
			<ul>
				<li>Vergleichbar mit dem tenary operator</li>
				<li>Semikolon</li>
			</ul>

			<h3>Übung</h3>
			<i>Falls es probleme gibt</i>
			<ul>
				<li>Setzt eine variabele temperature</li>
				<li>Je nach temperatur setzt ihr eine andere variabele auf Heiß, Kalt, oder Frostig</li>
				<li>Diese Variabele ausprinten</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>Enums</h2>
	</Slide>

	<Slide animate>
		<h2>Enums</h2>

		<Code>{samples.enums}</Code>

		<Notes>
			<ul>
				<li>Eine von verschiedene Optionen</li>
				<li><b>Viel</b> besser als in anderen Sprachen</li>
				<li><b>Viel</b> wichtiger als in anderen Sprachen</li>
			</ul>
		</Notes>
	</Slide>

	<Slide>
		<Code>{samples.enumsData}</Code>

		<Notes>
			<ul>
				<li>Auf einzelne Varianten Daten</li>
				<li>Nur diese Variante muss diese Daten haben!</li>
			</ul>
		</Notes>
	</Slide>
	<Slide>
		<Code>{samples.optionResult}</Code>

		<Notes>
			<h3>Result</h3>
			<ul>
				<li>Fehler management</li>
				<li>Unterschied zwischen try/catch und Result</li>
				<li>Beispielhaft erklären</li>
			</ul>
			<h3>Option</h3>
			<ul>
				<li>Null-safety</li>
				<li>Man kann nur auf Datenzugreifen wenn man sichergestellt hat, dass diese existieren</li>
			</ul>
			<h4>Für Haskell-Menschis</h4>
			Das sind monads
		</Notes>
	</Slide>
	<Slide>
		<Code>{samples.optionResultDef}</Code>
	</Slide>
	<Slide>
		<Code>{samples.match}</Code>
		<Notes>
			<ul>
				<li>Man muss alle Möglichkeiten abdecken</li>
				<li><pre class="inline">_</pre> als default</li>
				<li>Reihnfolge ist wichtig</li>
			</ul>
		</Notes>
	</Slide>
	<Slide>
		<Code>{samples.matchRes}</Code>
		<Notes>
			<ul>
				<li>Mit use kann man importieren</li>
				<li>
					<a href="https://doc.rust-lang.org/std/env/fn.var.html">Rust env::var docs</a>
				</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>Structs</h2>
	</Slide>

	<Slide animate>
		<h2>Structs</h2>

		<Code>
			{samples.struct}
		</Code>
		<Notes>
			<ul>
				<li>Kind of wie sehr feste/genau definierte js objects</li>
				<li>Ansatzweise wie Klassen</li>
				<li>Man kann verschiedene Datentypen ordentlich gruppieren</li>
			</ul>
		</Notes>
	</Slide>
	<Slide>
		<Code>
			{samples.structDeriveImpl}
		</Code>
		<Notes>
			<ul>
				<li></li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>Cargo</h2>
		<Notes>
			<ul>
				<li>Package manager von rust</li>
				<li>Wie npm oder pip</li>
			</ul>
		</Notes>
	</Slide>
	<Slide animate>
		<h2>Cargo</h2>
		<Code lines={false}>{"$ cargo add clap -F derive"}</Code>
		<Notes>
			<ul>
				<li>Rust libs nennt man crate</li>
				<li>crates können optionale features haben</li>
				<li>hier installiert man clap mit dem feature "derive"</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<h2>Happy coding</h2>
</Slide>
