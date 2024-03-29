import { Page } from 'puppeteer';

export default async (page: Page, url: string) => {
	let data;

	page.on('response', async (response) => {
		try {

			if (response.url().startsWith(`${url}encounter?`)) {
				const responseData = await response.json();
				data = responseData;
			} else {
				return true;
			}
		} catch(e) {
			console.log(e);
		}
  })

  const searchResultSelector = 'button.submit';

  await page.waitForSelector(searchResultSelector);

  await page.click(searchResultSelector);

	await page.waitForResponse(async response => {
		return (await response.text()).startsWith('{"id"');
	});

	return data;
}
