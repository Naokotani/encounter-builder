<script>
  export let options;
  export let value;
  export let label;
</script>

<label>{label}</label>
<select bind:value={value}>
  {#each options as option}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>
