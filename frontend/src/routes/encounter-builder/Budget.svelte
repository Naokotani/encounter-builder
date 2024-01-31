<script>
  export let percentages = [100, 0, 0, 0];
</script>

<style>
  .bar {
		margin-top: 2rem;
    display: flex;
    flex-direction: row;
    width: 500px; /* width of the bar */
    height: 20px; /* height of the bar */
    background-color: #8E9981; /* default background color */
		border-radius: 10px;
		border: solid 5px var(--lineColor);
  }

  .group1 {
    background-color: #993114; /* color for group 1 */
  }

  .group2 {
    background-color: #143199; /* color for group 2 */
  }

  .group3 {
    background-color: #5B9914; /* color for group 3 */
  }
  .group4 {
    background-color: #8E9981; /* color for group 1 */
  }
</style>

<div class="bar">
  {#each percentages as percentage, i}
    <div class="group{i+1} group" style="width: {percentage}%;"></div>
  {/each}
</div>
