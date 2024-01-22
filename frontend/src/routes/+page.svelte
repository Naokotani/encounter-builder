<script>
import { Radio, Select } from '$lib';
import { json } from '@sveltejs/kit';
	let formData = {
		level: null,
		party_size: null,
		difficulty: null,
		monster_types: null,
		bbeg_budget: null,
		bbeg_aquatic: null,
		bbeg_caster: null,
		bbeg_ranged: null,
		hench_budget: null,
		hench_aquatic: null,
		hench_caster: null,
		hench_ranged: null,
		lackey_budget: null,
		lackey_aquatic: null,
		lackey_caster: null,
		lackey_ranged: null,
	};

	let monster;
	async function handleSubmit() {
		console.log(formData);
		const params = new URLSearchParams({
			level: formData.level,
			party_size: formData.party_size,
			difficulty: formData.difficulty,
			monster_types: [formData.monster_types],
			bbeg_url: formData.bbeg_url,
			bbeg_budget: formData.bbeg_budget,
			bbeg_caster: formData.bbeg_caster.toLowerCase(),
			bbeg_ranged: formData.bbeg_ranged.toLowerCase(),
			hench_url: formData.bbeg_url,
			hench_budget: formData.hench_budget,
			hench_caster: formData.hench_caster.toLowerCase(),
			hench_ranged: formData.hench_ranged.toLowerCase(),
			lackey_url: formData.lackey_url,
			lackey_budget: formData.lackey_budget,
			lackey_caster: formData.lackey_caster.toLowerCase(),
			lackey_ranged: formData.lackey_ranged.toLowerCase(),
		});

		console.log(params.toString());
		const response = await fetch(`/encounter?${params.toString()}`);
		monster = await response.json(monster);
		console.log(monster);
	}
	let monsterBudget = [
		{value: 'all', label: 'All'},
		{value: 'more', label: 'Three Quarters'},
		{value: 'even', label: 'Half'},
		{value: 'less', label: 'One Quarter'},
		{value: 'none', label: 'None'},
	];

	let encounterDifficulty = [
		{value: 'trivial', label: 'Trivial'},
		{value: 'low', label: 'Low'},
		{value: 'moderate', label: 'Moderate'},
		{value: 'severe', label: 'Severe'},
		{value: 'extreme', label: 'Extreme'},
	]
	let eitherBools = [
		{value: 'either', label: 'Either'},
		{value: 'true', label: 'Yes'},
		{value: 'false', label: 'No'},
	];
	let monsterTypes = [
		{value: 'Undead', label: 'Undead'},
		{value: 'Humanoid', label: 'humanoid'},
		{value: 'Outsider', label: 'Outsider'},
		];
	let levels = []
	for (let i = 1; i < 21; i++) {
		levels.push({value: i, label: i.toString()})
	}
	let partySize = []
	for (let i = 1; i < 21; i++) {
		partySize.push({value: i, label: i.toString()})
	}
</script>

<h1>Pathfinder 2 Encounter Builder</h1>
<form on:submit|preventDefault={handleSubmit} style="width=50%; float: left;">
	<h2>Encounter</h2>
	<Select label="Party Level" options={levels} bind:value={formData.level}/>
	<Select
		label="Party Size"
		options={partySize}
		bind:value={formData.party_size}/>
	<Select
		label="Monster Types"
		options={monsterTypes}
		bind:value={formData.monster_types}/>
	<Select
		label="Encounter Difficulty"
		options={encounterDifficulty}
		bind:value={formData.difficulty}/>

	<h2>Big Bad Evil Guy</h2>
	<Select
		label="Solo Monster Budget"
		options={monsterBudget}
		bind:value={formData.bbeg_budget}/>
	<br/>
	<label>Ranged?</label>
	<Radio options={eitherBools} bind:userSelected={formData.bbeg_ranged}/>
	<label>Caster?</label>
	<Radio options={eitherBools} bind:userSelected={formData.bbeg_caster}/>

	<h2>Henchmen</h2>
	<Select
		label="Henchmen Budget"
		options={monsterBudget}
		bind:value={formData.hench_budget}/>
	<br/>
	<label>Ranged?</label>
	<Radio options={eitherBools} bind:userSelected={formData.hench_ranged}/>
	<label>Caster?</label>
	<Radio options={eitherBools} bind:userSelected={formData.hench_caster}/>

	<h2>Lackeys</h2>
	<Select
		label="Lackey Budget"
		options={monsterBudget}
		bind:value={formData.lackey_budget}/>
	<label>Ranged?</label>
	<Radio options={eitherBools} bind:userSelected={formData.lackey_ranged}/>
	<label>Caster?</label>
	<Radio options={eitherBools} bind:userSelected={formData.lackey_caster}/>

	<br/>
	<button type="submit">Roll the dice</button>
</form>

<div style="float:right">
{#if monster !== undefined}
	<a rel="external" href={monster.bbeg_url}>
		<h2>Big Bad Evil Guy</h2>
	</a>
	<h3>XP cost: {monster.bbeg_budget}</h3>
	<p>name: {monster.bbeg_name}</p>
	<p>number: {monster.bbeg_number}</p>
	<p>level: {monster.bbeg_level}</p>
	<p>alignment: {monster.bbeg_alignment}</p>
	<p>type: {monster.bbeg_monster_type}</p>
	<a rel="external" href={monster.hench_url}>
		<h2>Henchmen</h2>
	</a>
	<h3>XP cost: {monster.hench_budget}</h3>
	<p>name: {monster.hench_name}</p>
	<p>number: {monster.hench_number}</p>
	<p>level: {monster.hench_level}</p>
	<p>alignment: {monster.hench_alignment}</p>
	<p>type: {monster.hench_monster_type}</p>
	<a rel="external" href={monster.lackey_url}>
		<h2>Lackeys</h2>
	</a>
	<h3>XP cost: {monster.lackey_budget}</h3>
	<p>name: {monster.lackey_name}</p>
	<p>number: {monster.lackey_number}</p>
	<p>level: {monster.lackey_level}</p>
	<p>alignment: {monster.lackey_alignment}</p>
	<p>type: {monster.lackey_monster_type}</p>
{/if}
</div>
