<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Lock, Crown, X, Sparkles } from 'lucide-svelte';

  export let show = false;
  export let featureName = 'This feature';
  export let featureDescription = '';

  const dispatch = createEventDispatcher();

  const checkoutUrls = {
    monthly: 'https://chrono.lemonsqueezy.com/buy/monthly',
    yearly: 'https://chrono.lemonsqueezy.com/buy/yearly',
  };

  function close() {
    show = false;
    dispatch('close');
  }

  function upgrade(plan: 'monthly' | 'yearly') {
    window.open(checkoutUrls[plan], '_blank');
    close();
  }
</script>

{#if show}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    on:click|self={close}
    on:keydown={(e) => e.key === 'Escape' && close()}
  >
    <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden">
      <!-- Header with gradient -->
      <div class="bg-gradient-to-br from-indigo-500 to-purple-600 p-6 text-white relative">
        <button 
          class="absolute top-4 right-4 p-1 hover:bg-white/20 rounded-full transition-colors"
          on:click={close}
        >
          <X size={20} />
        </button>
        
        <div class="flex items-center gap-3 mb-3">
          <div class="p-3 bg-white/20 rounded-xl">
            <Crown size={28} />
          </div>
          <div>
            <h2 class="text-xl font-bold">Pro Feature</h2>
            <p class="text-white/80 text-sm">Unlock with Chrono Pro</p>
          </div>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        <div class="flex items-start gap-3 mb-6">
          <div class="p-2 bg-amber-100 rounded-lg">
            <Lock size={20} class="text-amber-600" />
          </div>
          <div>
            <h3 class="font-semibold text-gray-900">{featureName}</h3>
            {#if featureDescription}
              <p class="text-gray-600 text-sm mt-1">{featureDescription}</p>
            {:else}
              <p class="text-gray-600 text-sm mt-1">
                This feature is available exclusively for Pro users. Upgrade to unlock all premium features.
              </p>
            {/if}
          </div>
        </div>

        <!-- Pro benefits -->
        <div class="bg-gray-50 rounded-xl p-4 mb-6">
          <p class="text-sm font-medium text-gray-700 mb-3 flex items-center gap-2">
            <Sparkles size={16} class="text-amber-500" />
            Pro includes:
          </p>
          <ul class="space-y-2 text-sm text-gray-600">
            <li class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 bg-indigo-500 rounded-full"></span>
              Unlimited AI Advisor queries
            </li>
            <li class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 bg-indigo-500 rounded-full"></span>
              Professional invoicing & PDF export
            </li>
            <li class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 bg-indigo-500 rounded-full"></span>
              Financial simulator & projections
            </li>
            <li class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 bg-indigo-500 rounded-full"></span>
              Unlimited session types & goals
            </li>
            <li class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 bg-indigo-500 rounded-full"></span>
              Full analytics history
            </li>
          </ul>
        </div>

        <!-- Action buttons -->
        <div class="space-y-3">
          <button
            class="w-full py-3 px-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white font-semibold rounded-xl hover:from-indigo-600 hover:to-purple-700 transition-all shadow-lg shadow-indigo-500/25"
            on:click={() => upgrade('yearly')}
          >
            Upgrade to Pro - $59/year
            <span class="text-white/70 text-sm ml-2">Save 38%</span>
          </button>
          <button
            class="w-full py-3 px-4 bg-gray-100 text-gray-700 font-medium rounded-xl hover:bg-gray-200 transition-colors"
            on:click={() => upgrade('monthly')}
          >
            Monthly - $7.99/month
          </button>
          <button
            class="w-full py-2 text-gray-500 text-sm hover:text-gray-700 transition-colors"
            on:click={close}
          >
            Maybe later
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
