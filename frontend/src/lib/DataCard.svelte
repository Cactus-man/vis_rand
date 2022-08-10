<script lang="ts">
    import Card, { Content } from "@smui/card";
    import LinearProgress from "@smui/linear-progress";
    import DataTable, { Head, Row, Body, Cell } from "@smui/data-table";

    const total = 0;

    export let results: RollResult[] = [];
    export let rolled = 0;

    $: progress = rolled / total;
</script>

<Card>
    <div><h2 class="mdc-typography--headline6">Ergebnisse</h2></div>
    <Content>
        <DataTable sortable style="width: 100%">
            <Head>
                <Row>
                    <Cell numeric>Augen</Cell>
                    <Cell numeric>Anzahl</Cell>
                </Row>
            </Head>
            <Body>
                {#each results as [dots, amount]}
                    <Row>
                        <Cell numeric>{dots}</Cell>
                        <Cell numeric>{amount}</Cell>
                    </Row>
                {/each}
            </Body>
            <LinearProgress
                {progress}
                aria-label="{rolled} rolls out of {total}..."
            />
        </DataTable>
    </Content>
</Card>

<style>
    div {
        padding: 1rem;
    }

    h2.mdc-typography--headline6 {
        margin: 0;
    }
</style>
