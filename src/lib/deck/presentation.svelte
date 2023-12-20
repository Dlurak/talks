<script lang="ts">
	import Screenshot from '$lib/presentation/screenshot.svelte';
	import Slide from './slide.svelte';
	import QRCode from 'svelte-qrcode';
</script>

<Slide>
	<h2>Was ist Dlool?</h2>
</Slide>

<Slide>
	<ul>
		<li>Dlool ist ein kolloborativer Schulplaner</li>
		<li>Normale Hausaufgaben</li>
		<li>Kalendar für Klausuren</li>
		<li><i>Private</i> und öffentliche Notitzen</li>
	</ul>
</Slide>

<Slide>
	<h2>Warum kein normales Hausaufgabenheft?</h2>
</Slide>

<Slide>
	<ul>
		<li>Viele Schüler arbeiten digital, nutzen jedoch dennoch ein analoges Hausaufgabenheft</li>
		<li>Nach einem Jahr ist es nutzlos</li>
		<li>Wenn man es nicht dabei hat/verliert kann man nicht auf die Daten zugreifen</li>
		<li>Es gibt kein vorgegebenes System ⇒ die Daten werden unstrukturiert</li>
	</ul>
</Slide>

<Slide>
	<h2>Warum ausgerechnet Dlool?</h2>
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
		<li>Dlool ist Open-Source, hat keine Tracker und legt einen Schwerpunkt auf Open-data</li>
		<li>Dlool ist kollaborativ</li>
		<li>Viele kleine Details</li>
	</ul>
</Slide>

<Slide>
	<h2>Was bedeutet <i>kolloborativ</i> überhaupt?</h2>
</Slide>

<Slide>
	<h2>Wie nutzt man Dlool?</h2>
</Slide>

<Slide>
	<div class="flex h-[500px] items-center justify-evenly">
		<Screenshot name="register" />
		<Screenshot name="login" />
		<Screenshot name="homework" />
	</div>
</Slide>

<Slide>
	<div class="flex items-center justify-center">
		<QRCode size={500} value="https://dlool.vercel.app" />
	</div>
	<p class="font-mono">https://dlool.vercel.app</p>
</Slide>
