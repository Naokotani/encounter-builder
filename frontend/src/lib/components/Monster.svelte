<script>
	export let url;
	export let budget;
	export let name;
	export let number;
	export let alignment;
	export let type;
	export let level;
</script>

<a rel="external" href={url}>
	<h2>{name}</h2>
</a>
<h3>XP cost: {budget}</h3>
<ul>
	<li>number: {number}</li>
	<li>level: {level}</li>
	<li>alignment: {alignment}</li>
	<li>type: {type}</li>
</ul>

<style>
	h2 {
		font-size: var(--h3);
		margin: 0;
		border-bottom: var(--line);
	}

	h3 {
		margin: 0;
		font-size: var(--h4);
	}

	a {
		text-decoration: none;
		color: var(--anchorColor);
	}
</style>
