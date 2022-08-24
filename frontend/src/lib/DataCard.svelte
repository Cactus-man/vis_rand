<script lang="ts">
    import Card, { Content } from "@smui/card";
    import LinearProgress from "@smui/linear-progress";
    import DataTable, { Head, Row, Body, Cell } from "@smui/data-table";

    export let results: {
        rolls: number;
        res: RollResult[];
    };

    let rolled = 0;
    $: progress = rolled / results.rolls;

    const gcd = (a, b) => {
        a = Math.abs(a);
        b = Math.abs(b);

        if (b > a) [a, b] = [b, a];
        while (true) {
            if (b == 0) return a;
            a %= b;
            if (a == 0) return b;
            b %= a;
        }
    };
</script>

<Card>
    <div><h2 class="mdc-typography--headline6">Ergebnisse</h2></div>
    <Content>
        <DataTable sortable style="width: 100%">
            <Head>
                <Row>
                    <Cell numeric>Augen</Cell>
                    <Cell numeric>Anzahl</Cell>
                    <Cell numeric>Rel. Häufigkeit</Cell>
                </Row>
            </Head>
            <Body>
                {#each results.res as [dots, amount]}
                    <Row>
                        <Cell numeric>{dots}</Cell>
                        <Cell numeric>{amount}</Cell>
                        <Cell numeric>
                            {amount / gcd(amount, results.rolls)} / {results.rolls /
                                gcd(amount, results.rolls)}
                        </Cell>
                    </Row>
                {/each}
            </Body>
            <!-- <LinearProgress
                {progress}
                aria-label="{rolled} rolls out of {results.rolls}..."
            /> -->
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
