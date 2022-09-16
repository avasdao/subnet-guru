<template>
    <!-- Account profile -->
    <main class="bg-white xl:w-64 xl:flex-shrink-0 xl:border-r xl:border-gray-200">
        <div class="py-6 pl-4 pr-6 sm:pl-6 lg:pl-8 xl:pl-0">
            <div class="flex items-center justify-between">
                <div class="flex-1 space-y-8">
                    <div class="space-y-8 sm:flex sm:items-center sm:justify-between sm:space-y-0 xl:block xl:space-y-8">
                        <!-- Profile -->
                        <div class="flex items-center space-x-3">
                            <div class="h-12 w-12 flex-shrink-0">
                                <img class="h-12 w-12 rounded-full" src="https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=256&h=256&q=80" alt="" />
                            </div>

                            <div class="space-y-1">
                                <div class="text-sm font-medium text-gray-900">
                                    Ava Nakamoto
                                </div>

                                <a href="javascript://" class="group flex items-center space-x-2.5">
                                    <svg class="h-5 w-5 text-gray-400 group-hover:text-gray-500" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>

                                    <span class="text-sm font-medium text-gray-500 group-hover:text-gray-900">
                                        anakamoto
                                    </span>
                                </a>
                            </div>
                        </div>

                        <!-- Action buttons -->
                        <div class="flex flex-col sm:flex-row xl:flex-col space-y-4">

                            <button
                                @click="toggleGurus"
                                type="button"
                                class="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-xl font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 xl:w-full"
                            >
                                My Gurus
                            </button>

                            <button
                                @click="toggleSubnets"
                                type="button"
                                class="inline-flex items-center justify-center rounded-md border border-transparent bg-yellow-400 px-4 py-2 text-xl font-medium text-white shadow-sm hover:bg-yellow-500 focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 xl:w-full"
                            >
                                My Subnets
                            </button>

                            <hr class="mx-10 border-rose-500" />

                            <button
                                @click="toggleToolbox"
                                type="button"
                                class="inline-flex items-center justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:ml-3 xl:ml-0 xl:w-full"
                            >
                                Open Toolbox
                            </button>

                            <button
                                @click="toggleReports"
                                type="button"
                                class="inline-flex items-center justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:ml-3 xl:ml-0 xl:w-full"
                            >
                                View Reports
                            </button>

                        </div>
                    </div>

                    <!-- Meta info -->
                    <div class="pl-3 flex flex-col space-y-6 sm:flex-row sm:space-y-0 sm:space-x-8 xl:flex-col xl:space-x-0 xl:space-y-6">

                        <div class="flex items-center space-x-2">
                            <svg class="w-6 h-6 text-rose-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>

                            <span class="text-sm font-medium text-gray-500">
                                <span class="text-rose-700 text-xl">{{gurus.length}}x</span>
                                Guru Connections
                            </span>
                        </div>

                        <div class="flex items-center space-x-2">
                            <svg class="h-6 w-6 text-green-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path
                                    fill-rule="evenodd"
                                    d="M16.403 12.652a3 3 0 000-5.304 3 3 0 00-3.75-3.751 3 3 0 00-5.305 0 3 3 0 00-3.751 3.75 3 3 0 000 5.305 3 3 0 003.75 3.751 3 3 0 005.305 0 3 3 0 003.751-3.75zm-2.546-4.46a.75.75 0 00-1.214-.883l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>

                            <span class="text-sm font-medium text-gray-500">
                                Registered Validator
                            </span>
                        </div>

                        <div class="flex items-center space-x-2">
                            <svg class="h-6 w-6 text-green-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path
                                    fill-rule="evenodd"
                                    d="M16.403 12.652a3 3 0 000-5.304 3 3 0 00-3.75-3.751 3 3 0 00-5.305 0 3 3 0 00-3.751 3.75 3 3 0 000 5.305 3 3 0 003.75 3.751 3 3 0 005.305 0 3 3 0 003.751-3.75zm-2.546-4.46a.75.75 0 00-1.214-.883l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>

                            <span class="text-sm font-medium text-gray-500">
                                Registered Delegator
                            </span>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    </main>
</template>

<script>
export default {
    props: {
        gurus: Object,
    },
    data: () => ({
        //
    }),
    computed: {
        //
    },
    methods: {
        toggleGurus() {
            /* Open the (side) panel. */
            this.$emit('openPanel')

            /* Load (my) gurus. */
            this.$router.push('/gurus')
        },

        toggleSubnets() {
            alert('subnets')
        },

        toggleToolbox() {
            alert('toolbox')
        },

        toggleReports() {
            alert('reports')
        },

    },
    created: function () {
        //
    },
    mounted: function () {
        //
    },
}
</script>
