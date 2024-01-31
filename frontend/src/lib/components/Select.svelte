<script>
	import { onMount } from 'svelte';

  export let options;
  export let value;
  export let label;
  export let selected = 0;
	export let disabled = false;
	export let handleBudgetChange;

	onMount(() => {
		if (options.length > 0) {
			value = options[selected].value;
		}
	});

  function handleSelectChange(event) {
    value = event.target.value;
  }
</script>

<div class="grid aside-left">
<label for={label}>{label}</label>
<select disabled={disabled} bind:value={value} on:change={handleBudgetChange}>
  {#if options.length > 0}
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  {:else}
    <option value="">No options available</option>
  {/if}
</select>
</div>

<style>
	.grid {
		max-width: var(--formWidth);
	}
	select {
		width: 30%;
		max-height: 2rem;
		margin-bottom: 5px;
	}

@media (min-width: 767px) {
	select {
		width: 60%;
	}
}
</style>
