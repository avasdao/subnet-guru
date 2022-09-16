<template>
    <HeaderView
        @closePanel="closePanel"
        title="My Gurus"
     />

    <main class="mt-10 text-center">
        <svg class="mx-auto h-16 w-16 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>

        <h3 class="mt-2 text-2xl font-medium text-gray-900">
            You have NO saved Gurus
        </h3>

        <p class="mt-1 text-md text-gray-500">
            Get started by deploying a NEW Guru
        </p>

        <div class="mt-6">
            <button
                type="button"
                class="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-lg font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >
                <!-- Heroicon name: mini/plus -->
                <svg class="-ml-1 mr-2 h-8 w-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
                </svg>

                Add a New Guru
            </button>
        </div>
    </main>
</template>

<script>
/* Import components. */
import HeaderView from '@/components/HeaderView'

export default {
    components: {
        HeaderView,
    },
    data: () => ({
        //
    }),
    computed: {
        //
    },
    methods: {
        closePanel() {
            this.$emit('closePanel')
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
