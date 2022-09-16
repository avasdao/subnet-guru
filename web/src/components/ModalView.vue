<template>
    <main class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <!--
        Background backdrop, show/hide based on modal state.

        Entering: "ease-out duration-300"
          From: "opacity-0"
          To: "opacity-100"
        Leaving: "ease-in duration-200"
          From: "opacity-100"
          To: "opacity-0"
      -->
        <div class="fixed inset-0 bg-gray-800 bg-opacity-80 transition-opacity"></div>

        <div class="fixed inset-0 z-10 overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                <!--
            Modal panel, show/hide based on modal state.

            Entering: "ease-out duration-300"
              From: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              To: "opacity-100 translate-y-0 sm:scale-100"
            Leaving: "ease-in duration-200"
              From: "opacity-100 translate-y-0 sm:scale-100"
              To: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
          -->
                <div class="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                    <div class="sm:flex sm:items-start">
                        <div class="mx-auto flex h-16 w-16 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0">
                            <img :src="require('../assets/avax-logo.png')" />
                        </div>

                        <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                            <h3 class="text-4xl font-medium leading-6 text-red-600" id="modal-title">
                                Avalanche Hacks
                            </h3>

                            <h3 class="mt-5 text-3xl font-medium leading-6 text-gray-500" id="modal-title">
                                Submission Demo
                            </h3>

                            <div class="mt-5">
                                <p class="text-base sm:text-lg text-gray-500">
                                    Thanks for checking out this hackathon submission.
                                    This is a very limited <strong>DEMO</strong> of the <strong>SUBNET GURU</strong> application.
                                </p>

                                <p class="mt-3 text-base sm:text-lg text-gray-500">
                                    Your mileage may vary, so please be patient as we continue to <strong>BU!DL</strong> 💪
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="mt-5 sm:ml-10 sm:flex sm:pl-4">
                        <button
                            @click="this.$emit('closeModal')"
                            type="button"
                            class="inline-flex w-full justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-xl font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 sm:w-auto"
                        >
                            Enter Demo
                        </button>

                        <a
                            href="https://docs.subnet.guru" target="_blank"
                            class="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-xl font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto"
                        >
                            Learn more
                        </a>
                    </div>
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
