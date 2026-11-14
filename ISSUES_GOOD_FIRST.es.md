# Buenas primeras tareas

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · [Français](ISSUES_GOOD_FIRST.fr.md) · [Italiano](ISSUES_GOOD_FIRST.it.md) · **Español** · [Português](ISSUES_GOOD_FIRST.pt.md) · [Deutsch](ISSUES_GOOD_FIRST.de.md) · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Sustituir el `main.rs` provisional por un cargador WAV (crate hound)
- [ ] Implementar la STFT con ventana de Hann (rustfft) y probarla con una onda sinusoidal
- [ ] Implementar la STFT inversa con overlap-add y comprobar la reconstrucción perfecta
- [ ] Añadir un plugin de reducción de ruido por sustracción espectral
- [ ] Añadir un plugin sencillo de detección de actividad de voz basado en energía
- [ ] Renderizar el espectrograma a PNG (crate image) desde la línea de comandos
- [ ] Definir la ABI de los plugins (objeto-trait o WASM) en docs/plugins.md
- [ ] Añadir en la CLI `--select t0-t1,f0-f1 --delete` para regiones tiempo–frecuencia
- [ ] Configurar una CI multiplataforma (Windows, macOS, Linux)
- [ ] Añadir entrada FLAC y MP3 mediante symphonia

Créalas todas en GitHub con `scripts/create_issues.sh` (requiere la CLI `gh`).
