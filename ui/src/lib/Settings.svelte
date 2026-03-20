<script lang="ts">
  import type { ConfigResponse } from '../stores/tracker';

  export let config: ConfigResponse;
  export let onBack: () => void;
  export let onSave: (newConfig: ConfigResponse) => Promise<void>;
  export let onResetToday: () => Promise<void>;

  // Local copies that we mutate
  let localThreshold = config.threshold_mins;
  let localDuration = config.duration;
  let localDailyGoal = config.daily_goal_hours;
  let localShowMotivational = config.show_motivational_messages;
  let localLaunchAtLogin = config.launch_at_login;
  let localShowTimerInMenubar = config.show_timer_in_menubar;

  let resetConfirming = false;
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  function buildConfig(): ConfigResponse {
    return {
      threshold_mins: localThreshold,
      duration: localDuration,
      daily_goal_hours: localDailyGoal,
      show_motivational_messages: localShowMotivational,
      show_timer_in_menubar: localShowTimerInMenubar,
      launch_at_login: localLaunchAtLogin,
    };
  }

  function handleThresholdChange() {
    onSave(buildConfig());
  }

  function handleGoalChange() {
    onSave(buildConfig());
  }

  function handleDurationChange() {
    onSave(buildConfig());
  }

  function handleToggleMotivational() {
    localShowMotivational = !localShowMotivational;
    onSave(buildConfig());
  }

  function handleToggleLaunchAtLogin() {
    localLaunchAtLogin = !localLaunchAtLogin;
    onSave(buildConfig());
  }

  function handleToggleTimerInMenubar() {
    localShowTimerInMenubar = !localShowTimerInMenubar;
    onSave(buildConfig());
  }

  function handleResetClick() {
    if (resetConfirming) {
      if (resetTimer) clearTimeout(resetTimer);
      resetConfirming = false;
      onResetToday();
    } else {
      resetConfirming = true;
      resetTimer = setTimeout(() => {
        resetConfirming = false;
      }, 3000);
    }
  }

  $: goalLabel = localDailyGoal % 1 === 0
    ? `${localDailyGoal}h`
    : `${localDailyGoal}h`;
</script>

<div class="settings-panel slide-in">
  <!-- Header -->
  <div class="settings-header">
    <button class="back-btn" on:click={onBack} aria-label="Back to dashboard">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path d="M10 13L5 8L10 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <h2 class="settings-title">Settings</h2>
    <div class="header-spacer" />
  </div>

  <div class="settings-body">
    <!-- Tracking Section -->
    <section class="settings-section">
      <h3 class="section-title">Tracking</h3>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Idle threshold</span>
          <span class="setting-desc">Consider idle after {localThreshold} min</span>
        </div>
        <div class="slider-group">
          <input
            type="range"
            min="1"
            max="30"
            step="1"
            bind:value={localThreshold}
            on:change={handleThresholdChange}
          />
          <span class="slider-value">{localThreshold}m</span>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Session duration</span>
          <span class="setting-desc">e.g. "8h", "4h30m"</span>
        </div>
        <input
          type="text"
          class="text-input"
          bind:value={localDuration}
          on:change={handleDurationChange}
          placeholder="8h"
          aria-label="Session duration"
        />
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Show timer in menu bar</span>
        </div>
        <button
          class="toggle"
          class:toggle-on={localShowTimerInMenubar}
          on:click={handleToggleTimerInMenubar}
          role="switch"
          aria-checked={localShowTimerInMenubar}
          aria-label="Show timer in menu bar"
        >
          <span class="toggle-thumb" />
        </button>
      </div>
    </section>

    <!-- Goals Section -->
    <section class="settings-section">
      <h3 class="section-title">Goals</h3>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Daily focus goal</span>
          <span class="setting-desc">Daily goal: {goalLabel}</span>
        </div>
        <div class="slider-group">
          <input
            type="range"
            min="0.5"
            max="12"
            step="0.5"
            bind:value={localDailyGoal}
            on:change={handleGoalChange}
          />
          <span class="slider-value">{goalLabel}</span>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Motivational messages</span>
          <span class="setting-desc">Show contextual tips while tracking</span>
        </div>
        <button
          class="toggle"
          class:toggle-on={localShowMotivational}
          on:click={handleToggleMotivational}
          role="switch"
          aria-checked={localShowMotivational}
          aria-label="Toggle motivational messages"
        >
          <span class="toggle-thumb" />
        </button>
      </div>
    </section>

    <!-- System Section -->
    <section class="settings-section">
      <h3 class="section-title">System</h3>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">Launch at login</span>
          <span class="setting-desc">Start Neflo when you log in</span>
        </div>
        <button
          class="toggle"
          class:toggle-on={localLaunchAtLogin}
          on:click={handleToggleLaunchAtLogin}
          role="switch"
          aria-checked={localLaunchAtLogin}
          aria-label="Toggle launch at login"
        >
          <span class="toggle-thumb" />
        </button>
      </div>
    </section>

    <!-- Data Section -->
    <section class="settings-section">
      <h3 class="section-title">Data</h3>

      <div class="setting-row reset-row">
        <div class="setting-label-group">
          <span class="setting-label">Reset today's data</span>
          <span class="setting-desc">Clear all tracked time for today</span>
        </div>
        <button
          class="reset-btn"
          class:reset-btn-confirm={resetConfirming}
          on:click={handleResetClick}
        >
          {resetConfirming ? 'Confirm?' : 'Reset'}
        </button>
      </div>
    </section>
  </div>
</div>

<style>
  .settings-panel {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-height: 480px;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 16px 10px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .back-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-xs);
    color: var(--text-secondary);
    transition: color 0.2s ease, background 0.2s ease;
    flex-shrink: 0;
  }

  .back-btn:hover {
    color: var(--text);
    background: var(--card-bg);
  }

  .settings-title {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
    color: var(--text);
    flex: 1;
    text-align: center;
  }

  .header-spacer {
    width: 28px;
    flex-shrink: 0;
  }

  .settings-body {
    flex: 1;
    overflow-y: auto;
    padding: 8px 16px 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px 0;
  }

  .settings-section + .settings-section {
    border-top: 1px solid var(--border);
  }

  .section-title {
    margin: 0 0 8px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 6px 0;
    min-height: 44px;
  }

  .setting-label-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .setting-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    line-height: 1.3;
  }

  .setting-desc {
    font-size: 11px;
    color: var(--text-secondary);
    line-height: 1.3;
  }

  /* Slider group */
  .slider-group {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 110px;
    flex-shrink: 0;
  }

  .slider-group input[type='range'] {
    flex: 1;
    min-width: 0;
  }

  .slider-value {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    font-variant-numeric: tabular-nums;
    min-width: 28px;
    text-align: right;
    flex-shrink: 0;
  }

  /* Text input */
  .text-input {
    width: 80px;
    flex-shrink: 0;
    text-align: center;
  }

  /* Toggle switch */
  .toggle {
    position: relative;
    width: 42px;
    height: 24px;
    border-radius: 12px;
    background: var(--border);
    transition: background 0.25s ease;
    flex-shrink: 0;
    cursor: pointer;
    border: none;
    padding: 0;
  }

  .toggle-on {
    background: var(--focus-green);
  }

  .toggle-thumb {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #ffffff;
    transition: transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.3);
    pointer-events: none;
  }

  .toggle-on .toggle-thumb {
    transform: translateX(18px);
  }

  .toggle:hover {
    opacity: 0.9;
  }

  .toggle:focus-visible {
    outline: 2px solid var(--focus-green);
    outline-offset: 2px;
  }

  /* Reset button */
  .reset-row {
    align-items: center;
  }

  .reset-btn {
    font-size: 12px;
    font-weight: 600;
    padding: 6px 14px;
    border-radius: var(--radius-xs);
    border: 1.5px solid rgba(255, 69, 58, 0.5);
    color: rgba(255, 69, 58, 0.85);
    background: transparent;
    transition: background 0.2s ease, border-color 0.2s ease, color 0.2s ease;
    flex-shrink: 0;
    cursor: pointer;
    font-family: inherit;
    white-space: nowrap;
  }

  .reset-btn:hover {
    background: rgba(255, 69, 58, 0.1);
    border-color: rgba(255, 69, 58, 0.75);
    color: rgb(255, 69, 58);
  }

  .reset-btn-confirm {
    background: rgba(255, 69, 58, 0.15);
    border-color: rgb(255, 69, 58);
    color: rgb(255, 69, 58);
    animation: pulse 0.8s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
  }
</style>
