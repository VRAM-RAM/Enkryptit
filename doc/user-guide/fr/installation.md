# Installation 

Cette page explique comment compiler et installer **Enkryptit!** sur votre machine.

## Prérequis

* **Rust :** `1.85+` (le `rust-version` déclaré du workspace). L'édition `2024` est utilisée.
* Un gestionnaire de références natif pour `Type de clé = Trousseau système` :
  * **Linux :** `libsecret` (Trousseau GNOME / KSecretService).
  * **macOS :** le Trousseau système.
  * **Windows :** le Gestionnaire d'informations d'identification Windows.

## Compilation

```bash
git clone https://github.com/VRAM-RAM/Enkryptit
cd Enkryptit/

# Compilation en debug
cargo xtask build

# Compilation en release
cargo xtask build --release
```

Le binaire `eck` est produit dans `target/release/eck` (ou `target/debug/eck`).

> **Remarque :** le profil release est agressif (`lto = "fat"`, `codegen-units = 1`,
> `panic = "abort"`). La première compilation peut être *très* longue.

## Installation

```bash
cargo xtask install
```

Cette commande exécute `cargo install --path ./eck` et place `eck` dans le `$PATH`.

## Premier lancement

```bash
eck              # ouvre l'interface (TUI)
eck --help       # aide de la CLI
eck --version    # affiche la version d'Enkryptit
```

Voir [CLI](cli.md) et [TUI](tui.md) pour l'utilisation.