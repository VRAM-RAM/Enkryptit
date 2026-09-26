# TUI

L'interface interactive vous permet de parcourir et traiter des fichiers et dossiers sans
mémoriser les drapeaux de la ligne de commande.

> **Remarque :** les libellés de l'interface sont en anglais `Browse`, `Parameters`, …
> Ils sont expliqués ici en français.

## Lancement

```bash
eck        # sans argument -> TUI
eck ui     # commande TUI explicite
```

## Menu principal

```
Enkryptit
   Fast & Simple File Encryption Manager v0.0.3
? What do you want to do?
> Browse
  Parameters
  Help
  Exit
```

| Entrée | Description |
| ------ | ----------- |
| **Browse** | Choisir des fichiers, dossiers, ou les deux, puis les chiffrer / déchiffrer / inspecter. |
| **Parameters** | Configurer compression, type de clé et parallélisme. |
| **Help** | Affiche les commandes disponibles. |
| **Exit** | Quitte l'interface. |

## Panneau de parcours

```
Browser Panel
? What do you want to Browse?
> Browse Files
  Browse Folders
  Browse both Files & Folders
  Back to main menu
```

Après avoir choisi fichier(s)/dossier(s) via le sélecteur système, chaque sélection lance le
traitement standard (chiffrer le `.encky` / déchiffrer le contenu en clair).

> **Remarque :** la TUI demande le mot de passe de manière interactive si nécessaire.

## Panneau de paramètres

```
Parameters Panel
? What do you want to configure?
> Change compression type
  Change key type
  Change parallelism type
  Show current parameters
  Back to main menu
```

Les choix sont persistés dans le même `config.json` que la CLI (voir [Paramètres](../cli.md)).

## Liens

* [CLI](cli.md) — l'interface en ligne de commande équivalente, y compris la
  correspondance des paramètres.