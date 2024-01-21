<script>
	export let options;
  export let userSelected = options[0].value;
  const slugify = (str = "") =>
    str.toLowerCase().replace(/ /g, "-").replace(/\./g, "");
</script>

<div role="radiogroup" >
  {#each options as {value, label}}
    <input
      class="sr-only"
      type="radio"
      id={slugify(label)}
      bind:group={userSelected}
      value={value} />
    <label for={slugify(label)}> {label} </label>
  {/each}
</div>
