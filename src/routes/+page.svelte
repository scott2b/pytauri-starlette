<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let text = '';
  let result = '';
  let error = '';
  let isLoading = true;
  let isProcessing = false;

  async function initServer() {
    try {
      isLoading = true;
      await invoke('start_python_server');
      error = '';
    } catch (e) {
      error = e;
    } finally {
      isLoading = false;
    }
  }

  async function processText() {
    if (!text) return;
    
    try {
      isProcessing = true;
      error = '';
      result = await invoke('process_text', { text });
    } catch (e) {
      error = e;
      result = '';
    } finally {
      isProcessing = false;
    }
  }

  onMount(() => {
    initServer();
  });
</script>

<main>
  <h1>Python Text Processor</h1>

  {#if isLoading}
    <div class="loading">
      <span class="spinner"></span>
      <p>Starting server...</p>
    </div>
  {:else}
    <div class="input-group">
      <input
        type="text"
        bind:value={text}
        disabled={isProcessing}
        placeholder="Enter text to process"
      />
      <button 
        on:click={processText} 
        disabled={isProcessing || !text}
      >
        {#if isProcessing}
          Processing...
        {:else}
          Process Text
        {/if}
      </button>
    </div>

    {#if result}
      <div class="result">
        <h2>Result:</h2>
        <p>{result}</p>
      </div>
    {/if}

    {#if error}
      <div class="error">
        <p>{error}</p>
      </div>
    {/if}
  {/if}
</main>

<style>
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 2rem 0;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .input-group {
    display: flex;
    gap: 1rem;
    margin: 1rem 0;
  }

  .error {
    color: red;
    margin: 1rem 0;
  }

  .result {
    margin: 1rem 0;
  }
</style>
