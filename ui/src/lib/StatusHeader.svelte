<script lang="ts">
  export let state: string;
  export let elapsed_secs: number;
  export let paused: boolean;

  function formatElapsed(secs: number): string {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    const s = secs % 60;
    if (h > 0) {
      return `${h}h ${m}m ${s}s`;
    }
    if (m > 0) {
      return `${m}m ${s}s`;
    }
    return `${s}s`;
  }

  $: effectiveState = paused ? 'paused' : state;

  $: dotClass = (() => {
    if (effectiveState === 'focus') return 'dot dot-focus breathing';
    if (effectiveState === 'idle') return 'dot dot-idle breathing-amber';
    if (effectiveState === 'paused') return 'dot dot-paused';
    if (effectiveState === 'ended') return 'dot dot-ended';
    return 'dot dot-waiting';
  })();

  $: stateLabel = (() => {
    if (effectiveState === 'focus') return 'In Flow';
    if (effectiveState === 'idle') return 'Idle';
    if (effectiveState === 'paused') return 'Paused';
    if (effectiveState === 'ended') return 'Session Ended';
    return 'Waiting';
  })();

  $: stateColor = (() => {
    if (effectiveState === 'focus') return 'var(--focus-green)';
    if (effectiveState === 'idle') return 'var(--idle-amber)';
    if (effectiveState === 'paused') return 'rgba(255,255,255,0.5)';
    if (effectiveState === 'ended') return 'rgba(255,255,255,0.4)';
    return 'rgba(255,255,255,0.35)';
  })();
</script>

<div class="status-header">
  <div class="dot-row">
    <div class={dotClass} />
    <div class="state-info">
      <span class="state-label" style="color: {stateColor}">{stateLabel}</span>
      <span class="elapsed">{formatElapsed(elapsed_secs)}</span>
    </div>
  </div>
</div>

<style>
  .status-header {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .dot-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    flex-shrink: 0;
    transition: background-color 0.4s ease;
  }

  .dot-focus {
    background-color: var(--focus-green);
  }

  .dot-idle {
    background-color: var(--idle-amber);
  }

  .dot-paused {
    background-color: rgba(255, 255, 255, 0.4);
  }

  .dot-ended {
    background-color: rgba(255, 255, 255, 0.25);
  }

  .dot-waiting {
    background-color: rgba(255, 255, 255, 0.2);
  }

  .state-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .state-label {
    font-size: 18px;
    font-weight: 700;
    letter-spacing: -0.3px;
    line-height: 1.2;
    transition: color 0.4s ease;
  }

  .elapsed {
    font-size: 12px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.02em;
  }
</style>
