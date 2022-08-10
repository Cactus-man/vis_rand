<script lang="ts">
    import DiePanel from "./dice/DiePanel.svelte";

    import Accordion from "@smui-extra/accordion";
    import Card, { Content, Actions } from "@smui/card";
    import IconButton, { Icon } from "@smui/icon-button";
    import Textfield from "@smui/textfield";

    import { invoke } from "@tauri-apps/api";

    let rolls = 100;
    let dice: Panel[] = [
        { die: { variant: "simple", sides: 6 }, amount: 1 },
        {
            die: {
                variant: "weighted",
                sides: [
                    [1, 4],
                    [8, 1],
                    [6, 3],
                ],
            },
            amount: 12,
        },
        { die: { variant: "simple", sides: 8 }, amount: 4 },
    ];

    export let results: RollResult[];

    $: rollable = dice.length === 0;

    const remove = (d: Die) => (dice = dice.filter((p) => p.die !== d));

    const add = (die: Die, amount: number) =>
        (dice = [...dice, { die, amount }]);

    const roll = () => {
        invoke("roll", { rolls, dice }).then(
            (res: RollResult[]) => (results = res)
        );
        console.log(`${rolls}, ${dice}`);
    };
</script>

<Card>
    <div><h2 class="mdc-typography--headline6">Würfel</h2></div>
    <Actions class="dice-actions">
        <Textfield bind:value={rolls} type="number" suffix="Würfe" />
        <IconButton bind:disabled={rollable} on:click={roll}>
            <Icon class="material-icons">play_arrow</Icon>
        </IconButton>
        <IconButton on:click={() => add({ sides: 6, variant: "simple" }, 1)}>
            <Icon class="material-icons">add</Icon>
        </IconButton>
    </Actions>
    <Content>
        {#if dice.length !== 0}
            <Accordion multiple>
                {#each dice as { die, amount }}
                    <DiePanel bind:die bind:amount {remove} />
                {/each}
            </Accordion>
        {:else}
            <span class="empty-indicator">Gerade gibt es keine Würfel</span>
        {/if}
    </Content>
</Card>

<style>
    div {
        padding: 1rem;
    }

    h2.mdc-typography--headline6 {
        margin: 0;
    }

    .empty-indicator {
        padding: 0.5em;
    }

    .empty-indicator::after {
        content: "🎲";
        padding: 0.5em;
    }

    :global(.dice-actions) {
        align-self: center;
        gap: 5px;
    }
</style>
