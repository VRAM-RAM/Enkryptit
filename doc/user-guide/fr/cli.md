# Ligne de commande

`eck` est une CLI scriptable. Sans chemin donné, `eck` ouvre la [TUI](tui.md).

## Options globales

| Option | Description |
| ------ | ----------- |
| `eck <chemin>...` | Chiffre le contenu d'un path (si non chiffré) ou déchiffre son contenu si chiffré : `.encky` (fichier ou dossier). On peut renseigner plusieurs paths. |
| `-p, --password <motdepasse>` | Fournit le mot de passe directement au lieu de le demander. |
| `eck ui` | Ouvre la TUI. |
| `eck inspect <chemin>...` | Inspecte des paths sans chiffrer/déchiffrer. |

## Commandes

| Commande | Signification |
| -------- | ------------- |
| `eck <chemin>` | Chiffre un fichier / dossier en clair, ou déchiffre un `.encky`. |
| `eck <chemin> -p <motdepasse>` | Identique, avec le mot de passe en ligne de commande. |
| `eck ui` | Ouvre l'interface interactive (TUI). |
| `eck parameters` ou `eck params` | Affiche les paramètres courants. |
| `eck params -c <algo>` | Change l'algorithme de compression. Valeurs : `zstd`, `lz4`, `xz`, `none`, `auto`. |
| `eck params -k <type>` | Change le type de clé (`params -k`, alias `--keytype`). Valeurs : `os`, `file`, `pwd`/`password`. |
| `eck params -p <mode>` | Change le mode de parallélisme. Valeurs : `single`, `multi`, `multi:<threads>`. |
| `eck inspect <chemin>` | Inspecte un fichier ou une archive sans le/la chiffrer ni déchiffrer. |

`parameters` et `params` sont équivalents ; ils acceptent tous deux `-c`, `-k`
(`--keytype`) et `-p`. La variante `parameters` expose aussi l'alias visible `kt` pour
`--keytype`. Sans drapeau, ils affichent les paramètres courants.

> **Remarque :** `eck params -k os` et `eck parameters --keytype os` changent tous deux le
> type de clé.

## Comportement

```
eck /home/user/secrets/secrets.txt      -> secrets.txt.encky
eck /home/user/secrets.txt.encky        -> secrets.txt
```

* Plusieurs paths sont traités en un seul appel :

  ```
  eck /home/user/secrets/* /home/user/secret.txt /lib/secret.bin
  eck inspect mon_dossier/*
  ```

* Un **dossier** est transformé en une seule **archive** `<dossier>.encky`
  (voir [format](format.md)).
* Le chiffrement/déchiffrement est choisi automatiquement : les fichiers `.encky` sont
  déchiffrés, tout le reste est chiffré.

> [!NOTE]
> Lorsque vous chiffrez un objet avec le mode de clef '**File**, le fichier contenant la clef est stocké dans `/HOME/private_keys`

> ![WARNING]
> Le stockage des clefs dans le **keyring** OS est instable !

## Exemples

```bash
# Chiffrer un fichier avec un mot de passe
eck secrets.txt -p 's3cret!'

# Chiffrer un dossier (produit secrets.encky)
eck secrets/

# Le déchiffrer
eck secrets.encky

# Changer la compression (persisté dans la configuration)
eck params -c zstd

# Inspecter sans déchiffrer
eck inspect secrets.txt.encky
```

## Comportement de sortie

Le succès et les échecs sont rapportés via des bannières
(voir [erreurs](errors.md)). Les erreurs de paramètres CLI sortent avec un statut non nul
et un avertissement.