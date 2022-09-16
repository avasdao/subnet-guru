<template>
    <HeaderView title="Dashboard" />

    <main class="divide-y divide-gray-200">

        <section class="p-5 bg-gray-700">
            <h1 class="ml-3 text-sm text-gray-200 uppercase">
                Application Statistics
            </h1>

            <dl class="mt-3 grid grid-cols-2 gap-5">
                <div class="relative overflow-hidden rounded-lg bg-white px-4 pt-5 pb-12 shadow sm:px-6 sm:pt-6">
                    <dt>
                        <div class="absolute rounded-md bg-indigo-500 p-3">
                            <!-- Heroicon name: outline/users -->
                            <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
                                />
                            </svg>
                        </div>

                        <p class="ml-16 truncate text-sm font-medium text-gray-500">
                            Total Active Gurus
                        </p>
                    </dt>

                    <dd class="ml-16 flex items-baseline pb-2">
                        <p class="text-2xl font-semibold text-gray-900">
                            71,897
                        </p>

                        <p class="ml-2 flex items-baseline text-sm font-semibold text-green-600">
                            <!-- Heroicon name: mini/arrow-up -->
                            <svg class="h-5 w-5 flex-shrink-0 self-center text-green-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M10 17a.75.75 0 01-.75-.75V5.612L5.29 9.77a.75.75 0 01-1.08-1.04l5.25-5.5a.75.75 0 011.08 0l5.25 5.5a.75.75 0 11-1.08 1.04l-3.96-4.158V16.25A.75.75 0 0110 17z" clip-rule="evenodd" />
                            </svg>
                            <span class="sr-only"> Increased by </span>
                            122
                        </p>

                        <div class="absolute inset-x-0 bottom-0 bg-gray-50 px-4 py-4 sm:px-6">
                            <div class="text-sm">
                                <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">
                                    View all Gurus<span class="sr-only"> Total active gurus stats</span></a>
                            </div>
                        </div>
                    </dd>
                </div>

                <div class="relative overflow-hidden rounded-lg bg-white px-4 pt-5 pb-12 shadow sm:px-6 sm:pt-6">
                    <dt>
                        <div class="absolute rounded-md bg-indigo-500 p-3">
                            <svg class="w-6 h-6 text-gray-50" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"></path></svg>
                        </div>

                        <p class="ml-16 truncate text-sm font-medium text-gray-500">
                            Total Active Subnets
                        </p>
                    </dt>

                    <dd class="ml-16 flex items-baseline pb-2">
                        <p class="text-2xl font-semibold text-gray-900">
                            24
                        </p>

                        <p class="ml-2 flex items-baseline text-sm font-semibold text-red-600">
                            <!-- Heroicon name: mini/arrow-down -->
                            <svg class="h-5 w-5 flex-shrink-0 self-center text-red-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M10 3a.75.75 0 01.75.75v10.638l3.96-4.158a.75.75 0 111.08 1.04l-5.25 5.5a.75.75 0 01-1.08 0l-5.25-5.5a.75.75 0 111.08-1.04l3.96 4.158V3.75A.75.75 0 0110 3z" clip-rule="evenodd" />
                            </svg>
                            <span class="sr-only"> Decreased by </span>
                            1
                        </p>

                        <div class="absolute inset-x-0 bottom-0 bg-gray-50 px-4 py-4 sm:px-6">
                            <div class="text-sm">
                                <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">
                                    View all Subnets<span class="sr-only"> Total active subnet stats</span>
                                </a>
                            </div>
                        </div>
                    </dd>
                </div>
            </dl>
        </section>

        <section class="pb-6">
            <div class="h-24 bg-indigo-700 sm:h-20 lg:h-28"></div>

            <div class="lg:-mt-15 -mt-12 flow-root px-4 sm:-mt-8 sm:flex sm:items-end sm:px-6">
                <div>
                    <div class="-m-1 flex">
                        <div class="inline-flex overflow-hidden rounded-lg border-4 border-white">
                            <img
                                class="h-24 w-24 flex-shrink-0 sm:h-40 sm:w-40 lg:h-48 lg:w-48"
                                src="https://images.unsplash.com/photo-1501031170107-cfd33f0cbdcc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=256&h=256&q=80"
                                alt=""
                            />
                        </div>
                    </div>
                </div>

                <div class="mt-6 sm:ml-6 sm:flex-1">
                    <div>
                        <div class="flex items-center">
                            <h3 class="text-xl font-bold text-gray-900 sm:text-2xl">
                                Ashley Porter
                            </h3>

                            <span class="ml-2.5 inline-block h-2 w-2 flex-shrink-0 rounded-full bg-green-400">
                                <span class="sr-only">Online</span>
                            </span>
                        </div>

                        <p class="text-sm text-gray-500">
                            @ashleyporter
                        </p>
                    </div>

                    <div class="mt-5 flex flex-wrap space-y-3 sm:space-y-0 sm:space-x-3">
                        <router-link
                            to="/add"
                            class="inline-flex w-full flex-shrink-0 items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:flex-1"
                        >
                            Add a Guru
                        </router-link>

                        <router-link
                            to="/gurus"
                            class="inline-flex w-full flex-1 items-center justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                        >
                            My Gurus
                        </router-link>

                        <div class="ml-3 inline-flex sm:ml-0">
                            <div class="relative inline-block text-left">
                                <button
                                    type="button"
                                    class="inline-flex items-center rounded-md border border-gray-300 bg-white p-2 text-sm font-medium text-gray-400 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                                    id="options-menu-button"
                                    aria-expanded="false"
                                    aria-haspopup="true"
                                >
                                    <span class="sr-only">Open options menu</span>
                                    <!-- Heroicon name: mini/ellipsis-vertical -->
                                    <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" />
                                    </svg>
                                </button>

                                <!--
        Dropdown panel, show/hide based on dropdown state.

        Entering: "transition ease-out duration-100"
          From: "transform opacity-0 scale-95"
          To: "transform opacity-100 scale-100"
        Leaving: "transition ease-in duration-75"
          From: "transform opacity-100 scale-100"
          To: "transform opacity-0 scale-95"
      -->
                                <div
                                    class="hidden absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                    role="menu"
                                    aria-orientation="vertical"
                                    aria-labelledby="options-menu-button"
                                    tabindex="-1"
                                >
                                    <div class="py-1" role="none">
                                        <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
                                        <a href="javascript://" class="text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="options-menu-item-0">
                                            View profile
                                        </a>

                                        <a href="javascript://" class="text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="options-menu-item-1">
                                            Copy profile link
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="px-4 py-5 sm:px-0 sm:py-0">
            <dl class="space-y-8 sm:space-y-0 sm:divide-y sm:divide-gray-200">
                <div class="sm:flex sm:px-6 sm:py-5">
                    <dt class="text-sm font-medium text-gray-500 sm:w-40 sm:flex-shrink-0 lg:w-48">
                        Bio
                    </dt>

                    <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0 sm:ml-6">
                        <p>
                            Enim feugiat ut ipsum, neque ut. Tristique mi id elementum praesent.
                            Gravida in tempus feugiat netus enim aliquet a, quam scelerisque.
                            Dictumst in convallis nec in bibendum aenean arcu.
                        </p>
                    </dd>
                </div>

                <div class="sm:flex sm:px-6 sm:py-5">
                    <dt class="text-sm font-medium text-gray-500 sm:w-40 sm:flex-shrink-0 lg:w-48">
                        Location
                    </dt>

                    <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0 sm:ml-6">
                        New York, NY, USA
                    </dd>
                </div>

                <div class="sm:flex sm:px-6 sm:py-5">
                    <dt class="text-sm font-medium text-gray-500 sm:w-40 sm:flex-shrink-0 lg:w-48">
                        Website
                    </dt>

                    <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0 sm:ml-6">
                        ashleyporter.com
                    </dd>
                </div>

                <div class="sm:flex sm:px-6 sm:py-5">
                    <dt class="text-sm font-medium text-gray-500 sm:w-40 sm:flex-shrink-0 lg:w-48">
                        Birthday
                    </dt>

                    <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0 sm:ml-6">
                        <time datetime="1982-06-23">
                            June 23, 1982
                        </time>
                    </dd>
                </div>
            </dl>
        </section>
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
