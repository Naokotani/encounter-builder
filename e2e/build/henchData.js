var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import configurePage from './configurePage.js';
import formData from './formData.js';
import emptyData from './emptyData.js';
export default (level, difficulty, url) => __awaiter(void 0, void 0, void 0, function* () {
    const { page, browser } = yield configurePage(level);
    yield page.select('#difficulty', difficulty);
    yield page.select('#solo-monster-budget', "none");
    yield page.select('#henchmen-budget', "all");
    const res = yield formData(page, url);
    let data;
    if (res) {
        data = res;
    }
    else {
        data = emptyData;
    }
    yield browser.close();
    return data;
});
