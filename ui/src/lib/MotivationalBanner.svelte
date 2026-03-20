<script lang="ts">
  export let state: string;
  export let elapsed_secs: number;
  export let show: boolean;

  let prevMessage = '';
  let messageKey = 0;

  $: minutes = Math.floor(elapsed_secs / 60);

  $: message = (() => {
    if (state === 'focus') {
      if (minutes < 10) return 'Getting into the zone...';
      if (minutes < 30) return `Deep work in progress — ${minutes} minutes strong`;
      if (minutes < 50) return 'Incredible focus session!';
      return `Outstanding! Over ${minutes} minutes of deep work`;
    }
    if (state === 'idle') return 'Ready to dive back in?';
    if (state === 'ended') return 'Great session! Time to rest.';
    return '';
  })();

  $: {
    if (message !== prevMessage) {
      prevMessage = message;
      messageKey += 1;
    }
  }
</script>

{#if show && message}
  {#key messageKey}
    <div class="banner fade-in">
      <span class="banner-text">{message}</span>
    </div>
  {/key}
{/if}

<style>
  .banner {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px 10px;
    background: var(--card-bg);
    border-radius: var(--radius-xs);
    border: 1px solid var(--border);
    min-height: 32px;
  }

  .banner-text {
    font-size: 12px;
    color: var(--text-secondary);
    font-style: italic;
    text-align: center;
    line-height: 1.4;
  }
</style>
