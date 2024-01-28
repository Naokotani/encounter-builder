<script>
	import { Monster } from '$lib';
	import Form from './Form.svelte'
	import { PUBLIC_MONSTER_API, PUBLIC_ENCOUNTER_API } from '$env/static/public';

	class Group {
		constructor(bbeg) {
			this.url = "";
			this.level = 0;
			this.name = "";
			this.budget = 0;
			this.number = 0;
			this.alignment = "";
			this.type = "";
			this.budget_constant = 0;
			this.aquatic = false;
			this.is_caster = false;
			this.is_ranged = false;
			this.is_found = false;
			this.bbeg = bbeg;
		}

		initialGroup(json, budget) {
			this.updateGroupData(json)
			this.budget_constant = budget;
		}

	async newGroup() {
		console.log('foo');
		const params = new URLSearchParams({
			level: this.level,
			party_level: formData.level,
			number: this.number,
			monster_types: formData.monster_types,
			budget: this.budget_constant,
			is_caster: formData.bbeg.caster.toLowerCase(),
			is_ranged: formData.bbeg.ranged.toLowerCase(),
			bbeg: this.bbeg,
		});

  try {
    const response = await fetch(`${PUBLIC_MONSTER_API}${params.toString()}`);
    const json = await response.json();
		console.log(json);
    if (json) {
      this.updateGroupData(json);
			this.found = true;
    } else {
      this.is_found = false;
    }
  } catch (error) {
    console.error('Error fetching monster data:', error);
    this.is_found = false;
  }

	}

	updateGroupData(json) {
		this.url = json.url;
		this.level = json.level;
		this.name = json.name;
		this.budget = json.budget;
		this.number = json.number;
		this.alignment = json.alignment;
		this.type = json.type;
		this.aquatic = json.aquatic;
		this.is_caster = json.is_caster;
		this.is_ranged = json.is_ranged;
	}
}

	let bbeg1 = new Group(true);
	let hench1 = new Group(false);
	let lackey1 = new Group(false);

	let bbegData = {
		budget: 'all',
		aquatic: null,
		caster: 'either',
		ranged: 'either',
		type: null,
	}

	let henchData = {
		budget: 'all',
		aquatic: null,
		caster: 'either',
		ranged: 'either',
		type: null,
	}

	let lackeyData = {
		budget: 'all',
		aquatic: null,
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

	async function handleSubmit() {
		const params = new URLSearchParams({
			level: formData.level,
			party_size: formData.party_size,
			difficulty: formData.difficulty,
			monster_types: [formData.monster_types],
			bbeg_url: formData.bbeg_url,
			bbeg_budget: formData.bbeg.budget,
			bbeg_caster: formData.bbeg.caster.toLowerCase(),
			bbeg_ranged: formData.bbeg.ranged.toLowerCase(),
			hench_url: formData.bbeg_url,
			hench_budget: formData.hench.budget,
			hench_caster: formData.hench.caster.toLowerCase(),
			hench_ranged: formData.hench.ranged.toLowerCase(),
			lackey_url: formData.lackey_url,
			lackey_budget: formData.lackey.budget,
			lackey_caster: formData.lackey.caster.toLowerCase(),
			lackey_ranged: formData.lackey.ranged.toLowerCase(),
		});

		const response = await fetch(`${PUBLIC_ENCOUNTER_API}${params.toString()}`);
		const monster = await response.json();
		console.log(monster)

		const bbegData = {
			url: monster.bbeg_url,
			level: monster.bbeg_level,
			name: monster.bbeg_name,
			budget: monster.bbeg_budget,
			number: monster.bbeg_number,
			alignment: monster.bbeg_alignment,
			type: monster.bbeg_monster_type,
		}
		bbeg1.initialGroup(bbegData, monster.bbeg_budget);
		bbeg1 = bbeg1;

		const henchData = {
			url: monster.hench_url,
			level: monster.hench_level,
			name: monster.hench_name,
			budget: monster.hench_budget,
			number: monster.hench_number,
			alignment: monster.hench_alignment,
			type: monster.hench_monster_type,
		}
		hench1.initialGroup(henchData, monster.hench_budget);
		hench1 = hench1;

		const lackeyData = {
			url: monster.lackey_url,
			level: monster.lackey_level,
			name: monster.lackey_name,
			budget: monster.lackey_budget,
			number: monster.lackey_number,
			alignment: monster.lackey_alignment,
			type: monster.lackey_monster_type,
		}
		lackey1.initialGroup(lackeyData, monster.lackey_budget);
		lackey1 = lackey1;
	}

	async function handleBbeg() {
		await bbeg1.newGroup();
		bbeg1 = bbeg1;
	}

	async function handleHench() {
		await hench1.newGroup();
		hench1 = hench1;
	}

	async function handleLackey() {
		await lackey1.newGroup();
		lackey1 = lackey1;
	}


</script>
<div class="grid aside-left">
	<div class="form">
		<Form formData={formData} submit={handleSubmit} />
	</div>
	<div class="cards">
		{#if bbeg1.number !== 0 || bbeg1.name === "Failed To find Monster"}
			<div class="card">
				<Monster
					bind:url={bbeg1.url}
					bind:level={bbeg1.level}
					bind:name={bbeg1.name}
					bind:budget={bbeg1.budget}
					bind:number={bbeg1.number}
					bind:alignment={bbeg1.alignment}
					bind:type={bbeg1.type}
					/>
				<button on:click={handleBbeg}>New Monster</button>
			</div>
		{/if}

{#if hench1.number !== 0 || hench1.name === "Failed To find Monster"}
	<div class="card">
		<Monster
			bind:url={hench1.url}
			bind:name={hench1.name}
			bind:level={hench1.level}
			bind:budget={hench1.budget}
			bind:number={hench1.number}
			bind:alignment={hench1.alignment}
			bind:type={hench1.type}
			/>
		<button on:click={handleHench}>New Monster</button>
	</div>
{/if}

{#if lackey1.number !== 0 || lackey1.name === "Failed To find Monster"}
	<div class="card">
		<Monster
			bind:url={lackey1.url}
			bind:name={lackey1.name}
			bind:level={lackey1.level}
			bind:budget={lackey1.budget}
			bind:number={lackey1.number}
			bind:alignment={lackey1.alignment}
			bind:type={lackey1.monster_type}
			/>
		<button on:click={handleLackey}>New Monster</button>
	</div>
{/if}
	</div>
</div>

<style>
.card p, h3, h4, h5 {
	margin-top: 0;
	margin-bottom: 0;
}

.card {
	max-width: 30%;
  background: var(--cardBg);
  color: var(--cardText);
  padding: 2rem;
  border-radius: 3px;
	margin-bottom: 1rem;
}

.cards {
	margin-top: 3rem;
}

</style>
