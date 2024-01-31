<script>
  export let percentages = [100, 0, 0, 0];
</script>

<style>

	@media (min-width: 1025px) {
		.budget-head {
			display: inline;
		}

		.budget-bar {
			flex-direction: row;
			width: 30rem; /* width of the bar */
			height: 20px; /* height of the bar */
			background-color: #8E9981; /* default background color */
			border-radius: 10px;
			border: solid 5px var(--lineColor);
			display: flex;
		}

		.budget-head {
			display: block;
			margin-top: 2.2rem;
		}

	}

	@media only screen and (min-width: 768px) and (max-width: 1024px) {
		.budget-bar {
			display: none;
		}

		.budget-head {
			display: none;
		}
	}

  .group1 {
    background-color: var(--bbeg-color); /* color for group 1 */
		transition: all .25s linear;
  }

  .group2 {
    background-color: var(--hench-color); /* color for group 2 */
		transition: all .25s linear;
  }

  .group3 {
    background-color: var(--lackey-color); /* color for group 3 */
		transition: all .25s linear;
  }
  .group4 {
    background-color: #8E9981; /* color for group 1 */
		transition: all .25s linear;
  }

</style>

<h5 class="budget-head">Xp budget</h5>
<div class="budget-bar">
  {#each percentages as percentage, i}
    <div class="group{i+1} group" style="width: {percentage}%;"></div>
  {/each}
</div>
