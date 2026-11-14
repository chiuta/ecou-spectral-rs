# Ecou Spectral (Rust)

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0 · Last updated: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> A native Rust spectrogram editor with plugins for noise reduction and voice detection.

Trade-free: no tracking, no ads, no accounts, works offline.

## Status

Alpha: the repository holds the structure, the data schema or the first examples and the contribution guides. The scope below is the plan, not what works today.

## Planned scope

- A WAV loader (hound), an STFT with a Hann window (rustfft) tested against a sine wave, and an inverse STFT with overlap-add and perfect reconstruction.
- Spectrogram rendering to PNG from the CLI.
- Plugins: spectral-subtraction denoising and an energy-based voice-activity detector, with the plugin ABI (trait object or WASM) specified in `docs/plugins.md`.
- A CLI selection (`--select t0-t1,f0-f1 --delete`) for time–frequency regions.
- FLAC and MP3 input via symphonia, and cross-platform CI (Windows, macOS, Linux).

None of this is implemented yet: each item is a ready-made task in `ISSUES_GOOD_FIRST.md`.

## Contribute in 5 minutes

1. Fork this repository.
2. Copy `data/plugins/example.json` into `data/plugins/` and edit it.
3. Run `python scripts/validate.py`.
4. Open a pull request — data contributions need no coding.

## Good first issues

See `ISSUES_GOOD_FIRST.md` for ready-made tasks.

## License

Code: MIT. Contributed data: state its source; it is released under the same licence unless the file says otherwise.

## Support

If this project is useful, you can [buy me a coffee](https://buymeacoffee.com/echo.of.the.strings).

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

Add yourself with `@all-contributors please add @you for data` in your pull request.
