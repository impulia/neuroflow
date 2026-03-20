<script lang="ts">
  export let label: string;
  export let value: string;
  export let sublabel: string = '';
  export let color: string = 'var(--text)';
</script>

<div class="stat-card">
  <span class="card-label">{label}</span>
  <span class="card-value" style="color: {color}">{value}</span>
  {#if sublabel}
    <span class="card-sublabel">{sublabel}</span>
  {/if}
</div>

<style>
  .stat-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 3px;
    background: var(--card-bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 10px 8px;
    transition: background 0.2s ease, border-color 0.2s ease;
  }

  .stat-card:hover {
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
    line-height: 1.3;
  }

  .card-value {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.4px;
    line-height: 1.1;
    font-variant-numeric: tabular-nums;
    text-align: center;
    transition: color 0.4s ease;
  }

  .card-sublabel {
    font-size: 10px;
    color: var(--text-tertiary);
    text-align: center;
    line-height: 1.3;
  }
</style>
