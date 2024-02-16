var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import assert from "assert";
import bbegData from "../bbegData.js";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
import { describe, it } from "node:test";
import "dotenv/config";
const API_URL = process.env.API_URL;
looDifficulties();
function looDifficulties() {
    return __awaiter(this, void 0, void 0, function* () {
        for (const d of difficulties) {
            yield loopLevels(d);
        }
    });
}
function loopLevels(difficulty) {
    return __awaiter(this, void 0, void 0, function* () {
        describe(`Solo ${difficulty} for bbeg`, function () {
            return __awaiter(this, void 0, void 0, function* () {
                for (let i = 1; i <= 20; i++) {
                    test(difficulty, i);
                }
            });
        });
    });
}
function test(d, l) {
    return __awaiter(this, void 0, void 0, function* () {
        return new Promise((resolve, reject) => {
            try {
                it(`Solo bbeg should be filled at level ${l} difficulty: ${d}`, function () {
                    return __awaiter(this, void 0, void 0, function* () {
                        const data = yield bbegData(l, d, API_URL);
                        console.log(`${data.bbeg_name} id: ${data.id} level: ${data.bbeg_level} difficulty: ${d}`);
                        assert.equal(data.bbeg_status, "Filled");
                        assert.notEqual(data.bbeg_name, "Failed To find Monster");
                    });
                });
            }
            catch (e) {
                reject(e);
            }
            resolve("Test complete");
        });
    });
}
