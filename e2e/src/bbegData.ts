import configurePage from './configurePage.js';
import formData from './formData.js';
import Data from './Data.js'
import emptyData from './emptyData.js'

export default async (level: number, difficulty: string, url: string) => {
	const {page, browser} = await configurePage(level);

	await page.select('#difficulty', difficulty);
	await page.select('#solo-monster-budget', "all")

	const res = await formData(page, url);
	let data: Data;

	if (res) {
		data = res;
	} else {
		data = emptyData;
	}

	await browser.close();
	return data;
};
