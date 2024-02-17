var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
export default (page, url) => __awaiter(void 0, void 0, void 0, function* () {
    let data;
    page.on('response', (response) => __awaiter(void 0, void 0, void 0, function* () {
        try {
            if (response.url().startsWith(`${url}encounter?`)) {
                const responseData = yield response.json();
                data = responseData;
            }
            else {
                return true;
            }
        }
        catch (e) {
            console.log(e);
        }
    }));
    const searchResultSelector = 'button.submit';
    yield page.waitForSelector(searchResultSelector);
    yield page.click(searchResultSelector);
    yield page.waitForResponse((response) => __awaiter(void 0, void 0, void 0, function* () {
        return (yield response.text()).startsWith('{"id"');
    }));
    yield page.waitForNetworkIdle({ idleTime: 500 });
    return data;
});
