<script lang="ts">
    import Tab, { Label } from "@smui/tab";
    import TabBar from "@smui/tab-bar";
    import PieChart from "./PieChart.svelte";

    import Tabular from "./Tabular.svelte";

    export let rolls: number;

    const tabs = [
        { k: 0, name: "Tabelle" },
        { k: 1, name: "Kreisdiagram" },
    ];

    let active = tabs[0];
</script>

<TabBar class="solo-container" {tabs} let:tab bind:active key={(tab) => tab.k}>
    <Tab {tab}>
        <Label>{tab.name}</Label>
    </Tab>
</TabBar>

{#if active.k === 0}
    <Tabular {rolls} />
{:else if active.k === 1}
    <PieChart />
{/if}

<style>
    :global(.solo-container) {
        padding: 36px 18px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
