<script>
	import { onMount } from 'svelte';
	export let options;
  export let userSelected = 'either';
	export let type = "";
	export let disabled = false;

onMount(() => {
  const inputs = document.querySelectorAll("input");
	const ins = (Array.from(inputs));
	userSelected = 'either'
})

	const slugify = label =>
				label
				.toLowerCase()
				.trim()
				.replace(/[^\w\s-]/g, '')
				.replace(/[\s_-]+/g, '-')
				.replace(/^-+|-+$/g, '');
	const typeSlug = slugify(type);
</script>
<div role="radiogroup" >
	<label for={type + options[0].value}>{type}</label>
  {#each options as {value, label}}
		<div>
    <label for={`${label}+${typeSlug}`}>{label}
    <input
      class="sr-only"
			disabled={disabled}
      type="radio"
			name={`${label}+${typeSlug}`}
      id={`${label}+${typeSlug}`}
      bind:group={userSelected}
      value={value} />
		</label>
		</div>
  {/each}
</div>
