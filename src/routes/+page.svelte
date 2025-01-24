<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let inputText = ''
  let processedText = ''
  let error = ''

  onMount(async () => {
    try {
      await invoke('start_python_server')
    } catch (e) {
      error = `Failed to start Python server: ${e}`
    }
  })

  async function processText() {
    try {
      processedText = await invoke('process_text', { text: inputText })
      error = ''
    } catch (e) {
      error = `Error processing text: ${e}`
      processedText = ''
    }
  }
</script>

<main class="container">
  <h1>Python Text Processor</h1>
  
  <div class="row">
    <input
      type="text"
      bind:value={inputText}
      placeholder="Enter text to process"
    />
    <button on:click={processText}>Process Text</button>
  </div>

  {#if processedText}
    <div class="result">
      <h2>Processed Text:</h2>
      <p>{processedText}</p>
    </div>
  {/if}

  {#if error}
    <div class="error">
      <p>{error}</p>
    </div>
  {/if}
</main>

<style>
  .container {
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  .row {
    display: flex;
    gap: 1rem;
    margin: 2rem 0;
  }

  input {
    flex: 1;
    padding: 0.5rem;
    font-size: 1rem;
  }

  button {
    padding: 0.5rem 1rem;
    font-size: 1rem;
    background: #646cff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background: #747bff;
  }

  .result {
    margin-top: 2rem;
    padding: 1rem;
    background: #f0f0f0;
    border-radius: 4px;
  }

  .error {
    margin-top: 2rem;
    padding: 1rem;
    background: #ffebee;
    color: #c62828;
    border-radius: 4px;
  }
</style>
