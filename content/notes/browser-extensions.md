+++
title = "Browser Extensions"
date = 2017-08-26

[note]
kind = "technology"
+++

I will mostly be focusing on the Chrome Extensions API, but I will do so with a bias towards the "standardized" portion of it, the [Browser Extensions] standard, as adopted by FireFox in the form of the [WebExtensions API].

[Browser Extensions]: https://browserext.github.io/browserext/
[WebExtensions API]: https://developer.mozilla.org/en-US/Add-ons/WebExtensions

<nav id="toc"></nav>

## Portability

Mozilla seems to be the only one making an effort to provide documentation for the APIs with consideration for other browsers, such as detailing the discrepancies. They have a document on [differences][webextension-differences] between FireFox and Chrome's implementation of the APIs. Going further, they maintain a [polyfill package] which provides a unified interface to the common extension APIs in a promisified manner. They also provide information on [porting a Chrome extension].

[webextension-differences]: https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Chrome_incompatibilities
[polyfill package]: https://github.com/mozilla/webextension-polyfill
[porting a Chrome extension]: https://developer.mozilla.org/en-US/Add-ons/WebExtensions/Porting_a_Google_Chrome_extension

Overall, their documentation is often much more in-depth compared to Chrome's, though to be fair, many pages are actually imported from Chrome's and then later augmented.

## Anatomy

The structure of a browser extension is defined by a `manifest.json` file which contains the extension's metadata, including name, description, version, permissions it requires, etc. Here is an example manifest:

``` json
{
  "name": "My Extension",
  "version": "2.1",
  "description": "Gets information from Google.",
  "icons": { "128": "icon_128.png" },
  "background": {
    "persistent": false,
    "scripts": ["bg.js"]
  },
  "permissions": ["http://*.google.com/", "https://*.google.com/"],
  "browser_action": {
    "default_title": "",
    "default_icon": "icon_19.png",
    "default_popup": "popup.html"
  }
}
```

Overall an extension may be comprised of:

* manifest
* one or more HTML files
* zero or more JavaScript files
* zero or more miscellaneous files (e.g. images)

Extensions can communicate with servers or other pages via content scripts or cross-origin XMLHttpRequests.

Extensions can add to the browser UI by way of _browser actions_ or _page actions_. An extension can have only one of them or none at all.

A _browser action_ is for actions that make sense for any page (e.g. an ad blocker), while a _page action_ is for actions that only makes sense for certain pages.

Files can be referenced within other files using relative paths. Each path is also given an absolute path following the form:

```
chrome-extension://:extensionID/:pathToFile
```

where `:extensionID` is a unique identifier produced for the extension. The `:extensionID` for an extension can be obtained for any extension at the `chrome://extensions` page. Note that this identifier may change throughout the development process until the extension is packaged---usually by uploading it with the dashboard---after which it obtains a permanent identifier. To avoid hard-coding the identifier during development, the `@@extension_id` predefined message can be used. Once a permanent identifier is obtained, occurrences of the `@@extension_id` should be changed to use the real identifier.

A _background page_, `background.html`, is an invisible page that holds the extension's main logic. A _persistent background page_ is always open. An _event page_ is one that is opened and closed as needed, in response to events.

An extensions HTML pages have complete access to each other's DOMs, and are able to invoke functions on each other. This means that most of the code can be defined in `background.html` and then invoked by other pages.

A _content script_ defines code that executes in the context of a page loaded by the browser. It can be considered as being part of the loaded page, instead of as part of the actual extension, which is known as its _parent extension_.

Since content scripts run in the context of the loaded page, they can manipulate their DOM, but _not_ the DOM of their parent extension. Instead, a content script can exchange messages with the parent extension if it needs to communicate with it. For example, a background page might send a message to the content script directing it to change the appearance of some element, or a content script might send a message to the background page informing it of information it needs that it found on the page.

## Development Workflow

Extensions can be loaded at `chrome://extensions`. Ensure that Developer Mode is enabled and use the "Load unpacked extension" button.

The manifest file is only read when the extension is loaded, so it is necessary to reload the extension in order to observe changes made to it. This is done by clicking on the "Reload" button next to the extension, or by reloading the extensions page.

## Distribution

Extensions are packaged up into a ZIP archive with a `.crx` extension.

## Testing

There isn't a clear, straightforward way to test browser extensions. [Puppeteer](https://developers.google.com/web/tools/puppeteer) can be used to some extent. With Puppeteer, an extension can be loaded, but _only_ when headless mode is turned off.

Content scripts can be tested with Puppeteer by:

* Evaluating some marker like `window.PUPPETEER = true;`
* Modifying the content script so that if this flag is true, then only expose the module for use by tests:

    ``` typescript
    if (window.PUPPETEER !== undefined) {
      window.testedModule = testedModule;
    } else {
      // Typical content script code
    }
    ```

* Injecting the content script within a script tag in the page after navigating.
* Evaluating calls to functions and then asserting their outcome