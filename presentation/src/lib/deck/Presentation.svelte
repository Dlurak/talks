<script>
	import Code from './components/Code.svelte';
	import Notes from './components/Notes.svelte';
	import Slide from './components/Slide.svelte';
	import { samples } from '$lib/const/samples';
</script>

<Slide>
	<h1>Einen CLI mit Rust erstellen</h1>
	<div class="flex justify-center">
		<img src="/rust.png" alt="rust zeichnung">
	</div>
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
			<li>Der Type wird meistens erkannt, selten muss man ihn angeben</li>
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
		<Code lines={false}>{samples.init}</Code>
		<b>Eigene Übung (ﾉ◕ヮ◕)ﾉ</b>
		<Notes>
			Cargo schauen wir uns später nochmal genauer an <br>
			Namen und Alter ändern auf deinen <br/>
			<b>Für Expternten</b>: Erst Namen und Alter ausprinten und dann die Variablen verändern um das Alter um eins zu erhöhen
		</Notes>
	</Slide>
</Slide>

<Slide>
	<h2>Funktionen</h2>

	<Code>{samples.func}</Code>
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
			<h3>Expressions</h3>
			<ul>
				<li>Expressions sind Ausdrücke, die zu irgendeinem wert evaluiert werden</li>
				<li>blocks sind eine expression</li>
				<li>if sind expressions</li>
			</ul>
			<hr />
			<ul>
				<li>Vergleichbar mit dem tenary operator</li>
				<li>Semikolon</li>
			</ul>

			<h3>Übung</h3>
			<i>Falls es Probleme gibt, sonst nicht</i>
			<ul>
				<li>Setzt eine variabele temperature</li>
				<li>Je nach temperatur setzt ihr eine andere variabele auf Heiß, Kalt, oder Frostig</li>
				<li>Diese Variabele zurückgeben</li>
				<li>Diese Funktion ausprinten</li>
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
				<li>Ein leeres tuple ist basically nichts, wir haben keinen logischen rückgabewert brauchen aber einen return wert für den Ok wert</li>
			</ul>
			<h3>Option</h3>
			<ul>
				<li>Null-safety</li>
				<li>Man kann nur auf Datenzugreifen wenn man sichergestellt hat, dass diese existieren</li>
				<li>Also mit pattern matching ausliest</li>
				<li>Oder mit bestimmten spezifischen Methoden weiterverarbeitet</li>
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

<Slide animate>
	<Slide>
		<h2>Traits</h2>
		<Code>
			{samples.traits}
		</Code>
		<Notes>
			<ul>
				<li>Bischen wie interfaces in anderen Klassen</li>
				<li>Ermöglichen zusammen mit enums polymorphism</li>
			</ul>
		</Notes>
	</Slide>
	<Slide>
		<ul>
			<li>Eigenes trait auf beliebigen type</li>
			<li>Fremdes trait auf eigenen type</li>
		</ul>
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
				<li>das reduziert compile zeiten</li>
				<li>hier installiert man clap mit dem feature "derive"</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide>
		<h2>Borrow Checker</h2>
	</Slide>
	<Slide>
		<h2>Ownership</h2>
		<ul>
			<li>In Rust hat jeder Wert <b>genau einen</b> Owner</li>
			<li>Wenn der Owner out of scope geht dann wird er gelöscht</li>
		</ul>
		<Notes>
			<ul>
				<li>Memory management</li>
				<li>Verhindert use after frees</li>
			</ul>
		</Notes>
	</Slide>
	<Slide>
		<h2>Borrowing</h2>
		<ul>
			<li>Man kann beliebig viele Referenzen zu jedem Wert haben</li>
			<Code>{samples.borrow}</Code>
		</ul>
		<Notes>
			<ul>
				<li><b>ODER</b> eine mutable reference zu einem Objekt</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>Clap</h2>
		<Notes>
			<ul>
				<li>Library um command line arguments zu parsen z.b. `my-cli -n 5 -tgH --fav 5 --fav 6 "Hallo"`</li>
				<li>Macht auch help messages</li>
			</ul>
		</Notes>
	</Slide>
	<Slide animate>
		<h2>Clap</h2>
		<Code>{samples.clap.basic}</Code>
	</Slide>
	<Slide animate>
		<Code>{samples.clap.subcommands}</Code>
	</Slide>
</Slide>

<Slide>
	<h2>Happy coding</h2>
</Slide>
