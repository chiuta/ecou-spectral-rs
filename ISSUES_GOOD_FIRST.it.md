# Buoni primi compiti

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · [Français](ISSUES_GOOD_FIRST.fr.md) · **Italiano** · [Español](ISSUES_GOOD_FIRST.es.md) · [Português](ISSUES_GOOD_FIRST.pt.md) · [Deutsch](ISSUES_GOOD_FIRST.de.md) · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Sostituire il `main.rs` provvisorio con un caricatore WAV (crate hound)
- [ ] Implementare la STFT con finestra di Hann (rustfft) e testarla su un’onda sinusoidale
- [ ] Implementare la STFT inversa con overlap-add e verificare la ricostruzione perfetta
- [ ] Aggiungere un plugin di riduzione del rumore per sottrazione spettrale
- [ ] Aggiungere un semplice plugin di rilevamento dell’attività vocale basato sull’energia
- [ ] Renderizzare lo spettrogramma in PNG (crate image) dalla riga di comando
- [ ] Definire l’ABI dei plugin (trait object o WASM) in docs/plugins.md
- [ ] Aggiungere da CLI `--select t0-t1,f0-f1 --delete` per regioni tempo–frequenza
- [ ] Configurare una CI multipiattaforma (Windows, macOS, Linux)
- [ ] Aggiungere l’ingresso FLAC e MP3 tramite symphonia

Crea tutte le issue su GitHub con `scripts/create_issues.sh` (richiede la CLI `gh`).
