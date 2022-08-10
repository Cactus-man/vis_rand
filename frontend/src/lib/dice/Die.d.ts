type Simple = {
    variant: "simple"
    sides: number
};

type Weighted = {
    variant: "weighted"
    sides: [number, number][]
};

type Die = Simple | Weighted;
type Panel = { die: Die, amount: number }

type RollResult = [number, number];
