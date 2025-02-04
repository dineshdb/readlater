const defaultIcon = "gray";
const inProgressIcon = "orange";
const successIcon = "green";
const errorIcon = "red";

const setIcon = (variant) => {
  browser.browserAction.setIcon({ path: `icons/icon-${variant}-38.png` });
};

browser.browserAction.onClicked.addListener(async (tab) => {
  const { url, title } = tab;
  setIcon(inProgressIcon);

  const res = await browser.runtime.sendNativeMessage("readlater", {
    action: "save",
    url,
    title,
  });
  if (res.status = "ok") {
    setIcon(successIcon);
    setTimeout(() => {
      // resset to default icon
      setIcon(defaultIcon);
    }, 3000);
  } else {
    setIcon(errorIcon);
  }
});
