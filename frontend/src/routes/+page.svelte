<script>
	import { Monster } from '$lib';
	import Form from './Form.svelte'

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

	let monster;
	let bbeg_number = 0;
	let hench_number = 0;
	let lackey_number = 0;
	let hench_budget_constant;
	let lackey_budget_constant;

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
		bbeg.type = monster.bbeg_monster_type;

		hench.url = monster.hench_url;
		hench.level = monster.hench_level;
		hench.name = monster.hench_name;
		hench.budget = monster.hench_budget;
		hench.number = monster.hench_number;
		hench.alignment = monster.hench_alignment;
		hench.type = monster.hench_monster_type;
		hench_budget_constant = monster.hench_budget;

		lackey.url = monster.lackey_url;
		lackey.level = monster.lackey_level;
		lackey.name = monster.lackey_name;
		lackey.budget = monster.lackey_budget;
		lackey.number = monster.lackey_number;
		lackey.alignment = monster.lackey_alignment;
		lackey.type = monster.lackey_monster_type;
		lackey_budget_constant = monster.lackey_budget;
	}

	async function newBbeg() {
		const params = new URLSearchParams({
			level: bbeg.level,
			party_level: formData.level,
			number: bbeg.number,
			monster_types: formData.monster_types,
			budget: bbeg.budget,
			is_caster: formData.bbeg.caster.toLowerCase(),
			is_ranged: formData.bbeg.ranged.toLowerCase(),
			bbeg: true,
		});

		const response = await fetch(`/monster?${params.toString()}`);
		bbeg = await response.json();
	}

	async function newHench() {
		const params = new URLSearchParams({
			level: hench.level,
			party_level: formData.level,
			number: hench.number,
			budget: hench_budget_constant,
			monster_types: formData.monster_types,
			is_caster: formData.hench.caster.toLowerCase(),
			is_ranged: formData.hench.ranged.toLowerCase(),
			bbeg: false,
		});

		const number = hench.number;
		const response = await fetch(`/monster?${params.toString()}`);
		hench = await response.json();
	}

	async function newLackey() {
		const params = new URLSearchParams({
			level: lackey.level,
			party_level: formData.level,
			number: lackey.number,
			monster_types: formData.monster_types,
			budget: lackey_budget_constant,
			is_caster: formData.lackey.caster.toLowerCase(),
			is_ranged: formData.lackey.ranged.toLowerCase(),
			bbeg: false,
		});

		const res = await fetch(`/monster?${params.toString()}`);
		lackey = await res.json();
	}

	let bbeg = {
		alignment: "",
		aquatic: false,
		budget: 0,
		is_caster: false,
		is_found: false,
		is_ranged: false,
		level: 0,
		monster_type: "",
		name: "",
		number: 0,
		url: "",
	}

	let hench = {
		alignment: "",
		aquatic: false,
		budget: 0,
		is_caster: false,
		is_found: false,
		is_ranged: false,
		level: 0,
		monster_type: "",
		name: "",
		number: 0,
		url: "",
	}

	let lackey = {
		alignment: "",
		aquatic: false,
		budget: 0,
		is_caster: false,
		is_found: false,
		is_ranged: false,
		level: 0,
		monster_type: "",
		name: "",
		number: 0,
		url: "",
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
					bind:url={bbeg.url}
					bind:level={bbeg.level}
					bind:name={bbeg.name}
					bind:budget={bbeg.budget}
					bind:number={bbeg.number}
					bind:alignment={bbeg.alignment}
					bind:type={bbeg.monster_type}
					/>
				<button on:click={newBbeg}>New Monster</button>
			</div>
		{/if}

{#if hench_number !== 0}
	<div class="card">
		<Monster
			bind:url={hench.url}
			bind:name={hench.name}
			bind:level={hench.level}
			bind:budget={hench.budget}
			bind:number={hench.number}
			bind:alignment={hench.alignment}
			bind:type={hench.monster_type}
			/>
		<button on:click={newHench}>New Monster</button>
	</div>
{/if}

{#if lackey_number !== 0}
	<div class="card">
		<Monster
			bind:url={lackey.url}
			bind:name={lackey.name}
			bind:level={lackey.level}
			bind:budget={lackey.budget}
			bind:number={lackey.number}
			bind:alignment={lackey.alignment}
			bind:type={lackey.monster_type}
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
