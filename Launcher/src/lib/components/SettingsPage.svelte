<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings, Key, Check, X, Crown, Sparkles, ExternalLink, RefreshCw, Download, Globe } from 'lucide-svelte';
  import { getCurrentUsage, checkForUpdate, installUpdate } from '../api';
  import type { CurrentUsage } from '../types';
  import type { UpdateInfo } from '../api';
  import { Tier } from '../types';
  import { license as licenseStore, featureLimits, activateLicenseKey, deactivateLicenseKey } from '../stores/license';
  import { language, languages, t, type Language } from '../stores/i18n';

  // Subscribe to store
  $: license = $licenseStore;
  $: limits = $featureLimits;
  
  let usage: CurrentUsage | null = null;
  let loading = true;
  let error = '';
  
  // License key input
  let licenseKey = '';
  let activating = false;
  let activationError = '';
  let activationSuccess = false;

  // Update state
  let updateInfo: UpdateInfo | null = null;
  let checkingUpdate = false;
  let installingUpdate = false;
  let updateError = '';

  function handleLanguageChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    language.set(target.value as Language);
  }

  onMount(async () => {
    await loadData();
    // Check for updates on mount
    await handleCheckUpdate(true);
  });

  async function loadData() {
    try {
      loading = true;
      usage = await getCurrentUsage();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleActivate() {
    if (!licenseKey.trim()) {
      activationError = 'Please enter a license key';
      return;
    }
    
    activating = true;
    activationError = '';
    activationSuccess = false;
    
    try {
      const result = await activateLicenseKey(licenseKey);
      if (result.success) {
        activationSuccess = true;
        licenseKey = '';
        // Reload usage to reflect new limits
        usage = await getCurrentUsage();
      } else {
        activationError = result.error?.replace('Error: ', '') || 'Activation failed';
      }
    } catch (e) {
      activationError = String(e).replace('Error: ', '');
    } finally {
      activating = false;
    }
  }

  async function handleDeactivate() {
    if (confirm('Are you sure you want to deactivate your license? You will return to the Free plan.')) {
      try {
        await deactivateLicenseKey();
        usage = await getCurrentUsage();
      } catch (e) {
        error = String(e);
      }
    }
  }

  async function handleCheckUpdate(silent = false) {
    checkingUpdate = true;
    updateError = '';
    
    try {
      updateInfo = await checkForUpdate();
    } catch (e) {
      if (!silent) {
        updateError = String(e).replace('Error: ', '');
      }
    } finally {
      checkingUpdate = false;
    }
  }

  async function handleInstallUpdate() {
    installingUpdate = true;
    updateError = '';
    
    try {
      await installUpdate();
      // App will restart after install
    } catch (e) {
      updateError = String(e).replace('Error: ', '');
    } finally {
      installingUpdate = false;
    }
  }

  function getTierIcon(tier: Tier) {
    switch (tier) {
      case Tier.Lifetime: return Crown;
      case Tier.Pro: return Sparkles;
      default: return Settings;
    }
  }

  function getTierColor(tier: Tier): string {
    switch (tier) {
      case Tier.Lifetime: return 'amber';
      case Tier.Pro: return 'indigo';
      default: return 'gray';
    }
  }

  const proFeatures = [
    { name: 'Unlimited session types', free: '2 max', pro: 'Unlimited' },
    { name: 'Unlimited goals', free: '3 max', pro: 'Unlimited' },
    { name: 'Analytics history', free: '7 days', pro: 'Full history' },
    { name: 'Invoice generation', free: '❌', pro: '✅' },
    { name: 'AI Financial Advisor', free: '❌', pro: '✅' },
    { name: 'Voice input', free: '❌', pro: '✅' },
    { name: 'Earnings simulator', free: '❌', pro: '✅' },
    { name: 'PDF export', free: '❌', pro: '✅' },
  ];

  // LemonSqueezy checkout URLs
  const checkoutUrls = {
    monthly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/2163691e-c198-499f-b94e-3cbd188f2591',
    yearly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/cf7a46b3-4578-4320-bd56-5d50afd931b8',
  };

  let showUpgradeOptions = false;
</script>

<div class="p-6 space-y-6 max-w-4xl mx-auto">
  <!-- Header -->
  <div class="flex items-center gap-3">
    <div class="p-2 bg-gray-100 rounded-lg">
      <Settings size={24} class="text-gray-400" />
    </div>
    <div>
      <h1 class="text-2xl font-bold text-gray-900">{$t('settings.title')}</h1>
      <p class="text-gray-400 text-sm">{$t('settings.subtitle')}</p>
    </div>
  </div>

  {#if loading}
    <div class="text-center py-8 text-gray-400">{$t('common.loading')}</div>
  {:else if error}
    <div class="bg-red-50 text-red-600 p-4 rounded-lg">{error}</div>
  {:else}
    <!-- Language Section -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{$t('settings.language')}</h2>
      
      <div class="relative">
        <Globe size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
        <select
          class="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 appearance-none cursor-pointer"
          value={$language}
          on:change={handleLanguageChange}
        >
          {#each languages as lang}
            <option value={lang.code}>{lang.nativeName} ({lang.name})</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- Current Plan -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{$t('settings.currentPlan')}</h2>
      
      <div class="flex items-center justify-between p-4 rounded-xl bg-{getTierColor(license.tier)}-50 border border-{getTierColor(license.tier)}-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-{getTierColor(license.tier)}-100 rounded-lg">
            <svelte:component this={getTierIcon(license.tier)} size={24} class="text-{getTierColor(license.tier)}-600" />
          </div>
          <div>
            <h3 class="font-semibold text-gray-900">
              {license.tier === Tier.Free ? $t('settings.freePlan') : license.tier === Tier.Pro ? $t('settings.proPlan') : $t('settings.lifetimePlan')}
            </h3>
            {#if license.activated_at}
              <p class="text-sm text-gray-400">{$t('settings.activated')}: {new Date(license.activated_at).toLocaleDateString()}</p>
            {/if}
          </div>
        </div>
        
        {#if license.tier !== Tier.Free}
          <button
            class="text-sm text-gray-400 hover:text-red-600 transition-colors"
            on:click={handleDeactivate}
          >
            {$t('settings.deactivate')}
          </button>
        {/if}
      </div>

      <!-- Usage -->
      {#if usage && limits}
        <div class="mt-4 grid grid-cols-2 gap-4">
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="text-sm text-gray-400">{$t('sessions.sessionType')}</div>
            <div class="font-semibold text-gray-900">
              {usage.session_type_count}{limits.max_session_types ? ` / ${limits.max_session_types}` : ` (${$t('common.unlimited').toLowerCase()})`}
            </div>
          </div>
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="text-sm text-gray-400">{$t('nav.goals')}</div>
            <div class="font-semibold text-gray-900">
              {usage.goal_count}{limits.max_goals ? ` / ${limits.max_goals}` : ` (${$t('common.unlimited').toLowerCase()})`}
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- License Activation -->
    {#if license.tier === Tier.Free}
      <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">{$t('settings.activateLicense')}</h2>
        
        <div class="flex gap-3">
          <div class="relative flex-1">
            <Key size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              bind:value={licenseKey}
              placeholder="PRO-XXXX-XXXX-XXXX or LIFE-XXXX-XXXX-XXXX"
              class="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
              on:keydown={(e) => e.key === 'Enter' && handleActivate()}
            />
          </div>
          <button
            class="px-4 py-2.5 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            on:click={handleActivate}
            disabled={activating}
          >
            {#if activating}
              <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
              {$t('settings.activating')}
            {:else}
              {$t('settings.activate')}
            {/if}
          </button>
        </div>
        
        {#if activationError}
          <p class="mt-2 text-sm text-red-600 flex items-center gap-1">
            <X size={14} />
            {activationError}
          </p>
        {/if}
        
        {#if activationSuccess}
          <p class="mt-2 text-sm text-green-600 flex items-center gap-1">
            <Check size={14} />
            {$t('settings.activationSuccess')}
          </p>
        {/if}
        
        <p class="mt-3 text-sm text-gray-400">
          Don't have a license? 
          <button 
            class="text-indigo-600 hover:underline inline-flex items-center gap-1"
            on:click={() => showUpgradeOptions = true}
          >
            Get one here <ExternalLink size={12} />
          </button>
        </p>
      </div>
    {/if}

    <!-- Feature Comparison -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{$t('settings.featureComparison')}</h2>
      
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b border-gray-200">
              <th class="text-left py-3 px-2 text-sm font-medium text-gray-400">{$t('settings.features')}</th>
              <th class="text-center py-3 px-2 text-sm font-medium text-gray-400">{$t('common.free')}</th>
              <th class="text-center py-3 px-2 text-sm font-medium text-indigo-600">{$t('common.pro')} / {$t('common.lifetime')}</th>
            </tr>
          </thead>
          <tbody>
            {#each proFeatures as feature}
              <tr class="border-b border-gray-100">
                <td class="py-3 px-2 text-sm text-gray-600">{feature.name}</td>
                <td class="py-3 px-2 text-center text-sm text-gray-400">{feature.free}</td>
                <td class="py-3 px-2 text-center text-sm text-indigo-600 font-medium">{feature.pro}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      {#if license.tier === Tier.Free}
        <div class="mt-6 p-4 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-xl text-white">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="font-semibold">{$t('settings.upgradeToPro')}</h3>
              <p class="text-sm text-white/80">{$t('settings.unlockFeatures')}</p>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold">$5<span class="text-sm font-normal">{$t('settings.perMonth')}</span></div>
              <div class="text-xs text-white/70">or $39{$t('settings.perYear')}</div>
            </div>
          </div>
          <button
            class="mt-4 w-full py-2.5 bg-white text-indigo-600 font-medium rounded-lg hover:bg-gray-50 transition-colors"
            on:click={() => showUpgradeOptions = true}
          >
            {$t('settings.upgradeToPro')}
          </button>
        </div>
      {/if}
    </div>

    <!-- About -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
      <h2 class="text-lg font-semibold text-gray-900 mb-2">{$t('settings.about')}</h2>
      <p class="text-sm text-gray-600 mb-4">
        {$t('settings.aboutDescription')}
      </p>
      <div class="text-sm text-gray-400">
        <div>{$t('settings.version')} {updateInfo?.current_version || '1.0.0'}</div>
        <div class="mt-1">
          Built with Tauri, Svelte, and Rust.
          <br />
          Your data stays 100% local. No account required.
        </div>
      </div>

      <!-- Updates Section -->
      <div class="mt-4 pt-4 border-t border-gray-200">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-gray-900">{$t('settings.updates')}</div>
            {#if updateInfo?.available}
              <div class="text-sm text-green-600">
                {$t('settings.version')} {updateInfo.latest_version} {$t('settings.updateAvailable').toLowerCase()}
              </div>
            {:else if updateInfo}
              <div class="text-sm text-gray-400">{$t('settings.upToDate')}</div>
            {/if}
          </div>
          
          <div class="flex gap-2">
            {#if updateInfo?.available}
              <button
                class="px-3 py-1.5 bg-green-600 text-white text-sm font-medium rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50 flex items-center gap-2"
                on:click={handleInstallUpdate}
                disabled={installingUpdate}
              >
                {#if installingUpdate}
                  <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                  {$t('settings.installing')}
                {:else}
                  <Download size={16} />
                  {$t('settings.installUpdate')}
                {/if}
              </button>
            {/if}
            <button
              class="px-3 py-1.5 bg-gray-100 text-gray-600 text-sm font-medium rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50 flex items-center gap-2"
              on:click={() => handleCheckUpdate(false)}
              disabled={checkingUpdate}
            >
              {#if checkingUpdate}
                <RefreshCw size={16} class="animate-spin" />
                {$t('settings.checking')}
              {:else}
                <RefreshCw size={16} />
                {$t('settings.checkForUpdates')}
              {/if}
            </button>
          </div>
        </div>
        
        {#if updateError}
          <p class="mt-2 text-sm text-red-600">{updateError}</p>
        {/if}
        
        {#if updateInfo?.available && updateInfo.release_notes}
          <div class="mt-3 p-3 bg-gray-50 rounded-lg">
            <div class="text-sm font-medium text-gray-600 mb-1">{$t('settings.whatsNew')}:</div>
            <div class="text-sm text-gray-400 whitespace-pre-wrap">{updateInfo.release_notes}</div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<!-- Upgrade Options Modal -->
{#if showUpgradeOptions}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
  <div 
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    role="dialog"
    aria-modal="true"
    on:click|self={() => showUpgradeOptions = false}
  >
    <div class="bg-white rounded-2xl shadow-xl max-w-md w-full overflow-hidden">
      <div class="p-6 bg-gradient-to-r from-indigo-500 to-purple-600 text-white">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold">{$t('settings.upgradeToPro')}</h2>
            <p class="text-white/80 text-sm">{$t('settings.selectLanguage').replace('language', 'plan')}</p>
          </div>
          <button 
            class="p-1 hover:bg-white/20 rounded-lg transition-colors"
            on:click={() => showUpgradeOptions = false}
          >
            <X size={20} />
          </button>
        </div>
      </div>
      
      <div class="p-6 space-y-4">
        <!-- Monthly -->
        <button
          class="w-full p-4 border-2 border-gray-200 rounded-xl hover:border-indigo-500 transition-colors text-left group"
          on:click={() => { window.open(checkoutUrls.monthly, '_blank'); showUpgradeOptions = false; }}
        >
          <div class="flex justify-between items-center">
            <div>
              <div class="font-semibold text-gray-900">{$t('settings.monthly')}</div>
              <div class="text-sm text-gray-400">{$t('settings.billedMonthly')}</div>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-gray-900">$5</div>
              <div class="text-xs text-gray-400">{$t('settings.perMonth')}</div>
            </div>
          </div>
        </button>

        <!-- Yearly -->
        <button
          class="w-full p-4 border-2 border-indigo-500 rounded-xl hover:bg-indigo-50 transition-colors text-left relative"
          on:click={() => { window.open(checkoutUrls.yearly, '_blank'); showUpgradeOptions = false; }}
        >
          <div class="absolute -top-2.5 left-4 px-2 py-0.5 bg-indigo-500 text-white text-xs font-medium rounded-full">
            {$t('settings.save35')}
          </div>
          <div class="flex justify-between items-center">
            <div>
              <div class="font-semibold text-gray-900">{$t('settings.yearly')}</div>
              <div class="text-sm text-gray-400">{$t('settings.billedYearly')}</div>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-indigo-600">$39</div>
              <div class="text-xs text-gray-400">{$t('settings.perYear')}</div>
            </div>
          </div>
        </button>

        <p class="text-xs text-gray-400 text-center pt-2">
          After purchase, you'll receive a license key via email.<br />
          Enter it above to activate Pro features.
        </p>
      </div>
    </div>
  </div>
{/if}
