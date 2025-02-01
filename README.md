# `readlater://`

> readlater://save?url=https://example.org&tags

A protocol handler that will save any articles it receives to getpocket.

## Roadmap

- [x] Protocol Handler
  - [x] Save new urls to Pocket
- [ ] Get access token from keyring or `readlater.conf`
- [-] WebExtension
  - [-] Add a `readlater://` button that saves current tab to `readlater://` via native extension
  - [ ] Insert `readlater://` links in web pages for easier saving to `readlater://`
- [ ] CI
  - [ ] Tests
  - [ ] Release Binaries on Github packages
  - [ ] Add a install script

## Setup

- Install the binary `cargo install --git https://github.com/dineshdb/readlater`
- Register protocol handler `readlater handler register`
- Provide Pocket authentication `POCKET_CONSUMER_KEY`, `POCKET_ACCESS_TOKEN` via
  env variable.
- Start using the protocol handler by using the bookmarklet

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

## License

MIT
