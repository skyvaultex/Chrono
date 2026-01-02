<script lang="ts">
  import { onMount } from 'svelte';
  import { Settings, Key, Check, X, Crown, Sparkles, ExternalLink } from 'lucide-svelte';
  import { getLicense, getFeatureLimits, getCurrentUsage, activateLicense, deactivateLicense } from '../api';
  import type { License, FeatureLimits, CurrentUsage } from '../types';
  import { Tier } from '../types';

  let license: License = { tier: Tier.Free };
  let limits: FeatureLimits | null = null;
  let usage: CurrentUsage | null = null;
  let loading = true;
  let error = '';
  
  // License key input
  let licenseKey = '';
  let activating = false;
  let activationError = '';
  let activationSuccess = false;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      loading = true;
      [license, limits, usage] = await Promise.all([
        getLicense(),
        getFeatureLimits(),
        getCurrentUsage()
      ]);
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
      license = await activateLicense(licenseKey);
      limits = await getFeatureLimits();
      activationSuccess = true;
      licenseKey = '';
    } catch (e) {
      activationError = String(e).replace('Error: ', '');
    } finally {
      activating = false;
    }
  }

  async function handleDeactivate() {
    if (confirm('Are you sure you want to deactivate your license? You will return to the Free plan.')) {
      try {
        license = await deactivateLicense();
        limits = await getFeatureLimits();
      } catch (e) {
        error = String(e);
      }
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
      <Settings size={24} class="text-gray-600" />
    </div>
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
      <p class="text-gray-500 text-sm">Manage your plan and preferences</p>
    </div>
  </div>

  {#if loading}
    <div class="text-center py-8 text-gray-500">Loading...</div>
  {:else if error}
    <div class="bg-red-50 text-red-600 p-4 rounded-lg">{error}</div>
  {:else}
    <!-- Current Plan -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Current Plan</h2>
      
      <div class="flex items-center justify-between p-4 rounded-xl bg-{getTierColor(license.tier)}-50 border border-{getTierColor(license.tier)}-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-{getTierColor(license.tier)}-100 rounded-lg">
            <svelte:component this={getTierIcon(license.tier)} size={24} class="text-{getTierColor(license.tier)}-600" />
          </div>
          <div>
            <h3 class="font-semibold text-gray-900">
              {license.tier === Tier.Free ? 'Free Plan' : license.tier === Tier.Pro ? 'Pro Plan' : 'Lifetime Plan'}
            </h3>
            {#if license.activated_at}
              <p class="text-sm text-gray-500">Activated: {new Date(license.activated_at).toLocaleDateString()}</p>
            {/if}
          </div>
        </div>
        
        {#if license.tier !== Tier.Free}
          <button
            class="text-sm text-gray-500 hover:text-red-600 transition-colors"
            on:click={handleDeactivate}
          >
            Deactivate
          </button>
        {/if}
      </div>

      <!-- Usage -->
      {#if usage && limits}
        <div class="mt-4 grid grid-cols-2 gap-4">
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="text-sm text-gray-500">Session Types</div>
            <div class="font-semibold text-gray-900">
              {usage.session_type_count}{limits.max_session_types ? ` / ${limits.max_session_types}` : ' (unlimited)'}
            </div>
          </div>
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="text-sm text-gray-500">Goals</div>
            <div class="font-semibold text-gray-900">
              {usage.goal_count}{limits.max_goals ? ` / ${limits.max_goals}` : ' (unlimited)'}
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- License Activation -->
    {#if license.tier === Tier.Free}
      <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">Activate License</h2>
        
        <div class="flex gap-3">
          <div class="relative flex-1">
            <Key size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              bind:value={licenseKey}
              placeholder="PRO-XXXX-XXXX-XXXX or LIFE-XXXX-XXXX-XXXX"
              class="w-full pl-10 pr-4 py-2.5 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
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
              Activating...
            {:else}
              Activate
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
            License activated successfully!
          </p>
        {/if}
        
        <p class="mt-3 text-sm text-gray-500">
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
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">Feature Comparison</h2>
      
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b">
              <th class="text-left py-3 px-2 text-sm font-medium text-gray-500">Feature</th>
              <th class="text-center py-3 px-2 text-sm font-medium text-gray-500">Free</th>
              <th class="text-center py-3 px-2 text-sm font-medium text-indigo-600">Pro / Lifetime</th>
            </tr>
          </thead>
          <tbody>
            {#each proFeatures as feature}
              <tr class="border-b border-gray-50">
                <td class="py-3 px-2 text-sm text-gray-700">{feature.name}</td>
                <td class="py-3 px-2 text-center text-sm text-gray-500">{feature.free}</td>
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
              <h3 class="font-semibold">Upgrade to Pro</h3>
              <p class="text-sm text-white/80">Unlock all features and support development</p>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold">$5<span class="text-sm font-normal">/mo</span></div>
              <div class="text-xs text-white/70">or $39/year</div>
            </div>
          </div>
          <button
            class="mt-4 w-full py-2.5 bg-white text-indigo-600 font-medium rounded-lg hover:bg-gray-50 transition-colors"
            on:click={() => showUpgradeOptions = true}
          >
            Get Pro
          </button>
        </div>
      {/if}
    </div>

    <!-- About -->
    <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100">
      <h2 class="text-lg font-semibold text-gray-900 mb-2">About Chrono</h2>
      <p class="text-sm text-gray-600 mb-4">
        A personal work tracker for freelancers and side-hustlers who want to see exactly how their time converts to money.
      </p>
      <div class="text-sm text-gray-500">
        <div>Version 1.0.0</div>
        <div class="mt-1">
          Built with Tauri, Svelte, and Rust.
          <br />
          Your data stays 100% local. No account required.
        </div>
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
            <h2 class="text-xl font-bold">Upgrade to Pro</h2>
            <p class="text-white/80 text-sm">Choose your plan</p>
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
              <div class="font-semibold text-gray-900">Monthly</div>
              <div class="text-sm text-gray-500">Billed monthly, cancel anytime</div>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-gray-900">$5</div>
              <div class="text-xs text-gray-500">/month</div>
            </div>
          </div>
        </button>

        <!-- Yearly -->
        <button
          class="w-full p-4 border-2 border-indigo-500 rounded-xl hover:bg-indigo-50 transition-colors text-left relative"
          on:click={() => { window.open(checkoutUrls.yearly, '_blank'); showUpgradeOptions = false; }}
        >
          <div class="absolute -top-2.5 left-4 px-2 py-0.5 bg-indigo-500 text-white text-xs font-medium rounded-full">
            Save 35%
          </div>
          <div class="flex justify-between items-center">
            <div>
              <div class="font-semibold text-gray-900">Yearly</div>
              <div class="text-sm text-gray-500">Billed yearly, best value</div>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-indigo-600">$39</div>
              <div class="text-xs text-gray-500">/year</div>
            </div>
          </div>
        </button>

        <p class="text-xs text-gray-500 text-center pt-2">
          After purchase, you'll receive a license key via email.<br />
          Enter it above to activate Pro features.
        </p>
      </div>
    </div>
  </div>
{/if}
