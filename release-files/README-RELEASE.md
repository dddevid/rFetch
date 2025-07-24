# rFetch Release Files

Questo pacchetto contiene gli eseguibili precompilati di rFetch per diverse piattaforme.

## File inclusi

- `rfetch-macos-linux`: Eseguibile per macOS e Linux (x86_64)
- `rfetch-windows.exe`: Eseguibile per Windows (x86_64)

## Come usare

### macOS/Linux

```bash
# Rendi eseguibile il file
chmod +x rfetch-macos-linux

# Esegui direttamente
./rfetch-macos-linux

# Oppure copia in una directory nel PATH
sudo cp rfetch-macos-linux /usr/local/bin/rfetch
rfetch
```

### Windows

```cmd
# Esegui direttamente
rfetch-windows.exe

# Oppure rinomina e aggiungi al PATH
ren rfetch-windows.exe rfetch.exe
# Aggiungi la directory al PATH di Windows
rfetch
```

## Opzioni disponibili

```bash
# Mostra l'aiuto
rfetch --help

# Output minimale
rfetch --minimal

# Output JSON
rfetch --json

# Lista temi disponibili
rfetch --list-themes

# Usa un tema specifico
rfetch --theme neon
```

## Configurazione

I file di configurazione sono salvati in:
- **Linux/macOS**: `~/.config/rfetch/config.toml`
- **Windows**: `%APPDATA%\rfetch\config.toml`

## Supporto

Per problemi, bug report o richieste di funzionalità, visita:
https://github.com/dddevid/rFetch

## Versione

Questi eseguibili sono stati compilati dalla versione più recente del codice sorgente.