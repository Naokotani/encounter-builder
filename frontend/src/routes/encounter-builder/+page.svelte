<script>
	import { Monster } from '$lib';
	import Group from './group'
	import Form from './Form.svelte'
	import { PUBLIC_ENCOUNTER_API } from '$env/static/public';

	let bbeg1 = new Group(true);
	let hench1 = new Group(false);
	let lackey1 = new Group(false);

	let bbegData = {
		budget: 'all',
		aquatic: false,
		caster: 'either',
		ranged: 'either',
		type: null,
	}

	let henchData = {
		budget: 'all',
		aquatic: false,
		caster: 'either',
		ranged: 'either',
		type: null,
	}

	let lackeyData = {
		budget: 'all',
		aquatic: false,
		caster: 'either',
		ranged: 'either',
		type: null,
	}

	let formData = {
		level: null,
		party_size: null,
		difficulty: 'moderate',
		monster_types: null,
		bbeg: bbegData,
		hench: henchData,
		lackey: lackeyData,
	};

	let bbeg_type;
	let hench_type;
	let lackey_type;

	async function handleSubmit() {
		const params = new URLSearchParams({
			level: formData.level,
			party_size: formData.party_size,
			difficulty: formData.difficulty,
			monster_types: [formData.monster_types],
			bbeg_budget: formData.bbeg.budget,
			bbeg_caster: formData.bbeg.caster.toLowerCase(),
			bbeg_ranged: formData.bbeg.ranged.toLowerCase(),
			bbeg_aquatic: formData.bbeg.aquatic,
			hench_budget: formData.hench.budget,
			hench_caster: formData.hench.caster.toLowerCase(),
			hench_ranged: formData.hench.ranged.toLowerCase(),
			hench_aquatic: formData.hench.aquatic,
			lackey_budget: formData.lackey.budget,
			lackey_caster: formData.lackey.caster.toLowerCase(),
			lackey_ranged: formData.lackey.ranged.toLowerCase(),
			lackey_aquatic: formData.lackey.aquatic,
		});

		const response = await fetch(`${PUBLIC_ENCOUNTER_API}${params.toString()}`);
		const monster = await response.json();

		const bbegData = {
			url: monster.bbeg_url,
			level: monster.bbeg_level,
			name: monster.bbeg_name,
			budget: monster.bbeg_budget,
			number: monster.bbeg_number,
			alignment: monster.bbeg_alignment,
			status: monster.bbeg_status,
		}
		bbeg1.initialGroup(bbegData, monster.bbeg_budget);
		bbeg_type = monster.bbeg_monster_type;
		bbeg1 = bbeg1;

		const henchData = {
			url: monster.hench_url,
			level: monster.hench_level,
			name: monster.hench_name,
			budget: monster.hench_budget,
			number: monster.hench_number,
			alignment: monster.hench_alignment,
			status: monster.hench_status,
		}
		hench1.initialGroup(henchData, monster.hench_budget);
		hench_type = monster.hench_monster_type,
		hench1 = hench1;

		const lackeyData = {
			url: monster.lackey_url,
			level: monster.lackey_level,
			name: monster.lackey_name,
			budget: monster.lackey_budget,
			number: monster.lackey_number,
			alignment: monster.lackey_alignment,
			status: monster.lackey_status,
		}
		lackey1.initialGroup(lackeyData, monster.lackey_budget);
		lackey_type = monster.lackey_monster_type;
		lackey1 = lackey1;
	}

	async function handleBbeg() {
    const ranged = formData.bbeg.ranged;
    const caster = formData.bbeg.caster;
    const aquatic = formData.bbeg.aquatic;
		await bbeg1.newGroup(formData, bbeg1.name, ranged, caster, aquatic);
		bbeg1 = bbeg1;
	}

	async function handleHench() {
    const ranged = formData.hench.ranged;
    const caster = formData.hench.caster;
    const aquatic = formData.hench.aquatic;
		await hench1.newGroup(formData, hench1.name, ranged, caster, aquatic);
		hench1 = hench1;
	}

	async function handleLackey() {
    const ranged = formData.lackey.ranged;
    const caster = formData.lackey.caster;
    const aquatic = formData.lackey.aquatic;
		await lackey1.newGroup(formData, lackey1.name, ranged, caster, aquatic);
		lackey1 = lackey1;
	}
</script>
<svelte:head>
	<title>PF2e Encounter Builder</title>
	<meta
		name="description"
		content=
		"
		 An encounter builder for Pathfinder Second Edition.
		 The builder takes imports from users and aims to create
		 properly balanced encounters for different difficulties.
		 " />
</svelte:head>
<div class="form-grid">
	<div class="form">
		<Form formData={formData} submit={handleSubmit} />
	</div>
	<div class="cards" id="monster-div">
		{#if bbeg1.number !== 0 || bbeg1.status === 'Failed'}
			<div class="card slide-from-right">
				<Monster
					title="Big Bad Evil Guy"
					bind:url={bbeg1.url}
					bind:level={bbeg1.level}
					bind:name={bbeg1.name}
					bind:budget={bbeg1.budget}
					bind:number={bbeg1.number}
					bind:alignment={bbeg1.alignment}
					bind:type={bbeg_type}
					/>
				<button class="new-monster" on:click={handleBbeg}>New Monster</button>
			</div>
		{/if}

{#if hench1.number !== 0 || hench1.status === 'Failed'}
	<div class="card slide-from-right">
		<Monster
			title="Henchmen"
			bind:url={hench1.url}
			bind:name={hench1.name}
			bind:level={hench1.level}
			bind:budget={hench1.budget}
			bind:number={hench1.number}
			bind:alignment={hench1.alignment}
			bind:type={hench_type}
			/>
		<button class="new-monster" on:click={handleHench}>New Monster</button>
	</div>
{/if}

{#if lackey1.number !== 0 || lackey1.status === 'Failed'}
	<div class="card slide-from-right">
		<Monster
			title="Henchmen"
			bind:url={lackey1.url}
			bind:name={lackey1.name}
			bind:level={lackey1.level}
			bind:budget={lackey1.budget}
			bind:number={lackey1.number}
			bind:alignment={lackey1.alignment}
			bind:type={lackey_type}
			/>
		<button class="new-monster" on:click={handleLackey}>New Monster</button>
	</div>
{/if}
	</div>
</div>
<style>
.cards {
	margin-top: 3rem;
}

.card {
  background: var(--cardBg);
  color: var(--cardText);
  padding: 2rem;
  border-radius: 3px;
	margin-bottom: 1rem;
}

@media (min-width: 1500px) {
	.card {
		max-width: 33%;
	}

	.form-grid {
    grid-template-columns: 66% 33%;
		display: grid;
		gap: 5rem;
	}
}

@media only screen and (min-width: 768px) and (max-width: 1500px) {
	.card {
		max-width: 25vw;
		height: 25rem;
	}

	.cards {
		max-width: 95vw;
	}

	.new-monster {
		margin-bottom: 1rem;
	}

	#monster-div {
		display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 5px;
	}
}


</style>
