<script lang="ts">
  import type { StatsResponse } from '../stores/tracker';
  import ProgressRing from './ProgressRing.svelte';
  import StatCard from './StatCard.svelte';

  export let stats: StatsResponse;
  export let daily_goal_secs: number;

  function formatHoursMinutes(secs: number): string {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    if (h > 0 && m > 0) return `${h}h ${m}m`;
    if (h > 0) return `${h}h`;
    if (m > 0) return `${m}m`;
    return '0m';
  }

  function formatShort(secs: number): string {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    if (h > 0) return `${h}h ${m}m`;
    return `${m}m`;
  }

  $: goalProgress = daily_goal_secs > 0
    ? Math.min(1, stats.today_focus_secs / daily_goal_secs)
    : 0;

  $: focusDisplay = formatShort(stats.today_focus_secs);
  $: goalDisplay = formatHoursMinutes(daily_goal_secs);
  $: streakDisplay = formatHoursMinutes(stats.best_streak_secs);
  $: interruptionsDisplay = String(stats.today_interruptions);

  $: progressPct = Math.round(goalProgress * 100);
</script>

<div class="stats-row">
  <!-- Focus Time card with progress ring -->
  <div class="stat-card-custom">
    <span class="card-label">Focus</span>
    <ProgressRing
      value={goalProgress}
      size={60}
      strokeWidth={5}
      color="var(--focus-green)"
    >
      <span class="ring-inner">{focusDisplay}</span>
    </ProgressRing>
    <span class="card-sublabel">of {goalDisplay} goal</span>
  </div>

  <!-- Interruptions -->
  <StatCard
    label="Interruptions"
    value={interruptionsDisplay}
    sublabel="today"
    color={stats.today_interruptions === 0 ? 'var(--focus-green)' : 'var(--idle-amber)'}
  />

  <!-- Best Streak -->
  <StatCard
    label="Best Streak"
    value={streakDisplay}
    sublabel="today"
    color="var(--focus-green)"
  />
</div>

<style>
  .stats-row {
    display: flex;
    flex-direction: row;
    gap: 8px;
    align-items: stretch;
  }

  .stat-card-custom {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: var(--card-bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 10px 8px;
    transition: background 0.2s ease, border-color 0.2s ease;
  }

  .stat-card-custom:hover {
    background: var(--card-bg-hover);
    border-color: var(--border-hover);
  }

  .card-label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
    text-align: center;
  }

  .ring-inner {
    font-size: 10px;
    font-weight: 700;
    color: var(--text);
    text-align: center;
    font-variant-numeric: tabular-nums;
    line-height: 1.1;
    white-space: nowrap;
  }

  .card-sublabel {
    font-size: 10px;
    color: var(--text-tertiary);
    text-align: center;
    line-height: 1.3;
  }
</style>
