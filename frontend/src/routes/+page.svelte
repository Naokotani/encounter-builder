<script>
	import { Monster } from '$lib';
	import Form from './Form.svelte'

	let bbegData = {
		budget: 'all',
		aquatic: null,
		caster: 'either',
		ranged: 'either,'
	}

	let henchData = {
		budget: 'all',
		aquatic: null,
		caster: 'either',
		ranged: 'either,'
	}

	let lackeyData = {
		budget: 'all',
		aquatic: null,
		caster: 'either',
		ranged: 'either,'
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

	async function newBbeg() {
		const params = new URLSearchParams({
			level: bbeg.level,
			party_size: formData.party_size,
			monster_types: formData.monster_types,
			budget: bbeg.budget,
			is_caster: formData.bbeg.caster.toLowerCase(),
			is_ranged: formData.bbeg.ranged.toLowerCase(),
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
			is_caster: formData.hench.caster.toLowerCase(),
			is_ranged: formData.hench.ranged.toLowerCase(),
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
			is_caster: formData.lackey.caster.toLowerCase(),
			is_ranged: formData.lackey.ranged.toLowerCase(),
		});

		const response = await fetch(`/monster?${params.toString()}`);
		lackey = await response.json(monster);
	}

	async function newMonster(monster) {
		const params = new URLSearchParams({
			level: monster.level,
			party_size: formData.party_size,
			monster_types: formData.monster_types,
			budget: monster.budget,
			is_caster: formData.lackey_caster.toLowerCase(),
			is_ranged: formData.lackey_ranged.toLowerCase(),
		});

		const response = await fetch(`/monster?${params.toString()}`);
		lackey = await response.json(monster);
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
</script>
<div class="grid aside-left">
	<div class="form">
		<Form formData={formData} submit={handleSubmit} />
	</div>
	<div class="cards">
		{#if bbeg_number !== 0}
			<div class="card">
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
			</div>
		{/if}

{#if hench_number !== 0}
	<div class="card">
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
	</div>
{/if}

{#if lackey_number !== 0}
	<div class="card">
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
