<script>
	export let title;
	export let url;
	export let budget;
	export let name;
	export let number;
	export let alignment;
	export let type;
	export let level;
</script>

<a rel="external noopener noreferrer"
	 href={url} target="_blank">
	<h2>{name}</h2>
</a>
<p class="card-subheader">
	XP: {budget}</p>
<ul>
	<li>number: {number}</li>
	<li>level: {level}</li>
	<li>alignment: {alignment}</li>
	<li>type: {type}</li>
</ul>

<style>
	a {
		max-width: 100%;
	}

	h2 {
		font-size: var(--h3);
		margin: 0;
		text-overflow: ellipsis;
		overflow: hidden;
	}

	ul {
		padding: 0;
	}

	li {
		list-style: none;
	}

	.card-subheader {
		font-size: var(--h5);
		margin-top: 0.25rem;
		margin-bottom: 0;
		line-height: var(--h5);

	}

	h3 {
		margin: 0;
		font-size: var(--h4);
	}
</style>
