<script>
	import { onMount } from 'svelte';
	export let options;
  export let userSelected = 'either';
	export let type;
	export let disabled = false;
	export let sectionId = "";

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
<div role="radiogroup" class="flex radio-group" >
	<span>{type}</span>
  {#each options as {value, label}}
		<div class="radio">
			<label for={`${sectionId}-${typeSlug}-${value}`}>
				<input
					class="sr-only"
					disabled={disabled}
					name={`${sectionId}-${typeSlug}-${value}`}
					type="radio"
					id={`${sectionId}-${typeSlug}-${value}`}
					bind:group={userSelected}
					value={value} />
				{label}</label>
		</div>
  {/each}
</div>
<style>
	.radio {
		padding-right: 40%;
	}
	label {
		text-align: left;
	}
</style>
