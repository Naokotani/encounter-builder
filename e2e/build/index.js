var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import bbegData from './henchData.js';
import henchData from './henchData.js';
import lackeyData from './lackeyData.js';
import 'dotenv/config';
const API_URL = process.env.API_URL;
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const bbeg = yield bbegData(1, 'moderate', API_URL);
        console.log(bbeg);
        const hench = yield henchData(1, 'moderate', API_URL);
        console.log(hench);
        const lackey = yield lackeyData(1, 'moderate', API_URL);
        console.log(lackey);
    });
}
main();
