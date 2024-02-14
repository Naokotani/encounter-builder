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
import henchData from "../henchData.js";
import { describe, it } from "node:test";
import "dotenv/config";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
const API_URL = process.env.API_URL;
loopDifficulties();
function loopDifficulties() {
    return __awaiter(this, void 0, void 0, function* () {
        for (const d of difficulties) {
            yield soloHenchTest(d);
        }
    });
}
function soloHenchTest(difficulty) {
    return __awaiter(this, void 0, void 0, function* () {
        describe(`Solo ${difficulty} fill status`, function () {
            return __awaiter(this, void 0, void 0, function* () {
                for (let i = 1; i <= 20; i++) {
                    it(`Solo hench should be filled at level ${i}`, function () {
                        return __awaiter(this, void 0, void 0, function* () {
                            const data = yield henchData(i, difficulty, API_URL);
                            console.log(`${data.hench_name} id: ${data.id} level: ${data.hench_level}`);
                            assert.equal(data.hench_status, "Filled");
                            assert.notEqual(data.hench_name, "Failed To find Monster");
                            assert.ok(data.hench_level === i ||
                                data.hench_level === i - 1 ||
                                data.hench_level === i - 2, `Monster level: ${data.hench_level}`);
                        });
                    });
                }
            });
        });
    });
}
