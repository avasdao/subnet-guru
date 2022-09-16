<template>
    <main class="relative z-10" aria-labelledby="slide-over-title" role="dialog" aria-modal="true">

        <BaseView />

        <SidePanel v-if="showPanel" />
    </main>
</template>

<script>
/* Import home view. */
import BaseView from './views/BaseView'

/* Import side panel. */
import SidePanel from '@/components/SidePanel'

export default {
    components: {
        BaseView,
        SidePanel,
    },
    data: () => ({
        showPanel: null,
    }),
    computed: {
        //
    },
    methods: {
        //
    },
    created: function () {
        this.showPanel = false
    },
    mounted: function () {
        //
    },
}
</script>

<style>
/*  */
</style>
