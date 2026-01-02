<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X, Sparkles, Check } from 'lucide-svelte';
  
  export let show = false;
  export let feature: string = '';
  export let current: number = 0;
  export let limit: number = 0;
  
  const dispatch = createEventDispatcher();
  
  // LemonSqueezy checkout URLs
  const checkoutUrls = {
    monthly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/2163691e-c198-499f-b94e-3cbd188f2591',
    yearly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/cf7a46b3-4578-4320-bd56-5d50afd931b8',
  };
  
  let showPlanChoice = false;
  
  function close() {
    show = false;
    showPlanChoice = false;
    dispatch('close');
  }
  
  function upgrade() {
    showPlanChoice = true;
  }
  
  function selectPlan(plan: 'monthly' | 'yearly') {
    dispatch('upgrade');
    window.open(checkoutUrls[plan], '_blank');
    close();
  }
  
  // Feature-specific messaging
  $: message = getFeatureMessage(feature, current, limit);
  $: title = getFeatureTitle(feature);
  
  function getFeatureTitle(feat: string): string {
    switch (feat) {
      case 'session_types': return 'Session Type Limit Reached';
      case 'goals': return 'Goal Limit Reached';
      case 'invoices': return 'Invoices is a Pro Feature';
      case 'ai_advisor': return 'AI Advisor is a Pro Feature';
      case 'voice_input': return 'Voice Input is a Pro Feature';
      case 'simulator': return 'Simulator is a Pro Feature';
      default: return 'Upgrade to Pro';
    }
  }
  
  function getFeatureMessage(feat: string, curr: number, lim: number): string {
    switch (feat) {
      case 'session_types':
        return `You've created ${curr} of ${lim} session types available on the Free plan. Upgrade to track unlimited work categories.`;
      case 'goals':
        return `You've set ${curr} of ${lim} goals available on the Free plan. Upgrade to track unlimited financial goals.`;
      case 'invoices':
        return 'Generate professional invoices from your sessions. Export as PDF and get paid faster.';
      case 'ai_advisor':
        return 'Get personalized financial insights and recommendations powered by AI analysis of your work patterns.';
      case 'voice_input':
        return 'Log sessions hands-free with voice commands. Perfect for quick entries on the go.';
      case 'simulator':
        return 'Model different scenarios and see how changes in hours or rates affect your income goals.';
      default:
        return 'Unlock all features with Chrono Pro.';
    }
  }
  
  const proFeatures = [
    'Unlimited session types',
    'Unlimited goals',
    'Full analytics history',
    'Invoice generation',
    'AI financial advisor',
    'Voice input',
    'Earnings simulator',
    'PDF exports'
  ];
</script>

{#if show}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" on:click|self={close}>
    <div class="bg-white rounded-2xl shadow-xl max-w-md w-full overflow-hidden" role="dialog" aria-modal="true">
      <!-- Header -->
      <div class="bg-gradient-to-r from-indigo-500 to-purple-600 p-6 text-white relative">
        <button
          class="absolute top-4 right-4 p-1 rounded-full hover:bg-white/20 transition-colors"
          on:click={close}
        >
          <X size={20} />
        </button>
        <div class="flex items-center gap-3 mb-2">
          <Sparkles size={24} />
          <h2 class="text-xl font-bold">{title}</h2>
        </div>
        <p class="text-white/90 text-sm">{message}</p>
      </div>
      
      <!-- Body -->
      <div class="p-6">
        <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3">
          Everything in Pro
        </h3>
        <ul class="space-y-2 mb-6">
          {#each proFeatures as feat}
            <li class="flex items-center gap-2 text-sm text-gray-700">
              <Check size={16} class="text-green-500 flex-shrink-0" />
              {feat}
            </li>
          {/each}
        </ul>
        
        <!-- Pricing -->
        <div class="bg-gray-50 rounded-xl p-4 mb-6">
          <div class="flex items-baseline gap-2 mb-1">
            <span class="text-2xl font-bold text-gray-900">$5</span>
            <span class="text-gray-500">/month</span>
          </div>
          <p class="text-xs text-gray-500">or $39/year (save 35%)</p>
        </div>
        
        <!-- Actions -->
        {#if showPlanChoice}
          <div class="space-y-3">
            <button
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg hover:border-indigo-500 transition-colors text-left flex justify-between items-center"
              on:click={() => selectPlan('monthly')}
            >
              <span class="font-medium text-gray-900">Monthly</span>
              <span class="text-gray-600">$5/mo</span>
            </button>
            <button
              class="w-full px-4 py-3 border-2 border-indigo-500 bg-indigo-50 rounded-lg hover:bg-indigo-100 transition-colors text-left flex justify-between items-center relative"
              on:click={() => selectPlan('yearly')}
            >
              <span class="absolute -top-2 left-3 px-2 py-0.5 bg-indigo-500 text-white text-xs font-medium rounded-full">Best Value</span>
              <span class="font-medium text-gray-900">Yearly</span>
              <span class="text-indigo-600 font-medium">$39/yr</span>
            </button>
          </div>
        {:else}
          <div class="flex gap-3">
            <button
              class="flex-1 px-4 py-2.5 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 transition-colors"
              on:click={upgrade}
            >
              Upgrade to Pro
            </button>
            <button
              class="px-4 py-2.5 text-gray-600 font-medium rounded-lg hover:bg-gray-100 transition-colors"
              on:click={close}
            >
              Maybe Later
            </button>
          </div>
        {/if}
        
        <p class="text-xs text-center text-gray-400 mt-4">
          Your data stays local. No account required.
        </p>
      </div>
    </div>
  </div>
{/if}
