{#await import('./app.svelte')}
  <p>Loading...</p>
{:then App}
  <svelte:component this={App.default} />
{/await}
