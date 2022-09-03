<script lang="ts">
  import Editor from "./lib/configure/Editor.svelte";
  import ActionArea from "./lib/configure/ActionArea.svelte";
  import Visualization from "./lib/visualize/Visualization.svelte";
  import { dice, data } from "./lib/common";

  import { invoke } from "@tauri-apps/api";

  let total: number = 100,
    rolls: number = total;

  function roll() {
    rolls = total;
    invoke("roll", { dice: $dice, total }).then((r: typeof $data) =>
      data.set(r)
    );
  }
</script>

<main>
  <div>
    <ActionArea bind:total {roll} />
    <Editor />
  </div>
  <div>
    <Visualization {rolls} />
  </div>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      d Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
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
