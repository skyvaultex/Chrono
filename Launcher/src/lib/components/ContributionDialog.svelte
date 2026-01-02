<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { addContribution } from '../api';
  import type { FinancialGoal } from '../types';
  import { X, DollarSign, Plus } from 'lucide-svelte';

  export let goal: FinancialGoal;

  const dispatch = createEventDispatcher();

  let amount = 0;
  let error = '';
  let loading = false;

  async function handleSubmit() {
    error = '';

    if (amount <= 0) {
      error = 'Amount must be positive';
      return;
    }

    try {
      loading = true;
      await addContribution(goal.id, amount);
      dispatch('close');
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    dispatch('close');
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget && !loading) dispatch('close');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && !loading) dispatch('close');
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div 
  class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
  on:mousedown={handleBackdropClick}
  role="dialog"
  aria-modal="true"
>
  <div class="card p-6 w-full max-w-md" on:mousedown|stopPropagation>
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-bold flex items-center gap-2">
        <DollarSign size={24} class="text-green-500" />
        Add Contribution
      </h2>
      <button
        type="button"
        class="btn btn-sm variant-ghost p-1"
        on:click={handleCancel}
        disabled={loading}
        aria-label="Close"
      >
        <X size={20} />
      </button>
    </div>
    <p class="text-gray-600 dark:text-gray-400 mb-4">
      Contributing to: <span class="font-semibold">{goal.name}</span>
    </p>

    {#if error}
      <div class="alert variant-filled-error mb-4">{error}</div>
    {/if}

    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <label class="label">
        <span>Contribution Amount ($)</span>
        <input
          type="number"
          class="input"
          bind:value={amount}
          min="0.01"
          step="0.01"
          placeholder="0.00"
          autofocus
          required
        />
      </label>

      <!-- Preview -->
      {#if amount > 0}
        <div class="card variant-soft-success p-3">
          <div class="text-sm">New total:</div>
          <div class="text-xl font-bold">
            ${(goal.current_amount + amount).toFixed(2)} / ${goal.target_amount.toFixed(2)}
          </div>
          <div class="text-sm mt-1">
            Progress: {(((goal.current_amount + amount) / goal.target_amount) * 100).toFixed(1)}%
          </div>
        </div>
      {/if}

      <div class="flex justify-end space-x-2 pt-4">
        <button
          type="button"
          class="btn variant-ghost"
          on:click={handleCancel}
          disabled={loading}
        >
          Cancel
        </button>
        <button
          type="submit"
          class="btn variant-filled-success flex items-center gap-1"
          disabled={loading}
        >
          <Plus size={16} />
          {loading ? 'Adding...' : 'Add Contribution'}
        </button>
      </div>
    </form>
  </div>
</div>
