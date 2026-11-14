# Sarcini bune pentru început

[English](ISSUES_GOOD_FIRST.md) · **Română** · [Français](ISSUES_GOOD_FIRST.fr.md) · [Italiano](ISSUES_GOOD_FIRST.it.md) · [Español](ISSUES_GOOD_FIRST.es.md) · [Português](ISSUES_GOOD_FIRST.pt.md) · [Deutsch](ISSUES_GOOD_FIRST.de.md) · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Înlocuiește `main.rs` provizoriu cu un încărcător WAV (crate-ul hound)
- [ ] Implementează STFT cu fereastră Hann (rustfft) și testeaz-o unitar pe o undă sinusoidală
- [ ] Implementează STFT inversă cu overlap-add și verifică reconstrucția perfectă
- [ ] Adaugă un plugin de reducere a zgomotului prin scădere spectrală
- [ ] Adaugă un plugin simplu de detecție a activității vocale bazat pe energie
- [ ] Randează spectrograma în PNG (crate-ul image) din linia de comandă
- [ ] Definește ABI-ul pluginurilor (obiect-trait sau WASM) în docs/plugins.md
- [ ] Adaugă în CLI `--select t0-t1,f0-f1 --delete` pentru regiuni timp–frecvență
- [ ] Configurează CI multiplatformă (Windows, macOS, Linux)
- [ ] Adaugă intrare FLAC și MP3 prin symphonia

Creează-le pe toate pe GitHub cu `scripts/create_issues.sh` (cere CLI-ul `gh`).
