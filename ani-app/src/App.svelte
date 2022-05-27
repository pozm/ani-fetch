<script lang="ts">
	import './assets/index.css'
	import { invoke } from "@tauri-apps/api";
	import type {Torrent} from '../types/torrents';

	let torrents: Promise<Torrent[]> = invoke("get_torrents");
	let show_search = ""
	let show_ep;

	let show_result;

	function search() {
		invoke("search_torrent", {
			series: show_search,
			ep: show_ep
		}).then(result => {
			show_result = result;
		});
	}
</script>

<main class="text-gray-400 p-3" >
	<h1 class="text-gray-200 text-xl">Torrents</h1>
	{#await torrents}
	waiting for torrents....
	{:then TorPog}
		{#each TorPog as torrent}
			{torrent}
		{/each}
	{/await}
	<h1 class="text-gray-200 text-xl">Search for torrent</h1>
	<div class="flex flex-row space-x-2 items-start justify-start max-w-lg pt-3">

		<input bind:value={show_search} id="show" name="show" class="flex-1 appearance-none w-full text-neutral-200 placeholder-slate-400 bg-neutral-800 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border border-gray-800" placeholder="Show name">
		<input bind:value={show_ep} type="number" id="show" name="show" class="flex-[.4] appearance-none w-full text-neutral-200 placeholder-slate-400 bg-neutral-800 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border border-gray-800" placeholder="Show episode">
	</div>
	<button class="py-2 flex-1 shadow-sm px-12 transition-colors hover:bg-pink-500 text-gray-100 bg-pink-400 mt-2 rounded-lg" on:click="{search}">Search</button>

	<h1 class="text-gray-200 text-xl">Results</h1>
	{#if show_result}
		<div class="bg-neutral-800 my-2 p-2 rounded-lg shadow-lg cursor-pointer flex flex-row justify-items-center">
			{show_result.name}
		</div>
	{/if}
</main>

<style  >
	body {
		background: rgb(24, 24, 27) !important;
	}
</style>