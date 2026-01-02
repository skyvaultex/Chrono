<script lang="ts">
  import { onMount } from 'svelte';
  import { simulateFinancialScenario, getCurrentFinancialBaseline, getAllGoals } from '../api';
  import type { SimulationResult, GoalProjection, FinancialGoal } from '../types';
  import { Calculator, DollarSign, Clock, TrendingUp, Target, AlertTriangle, CheckCircle, Lightbulb } from 'lucide-svelte';

  let hoursPerWeek = 30;
  let hourlyRate = 30;
  let weeklyExpenses = 400;
  
  let result: SimulationResult | null = null;
  let goals: FinancialGoal[] = [];
  let loading = false;
  let baselineHours = 0;
  let baselineRate = 0;

  onMount(async () => {
    try {
      [baselineHours, baselineRate] = await getCurrentFinancialBaseline();
      goals = await getAllGoals();
      hoursPerWeek = Math.round(baselineHours) || 30;
      hourlyRate = Math.round(baselineRate) || 30;
      await simulate();
    } catch (e) {
      console.error('Failed to load baseline:', e);
    }
  });

  async function simulate() {
    loading = true;
    try {
      result = await simulateFinancialScenario(hoursPerWeek, hourlyRate, weeklyExpenses);
    } catch (e) {
      console.error('Simulation failed:', e);
    } finally {
      loading = false;
    }
  }

  // Debounce simulation
  let timeout: any;
  function debouncedSimulate() {
    clearTimeout(timeout);
    timeout = setTimeout(simulate, 300);
  }

  $: if (hoursPerWeek || hourlyRate || weeklyExpenses) {
    debouncedSimulate();
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(amount);
  }

  function formatDate(dateStr: string | null): string {
    if (!dateStr) return 'Never';
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
  }

  function getSustainabilityColor(score: number): string {
    if (score >= 70) return 'text-green-600';
    if (score >= 40) return 'text-amber-600';
    return 'text-red-600';
  }

  function getSustainabilityBg(score: number): string {
    if (score >= 70) return 'bg-green-100';
    if (score >= 40) return 'bg-amber-100';
    return 'bg-red-100';
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center gap-3 mb-6">
    <div class="p-2 bg-purple-100 rounded-lg">
      <Calculator size={24} class="text-purple-600" />
    </div>
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Financial Reality Simulator</h1>
      <p class="text-gray-500 text-sm">Model your financial future with interactive scenarios</p>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Input Sliders -->
    <div class="lg:col-span-1 space-y-6">
      <div class="card p-5 bg-white rounded-xl shadow-sm border">
        <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
          <Clock size={20} class="text-blue-600" />
          Work Schedule
        </h2>
        
        <div class="space-y-4">
          <div>
            <div class="flex justify-between mb-2">
              <span class="text-sm font-medium text-gray-700">Hours per Week</span>
              <span class="text-sm font-bold text-blue-600">{hoursPerWeek}h</span>
            </div>
            <input
              type="range"
              min="5"
              max="80"
              step="1"
              bind:value={hoursPerWeek}
              aria-label="Hours per Week"
              class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
            />
            <div class="flex justify-between text-xs text-gray-400 mt-1">
              <span>5h</span>
              <span>40h</span>
              <span>80h</span>
            </div>
          </div>

          <div>
            <div class="flex justify-between mb-2">
              <span class="text-sm font-medium text-gray-700">Hourly Rate</span>
              <span class="text-sm font-bold text-green-600">${hourlyRate}/hr</span>
            </div>
            <input
              type="range"
              min="10"
              max="200"
              step="5"
              bind:value={hourlyRate}
              aria-label="Hourly Rate"
              class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-green-600"
            />
            <div class="flex justify-between text-xs text-gray-400 mt-1">
              <span>$10</span>
              <span>$100</span>
              <span>$200</span>
            </div>
          </div>

          <div>
            <div class="flex justify-between mb-2">
              <span class="text-sm font-medium text-gray-700">Weekly Expenses</span>
              <span class="text-sm font-bold text-red-600">${weeklyExpenses}</span>
            </div>
            <input
              type="range"
              min="100"
              max="2000"
              step="50"
              bind:value={weeklyExpenses}
              aria-label="Weekly Expenses"
              class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-red-600"
            />
            <div class="flex justify-between text-xs text-gray-400 mt-1">
              <span>$100</span>
              <span>$1000</span>
              <span>$2000</span>
            </div>
          </div>
        </div>

        {#if baselineHours > 0}
          <div class="mt-4 p-3 bg-gray-50 rounded-lg text-sm text-gray-600">
            <span class="font-medium">Your baseline:</span> {baselineHours.toFixed(1)}h/week @ ${baselineRate.toFixed(0)}/hr
          </div>
        {/if}
      </div>
    </div>

    <!-- Results -->
    <div class="lg:col-span-2 space-y-6">
      {#if result}
        <!-- Income Summary -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="card p-4 bg-white rounded-xl shadow-sm border text-center">
            <p class="text-xs text-gray-500 uppercase tracking-wide">Weekly Income</p>
            <p class="text-2xl font-bold text-gray-900">{formatCurrency(result.weekly_income)}</p>
          </div>
          <div class="card p-4 bg-white rounded-xl shadow-sm border text-center">
            <p class="text-xs text-gray-500 uppercase tracking-wide">Weekly Savings</p>
            <p class="text-2xl font-bold {result.weekly_savings >= 0 ? 'text-green-600' : 'text-red-600'}">
              {formatCurrency(result.weekly_savings)}
            </p>
          </div>
          <div class="card p-4 bg-white rounded-xl shadow-sm border text-center">
            <p class="text-xs text-gray-500 uppercase tracking-wide">Monthly</p>
            <p class="text-2xl font-bold text-gray-900">{formatCurrency(result.monthly_income)}</p>
          </div>
          <div class="card p-4 bg-white rounded-xl shadow-sm border text-center">
            <p class="text-xs text-gray-500 uppercase tracking-wide">Yearly</p>
            <p class="text-2xl font-bold text-gray-900">{formatCurrency(result.yearly_income)}</p>
          </div>
        </div>

        <!-- Sustainability Score -->
        <div class="card p-5 bg-white rounded-xl shadow-sm border">
          <div class="flex items-center justify-between mb-3">
            <h3 class="font-semibold flex items-center gap-2">
              <TrendingUp size={18} class="text-purple-600" />
              Sustainability Score
            </h3>
            <span class="text-3xl font-bold {getSustainabilityColor(result.sustainability_score)}">
              {result.sustainability_score.toFixed(0)}%
            </span>
          </div>
          <div class="w-full h-3 bg-gray-200 rounded-full overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-500 {getSustainabilityBg(result.sustainability_score).replace('bg-', 'bg-')}"
              style="width: {result.sustainability_score}%; background-color: {result.sustainability_score >= 70 ? '#22c55e' : result.sustainability_score >= 40 ? '#f59e0b' : '#ef4444'}"
            />
          </div>
        </div>

        <!-- Insights -->
        {#if result.insights.length > 0}
          <div class="card p-5 bg-white rounded-xl shadow-sm border">
            <h3 class="font-semibold mb-3 flex items-center gap-2">
              <Lightbulb size={18} class="text-amber-500" />
              Insights
            </h3>
            <div class="space-y-2">
              {#each result.insights as insight}
                <div class="flex items-start gap-2 p-3 bg-gray-50 rounded-lg text-sm">
                  {#if insight.includes('⚠️')}
                    <AlertTriangle size={16} class="text-amber-500 mt-0.5 flex-shrink-0" />
                  {:else if insight.includes('✅')}
                    <CheckCircle size={16} class="text-green-500 mt-0.5 flex-shrink-0" />
                  {:else}
                    <Lightbulb size={16} class="text-blue-500 mt-0.5 flex-shrink-0" />
                  {/if}
                  <span class="text-gray-700">{insight.replace(/[⚠️✅💡]/g, '').trim()}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Goal Projections -->
        {#if result.goal_projections.length > 0}
          <div class="card p-5 bg-white rounded-xl shadow-sm border">
            <h3 class="font-semibold mb-4 flex items-center gap-2">
              <Target size={18} class="text-indigo-600" />
              Goal Projections
            </h3>
            <div class="space-y-3">
              {#each result.goal_projections as proj}
                <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div>
                    <p class="font-medium text-gray-900">{proj.goal_name}</p>
                    <p class="text-sm text-gray-500">{formatCurrency(proj.remaining)} remaining</p>
                  </div>
                  <div class="text-right">
                    {#if proj.weeks_to_complete && proj.weeks_to_complete > 0}
                      <p class="font-semibold text-indigo-600">
                        {proj.weeks_to_complete < 4 
                          ? `${Math.ceil(proj.weeks_to_complete)} weeks`
                          : proj.weeks_to_complete < 52
                            ? `${Math.ceil(proj.weeks_to_complete / 4.33)} months`
                            : `${(proj.weeks_to_complete / 52).toFixed(1)} years`
                        }
                      </p>
                      <p class="text-xs text-gray-500">{formatDate(proj.completion_date)}</p>
                    {:else}
                      <p class="text-sm text-red-500">Not achievable</p>
                      <p class="text-xs text-gray-500">Increase savings</p>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="card p-5 bg-gray-50 rounded-xl border border-dashed text-center">
            <Target size={32} class="text-gray-400 mx-auto mb-2" />
            <p class="text-gray-500">No financial goals set</p>
            <p class="text-sm text-gray-400">Add goals to see projections</p>
          </div>
        {/if}
      {:else if loading}
        <div class="card p-12 bg-white rounded-xl shadow-sm border text-center">
          <div class="animate-spin w-8 h-8 border-2 border-purple-600 border-t-transparent rounded-full mx-auto mb-3"></div>
          <p class="text-gray-500">Calculating...</p>
        </div>
      {/if}
    </div>
  </div>
</div>
