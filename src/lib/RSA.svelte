<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
  appWindow.listen(
    "RSA-Stap",
    (event: {
      event: string;
      id: number;
      payload: {
        from: number;
        stap: number;
        waht: string;
      };
      windowLabel: string;
    }) => {
      Loading = "Generate";
      payloadMsg = event.payload.waht;
      MaxStaps = event.payload.from;
      stap = event.payload.stap;
      if (event.payload.stap >= event.payload.from) {
        Loading = "Generatet";
      }
    }
  );
  let Publickey: string[] = [""];
  let PrivateKey: String[] = [""];
  let payloadMsg = ``;
  let MaxStaps = 0;
  let stap = 0;
  let Loading: "Must Generate" | "Generate" | "Generatet" = "Must Generate";
</script>

<h1>RSA</h1>
{#if Loading == "Must Generate"}
  <div class="div-must-Generate">
    <button
      class="Trigger"
      on:click={async () => {
        let a = await invoke("gen_key");
        Publickey = await a[0];
        //TODO Into a Fucking array BRO
        PrivateKey = await a[1];
        //TODO Into a Fucking array BRO
      }}>Generate</button
    >
    <details>
      <summary>Infos About RSA</summary>
      <p>Bitte Text Eifügen Danke</p>
    </details>
  </div>
{:else if Loading == "Generate"}
  <h1>Generate</h1>
  <label for="Loading">{payloadMsg}</label>
  <progress id="Loading" max={MaxStaps} value={stap}
    >{(MaxStaps / 100) * stap}%</progress
  >
{:else if Loading == "Generatet"}
  <div class="Gesamt">
    <div class="Left">
      <r>|--------------------------Public-Key-Start--------------------|</r><br
      />
      {#each Publickey as line}
        {line}<br />
      {/each}
      <r>|--------------------------Public-Key-End----------------------|</r>
    </div>
    <div class="Riht">
      <r>|--------------------------Private-Key-Start-------------------|</r><br
      />
      {#each PrivateKey as line}
        {line}<br />
      {/each}
      <r>|--------------------------Private-Key-End---------------------|</r>
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
    text-align: justify; /* für Edge */
    -moz-text-align-last: justify; /* für Firefox vor 58.0 */
    text-align-last: justify;
  }
  #Loading {
    width: 100%;
    height: 2.5rem;
  }
</style>
