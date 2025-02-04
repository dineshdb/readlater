const defaultIcon = "gray";
const inProgressIcon = "orange";
const successIcon = "green";
const errorIcon = "red";

const setIcon = (variant) => {
  browser.browserAction.setIcon({ path: `icons/icon-${variant}-48.png` });
};

browser.browserAction.onClicked.addListener(async (tab) => {
  const { url, title } = tab;
  setIcon(inProgressIcon);

  const res = await browser.runtime.sendNativeMessage("readlater", {
    action: "save",
    url,
    title,
  });
  let defaultIconTimeout = 2000;
  if (res.status = "ok") {
    setIcon(successIcon);
  } else {
    defaultIconTimeout = 5000;
    setIcon(errorIcon);
  }

  setTimeout(() => {
    // set default icon
    setIcon(defaultIcon);
  }, defaultIconTimeout);
});
