# ![readlater:// logo](webext/icons/icon.svg "Logo")

> readlater://save?url=https://example.org&tags

A protocol handler that will save any articles it receives to getpocket.

## Supported Features

This supports handling `readlater://save?url=<url>&title=<title>&tags=<tags>`
without installing web extension. If you install the
[webextension](https://addons.mozilla.org/en-US/firefox/addon/read_later/), it
provides additional features such as:

- toolbar button to save current url to pocket by clicking it.

Alternatively, you can use bookmarket to add items to the getpocket. Or just
paste properly formatted link to url bar.

## Setup

- Install the binary `cargo install --git https://github.com/dineshdb/readlater`
- Register protocol handler and WebExtension native-host `readlater register`
- Provide Pocket authentication `POCKET_CONSUMER_KEY`, `POCKET_ACCESS_TOKEN` via
  env variable.

## Test

Visit the [hosted version of this page](https://dbhattarai.info.np/readlater/)
and then click
[this link](readlater://save?url=https://github.com/dineshdb/readlater) to add
the this repo url to pocket using `readlater://`.

## Bookmarklet

```javascript
javascript: (function () {
  var currentUrl = encodeURIComponent(window.location.href);
  var tags = prompt("Enter tags (comma-separated):", "");
  if (tags !== null) {
    var encodedTags = encodeURIComponent(tags);
    var url = `readlater://save?url=${currentUrl}&tags=${encodedTags}`;
    window.location.href = url;
  }
})();
```

## Roadmap

- Protocol Handler
  - [x] Save new urls to Pocket
- WebExtension
  - [x] Add a `readlater://` button that saves current tab to `readlater://` via
        native extension
  - [x] Release the web extension
  - [x] Change toolbar icon based on result
  - [x] Firefox(on Linux) Support
  - [ ] Insert `readlater://` links in web pages for easier saving to
        `readlater://`
  - [ ] Keyboard shortcut
  - [ ] Support for Chrome and Chromium based browsers
- CI
  - [x] Tests
  - [ ] Release Binaries on Github packages
  - [ ] Add a install script
- [ ] Get access token from keyring or `readlater.conf`
- [ ] Decouple it from Pocket and make it pluggable.

## License

MIT
