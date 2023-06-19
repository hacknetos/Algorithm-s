<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
  appWindow.listen(
    "RSA-Stap",
    (event: {
      event: string;
      id: number;
      payload: {
        From: number;
        Stap: number;
        Waht: string;
      };
      windowLabel: string;
    }) => {
      console.log(event.payload);
      Loading = "Generate";
      if (event.payload.Stap >= event.payload.From) {
        Loading = "Generatet";
      }
    }
  );
  let bcounter = 0;
  const counterB = () => {
    bcounter += 1;
  };
  const counterRest = () => {
    bcounter = 0;
  };
  let Publickey = ``;
  let PrivateKey = ``;
  let Loading: "Must Generate" | "Generate" | "Generatet" = "Must Generate";
</script>

<h1>RSA</h1>
{#if Loading == "Must Generate"}
  <div class="div-must-Generate">
    <button
      class="Trigger"
      on:click={async () => {
        let a = await invoke("gen_key");
        Publickey = a[0];
        PrivateKey = a[1];
      }}>Generate</button
    >
    <details>
      <summary>Infos About RSA</summary>
      <p>Bitte Text Eifügen Danke</p>
    </details>
  </div>
{:else if Loading == "Generate"}
  <h1>Generate</h1>
{:else if Loading == "Generatet"}
  <div class="Gesamt">
    <div class="Left">
      |--------------------------Public-Key-Start--------------------|
      {#each Publickey as buchstabe}
        {#if bcounter >= 64}
          {counterRest()}
          <br />
        {/if}
        {counterB()}
        {buchstabe.toString()}
      {/each}

      |--------------------------Public-Key-End----------------------|
    </div>
    <div class="Riht">
      <!-- |--------------------------Private-Key-Start-------------------|
      {PrivateKey}
      |--------------------------Private-Key-End---------------------| -->
    </div>
  </div>
{/if}

<style>
  .div-must-Generate details {
    margin-top: 2rem;
  }
  .Gesamt {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: 1fr;
    grid-column-gap: 2rem;
    grid-row-gap: 0px;
  }
  .Left,
  .Riht {
    width: fit-content;
    hyphens: auto;
    text-align: justify; /* für Edge */
    -moz-text-align-last: justify; /* für Firefox vor 58.0 */
    text-align-last: justify;
  }
</style>
