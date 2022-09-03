type Name = number | string;

export type PieChartOptions<D> = {
    name?: (d: D) => Name; // given d in data, returns the (ordinal) label
    value?: (d: D) => number; // given d in data, returns the (quantitative) value
    title?: any; // given d in data, returns the title text
    width?: number; // outer width, in pixels
    height?: number; // outer height, in pixels
    innerRadius?: number, // inner radius of pie, in pixels (non-zero for donut)
    outerRadius?: number, // outer radius of pie, in pixels
    labelRadius?: number, // center radius of labels
    format?: string, // a format specifier for values (in the label)
    names?: Iterable<Name>, // array of names (the domain of the color scale)
    readonly colors?: Iterable<string>, // array of colors for names
    stroke?: string, // stroke separating widths
    strokeWidth?: 1, // width of stroke separating wedges
    strokeLinejoin?: string, // line join of stroke separating wedges
    padAngle?: number, // angular separation between wedges
};