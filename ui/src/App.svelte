<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    currentState,
    stats,
    weeklyData,
    config,
    startListening,
    stopListening,
    updateConfig,
    resetToday,
    type ConfigResponse,
  } from './stores/tracker';
  import StatusHeader from './lib/StatusHeader.svelte';
  import MotivationalBanner from './lib/MotivationalBanner.svelte';
  import StatsRow from './lib/StatsRow.svelte';
  import WeeklyChart from './lib/WeeklyChart.svelte';
  import Footer from './lib/Footer.svelte';
  import Settings from './lib/Settings.svelte';

  let showSettings = false;

  onMount(() => {
    startListening();
  });

  onDestroy(() => {
    stopListening();
  });

  function handleSettingsClick() {
    showSettings = true;
  }

  function handleBack() {
    showSettings = false;
  }

  async function handleSave(newConfig: ConfigResponse) {
    await updateConfig(newConfig);
  }

  async function handleResetToday() {
    await resetToday();
  }
</script>

<div class="app-wrapper">
  <div class="container glass-container">
    {#if showSettings}
      <div class="view settings-view slide-in-right">
        <Settings
          config={$config}
          onBack={handleBack}
          onSave={handleSave}
          onResetToday={handleResetToday}
        />
      </div>
    {:else}
      <div class="view dashboard-view slide-in">
        <StatusHeader
          state={$currentState.state}
          elapsed_secs={$currentState.elapsed_secs}
          paused={$currentState.paused}
        />
        <MotivationalBanner
          state={$currentState.state}
          elapsed_secs={$currentState.elapsed_secs}
          show={$config.show_motivational_messages}
        />
        <StatsRow
          stats={$stats}
          daily_goal_secs={$stats.daily_goal_secs}
        />
        <div class="chart-section">
          <p class="section-label">This Week</p>
          <WeeklyChart data={$weeklyData} />
        </div>
        <Footer
          week_focus_secs={$stats.week_focus_secs}
          onSettingsClick={handleSettingsClick}
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .app-wrapper {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: 0;
    background: transparent;
  }

  .container {
    width: 340px;
    min-height: 480px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }

  .view {
    display: flex;
    flex-direction: column;
    flex: 1;
    width: 100%;
  }

  .dashboard-view {
    padding: 16px;
    gap: 12px;
  }

  .settings-view {
    padding: 0;
  }

  .chart-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .section-label {
    margin: 0;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
  }
</style>
