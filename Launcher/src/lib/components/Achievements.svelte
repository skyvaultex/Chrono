<script lang="ts">
  import { onMount } from 'svelte';
  import { getAchievements, checkAndUnlockAchievements } from '../api';
  import type { Achievement } from '../types';
  import { AchievementCategory } from '../types';
  import { Award, Lock, CheckCircle2, Sparkles } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let achievements: Achievement[] = [];
  let loading = true;
  let newlyUnlocked: string[] = [];

  // Group achievements by category
  $: groupedAchievements = {
    [AchievementCategory.Presence]: achievements.filter(a => a.category === AchievementCategory.Presence),
    [AchievementCategory.Awareness]: achievements.filter(a => a.category === AchievementCategory.Awareness),
    [AchievementCategory.Balance]: achievements.filter(a => a.category === AchievementCategory.Balance),
    [AchievementCategory.Commitment]: achievements.filter(a => a.category === AchievementCategory.Commitment),
    [AchievementCategory.Financial]: achievements.filter(a => a.category === AchievementCategory.Financial),
  };

  $: unlockedCount = achievements.filter(a => a.unlocked).length;
  $: totalCount = achievements.length;
  $: allUnlocked = totalCount > 0 && unlockedCount === totalCount;
  $: progressPercent = totalCount > 0 ? Math.round((unlockedCount / totalCount) * 100) : 0;

  const categoryOrder: AchievementCategory[] = [
    AchievementCategory.Presence,
    AchievementCategory.Awareness,
    AchievementCategory.Balance,
    AchievementCategory.Commitment,
    AchievementCategory.Financial,
  ];

  const categoryInfo: Record<AchievementCategory, { title: string; color: string; bgColor: string }> = {
    [AchievementCategory.Presence]: { title: '🌱 Getting Started', color: 'text-green-600', bgColor: 'bg-green-50' },
    [AchievementCategory.Awareness]: { title: '🧠 Awareness & Insight', color: 'text-blue-600', bgColor: 'bg-blue-50' },
    [AchievementCategory.Balance]: { title: '⚖️ Balance & Sustainability', color: 'text-amber-600', bgColor: 'bg-amber-50' },
    [AchievementCategory.Commitment]: { title: '🏗️ Commitment & Continuity', color: 'text-purple-600', bgColor: 'bg-purple-50' },
    [AchievementCategory.Financial]: { title: '💰 Financial Awareness', color: 'text-emerald-600', bgColor: 'bg-emerald-50' },
  };

  onMount(async () => {
    try {
      // Check for any new unlocks first
      newlyUnlocked = await checkAndUnlockAchievements();
      
      // Then fetch all achievements
      achievements = await getAchievements();
      
      // Notify parent if all are unlocked
      if (achievements.length > 0 && achievements.every(a => a.unlocked)) {
        dispatch('allUnlocked');
      }
    } catch (e) {
      console.error('Failed to load achievements:', e);
    } finally {
      loading = false;
    }
  });

  function formatDate(dateStr: string | null): string {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
  }
</script>

<div class="p-6 max-w-4xl mx-auto">
  <!-- Header -->
  <div class="flex items-center gap-3 mb-6">
    <div class="p-2 bg-amber-100 rounded-lg">
      <Award size={24} class="text-amber-600" />
    </div>
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Milestones</h1>
      <p class="text-gray-500 text-sm">Quiet recognition of your journey</p>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-amber-600"></div>
    </div>
  {:else}
    <!-- Progress Overview -->
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 mb-6">
      <div class="flex items-center justify-between mb-3">
        <span class="text-sm text-gray-600">Progress</span>
        <span class="text-sm font-medium text-gray-900">{unlockedCount} of {totalCount}</span>
      </div>
      <div class="w-full h-2 bg-gray-100 rounded-full overflow-hidden">
        <div 
          class="h-full rounded-full transition-all duration-500 {allUnlocked ? 'bg-gradient-to-r from-amber-400 to-yellow-500' : 'bg-indigo-500'}"
          style="width: {progressPercent}%"
        ></div>
      </div>
      {#if allUnlocked}
        <div class="mt-3 flex items-center gap-2 text-amber-600">
          <Sparkles size={16} />
          <span class="text-sm font-medium">All milestones achieved! Your dedication is golden.</span>
        </div>
      {/if}
    </div>

    <!-- Newly Unlocked Toast -->
    {#if newlyUnlocked.length > 0}
      <div class="bg-gradient-to-r from-amber-50 to-yellow-50 border border-amber-200 rounded-xl p-4 mb-6">
        <div class="flex items-center gap-2 text-amber-700">
          <CheckCircle2 size={18} />
          <span class="font-medium">
            {newlyUnlocked.length === 1 ? 'New milestone unlocked!' : `${newlyUnlocked.length} new milestones unlocked!`}
          </span>
        </div>
      </div>
    {/if}

    <!-- Achievement Categories -->
    <div class="space-y-6">
      {#each categoryOrder as category}
        {#if groupedAchievements[category].length > 0}
          <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
            <div class="px-5 py-3 border-b border-gray-100 {categoryInfo[category].bgColor}">
              <h2 class="font-semibold {categoryInfo[category].color}">
                {categoryInfo[category].title}
              </h2>
            </div>
            <div class="divide-y divide-gray-50">
              {#each groupedAchievements[category] as achievement}
                <div class="px-5 py-4 flex items-center gap-4 {achievement.unlocked ? '' : 'opacity-50'}">
                  <div class="text-2xl w-10 flex justify-center {achievement.unlocked ? '' : 'grayscale'}">
                    {achievement.icon}
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                      <h3 class="font-medium text-gray-900">{achievement.name}</h3>
                      {#if newlyUnlocked.includes(achievement.id)}
                        <span class="text-xs bg-amber-100 text-amber-700 px-1.5 py-0.5 rounded">New!</span>
                      {/if}
                    </div>
                    <p class="text-sm text-gray-500">{achievement.description}</p>
                  </div>
                  <div class="flex-shrink-0">
                    {#if achievement.unlocked}
                      <div class="text-right">
                        <CheckCircle2 size={20} class="text-green-500 ml-auto" />
                        <p class="text-xs text-gray-400 mt-1">{formatDate(achievement.unlocked_at)}</p>
                      </div>
                    {:else}
                      <Lock size={18} class="text-gray-300" />
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {/each}
    </div>

    <!-- Philosophy Note -->
    <div class="mt-8 text-center text-sm text-gray-400">
      <p>Milestones are cumulative and never lost.</p>
      <p class="mt-1">No streaks. No pressure. Just recognition.</p>
    </div>
  {/if}
</div>
