var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import puppeteer from 'puppeteer';
import 'dotenv/config';
const CLIENT_URL = process.env.CLIENT_URL;
export default (level) => __awaiter(void 0, void 0, void 0, function* () {
    const browser = yield puppeteer.launch({
        headless: true,
    });
    const page = yield browser.newPage();
    yield page.goto(`${CLIENT_URL}encounter-builder/`);
    yield page.setViewport({ width: 1920, height: 1080 });
    yield page.waitForNetworkIdle();
    yield page.select('#party-level', level.toString());
    return {
        page: page,
        browser: browser,
    };
});
