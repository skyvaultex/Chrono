<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { Mic, MicOff, Loader2, X, Check, Clock, Briefcase, FileText } from 'lucide-svelte';
  import { getAllSessionTypes, addSession } from '../api';
  import type { SessionType, NewSession } from '../types';

  export let onClose: () => void = () => {};

  const dispatch = createEventDispatcher();

  let isListening = false;
  let transcript = '';
  let interimTranscript = '';
  let error = '';
  let recognition: any = null;
  let sessionTypes: SessionType[] = [];
  let supported = true;

  // Parsed session data
  let parsedSession: {
    hours: number | null;
    project: string | null;
    sessionType: SessionType | null;
    description: string | null;
  } = { hours: null, project: null, sessionType: null, description: null };

  let saving = false;
  let saved = false;

  onMount(async () => {
    // Check for speech recognition support
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    
    if (!SpeechRecognition) {
      supported = false;
      error = 'Speech recognition is not supported in this browser';
      return;
    }

    // Load session types
    try {
      sessionTypes = await getAllSessionTypes();
    } catch (e) {
      console.error('Failed to load session types:', e);
    }

    recognition = new SpeechRecognition();
    recognition.continuous = true;
    recognition.interimResults = true;
    recognition.lang = 'en-US';

    recognition.onresult = (event: any) => {
      let interim = '';
      let final = '';

      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i];
        if (result.isFinal) {
          final += result[0].transcript;
        } else {
          interim += result[0].transcript;
        }
      }

      if (final) {
        transcript += final;
        parseTranscript(transcript);
      }
      interimTranscript = interim;
    };

    recognition.onerror = (event: any) => {
      console.error('Speech recognition error:', event.error);
      if (event.error === 'not-allowed') {
        error = 'Microphone access denied. Please allow microphone access.';
      } else if (event.error === 'no-speech') {
        error = 'No speech detected. Please try again.';
      } else {
        error = `Error: ${event.error}`;
      }
      isListening = false;
    };

    recognition.onend = () => {
      if (isListening) {
        // Restart if still supposed to be listening
        try {
          recognition.start();
        } catch (e) {
          isListening = false;
        }
      }
    };
  });

  function toggleListening() {
    if (!recognition) return;

    if (isListening) {
      recognition.stop();
      isListening = false;
    } else {
      transcript = '';
      interimTranscript = '';
      error = '';
      parsedSession = { hours: null, project: null, sessionType: null, description: null };
      
      try {
        recognition.start();
        isListening = true;
      } catch (e) {
        error = 'Failed to start speech recognition';
      }
    }
  }

  function parseTranscript(text: string) {
    const lowerText = text.toLowerCase();

    // Parse hours - look for patterns like "2 hours", "1.5 hours", "30 minutes", "2 and a half hours"
    const hoursMatch = lowerText.match(/(\d+(?:\.\d+)?)\s*(?:hours?|hrs?)/i);
    const minutesMatch = lowerText.match(/(\d+)\s*(?:minutes?|mins?)/i);
    const halfHourMatch = lowerText.match(/(?:(\d+)\s*(?:and\s*)?)?(?:a\s*)?half\s*(?:an?\s*)?hours?/i);

    if (hoursMatch) {
      parsedSession.hours = parseFloat(hoursMatch[1]);
    } else if (minutesMatch) {
      parsedSession.hours = parseInt(minutesMatch[1]) / 60;
    } else if (halfHourMatch) {
      const baseHours = halfHourMatch[1] ? parseInt(halfHourMatch[1]) : 0;
      parsedSession.hours = baseHours + 0.5;
    }

    // Parse session type - match against known types
    for (const type of sessionTypes) {
      if (lowerText.includes(type.name.toLowerCase())) {
        parsedSession.sessionType = type;
        break;
      }
    }

    // Common work type keywords
    if (!parsedSession.sessionType) {
      if (lowerText.includes('coding') || lowerText.includes('programming') || lowerText.includes('development')) {
        parsedSession.sessionType = sessionTypes.find(t => 
          t.name.toLowerCase().includes('coding') || 
          t.name.toLowerCase().includes('dev') ||
          t.name.toLowerCase().includes('programming')
        ) || null;
      } else if (lowerText.includes('construction') || lowerText.includes('building') || lowerText.includes('physical')) {
        parsedSession.sessionType = sessionTypes.find(t => 
          t.name.toLowerCase().includes('construction') || 
          t.name.toLowerCase().includes('build') ||
          t.name.toLowerCase().includes('physical')
        ) || null;
      }
    }

    // Parse project name - look for "on [project]", "for [project]", "working on [project]"
    const projectMatch = text.match(/(?:on|for|working on|project)\s+([a-zA-Z0-9\s]+?)(?:\s+for|\s+doing|\s+hours?|\s+minutes?|$)/i);
    if (projectMatch) {
      parsedSession.project = projectMatch[1].trim();
    }

    // Use remaining text as description
    parsedSession.description = text;
  }

  async function saveSession() {
    if (!parsedSession.hours || !parsedSession.sessionType) {
      error = 'Please provide hours and session type';
      return;
    }

    saving = true;
    error = '';

    try {
      const today = new Date();
      const dateStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;

      const session: NewSession = {
        session_type_id: parsedSession.sessionType.id,
        date: dateStr,
        project_name: parsedSession.project || 'Voice Entry',
        hours: parsedSession.hours,
        description: parsedSession.description || undefined,
      };

      await addSession(session);
      saved = true;
      dispatch('saved');
      
      setTimeout(() => {
        onClose();
      }, 1500);
    } catch (e) {
      error = `Failed to save: ${e}`;
    } finally {
      saving = false;
    }
  }

  function clearAndRetry() {
    transcript = '';
    interimTranscript = '';
    parsedSession = { hours: null, project: null, sessionType: null, description: null };
    error = '';
  }
</script>

<div class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4">
  <div class="bg-white rounded-2xl shadow-2xl max-w-lg w-full overflow-hidden">
    <!-- Header -->
    <div class="bg-gradient-to-r from-indigo-600 to-purple-600 text-white p-5">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <Mic size={24} />
          <h2 class="text-xl font-bold">Voice Session Entry</h2>
        </div>
        <button on:click={onClose} class="p-1 hover:bg-white/20 rounded-lg transition-colors">
          <X size={20} />
        </button>
      </div>
      <p class="text-sm text-white/80 mt-2">
        Say something like: "I worked 2 hours on the website coding project"
      </p>
    </div>

    <div class="p-5 space-y-4">
      {#if !supported}
        <div class="text-center py-8">
          <MicOff size={48} class="text-gray-400 mx-auto mb-3" />
          <p class="text-gray-600 font-medium">Speech Recognition Not Supported</p>
          <p class="text-sm text-gray-500">Please use a modern browser like Chrome or Edge</p>
        </div>
      {:else}
        <!-- Microphone Button -->
        <div class="flex justify-center">
          <button
            on:click={toggleListening}
            class="relative w-24 h-24 rounded-full transition-all duration-300 {isListening 
              ? 'bg-red-500 hover:bg-red-600 animate-pulse' 
              : 'bg-indigo-600 hover:bg-indigo-700'}"
          >
            {#if isListening}
              <div class="absolute inset-0 rounded-full bg-red-400 animate-ping opacity-50"></div>
              <MicOff size={36} class="text-white mx-auto" />
            {:else}
              <Mic size={36} class="text-white mx-auto" />
            {/if}
          </button>
        </div>

        <p class="text-center text-sm text-gray-500">
          {isListening ? 'Listening... Tap to stop' : 'Tap to start speaking'}
        </p>

        <!-- Transcript -->
        {#if transcript || interimTranscript}
          <div class="bg-gray-50 rounded-lg p-4">
            <p class="text-sm text-gray-500 mb-1">What I heard:</p>
            <p class="text-gray-900">
              {transcript}<span class="text-gray-400">{interimTranscript}</span>
            </p>
          </div>
        {/if}

        <!-- Parsed Data -->
        {#if parsedSession.hours || parsedSession.sessionType || parsedSession.project}
          <div class="space-y-2">
            <p class="text-sm font-medium text-gray-700">Parsed session:</p>
            
            <div class="grid grid-cols-2 gap-2">
              {#if parsedSession.hours}
                <div class="flex items-center gap-2 p-2 bg-green-50 rounded-lg">
                  <Clock size={16} class="text-green-600" />
                  <span class="text-sm text-green-800">{parsedSession.hours.toFixed(1)} hours</span>
                </div>
              {:else}
                <div class="flex items-center gap-2 p-2 bg-gray-100 rounded-lg">
                  <Clock size={16} class="text-gray-400" />
                  <span class="text-sm text-gray-500">Hours not detected</span>
                </div>
              {/if}

              {#if parsedSession.sessionType}
                <div class="flex items-center gap-2 p-2 bg-green-50 rounded-lg">
                  <Briefcase size={16} class="text-green-600" />
                  <span class="text-sm text-green-800">{parsedSession.sessionType.name}</span>
                </div>
              {:else}
                <div class="flex items-center gap-2 p-2 bg-gray-100 rounded-lg">
                  <Briefcase size={16} class="text-gray-400" />
                  <span class="text-sm text-gray-500">Type not detected</span>
                </div>
              {/if}
            </div>

            {#if parsedSession.project}
              <div class="flex items-center gap-2 p-2 bg-green-50 rounded-lg">
                <FileText size={16} class="text-green-600" />
                <span class="text-sm text-green-800">Project: {parsedSession.project}</span>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Error -->
        {#if error}
          <div class="p-3 bg-red-50 text-red-700 rounded-lg text-sm">
            {error}
          </div>
        {/if}

        <!-- Success -->
        {#if saved}
          <div class="p-4 bg-green-50 rounded-lg text-center">
            <Check size={32} class="text-green-600 mx-auto mb-2" />
            <p class="text-green-700 font-medium">Session saved!</p>
          </div>
        {/if}

        <!-- Actions -->
        {#if !saved}
          <div class="flex gap-3 pt-2">
            <button
              on:click={clearAndRetry}
              class="flex-1 px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
              disabled={saving}
            >
              Clear & Retry
            </button>
            <button
              on:click={saveSession}
              disabled={!parsedSession.hours || !parsedSession.sessionType || saving}
              class="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {#if saving}
                <Loader2 size={18} class="animate-spin" />
                Saving...
              {:else}
                <Check size={18} />
                Save Session
              {/if}
            </button>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>
