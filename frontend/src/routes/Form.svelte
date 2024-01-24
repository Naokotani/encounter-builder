<script>
	import { Radio, Select } from '$lib';

	export let formData; 
	export let submit;

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
	];
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
	$: disabled = formData.bbeg.budget === 'all';
	$: disabledLackey = formData.hench.budget === 'all' || formData.bbeg.budget === 'all';
	$: formData.hench.budget = formData.bbeg.budget === 'all'?'none':formData.hench.budget;
	$: {
		formData.lackey.budget = formData.bbeg.budget === 'all' || formData.hench.budget === 'all'?
			'none':formData.lackey.budget;
	}
</script>

<form on:submit|preventDefault={submit} style="width=50%; float: left;">
	<h2>Encounter</h2>
	<Select label="Party Level" options={levels} bind:value={formData.level}/>
	<Select
		label="Party Size"
		options={partySize}
		selected={3}
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
		bind:value={formData.bbeg.budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="lackey"
		bind:userSelected={formData.bbeg.ranged}/>
	<Radio
		options={eitherBools}
		type="lackey"
		bind:userSelected={formData.bbeg.caster}/>

	<h2>Henchmen</h2>
	<Select
		label="Henchmen Budget"
		bind:disabled={disabled}
		options={monsterBudget}
		bind:value={formData.hench.budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="lackey"
		bind:disabled={disabled}
		bind:userSelected={formData.hench.ranged}/>

	<Radio
		options={eitherBools}
		type="lackey"
		bind:disabled={disabled}
		bind:userSelected={formData.hench.caster}/>

	<h2>Lackeys</h2>
	<Select
		label="Lackey Budget"
		bind:disabled={disabledLackey}
		options={monsterBudget}
		bind:value={formData.lackey.budget}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="lackey"
		bind:userSelected={formData.lackey.ranged}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="lackey"
		bind:userSelected={formData.lackey.caster}/>

	<br/>
	<button type="submit">Roll the dice</button>
</form>
