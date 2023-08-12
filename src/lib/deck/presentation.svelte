<script lang="ts">
	import AppScreenshot from '$lib/presentation/appScreenshot.svelte';
	import Code from './code.svelte';
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

	<Notes>
		<ul>
			<li>Beide Bilder sind aus Chicago und nur wenige Kilometer voneinander entfernt</li>
			<li>
				In (Zentral) Europa ist die Qualität in der Regel recht hoch, vereinzelt auch so hoch wie
				links
			</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<div class="flex gap-2 items-center justify-evenly h-[500px]">
		{#each ['OSMApp', 'OrganicMaps', 'OSMAnd', 'Komoot'] as name}
			<AppScreenshot {name} src="/screenshots/{name}.PNG" />
		{/each}
	</div>

	<Notes>
		<li>OSMApp ist Open Source und eigentlich keine app sondern eine pwa</li>
		<li>Komoot ist nicht open source</li>
		<li>Organic Maps ist recht simpel hat aber auch nicht so viele funktionen</li>
		<li>
			OSMAnd ist sehr komplex doch hat auch sehr viele funktionen, es ist open source doch nur
			eingeschränkt kostenlos außer im F-Droid store
		</li>
	</Notes>
</Slide>

<Slide>
	<h3>Datenstruktur</h3>
</Slide>
<Slide>
	<Slide>
		<h4>Verschiedene Arten von Objekten</h4>
	</Slide>
	<Slide>
		<h5>Node</h5>

		<p>Ein Punkt auf der Karte</p>

		<ul>
			<li>Die meisten Sitzbänke</li>
			<li>Viele Läden</li>
		</ul>
	</Slide>
	<Slide>
		<h5>Way</h5>

		<p>Eine Linie oder eine simpele Fläche</p>

		<ul>
			<li>Eine Straße</li>
			<li>Ein See</li>
			<li>Ein Gebäude</li>
		</ul>
	</Slide>

	<Slide>
		<h5>Relation</h5>

		<p>Ein Zusammenschluss von Nodes, Ways und Relations</p>

		<ul>
			<li>Ein Gebäude mit <i>Loch</i></li>
			<li>Eine Buslinie</li>
		</ul>
	</Slide>
</Slide>

<Slide>
	<h4>Tags</h4>

	<p>Tags sind Schlüssel-Wert-Paare</p>

	<Code lines="1|1-4|5-6">
		{`
            highway=primary
            name=Hauptstraße
            maxspeed=80
            lanes=4
            lanes:forward=3
            lanes:backward=1
        `}
	</Code>

	<Notes>
		Nur weil man weiß das etwas eine Linie ist weiß man noch lange nicht was genau es ist
		<hr />

		<ul>
			<li>Eine Bundesstraße</li>
			<li>Durch die kombi mehr daten</li>
			<li>Ein <code>:</code> detailiert den key noch genauer</li>
			<li>:forward ist in die richtung in die der osm way verläuft und backward die andere</li>
			<li>Mit einem <code>;</code> kombiniert man werte</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Overpass</h2>

	<h3>Hier füge ich noch den qr code ein</h3>

	<Notes>
		<ul>
			<li>Man kann OSM als DB ansehen, wie jede db gibt es eine query sprache</li>
			<li>Dafür nutzt man die Overpass API und die Overpass Query Language</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<Slide>
		<h3>Wizard</h3>

		<img
			src="/overpassWizardButtonHighlighted.png"
			alt="Eine Grafik um den Button leichter zu finden"
		/>

		<Notes>
			Man kann zwar selber die query schreiben aber für simple ist das aufwendiger als nötig Indem
			man auf den wizard button drückt kommt ein popup wo man die query simpeler zusammenstellen
			kann
		</Notes>
	</Slide>
	<Slide>
		<h4>Beispiele</h4>
	</Slide>

	<Slide>
		<Code>shop=books in Berlin</Code>

		<Notes>
			<ol>
				<li>
					shop=books defeniert das man nach Objekten suchen will wo das tag shop gleich books ist.
					Weitere Tags sind hierfür egal. Es muss <b>exakt</b> books sein
				</li>
				<li>
					in Berlin macht das nur Buchhandlungen in Berlin gesucht werden, um zu wissen was <i
						>Berlin</i
					> ist wird nominatim genutzt, der selbe wie auf osm.org
				</li>
			</ol>
		</Notes>
	</Slide>

	<Slide>
		<Code>cinema in tokio</Code>

		<Notes>
			<ol>
				<li>Es gibt einige presets die haben dann (mehrere) tags in einem wort kombiniert</li>
				<li>Die englischen kann man im wizard nutzen</li>
			</ol>
		</Notes>
	</Slide>

	<Slide>
		<Code>amenity=parking and fee=no in "Köln"</Code>

		<Notes>
			<ol>
				<li>amenity=parking: Parkplatz</li>
				<li>fee=no: kostenlos</li>
				<li>And ist logisches and, also beides muss erfüllt sein</li>
				<li>
					in "Köln", man kann im wizard nicht einfach so alle zeichen eingeben, wenn der ort ein
					leerzeichen hat oder allgemein ein <code>:</code> dann muss man <code>""</code> nutzen
				</li>
			</ol>
		</Notes>
	</Slide>

	<Slide>
		<Code>amenity=fast_food and name!="McDonald's" and name!="Burger King"</Code>

		<Notes>
			Mit != für nicht gleiche sachen <br />
			Es gibt auch regex support mit ~ und !~ das ist besser wenn man viele namen ausschließen will
		</Notes>
	</Slide>

	<Slide>
		<Code>amenity=fast_food and name!="McDonald's" and name!="Burger King"</Code>

		<Notes>
			<ol>
				<li>amenity=fast_food ist ein preset</li>
				<li>name!="McDonald's" und name!="Burger King" sind tags</li>
			</ol>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide>
		<h3>Query</h3>

		<Notes>Jetzt schauen wir uns die Query gleich an und machen sie leichter</Notes>
	</Slide>
	<Slide animate>
		<Code lines id="code">
			{`
                /*
                This has been generated by the overpass-turbo wizard.
                The original search was:
                “shop=books in Berlin”
                */
                [out:json][timeout:25];
                // fetch area “Berlin” to search in
                {{geocodeArea:Berlin}}->.searchArea;
                // gather results
                (
                    // query part for: “shop=books”
                    node["shop"="books"](area.searchArea);
                    way["shop"="books"](area.searchArea);
                    relation["shop"="books"](area.searchArea);
                );
                // print results
                out body;
                >;
                out skel qt;
            `}
		</Code>

		<Notes>Wir entfernen kommentar, sie können gut sein doch sind es nicht in dem fall</Notes>
	</Slide>

	<Slide animate>
		<Code lines id="code">
			{`
                [out:json][timeout:25];
                {{geocodeArea:Berlin}}->.searchArea;

                (
                    node["shop"="books"](area.searchArea);
                    way["shop"="books"](area.searchArea);
                    relation["shop"="books"](area.searchArea);
                );

                out body;
                >;
                out skel qt;
            `}
		</Code>

		<Notes>
			Man kann das ende moderniesieren <br />
			man sollte immer ein out; am ende haben, fast immer out geom oder out center
		</Notes>
	</Slide>

	<Slide animate>
		<Code lines id="code">
			{`
                [out:json][timeout:25];
                {{geocodeArea:Berlin}}->.searchArea;

                (
                    node["shop"="books"](area.searchArea);
                    way["shop"="books"](area.searchArea);
                    relation["shop"="books"](area.searchArea);
                );

                out geom;
            `}
		</Code>

		<Notes>
			Indem man node way und relation durch nwr ersetzt ist es besser und man kann die union
			entfernen. Für die Programmierer DRY. Wenn man jetzt z.b. statt nach shop=books nach
			shop=supermarket suchen will muss man es nur einmal nicht drei mal ändern
		</Notes>
	</Slide>

	<Slide animate>
		<Code lines id="code">
			{`
                [out:json][timeout:25];
                {{geocodeArea:Berlin}}->.searchArea;

                nwr["shop"="books"](area.searchArea);

                out geom;
            `}
		</Code>

		<Notes>Das ist jetzt eine sehr schöne query</Notes>
	</Slide>
</Slide>

<Slide>
	<h3>Query Language</h3>

	<ul>
		<li>Semikolon: <b>notwendig</b>.</li>
		<li>
			<i>Abkürzungen</i>: <i>node</i>, <i>way</i>, <i>rel</i> oder <i>relation</i>.
		</li>
		<li>Kombinationen: <i>nw</i>, <i>nwr</i>, <i>nr</i>, <i>wr</i>.</li>
		<li>Tagsuche: <i>[shop=books][opening_hours]</i> für Buchhandlungen mit Öffnungszeiten.</li>
	</ul>

	<Notes>
		<ul>
			<li>Semikolons sind notwendig sonst gibt es einen Fehler</li>
			<li>
				Es gibt auch noch area doch das ist bischen spezieller zumal auf osm flächen wege sind
			</li>
			<li>
				eckige klammern für tags <ul>
					<li>nur ein wort ist dann nur der key der wert ist egal</li>
					<li>es gibt den operator = != ~ und !~ das sind so die wichtigsten</li>
					<li>
						{`Es gibt auch kein < > >= <= das kann man aber auch machen mit regex oder der if syntax`}
					</li>
					<li>Nur wenn es zeitlich passt hier ein beispiel von der if syntax:</li>
					<li><code>(if: (is_number(t["maxspeed"]) && t["maxspeed"] > 30))</code></li>
				</ul>
			</li>
		</ul>
	</Notes>
</Slide>
