<script lang="ts">
	import Screenshot from '$lib/presentation/screenshot.svelte';
	import Notes from './notes.svelte';
	import Slide from './slide.svelte';
	import QRCode from 'svelte-qrcode';
</script>

<Slide>
	<h2>Was ist Dlool?</h2>

	<Notes>
		<ul>
			<li>Open source webapp Dlool vorstellen</li>
			<li>Im Juni dieses Jahres begonnen</li>
		</ul>

		<p class="text-sm">
			Hi ich möchte euch die Open-Source webapp Dlool vorstellen, die ich seit etwa einem halben
			Jahr entwickelt habe.
		</p>
	</Notes>
</Slide>

<Slide>
	<ul>
		<li>Kolloborativer Schulplaner</li>
		<li>Normale Hausaufgaben</li>
		<li>Kalendar für Klausuren</li>
		<li><i>Private</i> und öffentliche Notitzen</li>
	</ul>

	<Notes>
		<p>
			Dlool ist ein kolloborativer Schulplaner, ausgelegt darauf von Schülern als Klasse gemeinsam
			genutzt zu werden. Er ist cross plattform und unterstützt
		</p>
		<ul>
			<li>ha</li>
			<li>Kalender für vok tests, klausuren und Ähnliches</li>
			<li>
				Notitzen, jedoch sind private Notitzen nicht verschlüsselt sondern die meisten ahben bloß
				keinen Zugriff
			</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Warum kein normales Hausaufgabenheft?</h2>
</Slide>

<Slide>
	<ul>
		<li>Schüler arbeiten digital, nutzen aber dennoch analoge Schulplaner</li>
		<li>Nur ein Jahr nutzbar</li>
		<li>Verlust führt zu Datenverlust</li>
		<li>Kein vorgegebenes System ⇒ unstrukturierte Daten</li>
	</ul>

	<Notes>
		<ul>
			<li>Auch digitale arbeiten häufig analg, unnötig komplex</li>
			<li>nach einem jahr ist es nutzlos</li>
			<li>man muss sich dann ein neues kaufen was geld kostet</li>
			<li>wenn man es verliert dann sind die daten weg</li>
			<li>Wenn man es nicht dabei hat kann man nichts aufschreiben/die daten auslesen</li>
			<li>
				wenn man krank ist oder allgemein fehlt dann hat man keine daten und muss sich mühevoll
				erkunden
			</li>
			<li>Es gibt nicht wirklich ein organisiertes system</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Warum ausgerechnet Dlool?</h2>

	<Notes>Doch warum genau Dlool und keine andere app?</Notes>
</Slide>

<Slide>
	<ul>
		<li>
			Dlool bietet alles Nötige
			<ul>
				<li>Hausaufgaben</li>
				<li>Notitzen</li>
				<li>Kalender</li>
			</ul>
		</li>
		<li>Dlool ist Open-Source, hat keine Tracker und hat eine freie API</li>
		<li>Dlool ist kollaborativ</li>
		<li>Viele kleine Details</li>
	</ul>

	<Notes>
		Dlool hat alles was nötig ist <br />
		<ul>
			<li>Open source</li>
			<li>Keine tracker</li>
			<li>Dokumentierte freie API die teilweise JSON, ical, todotxt, csv und xml</li>
			<li>Man kriegt die daten leicht raus also ist ein Umstieg von dlool weg leicht</li>
		</ul>
		Dlool ist kollaborativ Viele kleine Details und Einstellungen
	</Notes>
</Slide>

<Slide>
	<h2>Was bedeutet <i>kolloborativ</i> überhaupt?</h2>

	<Notes>
		<ul>
			<li>Normalerweise schreiben alle Schüler das selbe auf</li>
			<li>Am Ende wird es dennoch geteilt wenn z.b. jemand krank ist.</li>
			<li>Dlool macht es automatisch</li>
			<li>Ein schüler lädt die daten in Dlool hoch und alle können es lesen</li>
			<li>Viele müssen gar nicht mehr aufschreiben was die ha sind</li>
		</ul>
	</Notes>
</Slide>

<Slide>
	<h2>Wie nutzt man Dlool?</h2>
</Slide>

<Slide>
	<div class="flex h-[500px] items-center justify-evenly">
		<Screenshot name="register" />
	</div>

	<Notes>
		<p>Man kann sich registrieren, ist nur nötig, wenn man Daten hochladen will</p>
	</Notes>
</Slide>

<Slide>
	<div class="flex h-[500px] items-center justify-evenly">
		<Screenshot name="login" />
	</div>

	<Notes>
		<p>Nach Login kann man daten hinzufügen</p>
	</Notes>
</Slide>

<Slide>
	<div class="flex h-[500px] items-center justify-evenly">
		<Screenshot name="homework" />
	</div>

	<Notes>
		<p>Man kann hausaufgaben simpel hinzufügen</p>
	</Notes>
</Slide>

<Slide>
	<div class="flex items-center justify-center">
		<QRCode size={500} value="https://www.dlool.me" />
	</div>
	<p class="font-mono">https://www.dlool.me</p>

	<Notes>
		Mit Dlool kann man zwar wissen was man machen muss, doch hausaufgaben löst Dlool leider nicht
		für einen, die muss man selber machen
	</Notes>
</Slide>
