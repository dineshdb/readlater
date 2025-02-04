const defaultIcon = "gray";
const inProgressIcon = "orange";
const successIcon = "green";
const errorIcon = "red";

const setIcon = (variant) => {
  browser.browserAction.setIcon({ path: `icons/icon-${variant}-38.png` });
};

async function saveForLater(url, title) {
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
}

browser.browserAction.onClicked.addListener(({ url, title }) =>
  saveForLater(url, title)
);

browser.commands.onCommand.addListener((command) => {
  if (command === "save-for-later") {
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
      const { url, title } = tabs[0];
      saveForLater(url, title);
    });
  }
});
