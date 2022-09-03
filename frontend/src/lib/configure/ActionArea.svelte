<script lang="ts">
    import { Icon } from "@smui/common";
    import { Input } from "@smui/textfield";
    import Fab from "@smui/fab";
    import Paper from "@smui/paper";

    import { DEFAULT_DIE, dice } from "../common";

    export let total: number;
    $: total = Math.max(0, Math.min(5_000_000, total));

    export let roll: () => void;
    const create = () => dice.update((d) => [...d, { ...DEFAULT_DIE }]);
</script>

<div class="solo-container">
    <Fab on:click={create} color="primary" mini class="solo-fab">
        <Icon class="material-icons">add</Icon>
    </Fab>
    <Paper class="solo-paper" elevation={6}>
        <Icon class="material-icons">casino</Icon>
        <Input
            bind:value={total}
            type="number"
            placeholder="Wiederholungen"
            class="solo-input"
        />
    </Paper>
    <Fab
        on:click={roll}
        exited={total <= 0 || $dice.length === 0}
        color="primary"
        mini
        class="solo-fab"
    >
        <Icon class="material-icons">play_arrow</Icon>
    </Fab>
</div>

<style>
    .solo-container {
        padding: 36px 18px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    * :global(.solo-paper) {
        display: flex;
        align-items: center;
        flex-grow: 1;
        max-width: 600px;
        margin: 0 12px;
        padding: 0 12px;
        height: 48px;
    }

    * :global(.solo-paper > *) {
        display: inline-block;
        margin: 0 12px;
    }

    * :global(.solo-input) {
        flex-grow: 1;
        color: var(--mdc-theme-on-surface, #000);
    }

    * :global(.solo-input::placeholder) {
        color: var(--mdc-theme-on-surface, #000);
        opacity: 0.6;
    }

    * :global(.solo-fab) {
        flex-shrink: 0;
    }
</style>
