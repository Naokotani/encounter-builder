import bbegData from './henchData.js';
import henchData from './henchData.js';
import lackeyData from './lackeyData.js';

async function main() {
	const bbeg = await bbegData(1, 'moderate');
	console.log(bbeg);

	const hench = await henchData(1, 'moderate');
	console.log(hench);

	const lackey = await lackeyData(1, 'moderate');
	console.log(lackey);
}

main()
