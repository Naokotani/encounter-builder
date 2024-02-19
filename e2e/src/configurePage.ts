import puppeteer from 'puppeteer';
import 'dotenv/config'
const CLIENT_URL = process.env.CLIENT_URL;

export default async (level: number) => {
  const browser = await puppeteer.launch({
		headless: true,
	});

  const page = await browser.newPage();

  await page.goto(`${CLIENT_URL}encounter-builder/`);

  await page.setViewport({width: 1920, height: 1080});

	await page.waitForNetworkIdle();

	await page.select('#party-level', level.toString())

	return {
		page: page,
		browser: browser,
	};
};
