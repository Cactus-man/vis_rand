<script lang="ts">
    import * as d3 from "d3";
    import { onMount, onDestroy } from "svelte";
    import type { PieChartOptions } from "./common";
    import { data } from "../common";

    let container: HTMLDivElement = document.createElement("div");
    export let options: PieChartOptions<typeof $data[0]> = {};

    let {
        width = 640, // outer width, in pixels
        height = 400, // outer height, in pixels
        innerRadius = Math.min(width, height) / 4, // inner radius of pie, in pixels (non-zero for donut)
        outerRadius = Math.min(width, height) / 2, // outer radius of pie, in pixels
        labelRadius = (innerRadius + outerRadius) / 2, // center radius of labels
        names, // array of names (the domain of the color scale)
        colors, // array of colors for names
        stroke = innerRadius > 0 ? "none" : "white", // stroke separating widths
        strokeWidth = 1, // width of stroke separating wedges
        strokeLinejoin = "round", // line join of stroke separating wedges
        padAngle = stroke === "none" ? 1 / outerRadius : 0, // angular separation between wedges
    } = options;

    function compute(data: typeof $data) {
        // Compute values.
        const N = d3.map(data, ({ dots }) => dots);
        const V = d3.map(data, ({ occurrences }) => occurrences);
        const I = d3.range(N.length).filter((i) => !isNaN(V[i]));

        // Unique the names.
        if (names === undefined) names = N;
        const unique_names = new d3.InternSet(names);

        // Chose a default color scheme based on cardinality.
        if (colors === undefined) colors = d3.schemeSpectral[unique_names.size];
        if (colors === undefined)
            colors = d3.quantize(
                (t) => d3.interpolateSpectral(t * 0.8 + 0.1),
                unique_names.size
            );

        // Construct scales.
        const color = d3.scaleOrdinal(names, colors);

        // Compute titles.
        const f = d3.format(",");
        const title = (i: number) => `${N[i]}\n${f(V[i])}`;

        // Construct arcs.
        const arcs = d3
            .pie()
            .padAngle(padAngle)
            .sort(null)
            .value((i: number) => V[i])(I);
        const arc = d3.arc().innerRadius(innerRadius).outerRadius(outerRadius);
        const arcLabel = d3
            .arc()
            .innerRadius(labelRadius)
            .outerRadius(labelRadius);

        const svg = d3
            .create("svg")
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", [-width / 2, -height / 2, width, height])
            .attr("style", "max-width: 100%; height: auto; height: intrinsic;");

        svg.append("g")
            .attr("stroke", stroke)
            .attr("stroke-width", strokeWidth)
            .attr("stroke-linejoin", strokeLinejoin)
            .selectAll("path")
            .data(arcs)
            .join("path")
            .attr("fill", (d) => color(N[d.data as number]))
            // @ts-ignore
            .attr("d", arc)
            .append("title")
            .text((d) => title(d.data as number));

        svg.append("g")
            .attr("font-family", "sans-serif")
            .attr("font-size", 10)
            .attr("text-anchor", "middle")
            .selectAll("text")
            .data(arcs)
            .join("text")
            // @ts-ignore
            .attr("transform", (d) => `translate(${arcLabel.centroid(d)})`)
            .selectAll("tspan")
            .data((d) => {
                const lines = `${title(d.data as number)}`.split(/\n/);
                return d.endAngle - d.startAngle > 0.25
                    ? lines
                    : lines.slice(0, 1);
            })
            .join("tspan")
            .attr("x", 0)
            .attr("y", (_, i) => `${i * 1.1}em`)
            .attr("font-weight", (_, i) => (i ? null : "bold"))
            .text((d) => d);

        return Object.assign(svg.node(), { scales: { color } });
    }
    
    const unsubscribe = data.subscribe((d) => {
        if (container) {
            const next = compute(d);
            if (container.hasChildNodes())
            container.replaceChild(next, container.firstChild);
            else container.appendChild(next);
        }
    });
    
    onMount(() => container.appendChild(compute($data)));
    onDestroy(unsubscribe);
</script>

<div bind:this={container} />
