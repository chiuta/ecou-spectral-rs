# Bonnes premières tâches

[English](ISSUES_GOOD_FIRST.md) · [Română](ISSUES_GOOD_FIRST.ro.md) · **Français** · [Italiano](ISSUES_GOOD_FIRST.it.md) · [Español](ISSUES_GOOD_FIRST.es.md) · [Português](ISSUES_GOOD_FIRST.pt.md) · [Deutsch](ISSUES_GOOD_FIRST.de.md) · [Magyar](ISSUES_GOOD_FIRST.hu.md)

- [ ] Remplacer le `main.rs` provisoire par un chargeur WAV (crate hound)
- [ ] Implémenter la STFT avec fenêtre de Hann (rustfft) et la tester sur une sinusoïde
- [ ] Implémenter la STFT inverse avec overlap-add et vérifier la reconstruction parfaite
- [ ] Ajouter un plugin de réduction de bruit par soustraction spectrale
- [ ] Ajouter un plugin simple de détection d’activité vocale fondé sur l’énergie
- [ ] Rendre le spectrogramme en PNG (crate image) depuis la ligne de commande
- [ ] Définir l’ABI des plugins (objet-trait ou WASM) dans docs/plugins.md
- [ ] Ajouter en CLI `--select t0-t1,f0-f1 --delete` pour des régions temps–fréquence
- [ ] Mettre en place une CI multiplateforme (Windows, macOS, Linux)
- [ ] Ajouter l’entrée FLAC et MP3 via symphonia

Créez-les toutes sur GitHub avec `scripts/create_issues.sh` (nécessite la CLI `gh`).
