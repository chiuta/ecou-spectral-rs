# Ecou Spectral (Rust)

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0 · Última atualização: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Editor nativo de espectrogramas em Rust, com plugins de redução de ruído e deteção de voz.

Trade-free: sem rastreamento, sem anúncios, sem contas, funciona offline.

## Estado

Alfa: o repositório contém a estrutura, o esquema de dados ou os primeiros exemplos e os guias de contribuição. O âmbito abaixo é o plano, não o que funciona hoje.

## Âmbito previsto

- Um carregador WAV (hound), uma STFT com janela de Hann (rustfft) testada com uma onda sinusoidal e uma STFT inversa com overlap-add e reconstrução perfeita.
- Renderização do espectrograma para PNG a partir da linha de comandos.
- Plugins: redução de ruído por subtração espectral e um detetor de atividade de voz baseado em energia, com a ABI dos plugins (objeto-trait ou WASM) especificada em `docs/plugins.md`.
- Uma seleção a partir da CLI (`--select t0-t1,f0-f1 --delete`) para regiões tempo–frequência.
- Entrada FLAC e MP3 através de symphonia e CI multiplataforma (Windows, macOS, Linux).

Nada disto está ainda implementado: cada ponto é uma tarefa preparada em `ISSUES_GOOD_FIRST.md`.

## Contribua em 5 minutos

1. Faça um fork deste repositório.
2. Copie `data/plugins/example.json` para `data/plugins/` e edite-o.
3. Execute `python scripts/validate.py`.
4. Abra um pull request — para dados não é preciso programar.

## Boas primeiras tarefas

Veja `ISSUES_GOOD_FIRST.md` para tarefas prontas.

## Licença

Código: MIT. Dados contribuídos: indique a fonte; são publicados sob a mesma licença, salvo indicação em contrário.

## Apoio

Se o projeto lhe for útil, pode [oferecer-me um café](https://buymeacoffee.com/echo.of.the.strings).
