# Gute erste Aufgaben

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · [Français](ISSUES_GOOD_FIRST.fr.md) · [Italiano](ISSUES_GOOD_FIRST.it.md) · [Español](ISSUES_GOOD_FIRST.es.md) · [Português](ISSUES_GOOD_FIRST.pt.md) · **Deutsch** · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Die vorläufige `main.rs` durch einen WAV-Lader ersetzen (Crate hound)
- [ ] Die STFT mit Hann-Fenster (rustfft) implementieren und an einer Sinuswelle testen
- [ ] Die inverse STFT mit Overlap-Add implementieren und die perfekte Rekonstruktion prüfen
- [ ] Ein Plugin zur Rauschminderung durch spektrale Subtraktion hinzufügen
- [ ] Ein einfaches energiebasiertes Plugin zur Sprachaktivitätserkennung hinzufügen
- [ ] Das Spektrogramm per CLI als PNG rendern (Crate image)
- [ ] Die Plugin-ABI (Trait-Objekt oder WASM) in docs/plugins.md definieren
- [ ] In der CLI `--select t0-t1,f0-f1 --delete` für Zeit-Frequenz-Bereiche hinzufügen
- [ ] Eine plattformübergreifende CI einrichten (Windows, macOS, Linux)
- [ ] FLAC- und MP3-Eingabe über symphonia hinzufügen

Lege sie alle auf GitHub mit `scripts/create_issues.sh` an (benötigt die `gh`-CLI).
