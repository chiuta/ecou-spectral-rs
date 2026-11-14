# Ecou Spectral (Rust)

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

Version: 0.1.0 · Letzte Aktualisierung: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Nativer Spektrogramm-Editor in Rust mit Plugins für Rauschminderung und Spracherkennung.

Trade-free: kein Tracking, keine Werbung, keine Konten, funktioniert offline.

## Status

Alpha: Das Repository enthält die Struktur, das Datenschema oder die ersten Beispiele und die Beitragsleitfäden. Der Umfang unten ist der Plan, nicht das, was heute funktioniert.

## Geplanter Umfang

- Ein WAV-Lader (hound), eine STFT mit Hann-Fenster (rustfft), getestet an einer Sinuswelle, und eine inverse STFT mit Overlap-Add und perfekter Rekonstruktion.
- Rendern des Spektrogramms als PNG über die Kommandozeile.
- Plugins: Rauschminderung durch spektrale Subtraktion und ein energiebasierter Sprachaktivitätsdetektor, mit der Plugin-ABI (Trait-Objekt oder WASM) in `docs/plugins.md`.
- Eine Auswahl per CLI (`--select t0-t1,f0-f1 --delete`) für Zeit-Frequenz-Bereiche.
- FLAC- und MP3-Eingabe über symphonia und plattformübergreifende CI (Windows, macOS, Linux).

Nichts davon ist bisher umgesetzt: Jeder Punkt ist eine fertig vorbereitete Aufgabe in `ISSUES_GOOD_FIRST.md`.

## In 5 Minuten mitmachen

1. Forke dieses Repository.
2. Kopiere `data/plugins/example.json` nach `data/plugins/` und bearbeite die Datei.
3. Führe `python scripts/validate.py` aus.
4. Öffne einen Pull Request – für Daten ist keine Programmierung nötig.

## Gute erste Aufgaben

Siehe `ISSUES_GOOD_FIRST.md` für fertige Aufgaben.

## Lizenz

Code: MIT. Beigesteuerte Daten: Quelle angeben; sie stehen unter derselben Lizenz, sofern die Datei nichts anderes sagt.

## Unterstützung

Wenn dir das Projekt nützt, kannst du [mir einen Kaffee spendieren](https://buymeacoffee.com/echo.of.the.strings).
