<script lang="ts">
    import Dialog, { Title, Content, Actions } from "@smui/dialog";
    import IconButton from "@smui/icon-button";
    import List, { Item } from "@smui/list";
    import Select, { Option } from "@smui/select";
    import Textfield from "@smui/textfield";

    import { dice, Die, DieVariant, Side } from "../common";

    export let die: Die, open: boolean = false;

    let d: Die = { ...die };

    let variant: DieVariant = die.variant;
    $: d.variant = variant;
    $: d.sides = variant === DieVariant.Simple ? 6 : [{ dots: 1, weight: 1 }];

    let rolls: number = die.rolls;
    $: rolls = Math.min(255, Math.max(1, rolls));
    $: d.rolls = rolls;

    const create = () =>
        (d.sides = [...(d.sides as Side[]), { dots: 1, weight: 1 }]);
    const update = (draft: Die, original: Die) =>
        ($dice[$dice.findIndex((e) => e === original)] = draft);
    const remove = (side: Side) =>
        (d.sides = (d.sides as Side[]).filter((e) => e !== side));
    const done = () => !(open = false) && update(d, die);
</script>

<Dialog bind:open>
    <Title>
        {#if d.variant === DieVariant.Simple}
            {d.sides}-seitiger Spielwürfel
        {:else if d.variant === DieVariant.Weighted}
            {d.sides.length}-seitiger Gezinkter Würfel
        {/if}
    </Title>
    <Content>
        <div class="edit-form">
            <Select required bind:value={variant}>
                <Option value={DieVariant.Simple}>Einfach</Option>
                <Option value={DieVariant.Weighted}>Gezinkt</Option>
            </Select>

            <Textfield bind:value={rolls} label="Würfe" type="number" />

            {#if d.variant === DieVariant.Simple}
                <Textfield bind:value={d.sides} label="Seiten" type="number" />
            {:else if d.variant === DieVariant.Weighted}
                <List>
                    {#each d.sides as side}
                        <Item class="edit-w_item">
                            <Textfield
                                label="Wert"
                                type="number"
                                bind:value={side.dots}
                            />
                            <Textfield
                                label="Gewicht"
                                type="number"
                                bind:value={side.weight}
                            />
                            <IconButton
                                class="material-icons"
                                size="mini"
                                on:click={() => remove(side)}
                            >
                                delete
                            </IconButton>
                        </Item>
                    {/each}
                </List>
                <IconButton class="material-icons" on:click={create}>
                    add
                </IconButton>
            {:else}
                <p>Kein Würfeltyp festgelegt</p>
            {/if}
        </div>
    </Content>
    <Actions>
        <IconButton on:click={done} class="material-icons">done</IconButton>
    </Actions>
</Dialog>

<style>
    .edit-form {
        display: grid;
        grid-template-rows: 1fr, 1fr, fit-content;
    }

    :global(.edit-w_item) {
        display: flex;
        flex-wrap: nowrap;
        justify-content: space-between;
        column-gap: 10px;
    }
</style>
