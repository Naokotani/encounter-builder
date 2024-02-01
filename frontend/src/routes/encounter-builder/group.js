import { PUBLIC_MONSTER_API } from '$env/static/public';

export default class Group {
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
			this.status = 'Pending';
			this.bbeg = bbeg;
		}

		initialGroup(json, budget) {
			this.updateGroupData(json)
			this.budget_constant = budget;
		}

	async newGroup(formData, name) {
		const params = new URLSearchParams({
			name: name,
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
    if (json) {
      this.updateGroupData(json);
			this.found = true;
    } else {
      this.is_found = false;
    }
  } catch (error) {
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
		this.status = json.status;
	}
}
