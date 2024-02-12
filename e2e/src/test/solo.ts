import assert from "assert";
import bbegData from "../bbegData.js";
import henchData from "../henchData.js";
import lackeyData from "../lackeyData.js";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
import { describe, it } from "node:test";

for (const d of difficulties) {
  soloTest(d);
}

async function soloTest(difficulty: string) {
  describe(`Solo ${difficulty} fill status`, function () {
    for (let i = 1; i <= 20; i++) {
      it(`Solo bbeg should be filled at level ${i}`, async function () {
        const data = await bbegData(i, difficulty);
        if (data) {
          console.log(data.bbeg_name);
          assert.equal(data.bbeg_status, "Filled");
          assert.notEqual(data.bbeg_name, "Failed To find Monster");
        }
      });
    }

    for (let i = 1; i <= 20; i++) {
      it(`Solo hench should be filled at level ${i}`, async function () {
        const data = await henchData(i, difficulty);
        if (data) {
          console.log(data.hench_name);
          assert.equal(data.hench_status, "Filled");
          assert.notEqual(data.hench_name, "Failed To find Monster");
          assert.ok(
            data.hench_level === i ||
              data.hench_level === i - 1 ||
              data.hench_level === i - 2,
						`Monster level: ${data.hench_level}`
          );
        }
      });
    }

    for (let i = 1; i <= 20; i++) {
      it(`Solo Lackey should be filled at level ${i}`, async function () {
        const data = await lackeyData(i, difficulty);
        if (data) {
          console.log(data.lackey_name);
          if (i === 1) {
            assert.equal(data.lackey_status, "Failed");
            assert.equal(data.lackey_name, "Failed To find Monster");
          } else {
            assert.equal(data.lackey_status, "Filled");
            assert.notEqual(data.lackey_name, "Failed To find Monster");
            assert.ok(
              data.lackey_level === i - 3 ||
								data.lackey_level === i - 4,
						`Monster level: ${data.hench_level}`
            );
          }
        }
      });
    }
  });
}
