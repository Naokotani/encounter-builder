<script>
	import { onMount } from 'svelte';
	export let options;
  export let userSelected = 'either';
	export let type;
	export let disabled = false;

onMount(() => {
  const inputs = document.querySelectorAll("input");
	const ins = (Array.from(inputs));
	userSelected = 'either'
})
</script>

<div role="radiogroup" class="flex radio-group" >
	<label for={type + options[0].value}>{type}</label>
  {#each options as {value, label}}
		<div class="radio">
			<label for={label}>
				<input
					class="sr-only"
					disabled={disabled}
					type="radio"
					id={label + type}
					bind:group={userSelected}
					value={value} />
				{label}</label>
		</div>
  {/each}
</div>

<style>
	.radio-group>label {
		padding-left: 2rem;
	}
	.radio {
		padding-right: 40%;
	}
	label {
		text-align: left;
	}
</style>
