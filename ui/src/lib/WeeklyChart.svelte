<script lang="ts">
  import type { DayChartData } from '../stores/tracker';

  export let data: DayChartData[];

  const CHART_HEIGHT = 100;
  const BAR_WIDTH = 28;
  const BAR_RADIUS = 4;

  interface TooltipState {
    visible: boolean;
    x: number;
    focusLabel: string;
    idleLabel: string;
    dayLabel: string;
  }

  let tooltip: TooltipState = {
    visible: false,
    x: 0,
    focusLabel: '',
    idleLabel: '',
    dayLabel: '',
  };

  function formatHM(secs: number): string {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    if (h > 0 && m > 0) return `${h}h ${m}m`;
    if (h > 0) return `${h}h`;
    if (m > 0) return `${m}m`;
    return '0m';
  }

  $: maxTotal = Math.max(
    1,
    ...data.map((d) => d.focus_secs + d.idle_secs)
  );

  function focusHeight(d: DayChartData): number {
    return (d.focus_secs / maxTotal) * CHART_HEIGHT;
  }

  function idleHeight(d: DayChartData): number {
    return (d.idle_secs / maxTotal) * CHART_HEIGHT;
  }

  function totalHeight(d: DayChartData): number {
    return focusHeight(d) + idleHeight(d);
  }

  function handleMouseEnter(event: MouseEvent, d: DayChartData, index: number) {
    const target = event.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    tooltip = {
      visible: true,
      x: index,
      focusLabel: formatHM(d.focus_secs),
      idleLabel: formatHM(d.idle_secs),
      dayLabel: d.label,
    };
  }

  function handleMouseLeave() {
    tooltip = { ...tooltip, visible: false };
  }

  $: filledData = data.length > 0 ? data : Array.from({ length: 7 }, (_, i) => ({
    label: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'][i],
    focus_secs: 0,
    idle_secs: 0,
    is_today: false,
  }));
</script>

<div class="chart-wrapper">
  <div class="bars-container">
    {#each filledData as day, i}
      <div
        class="bar-col"
        class:is-today={day.is_today}
        role="img"
        aria-label="{day.label}: {formatHM(day.focus_secs)} focus, {formatHM(day.idle_secs)} idle"
        on:mouseenter={(e) => handleMouseEnter(e, day, i)}
        on:mouseleave={handleMouseLeave}
      >
        <div class="bar-track">
          <div class="bar-stack" style="height: {CHART_HEIGHT}px;">
            {#if totalHeight(day) > 0}
              <!-- Idle segment on top -->
              {#if idleHeight(day) > 0}
                <div
                  class="bar-segment bar-idle"
                  style="height: {idleHeight(day)}px;"
                />
              {/if}
              <!-- Focus segment on bottom -->
              {#if focusHeight(day) > 0}
                <div
                  class="bar-segment bar-focus"
                  style="height: {focusHeight(day)}px;"
                />
              {/if}
            {:else}
              <div class="bar-empty" />
            {/if}
          </div>
        </div>
        <span class="day-label" class:today-label={day.is_today}>{day.label}</span>
      </div>
    {/each}
  </div>

  {#if tooltip.visible}
    <div
      class="tooltip"
      style="left: calc({tooltip.x} * (100% / {filledData.length}) + {BAR_WIDTH / 2}px)"
    >
      <span class="tooltip-day">{tooltip.dayLabel}</span>
      <span class="tooltip-line focus-line">
        <span class="tooltip-dot dot-focus" />
        {tooltip.focusLabel}
      </span>
      <span class="tooltip-line idle-line">
        <span class="tooltip-dot dot-idle" />
        {tooltip.idleLabel}
      </span>
    </div>
  {/if}
</div>

<style>
  .chart-wrapper {
    position: relative;
    width: 100%;
  }

  .bars-container {
    display: flex;
    flex-direction: row;
    align-items: flex-end;
    justify-content: space-between;
    width: 100%;
    gap: 4px;
  }

  .bar-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    cursor: default;
    position: relative;
  }

  .bar-track {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .bar-stack {
    width: 100%;
    max-width: 28px;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    border-radius: 4px 4px 3px 3px;
    overflow: hidden;
  }

  .bar-segment {
    width: 100%;
    transition: height 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
    flex-shrink: 0;
  }

  .bar-focus {
    background: var(--focus-green);
    opacity: 0.85;
  }

  .bar-idle {
    background: var(--idle-amber);
    opacity: 0.75;
  }

  .bar-empty {
    width: 100%;
    height: 3px;
    background: var(--border);
    border-radius: 2px;
    align-self: flex-end;
  }

  .is-today .bar-stack {
    filter: drop-shadow(0 0 4px rgba(52, 199, 89, 0.35));
  }

  .is-today .bar-focus {
    opacity: 1;
  }

  .day-label {
    font-size: 10px;
    font-weight: 500;
    color: var(--text-tertiary);
    text-align: center;
    line-height: 1;
    white-space: nowrap;
  }

  .today-label {
    color: var(--focus-green);
    font-weight: 700;
  }

  .tooltip {
    position: absolute;
    bottom: calc(100% + 4px);
    transform: translateX(-50%);
    background: rgba(40, 40, 45, 0.95);
    border: 1px solid var(--border);
    border-radius: var(--radius-xs);
    padding: 6px 8px;
    display: flex;
    flex-direction: column;
    gap: 3px;
    pointer-events: none;
    z-index: 10;
    backdrop-filter: blur(8px);
    min-width: 90px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
    animation: fadeIn 0.15s ease-out;
  }

  .tooltip-day {
    font-size: 11px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 1px;
  }

  .tooltip-line {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .tooltip-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .dot-focus {
    background: var(--focus-green);
  }

  .dot-idle {
    background: var(--idle-amber);
  }
</style>
