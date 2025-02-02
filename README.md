# ![readlater:// logo](webext/icons/icon.svg "Logo")

> readlater://save?url=https://example.org&tags

A protocol handler that will save any articles it receives to getpocket.

## Supported Features

This supports handling `readlater://save?url=<url>&title=<title>&tags=<tags>`
without installing web extension. If you install the webextension, it provides
additional features such as:

- toolbar button to save current url to pocket by clicking it.

Alternatively, you can use bookmarket to add items to the getpocket. Or just
paste properly formatted link to url bar.

## Setup

- Install the binary `cargo install --git https://github.com/dineshdb/readlater`
- Register protocol handler and WebExtension native-host `readlater register`
- Provide Pocket authentication `POCKET_CONSUMER_KEY`, `POCKET_ACCESS_TOKEN` via
  env variable.

## Test

Add this [link](readlater://save?url=https://github.com/dineshdb/readlater) to
pocket using `readlater://`

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

- [x] Protocol Handler
  - [x] Save new urls to Pocket
- [ ] Get access token from keyring or `readlater.conf`
- [-] WebExtension
  - [x] Add a `readlater://` button that saves current tab to `readlater://` via
        native extension
  - [ ] Insert `readlater://` links in web pages for easier saving to
        `readlater://`
  - [x] Release the web extension
  - [ ] Change toolbar icon based on result
- [ ] Decouple it from Pocket and make it pluggable.
- [-] CI
  - [x] Tests
  - [ ] Release Binaries on Github packages
  - [ ] Add a install script

## License

MIT
