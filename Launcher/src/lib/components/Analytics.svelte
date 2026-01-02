<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getAnalytics } from '../api';
  import { logAppEvent } from '../api';
  import type { AnalyticsData } from '../types';
  import { Clock, Calendar, TrendingUp, Award, DollarSign, BarChart3, PieChart, Activity, Zap, ArrowUpDown, ChevronUp, ChevronDown } from 'lucide-svelte';

  let analyticsData: AnalyticsData | null = null;
  let loading = true;
  let error = '';
  
  type RangeType = 'today' | '7d' | '30d' | '90d' | 'year' | 'all';
  let selectedRange: RangeType = '30d';

  let ApexCharts: any = null;
  let charts: any[] = [];

  // Interactive state
  let selectedCategory: string | null = null;
  
  // Chart view toggle (trend vs weekday)
  type ChartView = 'trend' | 'weekday';
  let chartView: ChartView = 'trend';
  
  // Sorting state for table
  type SortKey = 'category' | 'sessions' | 'hours' | 'pay';
  type SortDir = 'asc' | 'desc';
  let sortKey: SortKey = 'hours';
  let sortDir: SortDir = 'desc';

  // Computed insights
  $: consistencyScore = calculateConsistency(analyticsData);
  $: bestWorstDays = calculateBestWorstDays(analyticsData);
  $: avgHoursPerDay = analyticsData ? (analyticsData.summary.total_hours / Math.max(analyticsData.daily_hours.length, 1)) : 0;
  
  $: sortedCategories = analyticsData?.category_breakdown
    .filter(c => !selectedCategory || c.category === selectedCategory)
    .sort((a, b) => {
      const modifier = sortDir === 'asc' ? 1 : -1;
      switch (sortKey) {
        case 'category': return modifier * a.category.localeCompare(b.category);
        case 'sessions': return modifier * (a.sessions - b.sessions);
        case 'hours': return modifier * (a.hours - b.hours);
        case 'pay': return modifier * (a.pay - b.pay);
        default: return 0;
      }
    }) ?? [];

  function calculateConsistency(data: AnalyticsData | null): number {
    if (!data || data.daily_hours.length === 0) return 0;
    const { start, end } = getDateRange(selectedRange);
    const startDate = new Date(start);
    const endDate = new Date(end);
    const totalDays = Math.ceil((endDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24)) + 1;
    const daysWithWork = data.daily_hours.filter(d => d.hours >= 0.5).length; // At least 30 min
    return Math.round((daysWithWork / totalDays) * 100);
  }

  function calculateBestWorstDays(data: AnalyticsData | null): { best: { day: string; hours: number } | null; worst: { day: string; hours: number } | null } {
    if (!data || data.weekday_breakdown.length === 0) {
      return { best: null, worst: null };
    }
    const sorted = [...data.weekday_breakdown].sort((a, b) => b.hours - a.hours);
    const best = sorted[0];
    const worst = sorted[sorted.length - 1];
    return {
      best: best.hours > 0 ? { day: best.weekday, hours: best.hours } : null,
      worst: worst.hours >= 0 ? { day: worst.weekday, hours: worst.hours } : null
    };
  }

  function toggleSort(key: SortKey) {
    if (sortKey === key) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = key;
      sortDir = 'desc';
    }
  }

  function handleCategoryClick(category: string) {
    selectedCategory = selectedCategory === category ? null : category;
  }

  // Format date as YYYY-MM-DD using local timezone (not UTC)
  function formatLocalDate(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  }

  function getDateRange(range: RangeType): { start: string; end: string } {
    const today = new Date();
    const end = formatLocalDate(today);
    let start: Date;

    switch (range) {
      case 'today':
        start = today;
        break;
      case '7d':
        start = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000);
        break;
      case '30d':
        start = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);
        break;
      case '90d':
        start = new Date(today.getTime() - 90 * 24 * 60 * 60 * 1000);
        break;
      case 'year':
        start = new Date(today.getFullYear(), 0, 1);
        break;
      case 'all':
        start = new Date(2020, 0, 1);
        break;
      default:
        start = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);
    }

    return {
      start: formatLocalDate(start),
      end
    };
  }

  async function loadAnalytics() {
    loading = true;
    error = '';
    destroyCharts();
    
    try {
      const { start, end } = getDateRange(selectedRange);
      analyticsData = await getAnalytics(start, end);
      console.log('Analytics data loaded:', analyticsData);
    } catch (e: any) {
      error = e.toString();
      console.error('Analytics error:', e);
    } finally {
      loading = false;
    }
  }

  function destroyCharts() {
    charts.forEach(chart => {
      try { chart.destroy(); } catch (e) {}
    });
    charts = [];
  }

  // Svelte action for rendering charts
  function lineChartAction(node: HTMLElement) {
    if (!ApexCharts || !analyticsData || analyticsData.daily_hours.length === 0) return;

    // Calculate average for baseline
    const avgHours = analyticsData.daily_hours.reduce((sum, d) => sum + d.hours, 0) / analyticsData.daily_hours.length;

    const options = {
      chart: {
        type: 'area',
        height: 280,
        toolbar: { show: false },
        fontFamily: 'inherit',
        background: 'transparent',
        animations: { enabled: true }
      },
      series: [
        {
          name: 'Hours',
          type: 'area',
          data: analyticsData.daily_hours.map(d => ({ x: new Date(d.date).getTime(), y: Number(d.hours.toFixed(1)) }))
        },
        {
          name: 'Average',
          type: 'line',
          data: analyticsData.daily_hours.map(d => ({ x: new Date(d.date).getTime(), y: Number(avgHours.toFixed(1)) }))
        }
      ],
      dataLabels: {
        enabled: false
      },
      xaxis: {
        type: 'datetime',
        labels: { style: { colors: '#6B7280' } }
      },
      yaxis: {
        labels: { 
          style: { colors: '#6B7280' },
          formatter: (val: number) => val.toFixed(1) + 'h'
        }
      },
      stroke: { 
        curve: ['smooth', 'straight'],
        width: [3, 2],
        dashArray: [0, 5]
      },
      fill: {
        type: ['gradient', 'solid'],
        gradient: { shadeIntensity: 1, opacityFrom: 0.4, opacityTo: 0.1, stops: [0, 100] },
        opacity: [1, 0]
      },
      colors: ['#6366F1', '#9CA3AF'],
      grid: { borderColor: '#E5E7EB', strokeDashArray: 4 },
      legend: {
        show: true,
        position: 'top',
        horizontalAlign: 'right',
        markers: { width: 8, height: 8 }
      },
      tooltip: {
        x: { format: 'MMM dd, yyyy' },
        y: { 
          formatter: (val: number, { seriesIndex }: any) => {
            if (seriesIndex === 1) return val.toFixed(1) + 'h avg';
            const diff = val - avgHours;
            const diffStr = diff >= 0 ? `+${diff.toFixed(1)}h` : `${diff.toFixed(1)}h`;
            return `${val.toFixed(1)} hours (${diffStr} vs avg)`;
          }
        }
      }
    };

    const chart = new ApexCharts(node, options);
    chart.render();
    charts.push(chart);

    return {
      destroy() {
        try { chart.destroy(); } catch (e) {}
      }
    };
  }

  function donutChartAction(node: HTMLElement) {
    if (!ApexCharts || !analyticsData || analyticsData.category_breakdown.length === 0) return;

    const categories = analyticsData.category_breakdown;
    
    const options = {
      chart: {
        type: 'donut',
        height: 280,
        fontFamily: 'inherit',
        events: {
          dataPointSelection: (_event: any, _chartContext: any, config: any) => {
            const categoryName = categories[config.dataPointIndex]?.category;
            if (categoryName) {
              handleCategoryClick(categoryName);
            }
          }
        }
      },
      series: categories.map(c => Number(c.hours.toFixed(1))),
      labels: categories.map(c => c.category),
      colors: categories.map(c => c.color),
      legend: {
        position: 'bottom',
        labels: { colors: '#374151' },
        onItemClick: {
          toggleDataSeries: false
        }
      },
      states: {
        active: {
          filter: { type: 'none' }
        }
      },
      plotOptions: {
        pie: {
          donut: {
            size: '65%',
            labels: {
              show: true,
              total: {
                show: true,
                label: 'Total Tracked',
                formatter: () => analyticsData!.summary.total_hours.toFixed(1) + 'h'
              }
            }
          },
          expandOnClick: false
        }
      },
      tooltip: {
        y: { 
          formatter: (val: number) => {
            const pct = ((val / analyticsData!.summary.total_hours) * 100).toFixed(1);
            return `${val.toFixed(1)} hours (${pct}%)`;
          }
        }
      }
    };

    const chart = new ApexCharts(node, options);
    chart.render();
    charts.push(chart);

    return {
      destroy() {
        try { chart.destroy(); } catch (e) {}
      }
    };
  }

  function barChartAction(node: HTMLElement) {
    if (!ApexCharts || !analyticsData) return;

    const weekdayData = analyticsData.weekday_breakdown;
    const totalHours = weekdayData.reduce((sum, w) => sum + w.hours, 0);

    const options = {
      chart: {
        type: 'bar',
        height: 250,
        toolbar: { show: false },
        fontFamily: 'inherit'
      },
      series: [{
        name: 'Hours',
        data: weekdayData.map(w => Number(w.hours.toFixed(1)))
      }],
      xaxis: {
        categories: weekdayData.map(w => w.weekday),
        labels: { style: { colors: '#6B7280' } }
      },
      yaxis: {
        labels: { 
          style: { colors: '#6B7280' },
          formatter: (val: number) => val.toFixed(1) + 'h'
        }
      },
      colors: ['#22C55E'],
      plotOptions: {
        bar: { 
          borderRadius: 6, 
          columnWidth: '60%',
          dataLabels: {
            position: 'top'
          }
        }
      },
      dataLabels: {
        enabled: true,
        formatter: (val: number) => val > 0 ? val.toFixed(1) : '',
        offsetY: -20,
        style: {
          fontSize: '11px',
          colors: ['#6B7280']
        }
      },
      grid: { borderColor: '#E5E7EB', strokeDashArray: 4 },
      tooltip: {
        y: { 
          formatter: (val: number, { dataPointIndex }: any) => {
            const sessions = weekdayData[dataPointIndex]?.sessions ?? 0;
            const pct = totalHours > 0 ? ((val / totalHours) * 100).toFixed(1) : '0';
            return `${val.toFixed(1)} hours (${sessions} sessions, ${pct}% of week)`;
          }
        }
      }
    };

    const chart = new ApexCharts(node, options);
    chart.render();
    charts.push(chart);

    return {
      destroy() {
        try { chart.destroy(); } catch (e) {}
      }
    };
  }

  onMount(async () => {
    const module = await import('apexcharts');
    ApexCharts = module.default;
    console.log('ApexCharts loaded');
    await loadAnalytics();
  });

  onDestroy(() => {
    destroyCharts();
  });

  function handleRangeChange(range: RangeType) {
    selectedRange = range;
    loadAnalytics();
    
    // Log for "Zoomed Out" achievement
    logAppEvent('analytics_range', range).catch(console.error);
  }

  function formatHours(hours: number): string {
    if (hours < 1) return `${Math.round(hours * 60)}m`;
    const h = Math.floor(hours);
    const m = Math.round((hours - h) * 60);
    return m > 0 ? `${h}h ${m}m` : `${h}h`;
  }

  function formatPay(pay: number): string {
    return '$' + pay.toFixed(2);
  }
</script>

<div class="p-6 max-w-7xl mx-auto">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-indigo-100 rounded-lg">
        <BarChart3 size={24} class="text-indigo-600" />
      </div>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Analytics</h1>
        <p class="text-gray-500 text-sm">Track your productivity trends</p>
      </div>
    </div>

    <!-- Range Selector -->
    <div class="flex gap-1 bg-gray-100 p-1 rounded-lg">
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === 'today' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('today')}
      >Today</button>
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === '7d' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('7d')}
      >7D</button>
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === '30d' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('30d')}
      >30D</button>
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === '90d' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('90d')}
      >90D</button>
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === 'year' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('year')}
      >Year</button>
      <button
        class="px-3 py-1.5 text-sm font-medium rounded-md transition-all {selectedRange === 'all' 
          ? 'bg-white text-indigo-600 shadow-sm' 
          : 'text-gray-600 hover:text-gray-900'}"
        on:click={() => handleRangeChange('all')}
      >All</button>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
    </div>
  {:else if error}
    <div class="bg-red-50 text-red-600 p-4 rounded-lg">
      {error}
    </div>
  {:else if analyticsData}
    <!-- Summary Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-6 gap-4 mb-6">
      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-indigo-100 rounded-lg">
            <Clock size={20} class="text-indigo-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Total Time</p>
            <p class="text-xl font-bold text-gray-900">{formatHours(analyticsData.summary.total_hours)}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-green-100 rounded-lg">
            <Calendar size={20} class="text-green-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Sessions</p>
            <p class="text-xl font-bold text-gray-900">{analyticsData.summary.total_sessions}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-blue-100 rounded-lg">
            <TrendingUp size={20} class="text-blue-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Avg Session</p>
            <p class="text-xl font-bold text-gray-900">{formatHours(analyticsData.summary.avg_session_length)}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-amber-100 rounded-lg">
            <Award size={20} class="text-amber-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Longest</p>
            <p class="text-xl font-bold text-gray-900">{formatHours(analyticsData.summary.longest_session)}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-emerald-100 rounded-lg">
            <DollarSign size={20} class="text-emerald-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Total Pay</p>
            <p class="text-xl font-bold text-gray-900">{formatPay(analyticsData.summary.total_pay)}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-purple-100 rounded-lg">
            <Zap size={20} class="text-purple-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500 uppercase tracking-wide">Consistency</p>
            <p class="text-xl font-bold text-gray-900">{consistencyScore}%</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Insights Row -->
    {#if bestWorstDays.best || bestWorstDays.worst}
      <div class="bg-gradient-to-r from-indigo-50 to-purple-50 rounded-xl p-4 mb-6 border border-indigo-100">
        <div class="flex flex-wrap gap-6 text-sm">
          {#if bestWorstDays.best}
            <div class="flex items-center gap-2">
              <span class="text-green-600 font-medium">🏆 Best day:</span>
              <span class="text-gray-700">{bestWorstDays.best.day} ({bestWorstDays.best.hours.toFixed(1)}h)</span>
            </div>
          {/if}
          {#if bestWorstDays.worst}
            <div class="flex items-center gap-2">
              <span class="text-amber-600 font-medium">📉 Lightest day:</span>
              <span class="text-gray-700">{bestWorstDays.worst.day} ({bestWorstDays.worst.hours.toFixed(1)}h)</span>
            </div>
          {/if}
          {#if avgHoursPerDay > 0}
            <div class="flex items-center gap-2">
              <span class="text-indigo-600 font-medium">📊 Daily avg:</span>
              <span class="text-gray-700">{avgHoursPerDay.toFixed(1)}h/day</span>
            </div>
          {/if}
        </div>
      </div>
    {/if}

    <!-- Charts Row -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <!-- Trend / Weekday Chart with Toggle -->
      <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-2">
            {#if chartView === 'trend'}
              <Activity size={18} class="text-indigo-600" />
              <h3 class="font-semibold text-gray-900">Hours Trend</h3>
            {:else}
              <BarChart3 size={18} class="text-green-600" />
              <h3 class="font-semibold text-gray-900">Hours by Weekday</h3>
            {/if}
          </div>
          <!-- Toggle Switch -->
          <div class="flex items-center gap-2 bg-gray-100 p-1 rounded-lg">
            <button
              class="px-2.5 py-1 text-xs font-medium rounded-md transition-all {chartView === 'trend' 
                ? 'bg-white text-indigo-600 shadow-sm' 
                : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => chartView = 'trend'}
            >
              <Activity size={14} class="inline -mt-0.5" /> Trend
            </button>
            <button
              class="px-2.5 py-1 text-xs font-medium rounded-md transition-all {chartView === 'weekday' 
                ? 'bg-white text-green-600 shadow-sm' 
                : 'text-gray-500 hover:text-gray-700'}"
              on:click={() => chartView = 'weekday'}
            >
              <BarChart3 size={14} class="inline -mt-0.5" /> Weekday
            </button>
          </div>
        </div>
        
        {#if chartView === 'trend'}
          {#if analyticsData.daily_hours.length > 0}
            {#key analyticsData}
              <div use:lineChartAction></div>
            {/key}
          {:else}
            <div class="h-64 flex items-center justify-center text-gray-400">
              <div class="text-center">
                <Activity size={32} class="mx-auto mb-2 opacity-50" />
                <p>No sessions in this period yet</p>
                <p class="text-xs mt-1">Log some work to see your trends!</p>
              </div>
            </div>
          {/if}
        {:else}
          {#key analyticsData}
            <div use:barChartAction></div>
          {/key}
        {/if}
      </div>

      <!-- Donut Chart - Distribution -->
      <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-2">
            <PieChart size={18} class="text-indigo-600" />
            <h3 class="font-semibold text-gray-900">Time by Category</h3>
          </div>
          {#if selectedCategory}
            <button 
              class="text-xs text-indigo-600 hover:text-indigo-800 flex items-center gap-1"
              on:click={() => selectedCategory = null}
            >
              ✕ Clear filter: {selectedCategory}
            </button>
          {:else}
            <span class="text-xs text-gray-500">Click a slice to filter details below</span>
          {/if}
        </div>
        {#if analyticsData.category_breakdown.length > 0}
          {#key analyticsData}
            <div use:donutChartAction class="cursor-pointer"></div>
          {/key}
        {:else}
          <div class="h-64 flex items-center justify-center text-gray-400">
            <div class="text-center">
              <PieChart size={32} class="mx-auto mb-2 opacity-50" />
              <p>No sessions in this period yet</p>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Category Breakdown Table -->
    {#if analyticsData.category_breakdown.length > 0}
      <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 mt-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-semibold text-gray-900">Category Details</h3>
          {#if selectedCategory}
            <span class="text-sm text-indigo-600 bg-indigo-50 px-2 py-1 rounded">
              Showing: {selectedCategory}
            </span>
          {/if}
        </div>
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="text-left text-xs text-gray-500 uppercase tracking-wide border-b border-gray-100">
                <th class="pb-3 font-medium">
                  <button 
                    class="flex items-center gap-1 hover:text-gray-700 transition-colors"
                    on:click={() => toggleSort('category')}
                  >
                    Category
                    {#if sortKey === 'category'}
                      {#if sortDir === 'asc'}
                        <ChevronUp size={14} />
                      {:else}
                        <ChevronDown size={14} />
                      {/if}
                    {:else}
                      <ArrowUpDown size={14} class="opacity-30" />
                    {/if}
                  </button>
                </th>
                <th class="pb-3 font-medium text-right">
                  <button 
                    class="flex items-center gap-1 hover:text-gray-700 transition-colors ml-auto"
                    on:click={() => toggleSort('sessions')}
                  >
                    Sessions
                    {#if sortKey === 'sessions'}
                      {#if sortDir === 'asc'}
                        <ChevronUp size={14} />
                      {:else}
                        <ChevronDown size={14} />
                      {/if}
                    {:else}
                      <ArrowUpDown size={14} class="opacity-30" />
                    {/if}
                  </button>
                </th>
                <th class="pb-3 font-medium text-right">
                  <button 
                    class="flex items-center gap-1 hover:text-gray-700 transition-colors ml-auto"
                    on:click={() => toggleSort('hours')}
                  >
                    Hours
                    {#if sortKey === 'hours'}
                      {#if sortDir === 'asc'}
                        <ChevronUp size={14} />
                      {:else}
                        <ChevronDown size={14} />
                      {/if}
                    {:else}
                      <ArrowUpDown size={14} class="opacity-30" />
                    {/if}
                  </button>
                </th>
                <th class="pb-3 font-medium text-right">
                  <button 
                    class="flex items-center gap-1 hover:text-gray-700 transition-colors ml-auto"
                    on:click={() => toggleSort('pay')}
                  >
                    Pay
                    {#if sortKey === 'pay'}
                      {#if sortDir === 'asc'}
                        <ChevronUp size={14} />
                      {:else}
                        <ChevronDown size={14} />
                      {/if}
                    {:else}
                      <ArrowUpDown size={14} class="opacity-30" />
                    {/if}
                  </button>
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-50">
              {#each sortedCategories as cat}
                <tr class="hover:bg-gray-50 {selectedCategory === cat.category ? 'bg-indigo-50' : ''}">
                  <td class="py-3">
                    <div class="flex items-center gap-2">
                      <div class="w-3 h-3 rounded-full" style="background-color: {cat.color}"></div>
                      <span class="font-medium text-gray-900">{cat.category}</span>
                    </div>
                  </td>
                  <td class="py-3 text-right text-gray-600">{cat.sessions}</td>
                  <td class="py-3 text-right text-gray-600">{formatHours(cat.hours)}</td>
                  <td class="py-3 text-right font-medium text-emerald-600">{formatPay(cat.pay)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  {/if}
</div>
