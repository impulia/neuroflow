<script lang="ts">
  export let value: number = 0;
  export let size: number = 64;
  export let strokeWidth: number = 5;
  export let color: string = 'var(--focus-green)';

  $: radius = (size - strokeWidth) / 2;
  $: circumference = 2 * Math.PI * radius;
  $: clampedValue = Math.min(1, Math.max(0, value));
  $: dashoffset = circumference * (1 - clampedValue);
  $: center = size / 2;
</script>

<div class="ring-wrapper" style="width: {size}px; height: {size}px;">
  <svg
    width={size}
    height={size}
    viewBox="0 0 {size} {size}"
    style="transform: rotate(-90deg);"
  >
    <circle
      cx={center}
      cy={center}
      r={radius}
      fill="none"
      stroke="rgba(255,255,255,0.1)"
      stroke-width={strokeWidth}
    />
    <circle
      cx={center}
      cy={center}
      r={radius}
      fill="none"
      stroke={color}
      stroke-width={strokeWidth}
      stroke-linecap="round"
      stroke-dasharray={circumference}
      stroke-dashoffset={dashoffset}
      style="transition: stroke-dashoffset 0.6s ease;"
    />
  </svg>
  <div class="ring-content">
    <slot />
  </div>
</div>

<style>
  .ring-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  svg {
    position: absolute;
    top: 0;
    left: 0;
  }

  .ring-content {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 1;
  }
</style>
