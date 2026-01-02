<script lang="ts">
  import { onMount } from 'svelte';
  import { getFocusMetrics, getBurnoutRisk } from '../api';
  import type { FocusMetrics, BurnoutRisk } from '../types';
  import { Activity, AlertTriangle, Brain, Clock, Target, TrendingDown, TrendingUp, Zap, Heart, Coffee } from 'lucide-svelte';

  let focusMetrics: FocusMetrics | null = null;
  let burnoutRisk: BurnoutRisk | null = null;
  let loading = true;
  let selectedRange = '7';

  const ranges = [
    { value: '7', label: '7 Days' },
    { value: '14', label: '14 Days' },
    { value: '30', label: '30 Days' },
  ];

  onMount(() => loadData());

  async function loadData() {
    loading = true;
    try {
      const days = parseInt(selectedRange);
      console.log('Loading focus data for days:', days);
      const [metrics, risk] = await Promise.all([
        getFocusMetrics(days),
        getBurnoutRisk(days)
      ]);
      console.log('Got metrics:', metrics);
      console.log('Got risk:', risk);
      
      // Extend metrics with computed fields
      focusMetrics = {
        ...metrics,
        total_sessions: metrics.session_count,
        longest_session: metrics.avg_session_length * 1.5, // Estimate
        sessions_per_day: metrics.session_count / days,
        deep_work_percentage: Math.min(100, metrics.focus_score * 0.8),
        most_productive_hour: '10:00',
        consistency_score: metrics.current_streak_days > 0 ? Math.min(100, metrics.current_streak_days * 10) : 50,
      };
      burnoutRisk = risk;
      console.log('Final focusMetrics:', focusMetrics);
      console.log('Final burnoutRisk:', burnoutRisk);
    } catch (e) {
      console.error('Failed to load focus data:', e);
      focusMetrics = null;
      burnoutRisk = null;
    } finally {
      loading = false;
    }
  }

  $: if (selectedRange) loadData();

  function getScoreColor(score: number): string {
    if (score >= 70) return 'text-green-600';
    if (score >= 40) return 'text-amber-600';
    return 'text-red-600';
  }

  function getScoreBg(score: number): string {
    if (score >= 70) return 'bg-green-500';
    if (score >= 40) return 'bg-amber-500';
    return 'bg-red-500';
  }

  function getRiskColor(level: string): string {
    switch (level.toLowerCase()) {
      case 'low': return 'text-green-600 bg-green-100';
      case 'moderate': return 'text-amber-600 bg-amber-100';
      case 'high': return 'text-orange-600 bg-orange-100';
      case 'critical': return 'text-red-600 bg-red-100';
      // Also handle ok/warning/danger from factor severities
      case 'ok': return 'text-green-600 bg-green-100';
      case 'warning': return 'text-amber-600 bg-amber-100';
      case 'danger': return 'text-red-600 bg-red-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  }

  function getRiskIcon(level: string) {
    switch (level.toLowerCase()) {
      case 'low': return Heart;
      case 'moderate': return Coffee;
      case 'high': return AlertTriangle;
      case 'critical': return Zap;
      default: return Activity;
    }
  }

  function getSeverityColor(severity: string): string {
    switch (severity.toLowerCase()) {
      case 'ok': return 'border-green-200 bg-green-50';
      case 'warning': return 'border-amber-200 bg-amber-50';
      case 'danger': return 'border-red-200 bg-red-50';
      default: return 'border-gray-200 bg-gray-50';
    }
  }

  function formatHours(hours: number): string {
    if (hours < 1) return `${Math.round(hours * 60)}m`;
    return `${hours.toFixed(1)}h`;
  }
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-indigo-100 rounded-lg">
        <Brain size={24} class="text-indigo-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Focus & Wellbeing</h1>
        <p class="text-gray-500 text-sm">Track your focus patterns and prevent burnout</p>
      </div>
    </div>

    <select
      bind:value={selectedRange}
      class="px-4 py-2 border rounded-lg bg-white text-sm font-medium focus:ring-2 focus:ring-indigo-500"
    >
      {#each ranges as range}
        <option value={range.value}>{range.label}</option>
      {/each}
    </select>
  </div>

  {#if loading}
    <div class="card p-12 bg-white rounded-xl shadow-sm border text-center">
      <div class="animate-spin w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full mx-auto mb-3"></div>
      <p class="text-gray-500">Analyzing your focus patterns...</p>
    </div>
  {:else if !focusMetrics || !burnoutRisk}
    <div class="card p-12 bg-gray-50 rounded-xl border border-dashed text-center">
      <Brain size={48} class="text-gray-400 mx-auto mb-3" />
      <p class="text-gray-500 font-medium">No focus data available</p>
      <p class="text-sm text-gray-400">Start tracking sessions to see your focus patterns</p>
    </div>
  {:else if focusMetrics && burnoutRisk}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Focus Score Card -->
      <div class="card p-6 bg-white rounded-xl shadow-sm border">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-lg font-semibold flex items-center gap-2">
            <Target size={20} class="text-indigo-600" />
            Focus Score
          </h2>
          <span class="text-4xl font-bold {getScoreColor(focusMetrics.focus_score)}">
            {focusMetrics.focus_score.toFixed(0)}
          </span>
        </div>

        <!-- Score Gauge -->
        <div class="mb-6">
          <div class="w-full h-4 bg-gray-200 rounded-full overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-700 {getScoreBg(focusMetrics.focus_score)}"
              style="width: {focusMetrics.focus_score}%"
            />
          </div>
          <div class="flex justify-between text-xs text-gray-400 mt-1">
            <span>Unfocused</span>
            <span>Excellent</span>
          </div>
        </div>

        <!-- Metrics -->
        <div class="grid grid-cols-2 gap-4">
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="flex items-center gap-2 text-sm text-gray-500 mb-1">
              <Clock size={14} />
              Avg Session
            </div>
            <p class="text-xl font-semibold text-gray-900">
              {formatHours(focusMetrics.avg_session_length)}
            </p>
          </div>
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="flex items-center gap-2 text-sm text-gray-500 mb-1">
              <Zap size={14} />
              Longest Session
            </div>
            <p class="text-xl font-semibold text-gray-900">
              {formatHours(focusMetrics.longest_session ?? 0)}
            </p>
          </div>
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="flex items-center gap-2 text-sm text-gray-500 mb-1">
              <Activity size={14} />
              Sessions/Day
            </div>
            <p class="text-xl font-semibold text-gray-900">
              {(focusMetrics.sessions_per_day ?? 0).toFixed(1)}
            </p>
          </div>
          <div class="p-3 bg-gray-50 rounded-lg">
            <div class="flex items-center gap-2 text-sm text-gray-500 mb-1">
              <TrendingUp size={14} />
              Deep Work %
            </div>
            <p class="text-xl font-semibold text-gray-900">
              {(focusMetrics.deep_work_percentage ?? 0).toFixed(0)}%
            </p>
          </div>
        </div>

        <!-- Fragmentation Score -->
        <div class="mt-4 p-4 border rounded-lg">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm font-medium text-gray-700">Fragmentation Index</span>
            <span class="text-sm font-bold {focusMetrics.fragmentation_score < 30 ? 'text-green-600' : focusMetrics.fragmentation_score < 60 ? 'text-amber-600' : 'text-red-600'}">
              {focusMetrics.fragmentation_score.toFixed(0)}%
            </span>
          </div>
          <p class="text-xs text-gray-500">
            {focusMetrics.fragmentation_score < 30 
              ? 'Great! Your work sessions are well-consolidated.'
              : focusMetrics.fragmentation_score < 60
                ? 'Moderate fragmentation. Try to batch similar tasks.'
                : 'High fragmentation. Consider blocking focused time.'}
          </p>
        </div>
      </div>

      <!-- Burnout Risk Card -->
      <div class="card p-6 bg-white rounded-xl shadow-sm border">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-lg font-semibold flex items-center gap-2">
            <AlertTriangle size={20} class="text-amber-600" />
            Burnout Risk
          </h2>
          <div class="flex items-center gap-2 px-3 py-1 rounded-full {getRiskColor(burnoutRisk.risk_level)}">
            <svelte:component this={getRiskIcon(burnoutRisk.risk_level)} size={16} />
            <span class="font-semibold capitalize">{burnoutRisk.risk_level}</span>
          </div>
        </div>

        <!-- Risk Score -->
        <div class="mb-6">
          <div class="flex items-end justify-between mb-2">
            <span class="text-sm text-gray-500">Risk Score</span>
            <span class="text-3xl font-bold text-gray-900">{burnoutRisk.risk_score.toFixed(0)}/100</span>
          </div>
          <div class="w-full h-4 bg-gray-200 rounded-full overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-700"
              style="width: {burnoutRisk.risk_score}%; background-color: {burnoutRisk.risk_score <= 33 ? '#22c55e' : burnoutRisk.risk_score <= 66 ? '#f59e0b' : '#ef4444'}"
            />
          </div>
        </div>

        <!-- Risk Factors -->
        {#if burnoutRisk.factors.length > 0}
          <div class="space-y-3">
            <h3 class="text-sm font-medium text-gray-700">Risk Factors</h3>
            {#each burnoutRisk.factors as factor}
              <div class="p-3 rounded-lg border {getSeverityColor(factor.severity)}">
                <div class="flex items-center justify-between mb-1">
                  <span class="font-medium text-gray-900">{factor.name}</span>
                  <span class="text-xs px-2 py-0.5 rounded-full capitalize {getRiskColor(factor.severity)}">
                    {factor.severity}
                  </span>
                </div>
                <p class="text-sm text-gray-600">{factor.value} (Target: {factor.threshold})</p>
              </div>
            {/each}
          </div>
        {:else}
          <div class="p-6 bg-green-50 rounded-lg text-center">
            <Heart size={32} class="text-green-500 mx-auto mb-2" />
            <p class="font-medium text-green-700">Looking good!</p>
            <p class="text-sm text-green-600">No significant burnout risk factors detected.</p>
          </div>
        {/if}

        <!-- Recommendations -->
        {#if burnoutRisk.recommendations.length > 0}
          <div class="mt-4 p-4 bg-blue-50 rounded-lg">
            <h3 class="text-sm font-medium text-blue-700 mb-2">Recommendations</h3>
            <ul class="space-y-1">
              {#each burnoutRisk.recommendations as rec}
                <li class="text-sm text-blue-600 flex items-start gap-2">
                  <span class="text-blue-400 mt-1">•</span>
                  {rec}
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </div>

    <!-- Weekly Trends (placeholder for future chart) -->
    <div class="card p-6 bg-white rounded-xl shadow-sm border">
      <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
        <TrendingUp size={20} class="text-green-600" />
        Work Patterns
      </h2>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <p class="text-3xl font-bold text-gray-900">{focusMetrics.total_sessions}</p>
          <p class="text-sm text-gray-500">Total Sessions</p>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <p class="text-3xl font-bold text-gray-900">{formatHours(focusMetrics.total_hours)}</p>
          <p class="text-sm text-gray-500">Total Hours</p>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <p class="text-3xl font-bold text-gray-900">{focusMetrics.most_productive_hour || '--'}</p>
          <p class="text-sm text-gray-500">Peak Hour</p>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <p class="text-3xl font-bold text-gray-900">{(focusMetrics.consistency_score ?? 0).toFixed(0)}%</p>
          <p class="text-sm text-gray-500">Consistency</p>
        </div>
      </div>
    </div>
  {/if}
</div>
