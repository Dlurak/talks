<script lang="ts">
	export let src: string;
	export let name: string;
</script>

<div class="inline-flex flex-col h-[100%] justify-between items-center">
	<img {src} alt={name} class="object-contain h-[90%] inline-block" />
	<h4 class="inline">{name}</h4>
</div>
