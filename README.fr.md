# Ecou Spectral (Rust)

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0 · Dernière mise à jour : 2026-11-14

🌐 https://ecou-spectral-rs.chiuta.ro

> Éditeur de spectrogrammes natif en Rust, avec plugins de réduction de bruit et de détection vocale.

Trade-free : sans suivi, sans publicité, sans compte, fonctionne hors ligne.

## État

Alpha : le dépôt contient la structure, le schéma de données ou les premiers exemples et les guides de contribution. Le périmètre ci-dessous est le plan, pas ce qui fonctionne aujourd’hui.

## Périmètre prévu

- Un chargeur WAV (hound), une STFT à fenêtre de Hann (rustfft) testée sur une sinusoïde, et une STFT inverse avec overlap-add et reconstruction parfaite.
- Rendu du spectrogramme en PNG depuis la ligne de commande.
- Des plugins : réduction de bruit par soustraction spectrale et détecteur d’activité vocale fondé sur l’énergie, avec l’ABI des plugins (objet-trait ou WASM) spécifiée dans `docs/plugins.md`.
- Une sélection en ligne de commande (`--select t0-t1,f0-f1 --delete`) pour des régions temps–fréquence.
- Entrée FLAC et MP3 via symphonia, et CI multiplateforme (Windows, macOS, Linux).

Rien de tout cela n’est encore implémenté : chaque point est une tâche prête dans `ISSUES_GOOD_FIRST.md`.

## Contribuer en 5 minutes

1. Forkez ce dépôt.
2. Copiez `data/plugins/example.json` dans `data/plugins/` et modifiez-le.
3. Lancez `python scripts/validate.py`.
4. Ouvrez une pull request — aucune programmation n'est nécessaire pour les données.

## Bonnes premières tâches

Voir `ISSUES_GOOD_FIRST.md` pour des tâches toutes prêtes.

## Licence

Code : MIT. Données contribuées : indiquez la source ; elles sont publiées sous la même licence sauf mention contraire.

## Soutien

Si ce projet vous est utile, vous pouvez [m'offrir un café](https://buymeacoffee.com/echo.of.the.strings).
