<script>
	import { onMount } from 'svelte';
	export let options;
  export let userSelected = 'either';
	export let type;
	export let disabled = false;

onMount(() => {
  const inputs = document.querySelectorAll("input");
	const ins = (Array.from(inputs));
  inputs[0].checked = true;
	for (let i in inputs) {
		if (i % 3 === 0) {
			inputs[i].checked = true;
		}
	}
	userSelected = 'either'
})
</script>

<div role="radiogroup" >
	<label for={type + options[0].value}>{type}</label>
  {#each options as {value, label}}
    <input
      class="sr-only"
			disabled={disabled}
      type="radio"
      id={label + type}
      bind:group={userSelected}
      value={value} />
    <label for={label}>{label}</label>
  {/each}
</div>
