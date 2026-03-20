import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

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

let pollInterval: ReturnType<typeof setInterval> | null = null;

async function fetchAll(): Promise<void> {
  try {
    const [stateRes, statsRes, weekRes, configRes] = await Promise.all([
      invoke<StateResponse>('get_current_state'),
      invoke<StatsResponse>('get_stats'),
      invoke<DayChartData[]>('get_weekly_chart_data'),
      invoke<ConfigResponse>('get_config'),
    ]);
    currentState.set(stateRes);
    stats.set(statsRes);
    weeklyData.set(weekRes);
    config.set(configRes);
  } catch (err) {
    console.error('Failed to fetch tracker data:', err);
  }
}

export function startPolling(): void {
  if (pollInterval !== null) return;
  fetchAll();
  pollInterval = setInterval(fetchAll, 1000);
}

export function stopPolling(): void {
  if (pollInterval !== null) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}

export async function pauseTracking(): Promise<void> {
  try {
    await invoke('pause_tracking');
    await fetchAll();
  } catch (err) {
    console.error('Failed to pause tracking:', err);
  }
}

export async function resumeTracking(): Promise<void> {
  try {
    await invoke('resume_tracking');
    await fetchAll();
  } catch (err) {
    console.error('Failed to resume tracking:', err);
  }
}

export async function resetToday(): Promise<void> {
  try {
    await invoke('reset_today');
    await fetchAll();
  } catch (err) {
    console.error('Failed to reset today:', err);
  }
}

export async function updateConfig(newConfig: ConfigResponse): Promise<void> {
  try {
    await invoke('update_config', { newCfg: newConfig });
    config.set(newConfig);
  } catch (err) {
    console.error('Failed to update config:', err);
  }
}
