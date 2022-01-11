<script>
    import Trash from '../icons/Trash.svelte';
    import { fade } from 'svelte/transition';

    export let packageName;
    export let updateDisplayedPackages;
    export let deletePackage;

    let open = false;

    const openModal = () => {
        open = true;
    };

    const closeModal = () => {
        open = false;
    };
</script>

<button
    on:click={openModal}
    class="text-sm rounded px-1 bg-red- ring-1 ring-red-500 text-red-500 hover:text-gray-100 hover:bg-red-500 font-semibold hover:ring-0 mr-3"
    >Remove</button>

{#if open}
    <div
        transition:fade={{ duration: 150 }}
        class="fixed top-0 left-0 z-40 flex items-center justify-center w-screen h-screen bg-black/40">
        <div
            class="min-h-screen flex justify-center items-center text-gray-100">
            <!-- modal box-->
            <div class="bg-gray-800 rounded border border-gray-700">
                <div class="w-96 flex">
                    <div class="w-1/3 pt-6 flex justify-center">
                        <Trash
                            classNames={'w-16 h-16 bg-gray-700 p-3 rounded-full'} />
                    </div>
                    <div class="w-full pt-9 pr-4">
                        <h3 class="font-bold text-white">
                            Delete Confirmation?
                        </h3>
                        <p class="py-4 text-sm ">
                            This action will remove package:<span
                                class="block font-bold">{packageName}</span>
                        </p>
                    </div>
                </div>

                <div class="p-4 flex space-x-4">
                    <button
                        on:click={closeModal}
                        class="w-1/2 px-4 py-3 text-center bg-gray-800 hover:bg-gray-700 font-bold rounded text-sm"
                        >Cancel</button>
                    <button
                        on:click={closeModal}
                        on:click={deletePackage}
                        on:click={updateDisplayedPackages}
                        class="w-1/2 px-4 py-3 text-center bg-violet-600 rounded hover:bg-violet-700 hover:text-white font-bold text-sm">
                        Delete
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
