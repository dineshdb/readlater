browser.browserAction.onClicked.addListener(async (tab) => {
	const { url, title } = tab;
	const res = await browser.runtime.sendNativeMessage("readlater", { action: 'save', url, title });
	console.log(res)
});
