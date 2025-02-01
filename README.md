# README

> readlater://save?url=https://example.org&tags

A protocol handler that will save any articles it receives to getpocket.


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
- [ ] CI
  - [ ] Tests
  - [ ] Release Binaries on Github packages
  - [ ] Add a install script

## License

MIT
