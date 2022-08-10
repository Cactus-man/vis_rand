<script lang="ts">
    import Textfield from "@smui/textfield";

    export let sides: number;
</script>

<Textfield bind:value={sides} label="Seiten" type="number" />
