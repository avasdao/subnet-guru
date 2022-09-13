<template>
    <main class="bg-gray-50 pr-4 sm:pr-6 lg:flex-shrink-0 lg:border-l lg:border-gray-200 lg:pr-8 xl:pr-0">
        <div class="pl-6 lg:w-80">
            <div class="pt-6 pb-2">
                <h2 class="text-sm font-semibold">
                    Activity
                </h2>
            </div>

            <div>
                <ul role="list" class="divide-y divide-gray-200">

                    <li v-for="event of events" :key="event.id" class="py-4">
                        <div class="flex space-x-3">
                            <img class="h-6 w-6 rounded-full" src="https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=256&h=256&q=80" alt="" />
                            <div class="flex-1 space-y-1">
                                <div class="flex items-center justify-between">
                                    <h3 class="text-sm font-medium">
                                        {{event.source}}
                                    </h3>

                                    <p class="text-sm text-gray-500">
                                        {{event.createdAt}}
                                    </p>
                                </div>

                                <p class="text-sm text-gray-500">
                                    {{event.description}}
                                </p>
                            </div>
                        </div>
                    </li>

                    <!-- More items... -->
                </ul>

                <div class="border-t border-gray-200 py-4 text-sm">
                    <a href="javascript://" class="font-semibold text-indigo-600 hover:text-indigo-900">
                        View all activity
                        <span aria-hidden="true"> &rarr;</span>
                    </a>
                </div>
            </div>
        </div>
    </main>
</template>

<script>
export default {
    data: () => ({
        events: null
    }),
    computed: {
        //
    },
    methods: {
        //
    },
    created: function () {
        this.events = []

        this.events.push({
            id: '005da2d8-e211-4b89-bc83-d47a1550aac2',
            source: `You`,
            description: `Deployed Workcation (2d89f0c8 in master) to production`,
            createdAt: '1h'
        })

        this.events.push({
            id: 'c7655865-7f55-44b4-b3cb-63ff5780b30c',
            source: `You`,
            description: `Deployed KiteTail (249df660 in master) to staging`,
            createdAt: '3h'
        })

        this.events.push({
            id: 'b287837e-6504-4584-a6f4-bde5a1a57eb2',
            source: `You`,
            description: `Deployed Workcation (11464223 in master) to staging`,
            createdAt: '12h'
        })

        this.events.push({
            id: '7e17bc2d-6dd9-4e25-a8cc-57622a4cafb3',
            source: `You`,
            description: `Deployed Easywire (dad28e95 in master) to production`,
            createdAt: '2d'
        })

        this.events.push({
            id: 'b19da3f0-df48-4ed6-84cb-9ef4efd8bded',
            source: `You`,
            description: `Deployed Easywire (624bc94c in master) to production`,
            createdAt: '5d'
        })

        this.events.push({
            id: '8f6c5a1f-a6d9-4097-b7e5-f4980af99d8f',
            source: `You`,
            description: `Deployed Workcation (e111f80e in master) to production`,
            createdAt: '1w'
        })

        this.events.push({
            id: 'd6bbd5ab-1c3d-4900-813b-de98484ef2c3',
            source: `You`,
            description: `Deployed Resumaid (5e136005 in master) to staging`,
            createdAt: '1w'
        })

        this.events.push({
            id: 'cc808831-dd72-424e-934a-fc61baed62b8',
            source: `You`,
            description: `Deployed KiteTail (5c1fd07f in master) to production`,
            createdAt: '2w'
        })
    },
    mounted: function () {
        //
    },
}
</script>
