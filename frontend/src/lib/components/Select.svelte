<script>
	import { onMount } from 'svelte';

  export let options;
  export let value;
  export let label;
  export let selected = 0;
	export let disabled = false;

	onMount(() => {
		if (options.length > 0) {
			value = options[selected].value;
		}
	});

  function handleSelectChange(event) {
    value = event.target.value;
  }
</script>


<label for={label}>{label}</label>
<select disabled={disabled} bind:value={value} on:change={handleSelectChange}>
  {#if options.length > 0}
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  {:else}
    <option value="">No options available</option>
  {/if}
</select>
