<script lang="ts">
    import Card from "@smui/card";
    import DataTable, { Head, Row, Body, Cell } from "@smui/data-table";

    import * as d3 from "d3";
    import { data, dice } from "../common";

    export let rolls: number;
    const formatted = d3.format(".5f");
</script>

<Card>
    {#if $dice.length >= 0}
        <DataTable sortable style="width: 100%">
            <Head>
                <Row>
                    <Cell numeric>Augen</Cell>
                    <Cell numeric>Anzahl</Cell>
                    <Cell numeric>Rel. Häufigkeit</Cell>
                </Row>
            </Head>
            <!-- <LinearProgress closed={!$running} indeterminate slot="progress" /> -->
            <Body>
                {#each $data as { dots, occurrences }}
                    <Row>
                        <Cell numeric>{dots}</Cell>
                        <Cell numeric>{occurrences}</Cell>
                        <Cell numeric>{formatted(occurrences / rolls)}</Cell>
                    </Row>
                {/each}
            </Body>
        </DataTable>
    {:else}
        <p>Es gibt noch keine Daten!</p>
    {/if}
</Card>
