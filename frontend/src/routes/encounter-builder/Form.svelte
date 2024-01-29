<script>
	import { Radio, Select } from '$lib';
  import { onMount } from 'svelte';

	export let formData;
	export let submit;

	let monsterBudget = [
		{value: 'all', label: 'All'},
		{value: 'more', label: 'Three Quarters'},
		{value: 'even', label: 'Half'},
		{value: 'less', label: 'One Quarter'},
		{value: 'none', label: 'None'},
	];

	let lackeyBudget = [
		{value: 'all', label: 'All'},
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

  let isMobile = false;

  onMount(() => {
    const mediaQuery = window.matchMedia('(max-width: 767px)');
    isMobile = mediaQuery.matches;
		console.log(isMobile)
  });

  function scrollToId() {
    if (isMobile && !disableSubmit) {
      const element = document.getElementById('monster-div');
      if (element) {
				console.log("sroll")
        element.scrollIntoView({ behavior: 'smooth' });
      }
    }
  }

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
	$: disabledLackey = formData.hench.budget === 'all'
	|| formData.bbeg.budget === 'all'
	|| formData.level === '1';
	$: formData.hench.budget = formData.bbeg.budget === 'all'?'none':formData.hench.budget;
	$: {
		formData.lackey.budget = formData.bbeg.budget === 'all' || formData.hench.budget === 'all'?
			'none':formData.lackey.budget;
	}
	$: disableSubmit = formData.bbeg.budget !== 'all' &&
	formData.hench.budget !== 'all' && formData.lackey.budget !== 'all';
  $: submitInfoClass = disableSubmit? "":"submitInfoClass";
</script>
<form class="grid"  on:submit|preventDefault={submit} style="width=30%;">
	<div>

		<h2>Monsters Options</h2>
		<p  class="instructions ">
			Pick the budget you want to assign to each monster type.
			Setting a monster budget to 'all' will disable the monsters
			below it. 'None' will remove that monster group from the
			results. 'More', 'Half' and 'less' represents the amount of
			remaining budget that will be used on that monster group.
		</p>
		<div>
			<h3>Encounter Options</h3>
			<div>
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
						label="Difficulty"
						options={encounterDifficulty}
						selected={2}
						bind:value={formData.difficulty}/>
				</div>
			</div>
		</div>
		<h3>Big Bad Evil Guy</h3>
		<p class="instructions">Solo monster up to 4 levels about the party depending on budget.</p>
		<Select
			label="Solo Monster Budget"
			options={monsterBudget}
			bind:value={formData.bbeg.budget}/>
		<br/>
		<div class="grid">
			<Radio
				options={eitherBools}
				type="Ranged?"
				bind:userSelected={formData.bbeg.ranged}/>
			<Radio
				options={eitherBools}
				type="Caster?"
				bind:userSelected={formData.bbeg.caster}/>
		</div>
		<label>
			Aquatic?
			<input type="checkbox" bind:checked={formData.bbeg.aquatic} />
		</label>
	</div>
	<div>
		<h3>Henchmen</h3>
		<p class="instructions">
			Group of monsters between party level and party level -2. Set
			lackeys to "All" to enable more options.
		</p>
		<Select
			label="Henchmen Budget"
			bind:disabled={disabled}
			options={monsterBudget}
			bind:value={formData.hench.budget}/>
		<br/>
		<div class="grid">
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
		</div>
		<label>
			Aquatic?
			<input type="checkbox" bind:checked={formData.hench.aquatic} />
		</label>

		<h3>Lackeys</h3>
		<p class="instructions">
			Group of weaker monsters that are party level -3 or -4.
			Party must be at least level 2. To enable, neither Big
			Bad Evil Guy nor Henchmen can be set to "all".
		</p>
		<Select
			label="Lackey Budget"
			bind:disabled={disabledLackey}
			options={lackeyBudget}
			bind:value={formData.lackey.budget}/>
		<div class="grid">
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
		</div>
		<label>
			Aquatic?
			<input type="checkbox" bind:checked={formData.lackey.aquatic} />
		</label>
		<br/>
		<div>
		<button disabled={disableSubmit} on:click={scrollToId} class="submit" type="submit">Roll the dice</button>
		<br/>
		<small class={submitInfoClass}>
				You must use your full budget by setting one group to 'all'
		</small>
		</div>
	</div>

</form>

<style>
	.instructions {
		max-width: var(--formWidth);
	}

	.submit {
		font-size: var(--h5);
		padding: 0.5rem 1rem;
		margin-top: 1rem;
	}
	.submit:disabled {
		margin: 0;
		margin-top: 1rem;
	}

	.submitInfoClass {
		visibility: hidden;
		height: 0.8rem;
	}
</style>
