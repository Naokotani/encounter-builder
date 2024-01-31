<script>
	import { Radio, Select, Modal } from '$lib';
  import { onMount } from 'svelte';
	import Budget from './Budget.svelte'

	export let formData;
	export let submit;


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
  });

  function scrollToId() {
    if (isMobile && !disableSubmit) {
      const element = document.getElementById('monster-div');
      if (element) {
        element.scrollIntoView({ behavior: 'smooth' });
      }
    }
  }

	let monsterBudget = [
		{value: 'all', label: 'All'},
		{value: 'more', label: 'Three Quarters'},
		{value: 'even', label: 'Half'},
		{value: 'less', label: 'One Quarter'},
		{value: 'none', label: 'None'},
	];

	let budget = 100;
	let percentages = [0 ,100 , 0, 0]; // Example percentages for three groups
	function handleBudgetChange() {
		budget = 100;
		if (formData.bbeg.budget === 'all') {
			console.log(formData.bbeg.budget)
			percentages[0] = 100;
			budget -= 100;
		} else if (formData.bbeg.budget === 'even') {
			console.log(formData.bbeg.budget)
			percentages[0] = 50;
			budget -= 50;
		} else if (formData.bbeg.budget === 'less') {
			console.log(formData.bbeg.budget)
			percentages[0] = 25;
			budget -= 25;
		} else if (formData.bbeg.budget === 'none') {
			console.log(formData.bbeg.budget)
			percentages[0] = 0;
		} else if (formData.bbeg.budget === 'more') {
			percentages[0] = 75;
			budget -= 75;
		}

		console.log(budget)
		if (formData.hench.budget === 'all') {
			console.log(formData.hench.budget)
			percentages[1] = budget;
			budget = 0;
		} else if (formData.hench.budget === 'even') {
			console.log(formData.hench.budget)
			percentages[1] = budget * 0.5;
			budget -= budget * 0.5 
		} else if (formData.hench.budget === 'less') {
			console.log(formData.hench.budget)
			percentages[1] = budget * 0.25;
			budget -= budget * 0.25;
		} else if (formData.hench.budget === 'none') {
			console.log(formData.hench.budget)
			percentages[1] = 0;
		} else if (formData.hench.budget === 'more') {
			percentages[1] = budget * 0.75;
			budget -= budget * 0.75;
		}

		if (formData.lackey.budget === 'all') {
			console.log(formData.lackey.budget)
			percentages[2] = budget;
			budget = 0;
		} else if (formData.lackey.budget === 'even') {
			console.log(formData.lackey.budget)
			percentages[2] = budget * 0.5;
			budget -= budget * 0.5 
		} else if (formData.lackey.budget === 'less') {
			console.log(formData.lackey.budget)
			percentages[2] = budget * 0.25;
			budget -= budget * 0.25;
		} else if (formData.lackey.budget === 'none') {
			console.log(formData.lackey.budget)
			percentages[2] = 0;
		} else if (formData.lackey.budget === 'more') {
			percentages[2] = budget * 0.75;
			budget -= budget * 0.75;
		}

		console.log(percentages[1]);




		
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
		<div>
			<div>
				<div>
					<h2>Encounter Options</h2>
					<Modal />
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
			handleBudgetChange={handleBudgetChange}
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
		<Budget percentages={percentages}/>
	</div>
	<div>
		<h3>Henchmen</h3>
		<p class="instructions">
			Group of monsters between party level and party level -2.
		</p>
		<Select
			handleBudgetChange={handleBudgetChange}
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
			Party must be at least level 2.</p>
		<Select
			handleBudgetChange={handleBudgetChange}
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
