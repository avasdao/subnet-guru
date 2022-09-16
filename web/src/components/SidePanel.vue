<template>
    <div class="fixed inset-0 bg-gray-800 opacity-80"></div>

    <main class="fixed inset-0 overflow-hidden">
        <!-- This is an overlay for the RootView -->
        <div
            @click="closePanel"
            class="absolute inset-0 overflow-hidden"
        />

        <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16">
            <!--
      Slide-over panel, show/hide based on slide-over state.

      Entering: "transform transition ease-in-out duration-500 sm:duration-700"
        From: "translate-x-full"
        To: "translate-x-0"
      Leaving: "transform transition ease-in-out duration-500 sm:duration-700"
        From: "translate-x-0"
        To: "translate-x-full"
    -->
            <div class="pointer-events-auto w-screen max-w-2xl">
                <div class="flex h-full flex-col overflow-y-scroll bg-white shadow-xl">
                    <router-view
                        @closePanel="closePanel"
                    />
                </div>
            </div>
        </div>
    </main>
</template>

<script>
export default {
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
