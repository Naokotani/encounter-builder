import assert from "assert";
import lackeyData from "../lackeyData.js";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
import { describe, it } from "node:test";
import "dotenv/config";
const API_URL = process.env.API_URL;

loopDifficulties()

async function loopDifficulties() {
	for (const d of difficulties) {
		await soloTest(d);
	}
}

async function soloTest(difficulty: string) {
  describe(`Solo ${difficulty} fill status`, async function () {
    for (let i = 1; i <= 20; i++) {
        it(`Solo Lackey should be filled at level ${i}`, async function () {
					const data = await lackeyData(i, difficulty, API_URL);
					console.log(`${data.lackey_name} id: ${data.id} level: ${data.lackey_level}`);
          if (i === 1) {
            assert.equal(data.lackey_status, "Failed");
            assert.equal(data.lackey_name, "Failed To find Monster");
          } else {
            assert.equal(data.lackey_status, "Filled");
            assert.notEqual(data.lackey_name, "Failed To find Monster");
            assert.ok(
              data.lackey_level === i - 3 || data.lackey_level === i - 4,
              `Monster level: ${data.hench_level}`,
            );
          }
        });
      }
  });
}
