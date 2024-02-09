<script>
  import '../styles.css';
  import '../normalize.css';
</script>

<div class="layout">
	<nav>
		<h1>Pathfinder 2 Encounter Builder</h1>
	</nav>
	<slot />
</div>

<style>

	.layout {
		padding: 1rem var(--containerPadding);
		min-height: calc(100vh - var(--footerHeight));
		margin-top: var(--headerHeight);
	}


	nav h1 {
		color: var(--headerText);
		font-size: var(--h3);
		margin: 0;
	}

	nav {
		background: var(--navColor);
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		width: 100%;
		height: var(--headerHeight);
		display: flex;
		justify-content: space-between;
		align-items: center;
		z-index: 10;
		-webkit-transition: all 500ms ease;
		-moz-transition: all 500ms ease;
		-o-transition: all 500ms ease;
		transition: all 500ms ease;
		box-shadow: var(--level-3);
		padding: 0 var(--containerPadding) 0 var(--containerPadding); 
		box-sizing: border-box;
	}
</style>
