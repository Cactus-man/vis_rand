export const enum DieVariant {
    Simple = "simple",
    Weighted = "weighted"
}

type Simple = {
    variant: DieVariant.Simple
    sides: number
};

export type Side = { dots: number, weight: number };

type Weighted = {
    variant: DieVariant.Weighted
    sides: Side[]
};

export type Die = { rolls: number } & (Simple | Weighted);

export type RollData = {
    dots: number,
    occurrences: number
}

import { Writable, writable } from "svelte/store";

export const DEFAULT_DIE: Die = {
    variant: DieVariant.Simple,
    sides: 6,
    rolls: 1
}

const dice: Writable<Die[]> = writable([]);
const data: Writable<RollData[]> = writable([]);

// const running = writable(false);

export { dice, data };
