<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllSessions, getSessionsByTypeId, deleteSession, getAllSessionTypes, addSessionType, deleteSessionType, canCreateSessionType } from '../api';
  import type { WorkSession, SessionType, NewSessionType } from '../types';
  import { PayType } from '../types';
  import SessionForm from './SessionForm.svelte';
  import VoiceInput from './VoiceInput.svelte';
  import UpgradePrompt from './UpgradePrompt.svelte';
  import { Plus, Pencil, Trash2, ArrowUpDown, ArrowUp, ArrowDown, Clock, Settings, X, Mic } from 'lucide-svelte';

  let sessions: WorkSession[] = [];
  let filteredSessions: WorkSession[] = [];
  let sessionTypes: SessionType[] = [];
  let loading = true;
  let error = '';
  let activeTypeId: number | null = null;
  let showForm = false;
  let editingSession: WorkSession | null = null;
  let showTypeManager = false;
  let showVoiceInput = false;
  let newTypeName = '';
  let newTypeColor = '#6366F1';
  
  // Upgrade prompt state
  let showUpgradePrompt = false;
  let upgradeLimitCurrent = 0;
  let upgradeLimitMax = 0;

  type SortField = 'date' | 'project_name' | 'hours' | 'pay';
  type SortDirection = 'asc' | 'desc';
  let sortField: SortField = 'date';
  let sortDirection: SortDirection = 'desc';

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      loading = true;
      [sessions, sessionTypes] = await Promise.all([
        getAllSessions(),
        getAllSessionTypes()
      ]);
      if (sessionTypes.length > 0 && activeTypeId === null) {
        activeTypeId = sessionTypes[0].id;
      }
      filterAndSortSessions();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function filterAndSortSessions() {
    let filtered = activeTypeId 
      ? sessions.filter(s => s.session_type_id === activeTypeId)
      : sessions;

    filtered.sort((a, b) => {
      let comparison = 0;
      switch (sortField) {
        case 'date':
          comparison = a.date.localeCompare(b.date);
          break;
        case 'project_name':
          comparison = a.project_name.localeCompare(b.project_name);
          break;
        case 'hours':
          comparison = a.hours - b.hours;
          break;
        case 'pay':
          comparison = calculatePay(a) - calculatePay(b);
          break;
      }
      return sortDirection === 'asc' ? comparison : -comparison;
    });

    filteredSessions = filtered;
  }

  function calculatePay(session: WorkSession): number {
    if (session.pay_type === PayType.Hourly && session.hourly_rate) {
      return session.hourly_rate * session.hours;
    }
    if (session.pay_type === PayType.Fixed && session.fixed_amount) {
      return session.fixed_amount;
    }
    return 0;
  }

  function handleSort(field: SortField) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = 'desc';
    }
    filterAndSortSessions();
  }

  function handleTabChange(typeId: number) {
    activeTypeId = typeId;
    filterAndSortSessions();
  }

  function handleAddSession() {
    editingSession = null;
    showForm = true;
  }

  function handleEditSession(session: WorkSession) {
    editingSession = session;
    showForm = true;
  }

  async function handleDeleteSession(session: WorkSession) {
    if (confirm(`Delete session "${session.project_name}"?`)) {
      try {
        await deleteSession(session.id);
        await loadData();
      } catch (e) {
        error = String(e);
      }
    }
  }

  function handleFormClose() {
    showForm = false;
    editingSession = null;
    loadData();
  }

  async function handleAddType() {
    if (!newTypeName.trim()) return;
    
    // Check limit before adding
    try {
      const check = await canCreateSessionType();
      if (!check.allowed) {
        upgradeLimitCurrent = check.current;
        upgradeLimitMax = check.limit || 0;
        showUpgradePrompt = true;
        return;
      }
    } catch (e) {
      console.error('Failed to check limit:', e);
    }
    
    try {
      const newType: NewSessionType = {
        name: newTypeName.trim(),
        color: newTypeColor,
        hourly_rate: undefined
      };
      await addSessionType(newType);
      newTypeName = '';
      newTypeColor = '#6366F1';
      await loadData();
    } catch (e) {
      error = String(e);
    }
  }

  async function handleDeleteType(id: number) {
    if (confirm('Delete this session type? Sessions of this type will become unassigned.')) {
      try {
        await deleteSessionType(id);
        if (activeTypeId === id) {
          activeTypeId = sessionTypes.find(t => t.id !== id)?.id || null;
        }
        await loadData();
      } catch (e) {
        error = String(e);
      }
    }
  }

  function formatHours(hours: number): string {
    return hours.toFixed(1);
  }

  function formatPay(amount: number): string {
    return `$${amount.toFixed(2)}`;
  }

  function getActiveType(): SessionType | undefined {
    return sessionTypes.find(t => t.id === activeTypeId);
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-blue-100 rounded-lg">
        <Clock size={24} class="text-blue-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Work Sessions</h1>
        <p class="text-gray-500 text-sm">Track and manage your work</p>
      </div>
    </div>
    <div class="flex gap-2">
      <button 
        class="flex items-center gap-2 px-3 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
        on:click={() => showVoiceInput = true}
        title="Voice Entry"
      >
        <Mic size={18} class="text-indigo-600" />
      </button>
      <button 
        class="flex items-center gap-2 px-3 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
        on:click={() => showTypeManager = !showTypeManager}
      >
        <Settings size={18} class="text-gray-600" />
        <span class="text-sm font-medium text-gray-700">Types</span>
      </button>
      <button 
        class="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        on:click={handleAddSession}
      >
        <Plus size={18} />
        Add Session
      </button>
    </div>
  </div>

  {#if error}
    <div class="alert variant-filled-error">{error}</div>
  {/if}

  <!-- Type Manager Panel -->
  {#if showTypeManager}
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
      <h3 class="font-semibold text-gray-900">Session Types</h3>
      <div class="flex gap-2 flex-wrap">
        {#each sessionTypes as type}
          <div class="flex items-center gap-1 px-3 py-1.5 rounded-full border" style="background-color: {type.color}10; border-color: {type.color}30">
            <span class="text-sm font-medium" style="color: {type.color}">{type.name}</span>
            <button class="p-1 hover:bg-gray-200 rounded-full transition-colors" on:click={() => handleDeleteType(type.id)}>
              <X size={14} class="text-gray-500" />
            </button>
          </div>
        {/each}
      </div>
      <div class="flex gap-2 items-center">
        <input
          type="text"
          class="flex-1 px-3 py-2 border border-gray-200 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
          placeholder="New type name..."
          bind:value={newTypeName}
        />
        <input
          type="color"
          class="w-10 h-10 rounded-lg cursor-pointer border border-gray-200"
          bind:value={newTypeColor}
        />
        <button 
          class="p-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
          on:click={handleAddType}
        >
          <Plus size={18} />
        </button>
      </div>
    </div>
  {/if}

  <!-- Tabs -->
  <div class="flex space-x-2 border-b border-gray-200">
    {#each sessionTypes as type}
      <button
        class="px-4 py-2 flex items-center gap-2 {activeTypeId === type.id ? 'border-b-2 font-medium' : ''}"
        style="{activeTypeId === type.id ? `border-color: ${type.color}; color: ${type.color};` : 'color: #6B7280;'}"
        on:click={() => handleTabChange(type.id)}
      >
        <span class="w-2 h-2 rounded-full" style="background-color: {type.color}"></span>
        {type.name}
      </button>
    {/each}
  </div>

  <!-- Sessions Table -->
  {#if loading}
    <div class="bg-white rounded-xl p-12 shadow-sm border border-gray-100 text-center">
      <div class="animate-spin w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full mx-auto mb-3"></div>
      <p class="text-gray-500">Loading sessions...</p>
    </div>
  {:else if filteredSessions.length === 0}
    <div class="bg-white rounded-xl p-12 shadow-sm border border-gray-100 text-center">
      <Clock size={48} class="mx-auto mb-4 text-gray-300" />
      <p class="text-gray-500 mb-2">No {getActiveType()?.name.toLowerCase() || ''} sessions yet.</p>
      <p class="text-sm text-gray-400 mb-4">Track your work to see progress here!</p>
      <button 
        class="inline-flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        on:click={handleAddSession}
      >
        <Plus size={18} />
        Add Your First Session
      </button>
    </div>
  {:else}
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full text-left">
          <thead>
            <tr class="border-b border-gray-100">
              <th class="py-3 px-4 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none" on:click={() => handleSort('date')}>
                <span class="flex items-center gap-1">
                  Date
                  {#if sortField === 'date'}
                    {#if sortDirection === 'asc'}<ArrowUp size={14} />{:else}<ArrowDown size={14} />{/if}
                  {:else}
                    <ArrowUpDown size={14} class="text-gray-400" />
                  {/if}
                </span>
              </th>
              <th class="py-3 px-4 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none" on:click={() => handleSort('project_name')}>
                <span class="flex items-center gap-1">
                  Project
                  {#if sortField === 'project_name'}
                    {#if sortDirection === 'asc'}<ArrowUp size={14} />{:else}<ArrowDown size={14} />{/if}
                  {:else}
                    <ArrowUpDown size={14} class="text-gray-400" />
                  {/if}
                </span>
              </th>
              <th class="py-3 px-4 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none" on:click={() => handleSort('hours')}>
                <span class="flex items-center gap-1">
                  Hours
                  {#if sortField === 'hours'}
                    {#if sortDirection === 'asc'}<ArrowUp size={14} />{:else}<ArrowDown size={14} />{/if}
                  {:else}
                    <ArrowUpDown size={14} class="text-gray-400" />
                  {/if}
                </span>
              </th>
              <th class="py-3 px-4 text-xs font-medium text-gray-500 uppercase tracking-wide cursor-pointer select-none" on:click={() => handleSort('pay')}>
                <span class="flex items-center gap-1">
                  Pay
                  {#if sortField === 'pay'}
                    {#if sortDirection === 'asc'}<ArrowUp size={14} />{:else}<ArrowDown size={14} />{/if}
                  {:else}
                    <ArrowUpDown size={14} class="text-gray-400" />
                  {/if}
                </span>
              </th>
              <th class="py-3 px-4 text-xs font-medium text-gray-500 uppercase tracking-wide w-24">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredSessions as session}
              <tr class="border-b border-gray-50 hover:bg-gray-50">
                <td class="py-3 px-4 text-sm text-gray-700">{session.date}</td>
                <td class="py-3 px-4 text-sm text-gray-900 font-medium">{session.project_name}</td>
                <td class="py-3 px-4 text-sm text-gray-700">{formatHours(session.hours)}</td>
                <td class="py-3 px-4">
                  {#if calculatePay(session) > 0}
                    <span class="text-sm font-medium text-green-600">{formatPay(calculatePay(session))}</span>
                  {:else}
                    <span class="text-sm text-gray-400">unpaid</span>
                  {/if}
                </td>
                <td class="py-3 px-4 flex gap-1">
                  <button
                    class="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-500 hover:text-indigo-600"
                    on:click={() => handleEditSession(session)}
                    title="Edit session"
                  >
                    <Pencil size={16} />
                  </button>
                  <button
                    class="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-500 hover:text-red-600"
                    on:click|stopPropagation={() => handleDeleteSession(session)}
                    title="Delete session"
                  >
                    <Trash2 size={16} />
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>

{#if showForm}
  <SessionForm
    session={editingSession}
    defaultTypeId={activeTypeId}
    {sessionTypes}
    on:close={handleFormClose}
  />
{/if}

{#if showVoiceInput}
  <VoiceInput 
    onClose={() => showVoiceInput = false}
    on:saved={loadData}
  />
{/if}

<UpgradePrompt 
  bind:show={showUpgradePrompt}
  feature="session_types"
  current={upgradeLimitCurrent}
  limit={upgradeLimitMax}
  on:close={() => showUpgradePrompt = false}
/>
