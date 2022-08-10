<script lang="ts">
    import { Panel, Header, Content } from "@smui-extra/accordion";
    import IconButton, { Icon } from "@smui/icon-button";
    import Select, { Option } from "@smui/select";
    import Textfield from "@smui/textfield";

    export let amount: number = 1,
        die: Die,
        remove: (d: Die) => void;
    let open = false;

    $: label =
        die.variant === "simple"
            ? `${die.sides}-seitiger Spielwürfel`
            : "Gezinkter Würfel";

    let selected: "simple" | "weighted" = die.variant;
    $: if (selected === "simple") {
        die = { variant: "simple", sides: 6 };
    } else if (selected === "weighted") {
        die = { variant: "weighted", sides: [[1, 1]] };
    }

    import Simple from "./Simple.svelte";
    import Weighted from "./Weighted.svelte";
</script>

<Panel bind:open nonInteractive>
    <Header>
        {amount} &#215 {label}
        <div slot="icon">
            <IconButton on:click={() => remove(die)}>
                <Icon class="material-icons">delete</Icon>
            </IconButton>
            <IconButton toggle bind:pressed={open}>
                <Icon class="material-icons" on>expand_less</Icon>
                <Icon class="material-icons">expand_more</Icon>
            </IconButton>
        </div>
    </Header>
    <Content>
        <div />
        <div class="die-config">
            <Textfield bind:value={amount} label="Würfe" type="number" />
            <Select bind:value={selected} label="Würfelart">
                <Option value="simple">Einfach</Option>
                <Option value="weighted">Gezinkt</Option>
            </Select>
            {#if die.variant === "simple"}
                <Simple bind:sides={die.sides} />
            {:else if die.variant === "weighted"}
                <Weighted bind:sides={die.sides} />
            {/if}
        </div>
        <!-- die modifiers & modal dialog for editing -->
    </Content>
</Panel>

<style>
    .die-config {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        gap: 1em;
    }
</style>
