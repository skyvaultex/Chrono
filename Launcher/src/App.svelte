<script lang="ts">
  import './app.css';
  import { onMount } from 'svelte';

  import Dashboard from './lib/components/Dashboard.svelte';
  import Sessions from './lib/components/Sessions.svelte';
  import Goals from './lib/components/Goals.svelte';
  import Analytics from './lib/components/Analytics.svelte';
  import Advisor from './lib/components/Advisor.svelte';
  import Achievements from './lib/components/Achievements.svelte';
  import Simulator from './lib/components/Simulator.svelte';
  import FocusWellbeing from './lib/components/FocusWellbeing.svelte';
  import Invoices from './lib/components/Invoices.svelte';
  import SettingsPage from './lib/components/SettingsPage.svelte';
  import ProFeatureModal from './lib/components/ProFeatureModal.svelte';
  import { LayoutDashboard, Clock, Target, Lightbulb, Timer, BarChart3, Calculator, Brain, FileText, ChevronDown, Award, Settings, Lock } from 'lucide-svelte';
  import { getAchievements, checkAndUnlockAchievements, logAppEvent, getFeatureLimits } from './lib/api';
  import type { FeatureLimits } from './lib/types';

  type Tab = 'dashboard' | 'sessions' | 'goals' | 'analytics' | 'advisor' | 'achievements' | 'simulator' | 'focus' | 'invoices' | 'settings';
  let activeTab: Tab = 'dashboard';
  let showMoreMenu = false;
  let allAchievementsUnlocked = false;
  
  // Feature limits for gating
  let limits: FeatureLimits | null = null;
  let showProModal = false;
  let proModalFeature = '';
  let proModalDescription = '';

  onMount(async () => {
    // Load feature limits
    try {
      limits = await getFeatureLimits();
      console.log('Feature limits loaded:', limits);
    } catch (e) {
      console.error('Failed to load feature limits:', e);
    }
    
    // Check if all achievements are unlocked for golden logo
    try {
      await checkAndUnlockAchievements();
      const achievements = await getAchievements();
      allAchievementsUnlocked = achievements.length > 0 && achievements.every(a => a.unlocked);
    } catch (e) {
      console.error('Failed to check achievements:', e);
    }
  });

  function showProFeatureModal(feature: string, description: string) {
    proModalFeature = feature;
    proModalDescription = description;
    showProModal = true;
    showMoreMenu = false;
  }

  async function setTab(tab: Tab) {
    activeTab = tab;
    
    // Log events for achievement tracking
    try {
      if (tab === 'analytics') {
        await logAppEvent('view_analytics');
      } else if (tab === 'advisor') {
        await logAppEvent('view_advisor');
      }
    } catch (e) {
      console.error('Failed to log event:', e);
    }
  }

  function handleAllUnlocked() {
    allAchievementsUnlocked = true;
  }
</script>

<div class="h-screen flex flex-col" style="background-color: #F5F5F5;">
  <!-- Top Navigation -->
  <nav class="bg-white border-b" style="border-color: #E5E7EB;">
    <div class="flex items-center justify-between px-6 py-3">
      <button 
        class="flex items-center space-x-3 hover:opacity-80 transition-opacity"
        on:click={() => setTab('dashboard')}
        title={allAchievementsUnlocked ? 'All milestones achieved! ✨' : 'Chrono'}
      >
        {#if allAchievementsUnlocked}
          <!-- Golden Logo with achievement badge -->
          <div class="relative">
            <img src="/icons/icon.png" alt="Chrono" class="w-8 h-8 rounded-lg" style="filter: sepia(1) saturate(3) hue-rotate(5deg) brightness(1.1);" />
            <div class="absolute -top-1 -right-1 w-3 h-3 bg-gradient-to-br from-yellow-400 to-amber-500 rounded-full border-2 border-white"></div>
          </div>
          <h1 class="text-2xl font-bold bg-gradient-to-r from-amber-500 to-yellow-600 bg-clip-text text-transparent">Chrono</h1>
        {:else}
          <img src="/icons/icon.png" alt="Chrono" class="w-8 h-8 rounded-lg" />
          <h1 class="text-2xl font-bold" style="color: #111827;">Chrono</h1>
        {/if}
      </button>

      <div class="flex space-x-1">
        <button
          class="btn {activeTab === 'dashboard' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('dashboard')}
        >
          <LayoutDashboard size={18} class="mr-1" />
          Dashboard
        </button>
        <button
          class="btn {activeTab === 'sessions' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('sessions')}
        >
          <Clock size={18} class="mr-1" />
          Sessions
        </button>
        <button
          class="btn {activeTab === 'goals' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('goals')}
        >
          <Target size={18} class="mr-1" />
          Goals
        </button>
        <button
          class="btn {activeTab === 'analytics' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('analytics')}
        >
          <BarChart3 size={18} class="mr-1" />
          Analytics
        </button>
        <button
          class="btn {activeTab === 'advisor' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('advisor')}
        >
          <Lightbulb size={18} class="mr-1" />
          Advisor
        </button>

        <!-- Tools dropdown for additional features -->
        <div class="relative">
          <button
            class="btn {['simulator', 'focus', 'invoices', 'achievements', 'settings'].includes(activeTab) ? 'nav-active' : 'variant-ghost'}"
            on:click={() => showMoreMenu = !showMoreMenu}
          >
            <ChevronDown size={18} class="mr-1" />
            Tools
          </button>
          
          {#if showMoreMenu}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div 
              class="absolute right-0 mt-1 w-48 bg-white rounded-lg shadow-lg border py-1 z-50"
              on:mouseleave={() => showMoreMenu = false}
            >
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-gray-100 {activeTab === 'achievements' ? 'bg-indigo-50 text-indigo-600' : 'text-gray-700'}"
                on:click={() => { setTab('achievements'); showMoreMenu = false; }}
              >
                <Award size={16} class="mr-2" />
                Milestones
              </button>
              <button
                class="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-gray-100 {activeTab === 'simulator' ? 'bg-indigo-50 text-indigo-600' : 'text-gray-700'}"
                on:click={() => limits?.has_simulator ? (setTab('simulator'), showMoreMenu = false) : showProFeatureModal('Financial Simulator', 'Project your earnings, set financial goals, and simulate different work scenarios to optimize your income.')}
              >
                <span class="flex items-center">
                  <Calculator size={16} class="mr-2" />
                  Financial Simulator
                </span>
                {#if !limits?.has_simulator}
                  <Lock size={14} class="text-amber-500" />
                {/if}
              </button>
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-gray-100 {activeTab === 'focus' ? 'bg-indigo-50 text-indigo-600' : 'text-gray-700'}"
                on:click={() => { setTab('focus'); showMoreMenu = false; }}
              >
                <Brain size={16} class="mr-2" />
                Focus & Wellbeing
              </button>
              <button
                class="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-gray-100 {activeTab === 'invoices' ? 'bg-indigo-50 text-indigo-600' : 'text-gray-700'}"
                on:click={() => limits?.has_invoices ? (setTab('invoices'), showMoreMenu = false) : showProFeatureModal('Invoices & Billing', 'Create professional invoices, track payments, and export PDF invoices to send to clients.')}
              >
                <span class="flex items-center">
                  <FileText size={16} class="mr-2" />
                  Invoices
                </span>
                {#if !limits?.has_invoices}
                  <Lock size={14} class="text-amber-500" />
                {/if}
              </button>
              <div class="border-t my-1"></div>
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-gray-100 {activeTab === 'settings' ? 'bg-indigo-50 text-indigo-600' : 'text-gray-700'}"
                on:click={() => { setTab('settings'); showMoreMenu = false; }}
              >
                <Settings size={16} class="mr-2" />
                Settings
              </button>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </nav>

  <!-- Main Content -->
  <main class="flex-1 overflow-y-auto">
    {#if activeTab === 'dashboard'}
      <Dashboard on:navigate={(e) => setTab(e.detail.tab)} />
    {:else if activeTab === 'sessions'}
      <Sessions />
    {:else if activeTab === 'goals'}
      <Goals />
    {:else if activeTab === 'analytics'}
      <Analytics />
    {:else if activeTab === 'advisor'}
      <Advisor />
    {:else if activeTab === 'achievements'}
      <Achievements on:allUnlocked={handleAllUnlocked} />
    {:else if activeTab === 'simulator'}
      <Simulator />
    {:else if activeTab === 'focus'}
      <FocusWellbeing />
    {:else if activeTab === 'invoices'}
      <Invoices />
    {:else if activeTab === 'settings'}
      <SettingsPage />
    {/if}
  </main>
</div>

<!-- Pro Feature Modal -->
<ProFeatureModal 
  bind:show={showProModal} 
  featureName={proModalFeature}
  featureDescription={proModalDescription}
/>
