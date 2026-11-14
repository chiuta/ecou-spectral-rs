# Jó első feladatok

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · [Français](ISSUES_GOOD_FIRST.fr.md) · [Italiano](ISSUES_GOOD_FIRST.it.md) · [Español](ISSUES_GOOD_FIRST.es.md) · [Português](ISSUES_GOOD_FIRST.pt.md) · [Deutsch](ISSUES_GOOD_FIRST.de.md) · **Magyar**

- [ ] Az ideiglenes `main.rs` lecserélése WAV-betöltőre (hound crate)
- [ ] A Hann-ablakos STFT megvalósítása (rustfft) és tesztelése szinuszhullámmal
- [ ] Az inverz STFT megvalósítása overlap-add eljárással és a tökéletes rekonstrukció ellenőrzése
- [ ] Spektrális kivonásos zajcsökkentő bővítmény hozzáadása
- [ ] Egyszerű, energiaalapú beszédaktivitás-érzékelő bővítmény hozzáadása
- [ ] A spektrogram PNG-be renderelése (image crate) parancssorból
- [ ] A bővítmények ABI-jának (trait objektum vagy WASM) meghatározása a docs/plugins.md fájlban
- [ ] CLI-ben `--select t0-t1,f0-f1 --delete` hozzáadása idő–frekvencia tartományokhoz
- [ ] Többplatformos CI beállítása (Windows, macOS, Linux)
- [ ] FLAC és MP3 bemenet hozzáadása a symphonia-val

Hozd létre mindet GitHubon a `scripts/create_issues.sh` szkripttel (a `gh` CLI szükséges).
