# Ecou Spectral (Rust)

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0 · Última actualización: 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Editor nativo de espectrogramas en Rust, con plugins de reducción de ruido y detección de voz.

Trade-free: sin rastreo, sin anuncios, sin cuentas, funciona sin conexión.

## Estado

Alfa: el repositorio contiene la estructura, el esquema de datos o los primeros ejemplos y las guías de contribución. El alcance de abajo es el plan, no lo que funciona hoy.

## Alcance previsto

- Un cargador WAV (hound), una STFT con ventana de Hann (rustfft) probada con una onda sinusoidal y una STFT inversa con overlap-add y reconstrucción perfecta.
- Renderizado del espectrograma a PNG desde la línea de comandos.
- Plugins: reducción de ruido por sustracción espectral y un detector de actividad de voz basado en energía, con la ABI de los plugins (objeto-trait o WASM) especificada en `docs/plugins.md`.
- Una selección desde la CLI (`--select t0-t1,f0-f1 --delete`) para regiones tiempo–frecuencia.
- Entrada FLAC y MP3 mediante symphonia y CI multiplataforma (Windows, macOS, Linux).

Nada de esto está implementado todavía: cada punto es una tarea preparada en `ISSUES_GOOD_FIRST.md`.

## Contribuye en 5 minutos

1. Haz un fork de este repositorio.
2. Copia `data/plugins/example.json` en `data/plugins/` y edítalo.
3. Ejecuta `python scripts/validate.py`.
4. Abre un pull request: para los datos no hace falta programar.

## Buenas primeras tareas

Consulta `ISSUES_GOOD_FIRST.md` para tareas ya preparadas.

## Licencia

Código: MIT. Datos aportados: indica la fuente; se publican con la misma licencia salvo que el archivo diga otra cosa.

## Apoyo

Si el proyecto te resulta útil, puedes [invitarme a un café](https://buymeacoffee.com/echo.of.the.strings).
