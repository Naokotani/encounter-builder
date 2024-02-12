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
import henchData from "../henchData.js";
import lackeyData from "../lackeyData.js";
const difficulties = ["trivial", "low", "moderate", "severe", "extreme"];
import { describe, it } from "node:test";
for (const d of difficulties) {
    soloTest(d);
}
function soloTest(difficulty) {
    return __awaiter(this, void 0, void 0, function* () {
        describe(`Solo ${difficulty} fill status`, function () {
            for (let i = 1; i <= 20; i++) {
                it(`Solo bbeg should be filled at level ${i}`, function () {
                    return __awaiter(this, void 0, void 0, function* () {
                        const data = yield bbegData(i, difficulty);
                        if (data) {
                            console.log(data.bbeg_name);
                            assert.equal(data.bbeg_status, "Filled");
                            assert.notEqual(data.bbeg_name, "Failed To find Monster");
                        }
                    });
                });
            }
            for (let i = 1; i <= 20; i++) {
                it(`Solo hench should be filled at level ${i}`, function () {
                    return __awaiter(this, void 0, void 0, function* () {
                        const data = yield henchData(i, difficulty);
                        if (data) {
                            console.log(data.hench_name);
                            assert.equal(data.hench_status, "Filled");
                            assert.notEqual(data.hench_name, "Failed To find Monster");
                        }
                    });
                });
            }
            for (let i = 1; i <= 20; i++) {
                it(`Solo Lackey should be filled at level ${i}`, function () {
                    return __awaiter(this, void 0, void 0, function* () {
                        const data = yield lackeyData(i, difficulty);
                        if (data) {
                            console.log(data.lackey_name);
                            if (i === 1) {
                                assert.equal(data.lackey_status, "Failed");
                                assert.equal(data.lackey_name, "Failed To find Monster");
                            }
                            else {
                                assert.equal(data.lackey_status, "Filled");
                                assert.notEqual(data.lackey_name, "Failed To find Monster");
                            }
                        }
                    });
                });
            }
        });
    });
}
