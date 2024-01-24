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
		{value: 'Animal', label: 'Animal'},
		{value: 'Undead', label: 'Undead'},
		{value: 'Humanoid', label: 'Humanoid'},
		{value: 'Aberration', label: 'Aberration'},
		{value: 'Outsider', label: 'Outsider'},
		{value: 'Elemental', label: 'Elemental'},
		{value: 'Beast', label: 'Beast'},
		{value: 'Construct', label: 'Construct'},
		{value: 'Dragon', label: 'Dragon'},
		{value: 'Fiend', label: 'Fiend'},
		{value: 'Plant', label: 'Plant'},
		{value: 'Monitor', label: 'Monitor'},
		{value: 'Ooze', label: 'Ooze'},
		{value: 'Celestial', label: 'Celestial'},
		{value: 'Giant', label: 'Giant'},
		{value: 'Human', label: 'Human'},
		{value: 'Demon', label: 'Demon'},
		{value: 'Div', label: 'Div'},
		{value: 'Daemon', label: 'Daemon'},
		{value: 'Dream', label: 'Dream'},
		{value: 'Protean', label: 'Protean'},
		{value: 'Devil', label: 'Devil'},
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

<form on:submit|preventDefault={submit} style="width=30%;">
	<h2>Encounter Options</h2>
	<div>
	<Select label="Party Level" options={levels} bind:value={formData.level}/>
	<Select
		label="Monster Types"
		options={monsterTypes}
		selected={0}
		bind:value={formData.monster_types}/>
	</div>
	<div>
	<Select
		label="Party Size"
		options={partySize}
		selected={3}
		bind:value={formData.party_size}/>
	<Select
		label="Encounter Difficulty"
		options={encounterDifficulty}
		selected={2}
		bind:value={formData.difficulty}/>
	<br/>
		
	</div>
	<h2>Monsters Options</h2>
	<p class="instructions">
		Pick the budget you want to assign to each monster type.
		Setting a monster budget to 'all' will disable the monsters
		below it. 'None' will remove that monster group from the
		results. 'More', 'Half' and 'less' represents the amount of
		remaining budget that will be used on that monster group.
	</p>
	<button type="submit">Roll the dice</button>
	<h3>Big Bad Evil Guy</h3>
	<p class="instructions">Solo monster up to 4 levels about the party depending on budget.</p>
	<Select
		label="Solo Monster Budget"
		options={monsterBudget}
		bind:value={formData.bbeg.budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="Ranged?"
		bind:userSelected={formData.bbeg.ranged}/>
	<Radio
		options={eitherBools}
		type="Caster?"
		bind:userSelected={formData.bbeg.caster}/>
<label>
	Aquatic?
	<input type="checkbox" bind:checked={formData.bbeg.aquatic} />
</label>

	<h3>Henchmen</h3>
	<p class="instructions">Group of monsters between party level and -2 party level.</p>
	<Select
		label="Henchmen Budget"
		bind:disabled={disabled}
		options={monsterBudget}
		bind:value={formData.hench.budget}/>
	<br/>
	<Radio
		options={eitherBools}
		type="Ranged?"
		bind:disabled={disabled}
		bind:userSelected={formData.hench.ranged}/>

	<Radio
		options={eitherBools}
		type="Caster?"
		bind:disabled={disabled}
		bind:userSelected={formData.hench.caster}/>
	<label>
		Aquatic?
		<input type="checkbox" bind:checked={formData.hench.aquatic} />
	</label>

	<h3>Lackeys</h3>
	<p class="instructions">Group of weaker monsters that are party level -3 or -4.</p>
	<Select
		label="Lackey Budget"
		bind:disabled={disabledLackey}
		options={monsterBudget}
		bind:value={formData.lackey.budget}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="Ranged?"
		bind:userSelected={formData.lackey.ranged}/>
	<Radio
		options={eitherBools}
		bind:disabled={disabledLackey}
		type="Caster?"
		bind:userSelected={formData.lackey.caster}/>
	<label>
		Aquatic?
		<input type="checkbox" bind:checked={formData.lackey.aquatic} />
	</label>

	<br/>
</form>

<style>
	.instructions {
		max-width: 30rem;
	}
</style>
