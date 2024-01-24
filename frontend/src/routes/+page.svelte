<script>
	import { Radio, Select, Monster } from '$lib';
	import { json } from '@sveltejs/kit';
	let formData = {
		level: null,
		party_size: null,
		difficulty: 'moderate',
		monster_types: null,
		bbeg_budget: 'all',
		bbeg_aquatic: null,
		bbeg_caster: 'either',
		bbeg_ranged: 'either',
		hench_budget: 'all',
		hench_aquatic: null,
		hench_caster: 'either',
		hench_ranged: 'either',
		lackey_budget: 'all',
		lackey_aquatic: null,
		lackey_caster: 'either',
		lackey_ranged: 'either',
	};

	let monster;
	let bbeg_number = 0;
	let hench_number = 0;
	let lackey_number = 0;
	
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
		bbeg_number = monster.bbeg_number;
		hench_number = monster.hench_number;
		lackey_number = monster.lackey_number;

		bbeg.url = monster.bbeg_url;
		bbeg.level = monster.bbeg_level;
		bbeg.name = monster.bbeg_name;
		bbeg.budget = monster.bbeg_budget;
		bbeg.number = monster.bbeg_number;
		bbeg.alignment = monster.bbeg_alignment;
		bbeg.type = monster.bbeg_type;
		
		hench.url = monster.hench_url;
		hench.level = monster.hench_level;
		hench.name = monster.hench_name;
		hench.budget = monster.hench_budget;
		hench.number = monster.hench_number;
		hench.alignment = monster.hench_alignment;
		hench.type = monster.hench_type;

		lackey.url = monster.lackey_url;
		lackey.level = monster.lackey_level;
		lackey.name = monster.lackey_name;
		lackey.budget = monster.lackey_budget;
		lackey.number = monster.lackey_number;
		lackey.alignment = monster.lackey_alignment;
		lackey.type = monster.lackey_type;
	}


		let bbeg = {
			url: "",
			level: 0,
			name: "",
			budget: 0.0,
			number: 0,
			alignment: "",
			type: "",
	}

		let hench = {
			url: "",
			level: 0,
			name: "",
			budget: 0.0,
			number: 0,
			alignment: "",
			type: "",
		}
	
		let lackey = {
			url: "",
			level: 0,
			name: "",
			budget: 0.0,
			number: 0,
			alignment: "",
			type: "",
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
		levels.push({value: i.toString(), label: i.toString()});
	}
	let partySize = []
	for (let i = 1; i < 10; i++) {
		partySize.push({value: i.toString(), label: i.toString()});
	} 

	let disabled = true;
	let disabledLackey = true;
	$: disabled = formData.bbeg_budget === 'all';
	$: disabledLackey = formData.hench_budget === 'all' || formData.bbeg_budget === 'all';
	$: formData.hench_budget = formData.bbeg_budget === 'all'?'none':formData.hench_budget;
	$: {
		formData.lackey_budget = formData.bbeg_budget === 'all' || formData.hench_budget === 'all'?
			'none':formData.lackey_budget;
	}

	async function newBbeg() {
		const params = new URLSearchParams({
			level: bbeg.level,
			party_size: formData.party_size,
			monster_types: formData.monster_types,
			budget: bbeg.budget,
			is_caster: formData.bbeg_caster.toLowerCase(),
			is_ranged: formData.bbeg_ranged.toLowerCase(),
		});

		const response = await fetch(`/monster?${params.toString()}`);
		bbeg = await response.json(monster);
	}

	async function newHench() {
		const params = new URLSearchParams({
			level: hench.level,
			party_size: formData.party_size,
			monster_types: formData.monster_types,
			budget: hench.budget,
			is_caster: formData.hench_caster.toLowerCase(),
			is_ranged: formData.hench_ranged.toLowerCase(),
		});

		const response = await fetch(`/monster?${params.toString()}`);
		hench = await response.json(monster);
	}

	async function newLackey() {
		const params = new URLSearchParams({
			level: lackey.level,
			party_size: formData.party_size,
			monster_types: formData.monster_types,
			budget: lackey.budget,
			is_caster: formData.lackey_caster.toLowerCase(),
			is_ranged: formData.lackey_ranged.toLowerCase(),
		});

		const response = await fetch(`/monster?${params.toString()}`);
		lackey = await response.json(monster);
	}
</script>

<h1>Pathfinder 2 Encounter Builder</h1>
<form on:submit|preventDefault={handleSubmit} style="width=50%; float: left;">
	<h2>Encounter</h2>
	<Select label="Party Level" options={levels} bind:value={formData.level}/>
	<Select
		label="Party Size"
		options={partySize}
		selected={0}
		bind:value={formData.party_size}/>
	
	<Select
		label="Monster Types"
		options={monsterTypes}
		selected={0}
		bind:value={formData.monster_types}/>
	<Select
		label="Encounter Difficulty"
		options={encounterDifficulty}
		selected={2}
		bind:value={formData.difficulty}/>

	<h2>Big Bad Evil Guy</h2>
	<Select
		label="Solo Monster Budget"
		options={monsterBudget}
		bind:value={formData.bbeg_budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="lackey"
		bind:userSelected={formData.bbeg_ranged}/>
	<Radio options={eitherBools} type="lackey" bind:userSelected={formData.bbeg_caster}/>

	<h2>Henchmen</h2>
	<Select
		label="Henchmen Budget"
		bind:disabled={disabled}
		options={monsterBudget}
		bind:value={formData.hench_budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="lackey"
		bind:disabled={disabled}
		bind:userSelected={formData.hench_ranged}/>

	<Radio
		options={eitherBools}
		type="lackey"
		bind:disabled={disabled}
		bind:userSelected={formData.hench_caster}/>

	<h2>Lackeys</h2>
	<Select
		label="Lackey Budget"
		bind:disabled={disabledLackey}
		options={monsterBudget}
		bind:value={formData.lackey_budget}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="lackey"
		bind:userSelected={formData.lackey_ranged}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="lackey"
		bind:userSelected={formData.lackey_caster}/>

	<br/>
	<button type="submit">Roll the dice</button>
</form>

<div style="float:right">
	{#if bbeg_number !== 0}
	<Monster
		url={bbeg.url}
		level={bbeg.level}
		name={bbeg.name}
		budget={bbeg.budget}
		number={bbeg.number}
		alignment={bbeg.alignment}
		type={bbeg.monster_type}
		/>
	<button on:click={newBbeg}>New Monster</button>
	{/if}

	{#if hench_number !== 0}
	<Monster
		url={hench.url}
		name={hench.name}
		level={hench.level}
		budget={hench.budget}
		number={hench.number}
		alignment={hench.alignment}
		type={hench.monster_type}
		 />
	<button on:click={newHench}>New Monster</button>
	{/if}

	{#if lackey_number !== 0}
	<Monster
		url={lackey.url}
		name={lackey.name}
		level={lackey.level}
		budget={lackey.budget}
		number={lackey.number}
		alignment={lackey.alignment}
		type={lackey.monster_type}
		 />
	<button on:click={newLackey}>New Monster</button>
	{/if}
</div>
