import bbegData from './henchData.js';
import henchData from './henchData.js';
import lackeyData from './lackeyData.js';
import 'dotenv/config'
const API_URL = process.env.API_URL;

async function main() {
	const bbeg = await bbegData(1, 'moderate', API_URL);
	console.log(bbeg);

	const hench = await henchData(1, 'moderate', API_URL);
	console.log(hench);

	const lackey = await lackeyData(1, 'moderate', API_URL);
	console.log(lackey);
}

main()
