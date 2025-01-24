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

<main class="container">
  <div class="card">
    <h1>Python Text Processor</h1>

    {#if isLoading}
      <div class="loading">
        <div class="spinner"></div>
        <p>Starting server...</p>
      </div>
    {:else}
      <div class="input-section">
        <textarea
          class="text-input"
          bind:value={text}
          disabled={isProcessing}
          placeholder="Enter text to process..."
          rows="4"
        />
        <button 
          class="process-button"
          class:processing={isProcessing}
          on:click={processText} 
          disabled={isProcessing || !text}
        >
          <span class="button-content">
            {#if isProcessing}
              <div class="button-spinner"></div>
              Processing...
            {:else}
              Process Text
            {/if}
          </span>
        </button>
      </div>

      {#if result}
        <div class="result-section">
          <h2>Result</h2>
          <div class="result-content">
            {result}
          </div>
        </div>
      {/if}

      {#if error}
        <div class="error-section">
          <p>{error}</p>
        </div>
      {/if}
    {/if}
  </div>
</main>

<style>
  .container {
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
    padding: 2rem;
  }

  .card {
    background: white;
    border-radius: 1rem;
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
    padding: 2rem;
    width: 100%;
    max-width: 600px;
    transition: transform 0.2s ease;
  }

  .card:hover {
    transform: translateY(-2px);
  }

  h1 {
    color: #2d3748;
    font-size: 2rem;
    margin-bottom: 2rem;
    text-align: center;
    font-weight: 700;
  }

  h2 {
    color: #4a5568;
    font-size: 1.5rem;
    margin-bottom: 1rem;
    font-weight: 600;
  }

  .input-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .text-input {
    width: 100%;
    padding: 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 0.5rem;
    font-size: 1rem;
    transition: border-color 0.2s ease;
    resize: vertical;
    font-family: inherit;
  }

  .text-input:focus {
    outline: none;
    border-color: #4299e1;
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
  }

  .process-button {
    background: #4299e1;
    color: white;
    border: none;
    border-radius: 0.5rem;
    padding: 1rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .process-button:hover:not(:disabled) {
    background: #3182ce;
    transform: translateY(-1px);
  }

  .process-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .button-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .result-section {
    background: #f7fafc;
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-top: 2rem;
  }

  .result-content {
    color: #4a5568;
    font-size: 1.1rem;
    line-height: 1.6;
    white-space: pre-wrap;
  }

  .error-section {
    background: #fff5f5;
    color: #c53030;
    padding: 1rem;
    border-radius: 0.5rem;
    margin-top: 1rem;
    font-weight: 500;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2rem;
  }

  .spinner, .button-spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid #e2e8f0;
    border-top: 3px solid #4299e1;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .button-spinner {
    width: 1rem;
    height: 1rem;
    border-width: 2px;
    border-color: rgba(255, 255, 255, 0.3);
    border-top-color: white;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }

    .card {
      padding: 1.5rem;
    }

    h1 {
      font-size: 1.75rem;
    }
  }
</style>
