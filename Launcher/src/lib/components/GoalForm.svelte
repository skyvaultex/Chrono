<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { addGoal, updateGoal, getCurrentDate } from '../api';
  import { GoalType } from '../types';
  import type { FinancialGoal, NewGoal } from '../types';
  import { X } from 'lucide-svelte';

  export let goal: FinancialGoal | null = null;

  const dispatch = createEventDispatcher();

  let formData = {
    goalType: goal?.goal_type || GoalType.Debt,
    name: goal?.name || '',
    targetAmount: goal?.target_amount || 0,
    currentAmount: goal?.current_amount || 0,
    createdDate: goal?.created_date || '',
    targetDate: goal?.target_date || '',
  };

  let hasTargetDate = !!goal?.target_date;
  let error = '';
  let loading = false;

  onMount(async () => {
    if (!goal) {
      formData.createdDate = await getCurrentDate();
    }
  });

  async function handleSubmit() {
    error = '';

    if (!formData.name.trim()) {
      error = 'Goal name is required';
      return;
    }
    if (formData.targetAmount <= 0) {
      error = 'Target amount must be positive';
      return;
    }
    if (formData.currentAmount < 0) {
      error = 'Current amount cannot be negative';
      return;
    }

    try {
      loading = true;

      if (goal) {
        await updateGoal({
          id: goal.id,
          goal_type: formData.goalType,
          name: formData.name.trim(),
          target_amount: formData.targetAmount,
          current_amount: formData.currentAmount,
          created_date: formData.createdDate,
          target_date: hasTargetDate && formData.targetDate ? formData.targetDate : undefined,
        });
      } else {
        const newGoal: NewGoal = {
          goal_type: formData.goalType,
          name: formData.name.trim(),
          target_amount: formData.targetAmount,
          current_amount: formData.currentAmount,
          created_date: formData.createdDate,
          target_date: hasTargetDate && formData.targetDate ? formData.targetDate : undefined,
        };
        await addGoal(newGoal);
      }

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
  <div class="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto" on:mousedown|stopPropagation>
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-bold">
        {goal ? 'Edit Goal' : 'Add New Goal'}
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

    {#if error}
      <div class="alert variant-filled-error mb-4">{error}</div>
    {/if}

    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <label class="label">
        <span>Goal Type</span>
        <select class="select" bind:value={formData.goalType}>
          <option value={GoalType.Debt}>Debt Payoff</option>
          <option value={GoalType.Purchase}>Purchase</option>
          <option value={GoalType.Savings}>Savings</option>
        </select>
      </label>

      <label class="label">
        <span>Goal Name</span>
        <input
          type="text"
          class="input"
          bind:value={formData.name}
          placeholder="e.g., Pay off credit card, Buy Mavic 3T"
          required
        />
      </label>

      <label class="label">
        <span>Target Amount ($)</span>
        <input
          type="number"
          class="input"
          bind:value={formData.targetAmount}
          min="0.01"
          step="0.01"
          required
        />
      </label>

      <label class="label">
        <span>Current Amount ($)</span>
        <input
          type="number"
          class="input"
          bind:value={formData.currentAmount}
          min="0"
          step="0.01"
          required
        />
      </label>

      <label class="label">
        <span>Created Date</span>
        <input
          type="date"
          class="input"
          bind:value={formData.createdDate}
          required
        />
      </label>

      <div class="space-y-2">
        <label class="flex items-center space-x-2">
          <input type="checkbox" class="checkbox" bind:checked={hasTargetDate} />
          <span>Set target completion date</span>
        </label>
        {#if hasTargetDate}
          <input type="date" class="input" bind:value={formData.targetDate} />
        {/if}
      </div>

      {#if formData.targetAmount > 0}
        <div class="card variant-soft-primary p-3">
          <div class="text-sm mb-1">Progress:</div>
          <div class="w-full bg-gray-200 rounded-full h-2">
            <div
              class="bg-primary-500 h-2 rounded-full"
              style="width: {Math.min((formData.currentAmount / formData.targetAmount) * 100, 100)}%"
            ></div>
          </div>
          <div class="text-sm mt-1 flex justify-between">
            <span>${formData.currentAmount.toFixed(2)}</span>
            <span class="font-semibold">
              {((formData.currentAmount / formData.targetAmount) * 100).toFixed(1)}%
            </span>
            <span>${formData.targetAmount.toFixed(2)}</span>
          </div>
        </div>
      {/if}

      <div class="flex justify-end space-x-2 pt-4">
        <button type="button" class="btn variant-ghost" on:click={handleCancel} disabled={loading}>
          Cancel
        </button>
        <button type="submit" class="btn variant-filled-primary" disabled={loading}>
          {loading ? 'Saving...' : (goal ? 'Update' : 'Add')}
        </button>
      </div>
    </form>
  </div>
</div>
