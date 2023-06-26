<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
  import { KeyStore } from "./KeyStore";
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
  let cryptText = "";
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
        console.log(a);
        Publickey = await a[0][0];
        PrivateKey = await a[0][1];
        KeyStore.init(a[1], a[2], a[3]);
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
  <div class="erweiterung">
    <textarea
      name="Text"
      class="cryptText"
      width="95%"
      bind:value={cryptText}
      maxlength="500"
      placeholder="Pls Give Some Text to Crypt"
    />

    <div class="erweiterung-extra">
      <button
        on:click={async () => {
          cryptText = await invoke("encrypt_msg", {
            msg: cryptText,
            key: [$KeyStore[0], $KeyStore[1]],
          });
        }}>Verschlüsseln</button
      >
      <button
        type="button"
        on:click={async () => {
          cryptText = await invoke("dectypt_msg", {
            msg: cryptText,
            key: [$KeyStore[0], $KeyStore[2]],
          });
        }}>Endschlüsseln</button
      >
    </div>
  </div>
{/if}

<style>
  .div-must-Generate details {
    margin-top: 2rem;
  }

  .Left,
  .Riht {
    overflow: hidden;
    text-align: justify; /* für Edge */
    font-family: monospace;
    /* font-size: 20px; */
  }
  .erweiterung {
    margin-top: 1rem;
    overflow: hidden;
    display: grid;
    grid-template-columns: repeat(1, 1fr);
    grid-template-rows: repeat(2, 1fr);
    grid-column-gap: 0px;
    grid-row-gap: 1rem;
  }
  .erweiterung textarea {
    margin-left: 2%;
    max-width: 95%;
    width: 95%;
  }
  .erweiterung-extra {
  }
  @media only screen and (max-width: 850px) {
    .Gesamt {
      margin: auto, 0px;
      display: grid;
      grid-template-rows: repeat(2, 1fr);
      grid-template-rows: 1fr;
      grid-column-gap: 0px;
      grid-row-gap: 2rem;
    }
  }
  @media only screen and (min-width: 851px) {
    .Gesamt {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      grid-template-rows: 1fr;
      grid-column-gap: 2rem;
      grid-row-gap: 0px;
    }
  }
  #Loading {
    width: 100%;
    height: 2.5rem;
  }
</style>
