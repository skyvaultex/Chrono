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
  import { getAchievements, checkAndUnlockAchievements, logAppEvent } from './lib/api';
  import type { FeatureLimits } from './lib/types';
  import { featureLimits, initializeLicense } from './lib/stores/license';
  import { theme } from './lib/stores/theme';
  import { t } from './lib/stores/i18n';

  type Tab = 'dashboard' | 'sessions' | 'goals' | 'analytics' | 'advisor' | 'achievements' | 'simulator' | 'focus' | 'invoices' | 'settings';
  let activeTab: Tab = 'dashboard';
  let showMoreMenu = false;
  let allAchievementsUnlocked = false;
  
  // Feature limits for gating - now reactive from store
  $: limits = $featureLimits;
  let showProModal = false;
  let proModalFeature = '';
  let proModalDescription = '';

  onMount(async () => {
    // Initialize theme
    theme.init();
    
    // Load feature limits from store
    try {
      await initializeLicense();
      console.log('License initialized:', $featureLimits);
    } catch (e) {
      console.error('Failed to initialize license:', e);
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

<div class="h-screen flex flex-col bg-[var(--color-bg)]">
  <!-- Top Navigation -->
  <nav class="bg-[var(--color-bg-nav)] border-b border-[var(--color-border)]">
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
          <h1 class="text-2xl font-bold text-[var(--color-text)]">Chrono</h1>
        {/if}
      </button>

      <div class="flex space-x-1">
        <button
          class="btn {activeTab === 'dashboard' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('dashboard')}
        >
          <LayoutDashboard size={18} class="mr-1" />
          {$t('nav.dashboard')}
        </button>
        <button
          class="btn {activeTab === 'sessions' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('sessions')}
        >
          <Clock size={18} class="mr-1" />
          {$t('nav.sessions')}
        </button>
        <button
          class="btn {activeTab === 'goals' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('goals')}
        >
          <Target size={18} class="mr-1" />
          {$t('nav.goals')}
        </button>
        <button
          class="btn {activeTab === 'analytics' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('analytics')}
        >
          <BarChart3 size={18} class="mr-1" />
          {$t('nav.analytics')}
        </button>
        <button
          class="btn {activeTab === 'advisor' ? 'nav-active' : 'variant-ghost'}"
          on:click={() => setTab('advisor')}
        >
          <Lightbulb size={18} class="mr-1" />
          {$t('nav.advisor')}
        </button>

        <!-- Tools dropdown for additional features -->
        <div class="relative">
          <button
            class="btn {['simulator', 'focus', 'invoices', 'achievements', 'settings'].includes(activeTab) ? 'nav-active' : 'variant-ghost'}"
            on:click={() => showMoreMenu = !showMoreMenu}
          >
            <ChevronDown size={18} class="mr-1" />
            {$t('nav.tools')}
          </button>
          
          {#if showMoreMenu}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div 
              class="absolute right-0 mt-1 w-48 bg-[var(--color-card)] rounded-lg shadow-lg border border-[var(--color-border)] py-1 z-50"
              on:mouseleave={() => showMoreMenu = false}
            >
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-[var(--color-hover)] {activeTab === 'achievements' ? 'bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400' : 'text-[var(--color-text-secondary)]'}"
                on:click={() => { setTab('achievements'); showMoreMenu = false; }}
              >
                <Award size={16} class="mr-2" />
                {$t('nav.milestones')}
              </button>
              <button
                class="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-[var(--color-hover)] {activeTab === 'simulator' ? 'bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400' : 'text-[var(--color-text-secondary)]'}"
                on:click={() => limits?.has_simulator ? (setTab('simulator'), showMoreMenu = false) : showProFeatureModal('Financial Simulator', 'Project your earnings, set financial goals, and simulate different work scenarios to optimize your income.')}
              >
                <span class="flex items-center">
                  <Calculator size={16} class="mr-2" />
                  {$t('nav.financialSimulator')}
                </span>
                {#if !limits?.has_simulator}
                  <Lock size={14} class="text-amber-500" />
                {/if}
              </button>
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-[var(--color-hover)] {activeTab === 'focus' ? 'bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400' : 'text-[var(--color-text-secondary)]'}"
                on:click={() => { setTab('focus'); showMoreMenu = false; }}
              >
                <Brain size={16} class="mr-2" />
                {$t('nav.focusWellbeing')}
              </button>
              <button
                class="w-full flex items-center justify-between px-4 py-2 text-sm hover:bg-[var(--color-hover)] {activeTab === 'invoices' ? 'bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400' : 'text-[var(--color-text-secondary)]'}"
                on:click={() => limits?.has_invoices ? (setTab('invoices'), showMoreMenu = false) : showProFeatureModal('Invoices & Billing', 'Create professional invoices, track payments, and export PDF invoices to send to clients.')}
              >
                <span class="flex items-center">
                  <FileText size={16} class="mr-2" />
                  {$t('nav.invoices')}
                </span>
                {#if !limits?.has_invoices}
                  <Lock size={14} class="text-amber-500" />
                {/if}
              </button>
              <div class="border-t border-[var(--color-border)] my-1"></div>
              <button
                class="w-full flex items-center px-4 py-2 text-sm hover:bg-[var(--color-hover)] {activeTab === 'settings' ? 'bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400' : 'text-[var(--color-text-secondary)]'}"
                on:click={() => { setTab('settings'); showMoreMenu = false; }}
              >
                <Settings size={16} class="mr-2" />
                {$t('nav.settings')}
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
