<script>
	import { onMount } from 'svelte';

  export let options;
  export let value;
  export let label;
  export let selected = 0;
	export let disabled = false;
	export let handleBudgetChange = null;

	onMount(() => {
		if (options.length > 0) {
			value = options[selected].value;
		}
	});

  function handleSelectChange(event) {
    value = event.target.value;
  }

</script>

<div >
<label class="select-grid" for={label}>{label}
	<select disabled={disabled} bind:value={value} on:change={handleBudgetChange}>
		{#if options.length > 0}
			{#each options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		{:else}
			<option value="">No options available</option>
		{/if}
	</select>
</label>
</div>

<style>
	.select-grid {
		max-width: var(--formWidth);
    --gridCols: 2;
    display: grid;
    grid-template-columns: repeat(var(--gridCols), 1fr);
    gap: 5px;
	}

	.select-grid>select {
		min-width: 6rem;
	}
	select {
		width: 30%;
		max-height: 2rem;
		margin-bottom: 5px;
		display: inline;

	}

	label {
		display: flex;
	}

@media (min-width: 767px) {
	select {
		width: 60%;
	}
}
</style>
