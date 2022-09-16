<template>
    <HeaderView
        @closePanel="closePanel"
        title="Add a Guru"
     />

    <main class="mt-10">
        <section>
            <h1>Let's add a new Guru</h1>
        </section>

        <p>
            Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
        </p>
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
