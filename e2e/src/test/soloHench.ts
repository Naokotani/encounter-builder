import assert from "assert";
import henchData from "../henchData.js";
import { describe, it } from "node:test";
import "dotenv/config";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
const API_URL = process.env.API_URL;

loopDifficulties()

async function loopDifficulties() {
	for (const d of difficulties) {
		await soloHenchTest(d);
	}
}

async function soloHenchTest(difficulty: string) {
  describe(`Solo ${difficulty} fill status`, async function () {
    for (let i = 1; i <= 20; i++) {
      it(`Solo hench should be filled at level ${i}`, async function () {
				const data = await henchData(i, difficulty, API_URL);
        console.log(`${data.hench_name} id: ${data.id} level: ${data.hench_level}`);
        assert.equal(data.hench_status, "Filled");
        assert.notEqual(data.hench_name, "Failed To find Monster");
        assert.ok(
          data.hench_level === i ||
            data.hench_level === i - 1 ||
            data.hench_level === i - 2,
          `Monster level: ${data.hench_level}`,
        );
      });
    }
  });
}
