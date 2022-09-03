<script lang="ts">
    import Card from "@smui/card";
    import List, { Item, Label, Meta } from "@smui/list";
    import IconButton from "@smui/icon-button";
    import EditDialog from "./EditDialog.svelte";
    import { DieVariant, dice } from "../common";

    let items: boolean[] = $dice.map(() => false);
</script>

<Card>
    {#if $dice.length !== 0}
        <List>
            {#each $dice as die, i}
                <Item>
                    <Label>
                        {#if die.variant === DieVariant.Simple}
                            {die.sides}-seitiger Spielwürfel
                        {:else if die.variant === DieVariant.Weighted}
                            {die.sides.length}-seitiger Gezinkter Würfel
                        {/if}
                    </Label>
                    <Meta>
                        <IconButton
                            on:click={() => (items[i] = true)}
                            class="material-icons">edit</IconButton
                        >
                        <IconButton
                            on:click={() =>
                                ($dice = $dice.filter((_, idx) => idx !== i))}
                            class="material-icons">delete</IconButton
                        >
                    </Meta>
                </Item>
                <EditDialog bind:die bind:open={items[i]} />
            {/each}
        </List>
    {:else}
        <span class="empty-indicator"> Gerade gibt es keine Würfel 🎲 </span>
    {/if}
</Card>

<style>
    .empty-indicator {
        padding: 0.5em;
    }
</style>
