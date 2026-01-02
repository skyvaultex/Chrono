<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { addSession, updateSession, getCurrentDate, getProjectsByTypeId } from '../api';
  import type { WorkSession, NewSession, SessionType } from '../types';
  import { PayType } from '../types';
  import { X } from 'lucide-svelte';

  export let session: WorkSession | null = null;
  export let defaultTypeId: number | null = null;
  export let sessionTypes: SessionType[] = [];

  const dispatch = createEventDispatcher();

  let formData = {
    sessionTypeId: session?.session_type_id || defaultTypeId || (sessionTypes[0]?.id ?? 0),
    date: session?.date || '',
    projectName: session?.project_name || '',
    hours: session?.hours || 0,
    description: session?.description || '',
    payType: session?.pay_type || PayType.None,
    hourlyRate: session?.hourly_rate || 25,
    fixedAmount: session?.fixed_amount || 0,
  };

  let useCustomDate = false;
  let projects: string[] = [];
  let error = '';
  let loading = false;
  let submitted = false;

  onMount(async () => {
    if (!session) {
      formData.date = await getCurrentDate();
    } else {
      useCustomDate = true;
    }
    await loadProjects();
  });

  async function loadProjects() {
    try {
      if (formData.sessionTypeId) {
        projects = await getProjectsByTypeId(formData.sessionTypeId);
      }
    } catch (e) {
      console.error('Failed to load projects:', e);
    }
  }

  function handleTypeChange() {
    loadProjects();
  }

  async function handleSubmit() {
    if (loading || submitted) return;
    error = '';

    if (!formData.projectName.trim()) {
      error = 'Project name is required';
      return;
    }
    if (formData.hours < 0.1 || formData.hours > 24) {
      error = 'Hours must be between 0.1 and 24.0';
      return;
    }

    loading = true;
    submitted = true;

    try {
      if (session) {
        await updateSession({
          id: session.id,
          session_type_id: formData.sessionTypeId,
          session_type_name: sessionTypes.find(t => t.id === formData.sessionTypeId)?.name,
          date: formData.date,
          project_name: formData.projectName.trim(),
          hours: formData.hours,
          description: formData.description || undefined,
          pay_type: formData.payType,
          hourly_rate: formData.payType === PayType.Hourly ? formData.hourlyRate : undefined,
          fixed_amount: formData.payType === PayType.Fixed ? formData.fixedAmount : undefined,
        });
      } else {
        const newSession: NewSession = {
          session_type_id: formData.sessionTypeId,
          date: formData.date,
          project_name: formData.projectName.trim(),
          hours: formData.hours,
          description: formData.description || undefined,
          pay_type: formData.payType,
          hourly_rate: formData.payType === PayType.Hourly ? formData.hourlyRate : undefined,
          fixed_amount: formData.payType === PayType.Fixed ? formData.fixedAmount : undefined,
        };
        await addSession(newSession);
      }
      dispatch('close');
    } catch (e) {
      error = String(e);
      submitted = false;
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    if (!loading) dispatch('close');
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget && !loading) dispatch('close');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && !loading) dispatch('close');
  }

  function calculateEstimatedPay(): number {
    if (formData.payType === PayType.Hourly) {
      return formData.hourlyRate * formData.hours;
    }
    if (formData.payType === PayType.Fixed) {
      return formData.fixedAmount;
    }
    return 0;
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
        {session ? 'Edit Session' : 'Add New Session'}
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
      <!-- Session Type -->
      <label class="label">
        <span>Session Type</span>
        <select
          class="select"
          bind:value={formData.sessionTypeId}
          on:change={handleTypeChange}
          disabled={loading}
        >
          {#each sessionTypes as type}
            <option value={type.id}>{type.name}</option>
          {/each}
        </select>
      </label>

      <!-- Date -->
      <div class="space-y-2">
        <label class="flex items-center space-x-2">
          <input type="checkbox" class="checkbox" bind:checked={useCustomDate} disabled={loading} />
          <span>Use custom date</span>
        </label>
        {#if useCustomDate}
          <input type="date" class="input" bind:value={formData.date} disabled={loading} required />
        {:else}
          <input type="text" class="input" value={formData.date} disabled />
        {/if}
      </div>

      <!-- Project Name -->
      <label class="label">
        <span>Project Name</span>
        <input
          type="text"
          class="input"
          list="projects-list"
          bind:value={formData.projectName}
          placeholder="Enter or select project name"
          disabled={loading}
          required
        />
        {#if projects.length > 0}
          <datalist id="projects-list">
            {#each projects as project}
              <option value={project}></option>
            {/each}
          </datalist>
        {/if}
      </label>

      <!-- Hours -->
      <label class="label">
        <span>Hours Worked</span>
        <input
          type="number"
          class="input"
          bind:value={formData.hours}
          min="0.1"
          max="24"
          step="0.1"
          disabled={loading}
          required
        />
        <small class="text-gray-500">Between 0.1 and 24.0 hours</small>
      </label>

      <!-- Payment Type -->
      <label class="label">
        <span>Payment Type</span>
        <select class="select" bind:value={formData.payType} disabled={loading}>
          <option value={PayType.None}>Unpaid (Learning/Portfolio)</option>
          <option value={PayType.Hourly}>Hourly Rate</option>
          <option value={PayType.Fixed}>Fixed Project Price</option>
        </select>
      </label>

      <!-- Hourly Rate -->
      {#if formData.payType === PayType.Hourly}
        <label class="label">
          <span>Hourly Rate ($)</span>
          <input
            type="number"
            class="input"
            bind:value={formData.hourlyRate}
            min="1"
            step="0.5"
            placeholder="25"
            disabled={loading}
          />
        </label>
      {/if}

      <!-- Fixed Amount -->
      {#if formData.payType === PayType.Fixed}
        <label class="label">
          <span>Project Amount ($)</span>
          <input
            type="number"
            class="input"
            bind:value={formData.fixedAmount}
            min="1"
            step="1"
            placeholder="500"
            disabled={loading}
          />
        </label>
      {/if}

      <!-- Description -->
      <label class="label">
        <span>Description (optional)</span>
        <textarea
          class="textarea"
          bind:value={formData.description}
          rows="3"
          placeholder="Brief description of what you worked on..."
          disabled={loading}
        ></textarea>
      </label>

      <!-- Pay Preview -->
      {#if formData.payType !== PayType.None && formData.hours > 0}
        <div class="card variant-soft-success p-3">
          <span class="text-sm">Estimated Pay:</span>
          <span class="text-lg font-bold ml-2 text-green-600">
            ${calculateEstimatedPay().toFixed(2)}
          </span>
        </div>
      {/if}

      <!-- Actions -->
      <div class="flex justify-end space-x-2 pt-4">
        <button type="button" class="btn variant-ghost" on:click={handleCancel} disabled={loading}>
          Cancel
        </button>
        <button type="submit" class="btn variant-filled-primary" disabled={loading || submitted}>
          {#if loading}
            Saving...
          {:else}
            {session ? 'Update' : 'Add'}
          {/if}
        </button>
      </div>
    </form>
  </div>
</div>
