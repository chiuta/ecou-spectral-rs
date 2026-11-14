# Ecou Spectral (Rust)

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0 · Ultima actualizare: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Editor nativ de spectrograme în Rust, cu pluginuri pentru reducerea zgomotului și detecția vocii.

Fără comerț (trade-free): fără urmărire, fără reclame, fără conturi, funcționează offline.

## Stare

Alfa: depozitul conține structura, schema de date sau primele exemple și ghidurile de contribuție. Domeniul de mai jos este planul, nu ceea ce funcționează azi.

## Domeniu planificat

- Un încărcător WAV (hound), un STFT cu fereastră Hann (rustfft) testat pe o undă sinusoidală și un STFT invers cu overlap-add și reconstrucție perfectă.
- Randarea spectrogramei în PNG din linia de comandă.
- Pluginuri: reducere de zgomot prin scădere spectrală și un detector de activitate vocală bazat pe energie, cu ABI-ul pluginurilor (obiect-trait sau WASM) specificat în `docs/plugins.md`.
- O selecție din CLI (`--select t0-t1,f0-f1 --delete`) pentru regiuni timp–frecvență.
- Intrare FLAC și MP3 prin symphonia și CI multiplatformă (Windows, macOS, Linux).

Nimic din asta nu este implementat încă: fiecare punct este o sarcină gata pregătită în `ISSUES_GOOD_FIRST.md`.

## Contribuie în 5 minute

1. Fă fork acestui depozit.
2. Copiază `data/plugins/example.json` în `data/plugins/` și editează-l.
3. Rulează `python scripts/validate.py`.
4. Deschide un pull request — pentru date nu e nevoie de programare.

## Sarcini bune pentru început

Vezi `ISSUES_GOOD_FIRST.md` pentru sarcini gata pregătite.

## Licență

Cod: MIT. Datele contribuite: indică sursa; sunt publicate sub aceeași licență, dacă fișierul nu spune altfel.

## Sprijin

Dacă proiectul îți este util, poți [să-mi oferi o cafea](https://buymeacoffee.com/echo.of.the.strings).
