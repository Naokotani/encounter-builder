import puppeteer from 'puppeteer';

export default async (level: number) => {
  const browser = await puppeteer.launch({
		headless: true,
	});

  const page = await browser.newPage();

  await page.goto('https://alembichead.com/encounter-builder/');

  await page.setViewport({width: 1920, height: 1080});

	await page.select('#party-level', level.toString())

	return {
		page: page,
		browser: browser,
	};
};
