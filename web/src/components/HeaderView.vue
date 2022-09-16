<template>
    <header class="px-4 py-6 sm:px-6 border-b border-indigo-500 shadow">
        <div class="flex items-start justify-between">
            <h2 class="text-xl font-medium text-gray-700" id="slide-over-title">
                {{title}}
            </h2>

            <div class="ml-3 flex h-7 items-center">
                <button type="button" class="rounded-md bg-white text-gray-400 hover:text-gray-500 focus:ring-2 focus:ring-indigo-500">
                    <span class="sr-only">Close panel</span>
                    <!-- Heroicon name: outline/x-mark -->
                    <svg class="h-8 w-8" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>
    </header>
</template>

<script>
export default {
    props: {
        title: String,
    },
    data: () => ({
        //
    }),
    computed: {
        //
    },
    methods: {
        //
    },
    created: function () {
        //
    },
    mounted: function () {
        //
    },
}
</script>
