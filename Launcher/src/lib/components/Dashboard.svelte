<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { getTodaySummary, getRecentSessions, getPaySummary, getAllSessionTypes } from '../api';
  import type { TodaySummary, WorkSession, PaySummary, SessionType } from '../types';
  import { PayType } from '../types';
  import { Clock, DollarSign, Plus, Calendar, TrendingUp, Briefcase, Mic, ChevronDown, ChevronUp, ArrowUpDown, ArrowUp, ArrowDown } from 'lucide-svelte';
  import VoiceInput from './VoiceInput.svelte';

  const dispatch = createEventDispatcher();

  let summary: TodaySummary | null = null;
  let paySummary: PaySummary | null = null;
  let sessionTypes: SessionType[] = [];
  let recentSessions: WorkSession[] = [];
  let loading = true;
  let error = '';
  let selectedPeriod: 'today' | 'month' | 'year' | 'all' = 'all';
  let showVoiceInput = false;
  let sessionsExpanded = false;
  const COLLAPSED_COUNT = 5;
  
  // Sorting state
  type SortField = 'date' | 'type' | 'project_name' | 'hours' | 'pay';
  let sortField: SortField = 'date';
  let sortDirection: 'asc' | 'desc' = 'desc';

  onMount(async () => {
    try {
      [summary, paySummary, sessionTypes, recentSessions] = await Promise.all([
        getTodaySummary(),
        getPaySummary(),
        getAllSessionTypes(),
        getRecentSessions(20)
      ]);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  async function handleVoiceSaved() {
    // Refresh data after voice entry
    try {
      [summary, paySummary, recentSessions] = await Promise.all([
        getTodaySummary(),
        getPaySummary(),
        getRecentSessions(20)
      ]);
    } catch (e) {
      console.error('Failed to refresh:', e);
    }
  }

  function formatHours(hours: number): string {
    return hours.toFixed(1);
  }

  function formatPay(amount: number): string {
    return `$${amount.toFixed(2)}`;
  }

  // Helper to check if session is in selected period
  function isInPeriod(dateStr: string): boolean {
    const sessionDate = new Date(dateStr);
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
    const yearStart = new Date(now.getFullYear(), 0, 1);
    
    switch (selectedPeriod) {
      case 'today': return sessionDate >= today;
      case 'month': return sessionDate >= monthStart;
      case 'year': return sessionDate >= yearStart;
      case 'all': return true;
      default: return true;
    }
  }

  // Filter sessions by period
  $: filteredSessions = recentSessions.filter(s => isInPeriod(s.date));

  // Reactive pay display - recalculate when paySummary or selectedPeriod changes
  $: displayPay = paySummary ? (() => {
    switch (selectedPeriod) {
      case 'today': return paySummary.today ?? 0;
      case 'month': return paySummary.this_month ?? 0;
      case 'year': return paySummary.this_year ?? 0;
      case 'all': return paySummary.all_time ?? 0;
      default: return 0;
    }
  })() : 0;

  // Reactive hours display by session type
  $: totalHours = filteredSessions.reduce((sum, s) => sum + s.hours, 0);
  $: hoursByType = sessionTypes.reduce((acc, st) => {
    acc[st.name] = filteredSessions
      .filter(s => s.session_type_id === st.id)
      .reduce((sum, s) => sum + s.hours, 0);
    return acc;
  }, {} as Record<string, number>);

  // Sort sessions
  function handleSort(field: SortField) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = 'desc';
    }
  }

  $: sortedSessions = [...recentSessions].sort((a, b) => {
    const modifier = sortDirection === 'asc' ? 1 : -1;
    switch (sortField) {
      case 'date':
        return modifier * a.date.localeCompare(b.date);
      case 'type':
        return modifier * (a.session_type_name || '').localeCompare(b.session_type_name || '');
      case 'project_name':
        return modifier * a.project_name.localeCompare(b.project_name);
      case 'hours':
        return modifier * (a.hours - b.hours);
      case 'pay':
        return modifier * (calculateSessionPay(a) - calculateSessionPay(b));
      default:
        return 0;
    }
  });

  $: periodLabel = (() => {
    switch (selectedPeriod) {
      case 'today': return 'Today';
      case 'month': return 'This Month';
      case 'year': return 'This Year';
      case 'all': return 'All Time';
      default: return 'All Time';
    }
  })();

  function calculateSessionPay(session: WorkSession): number {
    if (session.pay_type === PayType.Hourly && session.hourly_rate) {
      return session.hourly_rate * session.hours;
    }
    if (session.pay_type === PayType.Fixed && session.fixed_amount) {
      return session.fixed_amount;
    }
    return 0;
  }

  function handleAddSession() {
    dispatch('navigate', { tab: 'sessions' });
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-indigo-100 rounded-lg">
        <Clock size={24} class="text-indigo-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p class="text-gray-500 text-sm">Your daily overview</p>
      </div>
    </div>
    <div class="flex items-center gap-2">
      <button 
        class="flex items-center gap-2 px-3 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
        on:click={() => showVoiceInput = true}
        title="Voice Entry"
      >
        <Mic size={18} class="text-indigo-600" />
        <span class="text-sm font-medium text-gray-700">Voice</span>
      </button>
      <button 
        class="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        on:click={handleAddSession}
      >
        <Plus size={18} />
        New Session
      </button>
    </div>
  </div>

  {#if loading}
    <div class="text-center py-8">Loading...</div>
  {:else if error}
    <div class="alert variant-filled-error">{error}</div>
  {:else if summary}
    <!-- Time Period Selector -->
    <div class="flex justify-end">
      <div class="flex gap-1 bg-gray-100 p-1 rounded-lg">
        <button
          class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedPeriod === 'today' ? 'bg-white text-indigo-600 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
          on:click={() => selectedPeriod = 'today'}
        >Today</button>
        <button
          class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedPeriod === 'month' ? 'bg-white text-indigo-600 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
          on:click={() => selectedPeriod = 'month'}
        >Month</button>
        <button
          class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedPeriod === 'year' ? 'bg-white text-indigo-600 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
          on:click={() => selectedPeriod = 'year'}
        >Year</button>
        <button
          class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedPeriod === 'all' ? 'bg-white text-indigo-600 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
          on:click={() => selectedPeriod = 'all'}
        >All Time</button>
      </div>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <!-- Total Hours -->
      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-2 mb-2">
          <Clock size={18} class="text-gray-500" />
          <h3 class="text-sm font-medium text-gray-500">Total Hours</h3>
        </div>
        <p class="text-2xl font-bold text-gray-900">{formatHours(totalHours)}</p>
      </div>

      <!-- Session Type Hours (first 2) -->
      {#each sessionTypes.slice(0, 2) as st}
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
          <div class="flex items-center gap-2 mb-2">
            <Briefcase size={18} style="color: {st.color}" />
            <h3 class="text-sm font-medium text-gray-500">{st.name} Hours</h3>
          </div>
          <p class="text-2xl font-bold" style="color: {st.color}">
            {formatHours(hoursByType[st.name] || 0)}
          </p>
        </div>
      {/each}
    </div>

    <!-- Pay Summary Card -->
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
      <div class="flex items-center gap-2 mb-3">
        <DollarSign size={18} class="text-green-600" />
        <h3 class="text-sm font-medium text-gray-500">{periodLabel} Pay</h3>
      </div>
      <p class="text-3xl font-bold text-green-600">{formatPay(displayPay)}</p>
    </div>

    <!-- Recent Sessions -->
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-2">
          <Calendar size={20} class="text-gray-500" />
          <h2 class="text-lg font-semibold text-gray-900">Recent Sessions</h2>
          {#if recentSessions.length > 0}
            <span class="text-sm text-gray-400">({recentSessions.length})</span>
          {/if}
        </div>
        {#if recentSessions.length > COLLAPSED_COUNT}
          <button
            class="flex items-center gap-1 text-sm text-indigo-600 hover:text-indigo-700 transition-colors"
            on:click={() => sessionsExpanded = !sessionsExpanded}
          >
            {#if sessionsExpanded}
              <ChevronUp size={16} />
              Show Less
            {:else}
              <ChevronDown size={16} />
              Show All
            {/if}
          </button>
        {/if}
      </div>
      {#if recentSessions.length === 0}
        <div class="text-center py-8">
          <Clock size={48} class="mx-auto mb-4 text-gray-300" />
          <p class="text-gray-500 mb-4">No sessions yet – start your first one!</p>
          <button 
            class="inline-flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
            on:click={handleAddSession}
          >
            <Plus size={18} />
            Add Session
          </button>
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="w-full text-left">
            <thead>
              <tr class="border-b border-gray-100">
                <th class="py-3 px-2 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none hover:text-gray-700" on:click={() => handleSort('date')}>
                  <span class="flex items-center gap-1">
                    Date
                    {#if sortField === 'date'}
                      {#if sortDirection === 'asc'}<ArrowUp size={12} />{:else}<ArrowDown size={12} />{/if}
                    {:else}
                      <ArrowUpDown size={12} class="text-gray-300" />
                    {/if}
                  </span>
                </th>
                <th class="py-3 px-2 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none hover:text-gray-700" on:click={() => handleSort('type')}>
                  <span class="flex items-center gap-1">
                    Type
                    {#if sortField === 'type'}
                      {#if sortDirection === 'asc'}<ArrowUp size={12} />{:else}<ArrowDown size={12} />{/if}
                    {:else}
                      <ArrowUpDown size={12} class="text-gray-300" />
                    {/if}
                  </span>
                </th>
                <th class="py-3 px-2 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none hover:text-gray-700" on:click={() => handleSort('project_name')}>
                  <span class="flex items-center gap-1">
                    Project
                    {#if sortField === 'project_name'}
                      {#if sortDirection === 'asc'}<ArrowUp size={12} />{:else}<ArrowDown size={12} />{/if}
                    {:else}
                      <ArrowUpDown size={12} class="text-gray-300" />
                    {/if}
                  </span>
                </th>
                <th class="py-3 px-2 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none hover:text-gray-700" on:click={() => handleSort('hours')}>
                  <span class="flex items-center gap-1">
                    Hours
                    {#if sortField === 'hours'}
                      {#if sortDirection === 'asc'}<ArrowUp size={12} />{:else}<ArrowDown size={12} />{/if}
                    {:else}
                      <ArrowUpDown size={12} class="text-gray-300" />
                    {/if}
                  </span>
                </th>
                <th class="py-3 px-2 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none hover:text-gray-700" on:click={() => handleSort('pay')}>
                  <span class="flex items-center gap-1">
                    Pay
                    {#if sortField === 'pay'}
                      {#if sortDirection === 'asc'}<ArrowUp size={12} />{:else}<ArrowDown size={12} />{/if}
                    {:else}
                      <ArrowUpDown size={12} class="text-gray-300" />
                    {/if}
                  </span>
                </th>
              </tr>
            </thead>
            <tbody>
              {#each sessionsExpanded ? sortedSessions : sortedSessions.slice(0, COLLAPSED_COUNT) as session}
                <tr class="border-b border-gray-50 hover:bg-gray-50">
                  <td class="py-3 px-2 text-sm text-gray-700">{session.date}</td>
                  <td class="py-3 px-2">
                    <span class="px-2 py-1 text-xs font-medium rounded-full" style="background-color: {sessionTypes.find(st => st.id === session.session_type_id)?.color || '#6366F1'}15; color: {sessionTypes.find(st => st.id === session.session_type_id)?.color || '#6366F1'}">
                      {session.session_type_name || 'Unknown'}
                    </span>
                  </td>
                  <td class="py-3 px-2 text-sm text-gray-900 font-medium">{session.project_name}</td>
                  <td class="py-3 px-2 text-sm text-gray-700">{formatHours(session.hours)}</td>
                  <td class="py-3 px-2">
                    {#if calculateSessionPay(session) > 0}
                      <span class="text-sm font-medium text-green-600">{formatPay(calculateSessionPay(session))}</span>
                    {:else}
                      <span class="text-gray-400">–</span>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        {#if !sessionsExpanded && sortedSessions.length > COLLAPSED_COUNT}
          <div class="text-center pt-3 border-t border-gray-100 mt-3">
            <button
              class="text-sm text-gray-500 hover:text-indigo-600 transition-colors"
              on:click={() => sessionsExpanded = true}
            >
              +{sortedSessions.length - COLLAPSED_COUNT} more sessions
            </button>
          </div>
        {/if}
      {/if}
    </div>
  {/if}

  <!-- Footer -->
  <div class="text-center pt-8 pb-4">
    <a 
      href="https://skyvaultex.com" 
      target="_blank" 
      rel="noopener noreferrer"
      class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
    >
      Powered by SkyVaultex
    </a>
  </div>
</div>

<!-- Voice Input Modal -->
{#if showVoiceInput}
  <VoiceInput 
    onClose={() => showVoiceInput = false}
    on:saved={handleVoiceSaved}
  />
{/if}
