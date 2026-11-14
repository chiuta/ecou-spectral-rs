# Ecou Spectral (Rust)

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0 · Ultimo aggiornamento: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Editor nativo di spettrogrammi in Rust, con plugin per riduzione del rumore e rilevamento vocale.

Trade-free: nessun tracciamento, nessuna pubblicità, nessun account, funziona offline.

## Stato

Alpha: il repository contiene la struttura, lo schema dati o i primi esempi e le guide al contributo. L’ambito qui sotto è il piano, non ciò che funziona oggi.

## Ambito previsto

- Un caricatore WAV (hound), una STFT con finestra di Hann (rustfft) testata su un’onda sinusoidale e una STFT inversa con overlap-add e ricostruzione perfetta.
- Rendering dello spettrogramma in PNG dalla riga di comando.
- Plugin: riduzione del rumore per sottrazione spettrale e un rilevatore di attività vocale basato sull’energia, con l’ABI dei plugin (trait object o WASM) specificata in `docs/plugins.md`.
- Una selezione da CLI (`--select t0-t1,f0-f1 --delete`) per regioni tempo–frequenza.
- Ingresso FLAC e MP3 tramite symphonia e CI multipiattaforma (Windows, macOS, Linux).

Nulla di tutto ciò è ancora implementato: ogni voce è un compito pronto in `ISSUES_GOOD_FIRST.md`.

## Contribuisci in 5 minuti

1. Fai il fork di questo repository.
2. Copia `data/plugins/example.json` in `data/plugins/` e modificalo.
3. Esegui `python scripts/validate.py`.
4. Apri una pull request: per i dati non serve programmare.

## Buone prime attività

Vedi `ISSUES_GOOD_FIRST.md` per attività già pronte.

## Licenza

Codice: MIT. Dati contribuiti: indica la fonte; sono pubblicati con la stessa licenza salvo diversa indicazione.

## Supporto

Se il progetto ti è utile, puoi [offrirmi un caffè](https://buymeacoffee.com/echo.of.the.strings).
