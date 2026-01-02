<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllGoals, deleteGoal, calculateAvgWeeklyIncome, getGoalEta, canCreateGoal } from '../api';
  import { GoalType } from '../types';
  import type { FinancialGoal } from '../types';
  import GoalForm from './GoalForm.svelte';
  import ContributionDialog from './ContributionDialog.svelte';
  import UpgradePrompt from './UpgradePrompt.svelte';
  import { Plus, Target, Pencil, Trash2, TrendingUp, DollarSign } from 'lucide-svelte';

  let goals: FinancialGoal[] = [];
  let loading = true;
  let error = '';
  let showForm = false;
  let editingGoal: FinancialGoal | null = null;
  let showContribution = false;
  let contributionGoal: FinancialGoal | null = null;
  let avgWeeklyIncome = 0;
  let etas: Map<number, string> = new Map();
  
  // Upgrade prompt state
  let showUpgradePrompt = false;
  let upgradeLimitCurrent = 0;
  let upgradeLimitMax = 0;

  onMount(async () => {
    await loadGoals();
    await loadIncome();
  });

  async function loadGoals() {
    try {
      loading = true;
      goals = await getAllGoals();
      for (const goal of goals) {
        const eta = await getGoalEta(goal.id);
        if (eta) etas.set(goal.id, eta);
      }
      etas = etas;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadIncome() {
    try {
      avgWeeklyIncome = await calculateAvgWeeklyIncome();
    } catch (e) {
      console.error('Failed to load income:', e);
    }
  }

  async function handleAddGoal() {
    // Check limit before adding
    try {
      const check = await canCreateGoal();
      if (!check.allowed) {
        upgradeLimitCurrent = check.current;
        upgradeLimitMax = check.limit || 0;
        showUpgradePrompt = true;
        return;
      }
    } catch (e) {
      console.error('Failed to check limit:', e);
    }
    
    editingGoal = null;
    showForm = true;
  }

  function handleEditGoal(goal: FinancialGoal) {
    editingGoal = goal;
    showForm = true;
  }

  async function handleDeleteGoal(goal: FinancialGoal) {
    if (confirm(`Delete goal "${goal.name}"?`)) {
      try {
        await deleteGoal(goal.id);
        await loadGoals();
      } catch (e) {
        error = String(e);
      }
    }
  }

  function handleAddContribution(goal: FinancialGoal) {
    contributionGoal = goal;
    showContribution = true;
  }

  function handleFormClose() {
    showForm = false;
    editingGoal = null;
    loadGoals();
  }

  function handleContributionClose() {
    showContribution = false;
    contributionGoal = null;
    loadGoals();
  }

  function calculateProgress(goal: FinancialGoal): number {
    if (goal.target_amount <= 0) return 0;
    return Math.min((goal.current_amount / goal.target_amount) * 100, 100);
  }

  function formatAmount(amount: number): string {
    return `$${amount.toFixed(2)}`;
  }

  function getGoalTypeColor(type: GoalType): string {
    switch (type) {
      case GoalType.Debt: return 'badge-debt';
      case GoalType.Purchase: return 'badge-purchase';
      case GoalType.Savings: return 'badge-savings';
    }
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-green-100 rounded-lg">
        <Target size={24} class="text-green-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Financial Goals</h1>
        {#if avgWeeklyIncome > 0}
          <p class="text-gray-500 text-sm flex items-center gap-1">
            <TrendingUp size={14} />
            Average weekly income: {formatAmount(avgWeeklyIncome)}
          </p>
        {:else}
          <p class="text-gray-500 text-sm">Track your debts, savings, and purchases</p>
        {/if}
      </div>
    </div>
    <button 
      class="flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
      on:click={handleAddGoal}
    >
      <Plus size={18} />
      Add Goal
    </button>
  </div>

  {#if error}
    <div class="bg-red-50 text-red-600 p-4 rounded-lg">{error}</div>
  {/if}

  {#if loading}
    <div class="bg-white rounded-xl p-12 shadow-sm border border-gray-100 text-center">
      <div class="animate-spin w-8 h-8 border-2 border-green-600 border-t-transparent rounded-full mx-auto mb-3"></div>
      <p class="text-gray-500">Loading goals...</p>
    </div>
  {:else if goals.length === 0}
    <div class="bg-white rounded-xl p-12 shadow-sm border border-gray-100 text-center">
      <Target size={48} class="mx-auto mb-4 text-gray-300" />
      <p class="text-gray-500 mb-2">No financial goals yet.</p>
      <p class="text-sm text-gray-400 mb-4">Track your debts, savings, and purchase goals!</p>
      <button 
        class="inline-flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
        on:click={handleAddGoal}
      >
        <Plus size={18} />
        Set Your First Goal
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      {#each goals as goal}
        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <h3 class="text-lg font-semibold text-gray-900">{goal.name}</h3>
              <span class="inline-block px-2 py-0.5 text-xs font-medium rounded-full mt-2 {getGoalTypeColor(goal.goal_type)}">
                {goal.goal_type}
              </span>
            </div>
            <div class="flex gap-1">
              <button 
                class="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-500 hover:text-indigo-600" 
                on:click={() => handleEditGoal(goal)}
              >
                <Pencil size={16} />
              </button>
              <button 
                class="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-500 hover:text-red-600" 
                on:click={() => handleDeleteGoal(goal)}
              >
                <Trash2 size={16} />
              </button>
            </div>
          </div>

          <div class="space-y-2">
            <div class="flex justify-between text-sm">
              <span class="text-gray-600">{formatAmount(goal.current_amount)}</span>
              <span class="font-medium text-gray-900">{calculateProgress(goal).toFixed(1)}%</span>
              <span class="text-gray-600">{formatAmount(goal.target_amount)}</span>
            </div>
            <div class="w-full h-2 bg-gray-100 rounded-full overflow-hidden">
              <div class="h-full bg-green-500 rounded-full transition-all" style="width: {calculateProgress(goal)}%"></div>
            </div>
            <div class="flex justify-between text-sm text-gray-500">
              <span>Remaining: {formatAmount(goal.target_amount - goal.current_amount)}</span>
              {#if etas.has(goal.id)}
                <span>ETA: {etas.get(goal.id)}</span>
              {/if}
            </div>
          </div>

          <div class="text-sm text-gray-500">
            <div>Created: {goal.created_date}</div>
            {#if goal.target_date}
              <div>Target: {goal.target_date}</div>
            {/if}
          </div>

          <button 
            class="w-full flex items-center justify-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
            on:click={() => handleAddContribution(goal)}
          >
            <DollarSign size={18} />
            Add Contribution
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if showForm}
  <GoalForm goal={editingGoal} on:close={handleFormClose} />
{/if}

{#if showContribution && contributionGoal}
  <ContributionDialog goal={contributionGoal} on:close={handleContributionClose} />
{/if}

<UpgradePrompt 
  bind:show={showUpgradePrompt}
  feature="goals"
  current={upgradeLimitCurrent}
  limit={upgradeLimitMax}
  on:close={() => showUpgradePrompt = false}
/>
