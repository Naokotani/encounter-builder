<script>
	import { onMount } from 'svelte';
	export let options;
  export let userSelected = 'either';
	export let type;
	export let disabled = false;
onMount(() => {
  const inputs = document.querySelectorAll("input");
  const hasChecked = Array.from(inputs).some((input) => input.checked);
  if (!hasChecked) {
    inputs[0].checked = true;
		for (let i in inputs) {
			if (i % 3 === 0) {
				inputs[i].checked = true;
			}
		}
  }
})
</script>

<div role="radiogroup" >
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
