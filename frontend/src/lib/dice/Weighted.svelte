<script lang="ts">
    import Button, { Label } from "@smui/button";
    import DataTable, { Body, Cell, Head, Row } from "@smui/data-table";
    import { Input } from "@smui/textfield";

    const push = (dots: number, weight: number) => {
        sides = [...sides, [dots, weight]];
    };

    export let sides: [number, number][];
</script>

<DataTable style="width: 100%">
    <Head>
        <Row>
            <Cell>Augen</Cell>
            <Cell>Gewichtung</Cell>
        </Row>
    </Head>
    <Body>
        {#each sides as [dots, weight]}
            <Row>
                <Cell>
                    <Input
                        type="number"
                        bind:value={dots}
                        placeholder="Augenanzahl"
                    />
                </Cell>
                <Cell>
                    <Input
                        type="number"
                        bind:value={weight}
                        placeholder="Augenanzahl"
                    />
                </Cell>
            </Row>
        {/each}
    </Body>
</DataTable>
<Button on:click={() => push(1, 1)}>
    <Label>Eine Seite hinzufügen</Label>
</Button>
