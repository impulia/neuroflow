import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export interface StateResponse {
  state: string;
  elapsed_secs: number;
  session_elapsed_secs: number;
  idle_time: number;
  paused: boolean;
}

export interface StatsResponse {
  today_focus_secs: number;
  today_idle_secs: number;
  today_interruptions: number;
  session_focus_secs: number;
  session_idle_secs: number;
  best_streak_secs: number;
  daily_goal_secs: number;
  week_focus_secs: number;
}

export interface DayChartData {
  label: string;
  focus_secs: number;
  idle_secs: number;
  is_today: boolean;
}

export interface ConfigResponse {
  threshold_mins: number;
  duration: string | null;
  daily_goal_hours: number;
  show_timer_in_menubar: boolean;
  show_motivational_messages: boolean;
  launch_at_login: boolean;
}

interface TrackerUpdate {
  state: StateResponse;
  stats: StatsResponse;
  weekly: DayChartData[];
  config: ConfigResponse;
}

const defaultState: StateResponse = {
  state: 'waiting',
  elapsed_secs: 0,
  session_elapsed_secs: 0,
  idle_time: 0,
  paused: false,
};

const defaultStats: StatsResponse = {
  today_focus_secs: 0,
  today_idle_secs: 0,
  today_interruptions: 0,
  session_focus_secs: 0,
  session_idle_secs: 0,
  best_streak_secs: 0,
  daily_goal_secs: 14400,
  week_focus_secs: 0,
};

const defaultConfig: ConfigResponse = {
  threshold_mins: 5,
  duration: null,
  daily_goal_hours: 4,
  show_timer_in_menubar: false,
  show_motivational_messages: true,
  launch_at_login: false,
};

export const currentState = writable<StateResponse>(defaultState);
export const stats = writable<StatsResponse>(defaultStats);
export const weeklyData = writable<DayChartData[]>([]);
export const config = writable<ConfigResponse>(defaultConfig);

let unlisten: UnlistenFn | null = null;

/**
 * Subscribe to the `tracker-update` event pushed by the Rust backend
 * every second. Replaces the old 1-second polling approach.
 */
export async function startListening(): Promise<void> {
  if (unlisten !== null) return;

  unlisten = await listen<TrackerUpdate>('tracker-update', (event) => {
    const { state, stats: s, weekly, config: c } = event.payload;
    currentState.set(state);
    stats.set(s);
    weeklyData.set(weekly);
    config.set(c);
  });
}

export function stopListening(): void {
  if (unlisten !== null) {
    unlisten();
    unlisten = null;
  }
}

export async function pauseTracking(): Promise<void> {
  try {
    await invoke('pause_tracking');
  } catch (err) {
    console.error('Failed to pause tracking:', err);
  }
}

export async function resumeTracking(): Promise<void> {
  try {
    await invoke('resume_tracking');
  } catch (err) {
    console.error('Failed to resume tracking:', err);
  }
}

export async function resetToday(): Promise<void> {
  try {
    await invoke('reset_today');
  } catch (err) {
    console.error('Failed to reset today:', err);
  }
}

export async function updateConfig(newConfig: ConfigResponse): Promise<void> {
  try {
    await invoke('update_config', { newCfg: newConfig });
  } catch (err) {
    console.error('Failed to update config:', err);
  }
}
