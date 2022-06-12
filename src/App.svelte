<script lang="ts">
	import './assets/index.css'
	import { invoke } from "@tauri-apps/api";
	import type {Torrent,matchedData} from '../types/torrents';

	let torrents: Promise<Torrent[]> = invoke("get_torrents");
	let show_search = ""
	let show_ep;
	let show_season;
	let search_many = false;

	let show_result : matchedData.RootObject[] = [];
	import refreshIco from './assets/svg/refresh.svg';


	function download_results() {
		let torrents_to_download = show_result.map(x => x.link);
		let first = show_result[0];
		invoke("download_torrents", {torrents:torrents_to_download,downloadTo:`D:\\pog\\anime\\${first.metadata.title}\\Season ${first.metadata.season ?? 1}`}).then(console.log);
	}

	function open_in_mpv(thing:string) {
		invoke("open_in_mpv", {path: thing});
	}

	function search() {
		invoke("search_torrent", {
			series: show_search,
			ep: show_ep,
			season: show_season,
			searchType:Number(search_many)
		}).then(result => {
			console.log(result);
			show_result = result as any;
		});
	}
</script>

<main class="text-gray-400 p-3" >
	<h1 class="text-gray-200 text-xl">Torrents</h1>
	<div>
	{#await torrents}

		waiting for torrents....
		{:then TorPog}
		{#each TorPog as torrent}
		<div on:click={open_in_mpv.bind(this,torrent.content_path)} class="bg-neutral-800 my-2 p-2 rounded-lg shadow-lg cursor-pointer flex flex-row justify-items-center" >{torrent.name}</div>
		{/each}
		{:catch e}
		unable to get torrents {e && e.length ? `: ${e}` : ""}
		{/await}
	</div>
	<button on:click={()=>torrents = invoke("get_torrents")} class="p-3 shadow-sm transition-colors hover:bg-pink-600 text-gray-100 bg-pink-500 disabled:bg-pink-600/50 mt-2 rounded-lg" >
			<svelte:component class="w-4 h-4" this={refreshIco}/>
	</button>
	<h1 class="text-gray-200 text-xl">Search for torrent</h1>
	<div class="flex flex-row space-x-2 items-start justify-start py-3">

		<input bind:value={show_search} id="show" name="show" class="flex-1 appearance-none w-full text-neutral-200 placeholder-slate-400 bg-neutral-800 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border border-gray-800" placeholder="Show name">
		<input bind:value={show_ep} type="number" id="showep" name="showep" class="flex-[.4] appearance-none w-full text-neutral-200 placeholder-slate-400 bg-neutral-800 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border border-gray-800" placeholder="Show episode">
		<input bind:value={show_season} type="number" id="showsea" name="showsea" class="flex-[.4] appearance-none w-full text-neutral-200 placeholder-slate-400 bg-neutral-800 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border border-gray-800" placeholder="Show season">
	</div>
	<div class="flex items-center" >
		<input bind:checked={search_many} type="checkbox" id="smany" name="smany" class="form-check-input appearance-none h-4 w-4 border border-gray-300 rounded-sm bg-white checked:bg-pink-600 text-pink-600 checked:ring-pink-600 focus:ring-pink-600 checked:border-pink-600 focus:outline-none transition duration-200 mt-1 align-top bg-no-repeat bg-center bg-contain float-left mr-2 cursor-pointer" />
		<label for="smany" class="text-gray-400 select-none" >
			Search for many?
		</label>
	</div>
	<!-- <br/> -->
	<button class="py-2 flex-1 shadow-sm px-12 transition-colors hover:bg-pink-500 text-gray-100 bg-pink-400 mt-2 rounded-lg" on:click="{search}">Search</button>

	<h1 class="text-gray-200 text-xl">Results</h1>
	{#if show_result.length}
		{#each show_result as res}
		{@const pog = `D:\\pog\\anime\\${res.metadata.title}\\Season ${res.metadata.season ?? 1}`}
			<div on:click={open_in_mpv.bind(this,res.link)} class="bg-neutral-800 my-2 p-2 rounded-lg shadow-lg cursor-pointer flex flex-row justify-items-center">
				{res.metadata.uploader} | {res.metadata.title} Season {res.metadata.season ?? 1} - {res.metadata.ep}
			</div>
		{/each}
		<button class="py-2 flex-1 shadow-sm px-12 transition-colors hover:bg-pink-500 text-gray-100 bg-pink-400 mt-2 rounded-lg" on:click="{download_results}">download all</button>
	{/if}
</main>

<style  >
	body {
		background: rgb(24, 24, 27) !important;
	}
</style>