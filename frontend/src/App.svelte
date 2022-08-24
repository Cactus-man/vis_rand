<script lang="ts">
  import DiceDash from "./lib/DiceCard.svelte";
  import RollsDash from "./lib/DataCard.svelte";

  let results: {
    rolls: number;
    res: RollResult[];
  } = {
    rolls: 0,
    res: [],
  };
</script>

<main>
  <DiceDash bind:results />
  <RollsDash bind:results />
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      d Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  :global(body) {
    margin: 0;
    padding: 8px;
  }

  main {
    width: calc(100vw - 16px);
    height: calc(100vh - 16px);
    text-align: center;
    margin: 0 auto;
    display: flex;
    gap: 8px;
  }

  @media only screen and (min-aspect-ratio: 1/1) {
    :global(main > div) {
      width: calc(50% - 4px);
      height: 100%;
    }

    main {
      justify-content: center;
      align-items: center;
    }
  }

  @media only screen and (max-aspect-ratio: 1/1) {
    :global(main > div) {
      width: 100%;
      height: fit-content;
    }

    main {
      flex-direction: column;
    }
  }
</style>
