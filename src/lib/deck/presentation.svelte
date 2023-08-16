<script lang="ts">
	import AppScreenshot from '$lib/presentation/appScreenshot.svelte';
	import Code from './code.svelte';
	import Notes from './notes.svelte';
	import Slide from './slide.svelte';
</script>

<Slide>
	<h1>Openstreetmap und Overpass</h1>

	<Notes>
		<h3>Dimi</h3>
	</Notes>
</Slide>

<Slide animate>
	<h2>Openstreetmap</h2>

	<Notes>
		<h3>Dimi</h3>
	</Notes>
</Slide>

<Slide animate>
	<h2>Openstreetmap</h2>
	<ul>
		<li>Openstreetmap ist eine freie Karte, von daher <i>Open data</i></li>
		<li>Jeder kann sie bearbeiten</li>
		<li>Die Karte lebt von Bearbeitungen</li>
	</ul>

	<Notes>
		<h3>David</h3>
	</Notes>
</Slide>

<Slide>
	<div class="flex flex-wrap justify-evenly items-center">
		<img src="detailedMap.png" alt="Eine sehr detaillierte Karte" width="45%" />
		<img src="unDetailedMap.png" alt="Eine sehr ungenaue Karte" width="45%" />
	</div>

	<Notes>
		<h3>David</h3>
		<ul>
			<li>Die Qualität der Kartenkann extrem hoch werden, kann jedoch auchh sehr shlecht sein</li>
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
		<h3>David</h3>

		<li>
			OSMApp ist Open Source und eigentlich keine app sondern eine pwa (Progressive Web Application)
		</li>
		<li><i>(Maps.me)</i> Organic Maps ist recht simpel hat aber auch nicht so viele funktionen</li>
		<li>
			OSMAnd ist sehr komplex doch hat auch sehr viele funktionen, es ist open source doch nur
			eingeschränkt kostenlos außer im F-Droid store
		</li>
		<li>Komoot ist nicht open source</li>
	</Notes>
</Slide>

<Slide>
	<h3>Datenstruktur</h3>

	<Notes>
		<h3>Dimi</h3>
	</Notes>
</Slide>

<Slide>
	<Slide>
		<h4>Verschiedene Arten von Objekten</h4>

		<Notes>
			<h3>Dimi</h3>
			<hr />
			<i>Nodes, ways, relations</i>
		</Notes>
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
		<h3>David</h3>

		Tags spezifizieren was genau ein Objekt ist
		<ul>
			<li>Eine Bundesstraße</li>
			<li>Durch die kombi mehr daten</li>
			<li>Ein <code>:</code> detailiert den key noch mehr</li>
			<li>:forward ist in die richtung in die der osm way verläuft und backward die andere</li>
			<li>Mit einem <code>;</code> kombiniert man werte</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Overpass</h2>

	<div class="flex items-center justify-center h-96">
		<img src="/overpassQR.gif" alt="QR-Code zu overpass" class="h-full aspect-square" />
	</div>

	<Notes>
		<h3>David</h3>

		<ul>
			<li>Man kann OSM als Datenbank ansehen, wie jede Datenbank gibt es eine query sprache</li>
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
			<h3>David</h3>

			<ul>
				<li>FÜr simple Queries kann man den Wizard nutzen</li>
				<li>Wenn man den Wizard button drückt kommt ein popup</li>
				<li>dort kann man simple Queries erstellen, aber</li>
			</ul>
			Man kann zwar selber die query schreiben aber für simple Queries ist das aufwendiger als nötig
			Indem man auf den Wizard button drückt kommt ein popup wo man die query simpeler zusammenstellen
			kann
		</Notes>
	</Slide>

	<Slide>
		<h3>Live Demo</h3>

		<Notes>
			<h3>Dimi</h3>

			<a href="https://www.overpass-turbo.eu">Overpass turbo link</a>
		</Notes>
	</Slide>

	<Slide>
		<h4>Beispiele</h4>
	</Slide>

	<Slide>
		<Code>shop=books in Berlin</Code>

		<Notes>
			<h3>David</h3>
			<b>Lass es die user einfach mal abtippen um sehen zu lassen</b> <br />
			<i>Warnung wegen mobile</i>
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
			<h3>Dimi</h3>
			<ol>
				<li>Es gibt einige presets die haben dann (mehrere) tags in einem wort kombiniert</li>
				<li>Die englischen kann man im wizard nutzen</li>
			</ol>
		</Notes>
	</Slide>

	<Slide>
		<Code>amenity=parking and fee=no in "Köln"</Code>

		<Notes>
			<h3>David</h3>
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
			<h3>Dimi</h3>
			<ol>
				<li>!= ist nicht gleich</li>
				<li>Es gibt kein {'< > <= >= '}</li>
				<li>name!="McDonald's" und name!="Burger King" sind tags</li>
			</ol>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide>
		<h3>Query</h3>

		<Notes>
			<h3>David</h3>
			Jetzt schauen wir uns die Query gleich an und machen sie leichter
		</Notes>
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
		<Code lines="|10-12" id="code">
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
			man sollte immer ein out; am ende haben, fast immer out geom oder out center <br />
			<table>
				<tbody>
					<tr>
						<th>out geom</th>
						<td>Gesamte objekte</td>
					</tr>
					<tr>
						<th>out center</th>
						<td>Nur das Zentrum von einem Objekt</td>
					</tr>
				</tbody>
			</table>
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
			<ul>
				<li>Node, way und relation durch nwr ersetzen</li>
				<li>Für programmierer: DRY (Don't repeat yourself)</li>
				<li>Änderungen sind leichter <i>(man muss nur einmal statt dreimal etwas ändern)</i></li>
			</ul>
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
		<h3>Dimi</h3>

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

<Slide>
	<Slide animate>
		<h4>Sets und Unions</h4>

		<Notes>
			<h3>David</h3>
		</Notes>
	</Slide>
	<Slide animate>
		<h4>Sets und Unions</h4>

		<Code lines id="code">
			{`
                node[shop=books];
                way[shop=books];
                relation[shop=books];
            `}
		</Code>

		<Notes>
			<h3>David</h3>

			<ol>
				<li>Es wird nach nodes gesucht</li>
				<li>Das ergebniss wird erst mit ways dann relations überschrieben</li>
			</ol>

			<ul>
				<li>Es gibt sets</li>
				<li>Sets starten mit .</li>
				<li>Default set ist ._</li>
				<li>._ wurde hier drei mal überschrieben</li>
				<li>Unions nutzen um das zu vermeiden</li>
			</ul>
		</Notes>
	</Slide>

	<Slide animate>
		<h4>Sets und Unions</h4>

		<Code lines id="code">
			{`
                (
                    node[shop=books];
                    way[shop=books];
                    relation[shop=books];
                );
            `}
		</Code>

		<Notes>
			<h3>David</h3>

			<ul>
				<li>Klammern: Unions</li>
				<li>Problem gelöst</li>
			</ul>
		</Notes>
	</Slide>

	<Slide animate>
		<h4>Sets und Unions</h4>

		<Code lines id="code">
			{`
                node[shop=books]->.bookStores;

                .bookStores;

                out center;
            `}
		</Code>

		<Notes>
			<h3>David</h3>

			Hier wird nun in das set .bookStores geschrieben. Das kann in größeren queries sehr sehr
			hilfreich sein.<br />
			Dadurch das man nochmals .bookStores direkt vor dem out schreibt wird dieses set ausgegeben

			<ul>
				<li>Pfeil sytax zum schreiben</li>
				<li>Es wird in .bookStores geschrieben</li>
				<li>Wir schreiben .bookStores direkt vor dem out um es auszugeben</li>
				<li>Hier unnütz, doch in großen queries kann es hilfreich sein</li>
			</ul>
		</Notes>
	</Slide>
</Slide>

<Slide>
	<Slide animate>
		<h2>Beispiele</h2>

		<Notes>
			<h3>Dimi</h3>
		</Notes>
	</Slide>

	<Slide animate>
		<h2>Beispiele</h2>

		<Code lines>
			{`
                [out:csv(::id,::lat,::lon;true;";")];
                
                // Das Camp Gelände mit der ID
                area(442428017)->.searchArea;

                // Bäume in der searchArea
                node["natural"="tree"](area.searchArea);
                out;
            `}
		</Code>

		<Notes>
			<h3>Dimi</h3>
			<ul>
				<li>
					<b>DIMI</b>: Zeile 1 erklären
				</li>
				<li>
					Wir suchen nach der area mit der id und schreiben es in die search area. Es handelt scih
					um das camp gelände
				</li>
				<li>Dann suchen wir nach bäumen in der area durch area.searchArea</li>
				<li>Out</li>
			</ul>
		</Notes>
	</Slide>

	<Slide>
		<h2>Beispiele</h2>

		<Code lines>
			{`
                [out:json][timeout:25];
                {{geocodeArea:Hamburg}}->.searchArea;
                nwr["amenity"="restaurant"][cuisine~"korean"](area.searchArea);
            
                out center;
            `}
		</Code>

		<Notes>
			<ul>
				<li>geocode area ist die nominatim suche (standard auf osm website)</li>
				<li>cuisine ~ korean ist regex</li>
				<li>dieser regex sucht für strings mit "korean" drin</li>
			</ul>
		</Notes>
	</Slide>

	<Slide animate>
		<h2>Beispiele</h2>

		<Code lines="1-11|1|2|4-8|10|11|1,11" id="code">
			{`
                [out:csv(ref, colour; false; ';')];
                node(205364328)->.center;

                (
                    node(around.center:150)["public_transport"="stop_position"];
                    nw(around.center:150)["highway"="bus_stop"];
                    nwr(around.center:150)["amenity"="ferry_terminal"];
                ) -> .stops;

                rel(bn.stops)["route"~"bus|train|tram|subway|light_rail|ferry|monorail"];
                out;
            `}
		</Code>

		<Notes>
			<h3>Dimi</h3>
			<ul>
				<li>
					Diese query habe ich letztens in einem pull request for die osm website osmapp genutzt.
					Sie findet alle bus/s-bahn/bahn... linien rund um einen bahnhof und gibt die offiziele
					farbe der linie und die referenz aus.
				</li>
				<li>
					Zuerst out csv, damit bekommt man die daten im csv format, in dem fall erst mit dem tag
					ref dann einem selikonom und dann colour ohne eine erste zeile die definiert was was ist
				</li>
				<li>
					Dann wird der node mit der osm id gesucht und in .center geschrieben, das ist der
					frankfurter hauptbahnhof
				</li>
				<li>
					Dann wird eine union gemacht mit allen haltepunkten die es so geben kann im ukreis von 150
					metern und in stops geschrieben
				</li>
				<li>
					dann wird nach allen relationens gesucht die einen der stops beinhalten und einen der tags
					haben
				</li>
				<li>
					Weil out:csv benutzt wird wird das ergebniss als csv ausgegeben und man kann nur out
					nutzen
				</li>
			</ul>
		</Notes>
	</Slide>
</Slide>
