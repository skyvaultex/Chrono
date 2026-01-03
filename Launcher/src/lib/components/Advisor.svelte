<script lang="ts">
  import { onMount } from 'svelte';
  import { getTodaySummary, getAllGoals, calculateAvgWeeklyIncome, askAiAdvisor, getAnalytics, getRecentSessions, getSessionsByDate, getFinancialAnalysis } from '../api';
  import type { TodaySummary, FinancialGoal, AnalyticsData, WorkSession, FinancialAnalysis, FinancialInsight } from '../types';
  import type { AIContext, AIAdvisorResponse } from '../api';
  import { MessageSquare, Send, User, Bot, Clock, DollarSign, Target, TrendingUp, ChevronDown, ChevronUp, Trash2, Brain, BarChart3, AlertTriangle, CheckCircle, Info, Lightbulb, Zap, Crown } from 'lucide-svelte';
  import { GoalType } from '../types';

  let summary: TodaySummary | null = null;
  let goals: FinancialGoal[] = [];
  let avgWeeklyIncome = 0;
  let analyticsData: AnalyticsData | null = null;
  let recentSessions: WorkSession[] = [];
  let todaySessions: WorkSession[] = [];
  let financialAnalysis: FinancialAnalysis | null = null;
  let loading = true;
  let question = '';
  let response = '';
  let thinking = false;
  let error = '';
  let goalsExpanded = false;
  let showInsights = true;
  
  // Rate limit tracking
  let remainingQueries: number | null = null;
  let dailyLimit: number | null = null;
  let showRateLimitModal = false;
  let rateLimitResetTime = '';

  const checkoutUrls = {
    monthly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/2163691e-c198-499f-b94e-3cbd188f2591',
    yearly: 'https://skyvaultex.lemonsqueezy.com/checkout/buy/cf7a46b3-4578-4320-bd56-5d50afd931b8',
  };

  let chatHistory: Array<{role: 'user' | 'assistant', content: string}> = [];

  const quickPrompts = [
    "Analyze my productivity patterns",
    "How long until I reach my goals?",
    "What's my best work strategy?",
    "Where can I improve?",
    "Review my work-life balance"
  ];

  onMount(async () => {
    try {
      // Get today's data
      const today = new Date().toISOString().split('T')[0];
      const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0];
      
      // Fetch all context in parallel
      const [summaryData, goalsData, weeklyIncome, analytics, recent, todaySessionsData, finAnalysis] = await Promise.all([
        getTodaySummary(),
        getAllGoals(),
        calculateAvgWeeklyIncome(),
        getAnalytics(thirtyDaysAgo, today),
        getRecentSessions(10),
        getSessionsByDate(today),
        getFinancialAnalysis().catch(() => null)
      ]);
      
      summary = summaryData;
      goals = goalsData;
      avgWeeklyIncome = weeklyIncome;
      analyticsData = analytics;
      recentSessions = recent;
      todaySessions = todaySessionsData;
      financialAnalysis = finAnalysis;
    } catch (e) {
      console.error('Failed to load data:', e);
    } finally {
      loading = false;
    }
  });

  function getInsightIcon(severity: string) {
    switch (severity.toLowerCase()) {
      case 'critical': return AlertTriangle;
      case 'warning': return AlertTriangle;
      case 'success': return CheckCircle;
      case 'info': return Info;
      default: return Lightbulb;
    }
  }

  function getInsightColor(severity: string): string {
    switch (severity.toLowerCase()) {
      case 'critical': return 'bg-red-50 border-red-200 text-red-800';
      case 'warning': return 'bg-amber-50 border-amber-200 text-amber-800';
      case 'success': return 'bg-green-50 border-green-200 text-green-800';
      case 'info': return 'bg-blue-50 border-blue-200 text-blue-800';
      default: return 'bg-gray-50 border-gray-200 text-gray-800';
    }
  }

  function getInsightIconColor(severity: string): string {
    switch (severity.toLowerCase()) {
      case 'critical': return 'text-red-500';
      case 'warning': return 'text-amber-500';
      case 'success': return 'text-green-500';
      case 'info': return 'text-blue-500';
      default: return 'text-gray-500';
    }
  }

  function buildGoalsSummary(): string {
    if (goals.length === 0) return "No active goals";
    return goals.map(g =>
      `- ${g.name} (${g.goal_type}): $${g.current_amount.toFixed(2)} / $${g.target_amount.toFixed(2)} (${((g.current_amount / g.target_amount) * 100).toFixed(0)}%)`
    ).join('\n');
  }

  function buildCategoriesSummary(): string {
    if (!analyticsData || analyticsData.category_breakdown.length === 0) return "No category data";
    return analyticsData.category_breakdown.map(c =>
      `- ${c.category}: ${c.hours.toFixed(1)}h across ${c.sessions} sessions ($${c.pay.toFixed(2)} earned)`
    ).join('\n');
  }

  function buildWeekdaySummary(): string {
    if (!analyticsData || analyticsData.weekday_breakdown.length === 0) return "No weekday data";
    return analyticsData.weekday_breakdown.map(w =>
      `- ${w.weekday}: ${w.hours.toFixed(1)}h (${w.sessions} sessions)`
    ).join('\n');
  }

  function getBestWorstWeekday(): { best: string; worst: string } {
    if (!analyticsData || analyticsData.weekday_breakdown.length === 0) {
      return { best: 'N/A', worst: 'N/A' };
    }
    const sorted = [...analyticsData.weekday_breakdown].sort((a, b) => b.hours - a.hours);
    return {
      best: `${sorted[0].weekday} (${sorted[0].hours.toFixed(1)}h)`,
      worst: `${sorted[sorted.length - 1].weekday} (${sorted[sorted.length - 1].hours.toFixed(1)}h)`
    };
  }

  function buildRecentSessionsSummary(): string {
    if (recentSessions.length === 0) return "No recent sessions";
    return recentSessions.slice(0, 5).map(s =>
      `- ${s.date}: ${s.project_name} (${s.hours.toFixed(1)}h)`
    ).join('\n');
  }

  function calculateConsistency(): number {
    if (!analyticsData || analyticsData.daily_hours.length === 0) return 0;
    const daysWithWork = analyticsData.daily_hours.filter(d => d.hours >= 0.5).length;
    return Math.round((daysWithWork / 30) * 100);
  }

  async function handleAskAdvisor(promptOverride?: string) {
    const q = promptOverride || question;
    if (!q.trim()) return;

    thinking = true;
    error = '';

    chatHistory = [...chatHistory, { role: 'user', content: q }];
    question = '';

    try {
      const { best, worst } = getBestWorstWeekday();
      
      // Calculate goal totals
      const totalDebt = goals
        .filter(g => g.goal_type === GoalType.Debt)
        .reduce((sum, g) => sum + (g.target_amount - g.current_amount), 0);
      const totalSavingsTarget = goals
        .filter(g => g.goal_type === GoalType.Savings || g.goal_type === GoalType.Purchase)
        .reduce((sum, g) => sum + g.target_amount, 0);

      const context: AIContext = {
        // Today
        today_hours: summary?.total_hours || 0,
        today_pay: summary?.total_pay || 0,
        today_sessions: todaySessions.length,
        
        // 30-day period
        period_total_hours: analyticsData?.summary.total_hours || 0,
        period_total_sessions: analyticsData?.summary.total_sessions || 0,
        period_total_pay: analyticsData?.summary.total_pay || 0,
        period_avg_session: analyticsData?.summary.avg_session_length || 0,
        period_longest_session: analyticsData?.summary.longest_session || 0,
        
        // Averages
        avg_weekly_income: avgWeeklyIncome,
        avg_daily_hours: analyticsData ? (analyticsData.summary.total_hours / Math.max(analyticsData.daily_hours.length, 1)) : 0,
        consistency_score: calculateConsistency(),
        
        // Categories
        categories_summary: buildCategoriesSummary(),
        
        // Weekday patterns
        best_weekday: best,
        worst_weekday: worst,
        weekday_summary: buildWeekdaySummary(),
        
        // Goals
        goals_count: goals.length,
        goals_summary: buildGoalsSummary(),
        total_debt: totalDebt,
        total_savings_target: totalSavingsTarget,
        
        // Recent
        recent_sessions_summary: buildRecentSessionsSummary()
      };

      const aiResponse = await askAiAdvisor(q, context);
      chatHistory = [...chatHistory, { role: 'assistant', content: aiResponse.content }];
      response = aiResponse.content;
      remainingQueries = aiResponse.remaining_queries;
      dailyLimit = aiResponse.daily_limit;
    } catch (e) {
      const errStr = String(e);
      // Check if it's a rate limit error
      if (errStr.includes('Daily limit reached') || errStr.includes('429') || errStr.includes('limit')) {
        showRateLimitModal = true;
        // Try to extract reset time
        const resetMatch = errStr.match(/(\d{4}-\d{2}-\d{2}T[\d:]+)/);
        if (resetMatch) {
          rateLimitResetTime = new Date(resetMatch[1]).toLocaleTimeString();
        } else {
          rateLimitResetTime = 'midnight';
        }
        remainingQueries = 0;
      } else {
        error = errStr;
        chatHistory = [...chatHistory, { role: 'assistant', content: `Error: ${error}` }];
      }
    } finally {
      thinking = false;
    }
  }

  function handleQuickPrompt(prompt: string) {
    handleAskAdvisor(prompt);
  }

  function clearChat() {
    chatHistory = [];
    response = '';
    error = '';
  }

  function formatAmount(amount: number): string {
    return `$${amount.toFixed(2)}`;
  }

  function formatHours(hours: number): string {
    if (hours < 1) return `${Math.round(hours * 60)}m`;
    const h = Math.floor(hours);
    const m = Math.round((hours - h) * 60);
    return m > 0 ? `${h}h ${m}m` : `${h}h`;
  }

  function toggleGoals() {
    goalsExpanded = !goalsExpanded;
  }

  $: displayedGoals = goalsExpanded ? goals : goals.slice(0, 3);
  $: hasMoreGoals = goals.length > 3;
  $: consistencyScore = calculateConsistency();
</script>

<div class="p-6 space-y-6 max-w-6xl mx-auto">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-indigo-100 rounded-lg">
        <Brain size={24} class="text-indigo-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Advisor</h1>
        <p class="text-gray-500 text-sm">AI-powered insights and guidance</p>
      </div>
    </div>
    <div class="flex items-center gap-3">
      {#if remainingQueries !== null}
        <div class="text-sm px-3 py-1 rounded-full flex items-center gap-2
          {remainingQueries === 0 ? 'bg-red-100 text-red-700' : remainingQueries <= 3 ? 'bg-amber-100 text-amber-700' : 'bg-gray-100 text-gray-500'}">
          {#if remainingQueries === 0}
            <AlertTriangle size={14} />
            Limit reached
          {:else}
            <Zap size={14} />
            {remainingQueries}/{dailyLimit} queries left
          {/if}
        </div>
      {/if}
      {#if chatHistory.length > 0}
        <button 
          class="flex items-center gap-2 px-3 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors text-gray-700"
          on:click={clearChat}
        >
          <Trash2 size={16} />
          <span class="text-sm">Clear Chat</span>
        </button>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="bg-white rounded-xl p-12 shadow-sm border border-gray-100 text-center">
      <div class="animate-spin w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full mx-auto mb-3"></div>
      <p class="text-gray-500">Loading your data...</p>
    </div>
  {:else}
    <!-- Context Summary Panel -->
    <div class="bg-gradient-to-r from-indigo-50 to-purple-50 rounded-xl p-5 border border-indigo-100">
      <div class="flex items-center gap-2 mb-4">
        <BarChart3 size={18} class="text-indigo-600" />
        <h2 class="font-semibold text-gray-900">Context Available to Advisor</h2>
      </div>
      
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
        <div>
          <p class="text-gray-500 text-xs uppercase tracking-wide">Today</p>
          <p class="font-semibold text-gray-900">{formatHours(summary?.total_hours || 0)} • {formatAmount(summary?.total_pay || 0)}</p>
        </div>
        <div>
          <p class="text-gray-500 text-xs uppercase tracking-wide">30-Day Total</p>
          <p class="font-semibold text-gray-900">{formatHours(analyticsData?.summary.total_hours || 0)} • {formatAmount(analyticsData?.summary.total_pay || 0)}</p>
        </div>
        <div>
          <p class="text-gray-500 text-xs uppercase tracking-wide">Consistency</p>
          <p class="font-semibold text-gray-900">{consistencyScore}% of days worked</p>
        </div>
        <div>
          <p class="text-gray-500 text-xs uppercase tracking-wide">Goals</p>
          <p class="font-semibold text-gray-900">{goals.length} active</p>
        </div>
      </div>
      
      <p class="text-xs text-gray-500 mt-3">
        The advisor can see your work patterns, categories, weekday trends, goals, and recent sessions.
      </p>
    </div>

    <!-- Financial Insights Panel -->
    {#if financialAnalysis && (financialAnalysis.insights.length > 0 || financialAnalysis.recommendations.length > 0)}
      <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
        <button 
          class="w-full flex items-center justify-between p-4 hover:bg-gray-50 transition-colors"
          on:click={() => showInsights = !showInsights}
        >
          <div class="flex items-center gap-2">
            <Zap size={20} class="text-amber-500" />
            <h2 class="font-semibold text-gray-900">Smart Financial Insights</h2>
            <span class="px-2 py-0.5 text-xs font-medium bg-amber-100 text-amber-700 rounded-full">
              {financialAnalysis.insights.length + financialAnalysis.recommendations.length}
            </span>
          </div>
          <svelte:component this={showInsights ? ChevronUp : ChevronDown} size={20} class="text-gray-400" />
        </button>

        {#if showInsights}
          <div class="px-4 pb-4 space-y-3">
            <!-- Projections Summary -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3 p-3 bg-gradient-to-r from-purple-50 to-indigo-50 rounded-lg">
              <div class="text-center">
                <p class="text-xs text-gray-500 uppercase">Weekly Avg</p>
                <p class="text-lg font-bold text-gray-900">{formatHours(financialAnalysis.avg_weekly_hours)}</p>
              </div>
              <div class="text-center">
                <p class="text-xs text-gray-500 uppercase">Weekly Income</p>
                <p class="text-lg font-bold text-green-600">{formatAmount(financialAnalysis.avg_weekly_income)}</p>
              </div>
              <div class="text-center">
                <p class="text-xs text-gray-500 uppercase">Monthly Proj.</p>
                <p class="text-lg font-bold text-gray-900">{formatAmount(financialAnalysis.projected_monthly_income)}</p>
              </div>
              <div class="text-center">
                <p class="text-xs text-gray-500 uppercase">Yearly Proj.</p>
                <p class="text-lg font-bold text-indigo-600">{formatAmount(financialAnalysis.projected_yearly_income)}</p>
              </div>
            </div>

            <!-- Income Trend -->
            {#if financialAnalysis.income_trend}
              <div class="flex items-center gap-2 p-3 bg-gray-50 rounded-lg">
                <TrendingUp size={18} class="{financialAnalysis.income_trend.includes('up') || financialAnalysis.income_trend.includes('increasing') ? 'text-green-500' : financialAnalysis.income_trend.includes('down') || financialAnalysis.income_trend.includes('decreasing') ? 'text-red-500' : 'text-gray-500'}" />
                <span class="text-sm text-gray-700">Income Trend: <strong class="capitalize">{financialAnalysis.income_trend}</strong></span>
              </div>
            {/if}

            <!-- Insights -->
            {#if financialAnalysis.insights.length > 0}
              <div class="space-y-2">
                {#each financialAnalysis.insights as insight}
                  <div class="flex items-start gap-3 p-3 rounded-lg border {getInsightColor(insight.severity)}">
                    <svelte:component this={getInsightIcon(insight.severity)} size={18} class="{getInsightIconColor(insight.severity)} flex-shrink-0 mt-0.5" />
                    <div class="flex-1 min-w-0">
                      <p class="font-medium text-sm">{insight.title}</p>
                      <p class="text-sm opacity-80">{insight.message}</p>
                      {#if insight.action}
                        <p class="text-xs mt-1 font-medium opacity-70">💡 {insight.action}</p>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}

            <!-- Recommendations -->
            {#if financialAnalysis.recommendations.length > 0}
              <div class="p-3 bg-blue-50 rounded-lg border border-blue-200">
                <h4 class="text-sm font-medium text-blue-800 mb-2 flex items-center gap-2">
                  <Lightbulb size={16} />
                  Recommendations
                </h4>
                <ul class="space-y-1">
                  {#each financialAnalysis.recommendations as rec}
                    <li class="text-sm text-blue-700 flex items-start gap-2">
                      <span class="text-blue-400 mt-1">•</span>
                      {rec}
                    </li>
                  {/each}
                </ul>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}

    <!-- Quick Prompts -->
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
      <h3 class="font-semibold mb-3 text-gray-900">Quick Questions</h3>
      <div class="flex flex-wrap gap-2">
        {#each quickPrompts as prompt}
          <button
            class="px-3 py-1.5 text-sm bg-gray-100 hover:bg-indigo-100 hover:text-indigo-700 rounded-lg transition-colors"
            on:click={() => handleQuickPrompt(prompt)}
            disabled={thinking}
          >
            {prompt}
          </button>
        {/each}
      </div>
    </div>

    <!-- Chat History -->
    {#if chatHistory.length > 0}
      <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4 max-h-[400px] overflow-y-auto">
        <h2 class="font-semibold flex items-center gap-2 text-gray-900">
          <MessageSquare size={18} />
          Conversation
        </h2>
        {#each chatHistory as message}
          <div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
            <div class="max-w-[85%] p-3 rounded-xl {message.role === 'user' ? 'bg-indigo-600 text-white' : 'bg-gray-100'}">
              <div class="text-xs font-medium mb-1 flex items-center gap-1 {message.role === 'user' ? 'text-indigo-200' : 'text-gray-500'}">
                {#if message.role === 'user'}
                  <User size={12} /> You
                {:else}
                  <Bot size={12} /> Advisor
                {/if}
              </div>
              <div class="whitespace-pre-wrap text-sm">{message.content}</div>
            </div>
          </div>
        {/each}
        {#if thinking}
          <div class="flex justify-start">
            <div class="p-3 bg-gray-100 rounded-xl">
              <div class="text-xs font-medium mb-1 flex items-center gap-1 text-gray-500">
                <Bot size={12} /> Advisor
              </div>
              <div class="flex items-center gap-2">
                <span class="text-gray-500">Analyzing your data...</span>
                <div class="flex gap-1">
                  <div class="w-2 h-2 rounded-full animate-bounce bg-indigo-500" style="animation-delay: 0ms"></div>
                  <div class="w-2 h-2 rounded-full animate-bounce bg-indigo-500" style="animation-delay: 150ms"></div>
                  <div class="w-2 h-2 rounded-full animate-bounce bg-indigo-500" style="animation-delay: 300ms"></div>
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Question Input -->
    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
      <h2 class="font-semibold text-gray-900">Ask a Question</h2>

      <textarea
        class="w-full p-3 border border-gray-200 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
        bind:value={question}
        rows="3"
        placeholder="Ask about your productivity, finances, goals, work patterns..."
        on:keydown={(e) => e.key === 'Enter' && !e.shiftKey && (e.preventDefault(), handleAskAdvisor())}
      ></textarea>

      <div class="flex items-center justify-between">
        <p class="text-xs text-gray-400">
          Enter to send • Shift+Enter for new line
        </p>
        <button
          class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg flex items-center gap-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          on:click={() => handleAskAdvisor()}
          disabled={thinking || !question.trim()}
        >
          <Send size={16} />
          {thinking ? 'Thinking...' : 'Send'}
        </button>
      </div>
    </div>

    <p class="text-center text-xs text-gray-400">
      Powered by GPT-4o • Your data stays private
    </p>
  {/if}
</div>

<!-- Rate Limit Modal -->
{#if showRateLimitModal}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    on:click|self={() => showRateLimitModal = false}
    on:keydown={(e) => e.key === 'Escape' && (showRateLimitModal = false)}
  >
    <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden">
      <!-- Header -->
      <div class="bg-gradient-to-br from-amber-500 to-orange-600 p-6 text-white">
        <div class="flex items-center gap-3">
          <div class="p-3 bg-white/20 rounded-xl">
            <Zap size={28} />
          </div>
          <div>
            <h2 class="text-xl font-bold">Daily Limit Reached</h2>
            <p class="text-white/80 text-sm">You've used all your free queries</p>
          </div>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        <div class="bg-amber-50 rounded-xl p-4 mb-6">
          <p class="text-amber-800 text-sm">
            You've reached your limit of <strong>10 AI queries</strong> for today.
            {#if rateLimitResetTime}
              Your limit resets at <strong>{rateLimitResetTime}</strong>.
            {:else}
              Your limit resets at <strong>midnight</strong>.
            {/if}
          </p>
        </div>

        <div class="mb-6">
          <p class="text-gray-600 text-sm mb-3">
            Upgrade to Pro for <strong>100 queries/day</strong> plus:
          </p>
          <ul class="space-y-2 text-sm text-gray-600">
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
          </ul>
        </div>

        <!-- Action buttons -->
        <div class="space-y-3">
          <button
            class="w-full py-3 px-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white font-semibold rounded-xl hover:from-indigo-600 hover:to-purple-700 transition-all shadow-lg shadow-indigo-500/25 flex items-center justify-center gap-2"
            on:click={() => { window.open(checkoutUrls.yearly, '_blank'); showRateLimitModal = false; }}
          >
            <Crown size={18} />
            Upgrade to Pro
          </button>
          <button
            class="w-full py-2 text-gray-500 text-sm hover:text-gray-700 transition-colors"
            on:click={() => showRateLimitModal = false}
          >
            Wait until tomorrow
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
