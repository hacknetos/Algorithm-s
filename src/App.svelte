<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import RSA from "./lib/RSA.svelte";
  import NNA from "./lib/NNA.svelte";

  let Active = "RSA";
</script>

<main class="container">
  <div class="div-line">
    <button
      class:selected={Active === "RSA"}
      class="Trigger"
      on:click={() => {
        Active = "RSA";
      }}>RSA</button
    >
    <button
      class="Trigger"
      class:selected={Active === "NNA"}
      on:click={() => {
        Active = "NNA";
      }}>NNA</button
    >
    <button class="Trigger">Other</button>
  </div>
  <div class="content">
    {#if Active === "RSA"}
      <RSA />
    {:else if Active === "NNA"}
      <NNA />
    {:else if !false}
      {Active}
    {/if}
  </div>
</main>

<style>
  .selected {
    border-bottom: transparent;
    border-bottom-left-radius: 0px;
    border-bottom-right-radius: 0px;
  }
  .content {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
</style>
