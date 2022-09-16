<template>
    <HeaderView
        @closePanel="closePanel"
        title="Blank"
     />

    <main>
        <h1>Blank</h1>

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
