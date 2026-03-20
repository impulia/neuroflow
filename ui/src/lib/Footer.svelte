<script lang="ts">
  export let week_focus_secs: number;
  export let onSettingsClick: () => void;

  function formatWeekFocus(secs: number): string {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    if (h > 0 && m > 0) return `${h}h ${m}m`;
    if (h > 0) return `${h}h`;
    if (m > 0) return `${m}m`;
    return '0m';
  }

  $: weekLabel = formatWeekFocus(week_focus_secs);
</script>

<div class="footer">
  <span class="week-label">This week: <strong>{weekLabel}</strong> focused</span>
  <button
    class="settings-btn"
    on:click={onSettingsClick}
    aria-label="Open settings"
    title="Settings"
  >
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5ZM4.5 8a3.5 3.5 0 1 1 7 0 3.5 3.5 0 0 1-7 0Z"
        fill="currentColor"
      />
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M6.794.793A1 1 0 0 1 7.75 0h.5a1 1 0 0 1 .956.793l.229 1.03a5.52 5.52 0 0 1 1.09.633l1.008-.34a1 1 0 0 1 1.157.434l.25.433a1 1 0 0 1-.16 1.21l-.753.7a5.56 5.56 0 0 1 0 1.114l.753.7a1 1 0 0 1 .16 1.21l-.25.433a1 1 0 0 1-1.157.434l-1.009-.34a5.52 5.52 0 0 1-1.089.633l-.23 1.03A1 1 0 0 1 8.25 11h-.5a1 1 0 0 1-.956-.793l-.23-1.03a5.52 5.52 0 0 1-1.089-.633l-1.009.34a1 1 0 0 1-1.157-.434l-.25-.433a1 1 0 0 1 .16-1.21l.753-.7a5.56 5.56 0 0 1 0-1.114l-.753-.7a1 1 0 0 1-.16-1.21l.25-.433A1 1 0 0 1 4.466 2.5l1.008.34a5.52 5.52 0 0 1 1.09-.633l.23-1.03V.793ZM8.25 1h-.5l-.275 1.232a.5.5 0 0 1-.358.373 4.52 4.52 0 0 0-1.364.793.5.5 0 0 1-.508.076L4.216 3.13l-.25.433.899.836a.5.5 0 0 1 .143.468 4.563 4.563 0 0 0 0 1.266.5.5 0 0 1-.143.468l-.899.836.25.433 1.03-.348a.5.5 0 0 1 .507.077 4.52 4.52 0 0 0 1.364.793.5.5 0 0 1 .358.372L7.75 10h.5l.275-1.232a.5.5 0 0 1 .358-.372 4.52 4.52 0 0 0 1.364-.793.5.5 0 0 1 .507-.077l1.03.348.25-.433-.899-.836a.5.5 0 0 1-.143-.468 4.563 4.563 0 0 0 0-1.266.5.5 0 0 1 .143-.468l.899-.836-.25-.433-1.03.348a.5.5 0 0 1-.507-.076 4.52 4.52 0 0 0-1.364-.793.5.5 0 0 1-.358-.373L8.25 1Z"
        fill="currentColor"
      />
    </svg>
  </button>
</div>

<style>
  .footer {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    margin-top: auto;
    padding-top: 4px;
  }

  .week-label {
    font-size: 11px;
    color: var(--text-secondary);
  }

  .week-label strong {
    color: var(--text);
    font-weight: 600;
  }

  .settings-btn {
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

  .settings-btn:hover {
    color: var(--text);
    background: var(--card-bg);
  }

  .settings-btn:active {
    background: var(--card-bg-hover);
  }
</style>
