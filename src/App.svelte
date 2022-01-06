<script>
	// icons
	import Icon_search from './components/icons/Search.svelte';
	import Icon_package from './components/icons/Package.svelte';
	import Modal_confirm from './components/modals/Confirm.svelte';

	// init variable
	let search = '';

	async function getPackages() {
		const packages = await __TAURI__.invoke('get_packages');
		return packages;
	}
</script>

<div class="flex">
	<!-- nav -->
	<nav
		class="w-[4rem] min-h-screen bg-gray-700 flex items-center flex-col z-30 p-2 space-y-2">
		<button
			class="grid rounded-full w-11 place-content-center aspect-square bg-violet-600">
			<Icon_package classNames="w-6 h-6 text-gray-100" />
		</button>
	</nav>

	<!-- content -->
	<div class="w-full p-3 space-y-4">
		<header
			class="w-full flex justify-between items-center rounded bg-gray-700 p-2">
			<div class="text-lg font-bold">
				{#await getPackages()}
					loading
				{:then data}
					Packages ({data.length})
				{/await}
			</div>

			<div
				class="border group focus-within:border-gray-400 border-gray-500 p-1 flex items-center space-x-2 rounded">
				<Icon_search />
				<input
					type="text"
					placeholder="package name..."
					bind:value={search}
					class="w-full bg-transparent focus:outline-none placeholder:italic" />
			</div>
		</header>

		<div class="space-y-3 h-[85vh] lg:h-[90vh] overflow-y-auto">
			{#await getPackages()}
				loading...
			{:then packages}
				{#each packages.filter((item) => item
						.toLocaleLowerCase()
						.includes(search.toLocaleLowerCase())) as filteredPackage}
					<div
						class="rounded bg-gray-700 p-2 flex item-center justify-between">
						<p>{filteredPackage}</p>
						<Modal_confirm packageName={filteredPackage} />
					</div>
				{/each}
			{:catch error}
				<p>An error occurred!</p>
			{/await}
		</div>
	</div>
</div>
