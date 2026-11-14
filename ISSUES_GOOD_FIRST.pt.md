# Boas primeiras tarefas

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · [Français](ISSUES_GOOD_FIRST.fr.md) · [Italiano](ISSUES_GOOD_FIRST.it.md) · [Español](ISSUES_GOOD_FIRST.es.md) · **Português** · [Deutsch](ISSUES_GOOD_FIRST.de.md) · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Substituir o `main.rs` provisório por um carregador WAV (crate hound)
- [ ] Implementar a STFT com janela de Hann (rustfft) e testá-la com uma onda sinusoidal
- [ ] Implementar a STFT inversa com overlap-add e verificar a reconstrução perfeita
- [ ] Acrescentar um plugin de redução de ruído por subtração espectral
- [ ] Acrescentar um plugin simples de deteção de atividade de voz baseado em energia
- [ ] Renderizar o espectrograma para PNG (crate image) a partir da linha de comandos
- [ ] Definir a ABI dos plugins (objeto-trait ou WASM) em docs/plugins.md
- [ ] Acrescentar na CLI `--select t0-t1,f0-f1 --delete` para regiões tempo–frequência
- [ ] Configurar uma CI multiplataforma (Windows, macOS, Linux)
- [ ] Acrescentar entrada FLAC e MP3 através de symphonia

Cria-as todas no GitHub com `scripts/create_issues.sh` (requer a CLI `gh`).
