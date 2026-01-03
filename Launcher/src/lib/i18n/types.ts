export interface Translations {
  // Navigation
  nav: {
    dashboard: string;
    sessions: string;
    goals: string;
    analytics: string;
    advisor: string;
    tools: string;
    milestones: string;
    financialSimulator: string;
    focusWellbeing: string;
    invoices: string;
    settings: string;
  };
  
  // Common
  common: {
    loading: string;
    save: string;
    cancel: string;
    delete: string;
    edit: string;
    add: string;
    close: string;
    confirm: string;
    success: string;
    error: string;
    warning: string;
    today: string;
    yesterday: string;
    thisWeek: string;
    thisMonth: string;
    allTime: string;
    hours: string;
    minutes: string;
    unlimited: string;
    free: string;
    pro: string;
    lifetime: string;
  };
  
  // Dashboard
  dashboard: {
    title: string;
    todayEarnings: string;
    weeklyEarnings: string;
    monthlyEarnings: string;
    hoursWorked: string;
    activeTimer: string;
    startSession: string;
    stopSession: string;
    quickStats: string;
    recentSessions: string;
    noSessions: string;
  };
  
  // Sessions
  sessions: {
    title: string;
    newSession: string;
    editSession: string;
    sessionType: string;
    startTime: string;
    endTime: string;
    duration: string;
    hourlyRate: string;
    earnings: string;
    notes: string;
    project: string;
    noSessions: string;
    addSessionType: string;
    manageTypes: string;
  };
  
  // Goals
  goals: {
    title: string;
    newGoal: string;
    editGoal: string;
    goalName: string;
    targetAmount: string;
    currentAmount: string;
    deadline: string;
    progress: string;
    completed: string;
    inProgress: string;
    noGoals: string;
    debt: string;
    savings: string;
    purchase: string;
  };
  
  // Analytics
  analytics: {
    title: string;
    earnings: string;
    hoursWorked: string;
    averageRate: string;
    bestDay: string;
    totalSessions: string;
    bySessionType: string;
    earningsTrend: string;
    hoursTrend: string;
    noData: string;
  };
  
  // Advisor
  advisor: {
    title: string;
    askQuestion: string;
    placeholder: string;
    analyzing: string;
    suggestions: string;
    insights: string;
    proRequired: string;
  };
  
  // Settings
  settings: {
    title: string;
    subtitle: string;
    currentPlan: string;
    freePlan: string;
    proPlan: string;
    lifetimePlan: string;
    activateLicense: string;
    licenseKey: string;
    activate: string;
    activating: string;
    deactivate: string;
    activated: string;
    activationSuccess: string;
    activationError: string;
    enterLicenseKey: string;
    features: string;
    featureComparison: string;
    upgradeToPro: string;
    unlockFeatures: string;
    monthly: string;
    yearly: string;
    perMonth: string;
    perYear: string;
    save35: string;
    billedMonthly: string;
    billedYearly: string;
    cancelAnytime: string;
    bestValue: string;
    about: string;
    aboutDescription: string;
    version: string;
    updates: string;
    checkForUpdates: string;
    checking: string;
    upToDate: string;
    updateAvailable: string;
    installUpdate: string;
    installing: string;
    whatsNew: string;
    appearance: string;
    theme: string;
    light: string;
    dark: string;
    system: string;
    language: string;
    selectLanguage: string;
  };
  
  // Features
  features: {
    unlimitedSessionTypes: string;
    unlimitedGoals: string;
    analyticsHistory: string;
    invoiceGeneration: string;
    aiAdvisor: string;
    voiceInput: string;
    simulator: string;
    pdfExport: string;
    days7: string;
    fullHistory: string;
  };
  
  // Simulator
  simulator: {
    title: string;
    projectedEarnings: string;
    scenario: string;
    hoursPerWeek: string;
    weeks: string;
    calculate: string;
  };
  
  // Focus & Wellbeing
  focus: {
    title: string;
    focusTimer: string;
    breakReminder: string;
    dailyGoal: string;
    streak: string;
    startFocus: string;
    takeBreak: string;
  };
  
  // Invoices
  invoices: {
    title: string;
    newInvoice: string;
    client: string;
    amount: string;
    status: string;
    paid: string;
    pending: string;
    overdue: string;
    exportPdf: string;
  };
  
  // Achievements
  achievements: {
    title: string;
    unlocked: string;
    locked: string;
    progress: string;
    allUnlocked: string;
  };
}
