import assert from "assert";
import bbegData from "../bbegData.js";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
import { describe, it } from "node:test";
import "dotenv/config";
const API_URL = process.env.API_URL;

looDifficulties()

async function looDifficulties() {
	for (const d of difficulties) {
		await loopLevels(d);
	}
}

async function loopLevels(difficulty: string) {
  describe(`Solo ${difficulty} for bbeg`, async function () {
    for (let i = 1; i <= 20; i++) {
			test(difficulty, i)
    }
  });
}

async function test(d: string, l: number) {
	return new Promise((resolve, reject) => {
		try {
			it(`Solo bbeg should be filled at level ${l}`, async function () {
				const data = await bbegData(l, d, API_URL);
				console.log(`${data.bbeg_name} id: ${data.id} level: ${data.bbeg_level}`);
				assert.equal(data.bbeg_status, "Filled");
				assert.notEqual(data.bbeg_name, "Failed To find Monster");
			});
		} catch(e) {
			reject(e);
		}
		resolve("Test complete");
	})
}

