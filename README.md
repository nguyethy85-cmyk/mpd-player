# MPD Player

A browser-based MPEG-DASH player powered by [Shaka Player](https://github.com/shaka-project/shaka-player), with a Rust CLI for inspecting DRM metadata in MPD files.

## Web player

Open `index.html` through a local web server or publish the repository with GitHub Pages.

- **No DRM:** enter an MPD URL and play.
- **Widevine:** enter the MPD URL, the Widevine License Server URL supplied by the provider, and optional HTTP headers as a JSON object.
- **ClearKey:** enter a 16-byte KID and 16-byte key in hex, only for content you are authorized to decrypt and play.

Example Widevine headers:

```json
{
  "Authorization": "Bearer YOUR_AUTHORIZATION_TOKEN"
}
```

The player uses the browser's supported Widevine CDM. A WVD device file is not required in the browser. The MPD host and license server must permit browser requests through CORS. Do not commit tokens, cookies, WVD files, or private keys to this repository.

## Rust MPD DRM Inspector

Requirements: stable Rust toolchain.

Run:

```sh
cd tools/mpd-inspect
cargo run --release -- ../../sample.mpd
```

The command prints JSON containing each `ContentProtection` element, its scheme URI, value, default KID, PSSH Base64 string, and decoded PSSH byte length. It only inspects MPD metadata. It does not contact license servers, bypass DRM, or extract content keys.

## GitHub Pages

In repository **Settings → Pages**, choose **Deploy from a branch**, select `main` and `/(root)`, then save.

## Troubleshooting

- **Shaka network error:** verify the MPD URL, network access, and CORS headers.
- **License request error:** verify the provider's License Server URL, required headers, token expiry, and account entitlement.
- **No supported key system:** use a browser/device with Widevine support and compatible codecs.
- **Codec/manifest error:** verify that the MPD and media segments are valid DASH content supported by the browser.

Use only streams and license services you are authorized to access.
