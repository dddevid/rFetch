# rFetch

Un tool di informazioni di sistema veloce e bello scritto in Rust - una versione migliorata di fastfetch.

## Caratteristiche

- 🚀 **Veloce**: Scritto in Rust per prestazioni ottimali
- 🎨 **Bello**: Loghi ASCII colorati per molte distribuzioni
- 🔧 **Configurabile**: File di configurazione TOML flessibile
- 🌍 **Cross-platform**: Supporta Linux, macOS, Windows e Termux (Android)
- 📊 **Informazioni complete**: CPU, GPU, memoria, disco, batteria e altro
- 🎯 **Modalità multiple**: Normale, minimale, verbosa e JSON
- 🌈 **Temi personalizzabili**: Sistema di temi integrato con colori e layout
- 💾 **GPU e SSD migliorati**: Rilevamento avanzato di GPU e informazioni disco dettagliate
- 📱 **Supporto Termux**: Ottimizzato per l'ambiente Android/Termux

## Installazione

### Da sorgente

```bash
git clone https://github.com/yourusername/rfetch.git
cd rfetch
cargo build --release
sudo cp target/release/rfetch /usr/local/bin/
```

### Con Cargo

```bash
cargo install rfetch
```

### Su Termux (Android)

```bash
# Installa Rust e le dipendenze necessarie
pkg update && pkg upgrade
pkg install rust git

# Clona e compila
git clone https://github.com/yourusername/rfetch.git
cd rfetch
cargo build --release

# Copia l'eseguibile
cp target/release/rfetch $PREFIX/bin/
```

## Utilizzo

### Utilizzo base

```bash
rfetch
```

### Opzioni disponibili

```bash
rfetch --help
```

- `-c, --config <FILE>`: Usa un file di configurazione personalizzato
- `-l, --logo <LOGO>`: Tipo di logo (auto, ascii, small, none)
- `--color <WHEN>`: Quando usare i colori (auto, always, never)
- `-j, --json`: Output in formato JSON
- `-m, --minimal`: Mostra informazioni minimali
- `-v, --verbose`: Mostra informazioni verbose
- `--theme <THEME>`: Usa un tema specifico
- `--list-themes`: Elenca tutti i temi disponibili

### Esempi

```bash
# Output minimale
rfetch --minimal

# Output JSON
rfetch --json

# Logo specifico
rfetch --logo small

# Senza colori
rfetch --color never

# Usa il tema neon
rfetch --theme neon

# Elenca tutti i temi
rfetch --list-themes

# Configurazione personalizzata
rfetch --config ~/.config/rfetch/custom.toml
```

## Temi

rFetch include un sistema di temi integrato con diversi temi predefiniti:

- **default**: Tema predefinito con colori bilanciati
- **minimal**: Tema pulito e minimale con informazioni essenziali
- **neon**: Colori vivaci e brillanti per un look moderno
- **retro**: Stile vintage con colori verdi

### Utilizzo dei temi

```bash
# Elenca tutti i temi disponibili
rfetch --list-themes

# Usa un tema specifico
rfetch --theme neon
rfetch --theme minimal
rfetch --theme retro
```

## Configurazione

rFetch usa un file di configurazione TOML. Il file predefinito si trova in:

- Linux/macOS: `~/.config/rfetch/config.toml`
- Windows: `%APPDATA%\rfetch\config.toml`

### Esempio di configurazione

```toml
[display]
logo_type = "auto"
color_mode = "auto"
output_format = "normal"
minimal = false
verbose = false
separator = ": "
padding = 2

[info]
show_os = true
show_kernel = true
show_uptime = true
show_packages = true
show_shell = true
show_resolution = true
show_de = true
show_wm = true
show_theme = false
show_icons = false
show_font = false
show_cursor = false
show_terminal = true
show_cpu = true
show_gpu = true
show_memory = true
show_disk = true
show_battery = true
show_locale = false
show_local_ip = false
show_public_ip = false
show_users = false
show_date = true

[colors]
title = "cyan"
subtitle = "blue"
key = "yellow"
value = "white"
separator = "white"
logo = "cyan"
```

## Informazioni supportate

- **Sistema**: OS, kernel, uptime
- **Hardware**: CPU, GPU (con core count), memoria, disco (con spazio usato/totale), batteria
- **Software**: Shell, terminal, DE, WM, pacchetti
- **Rete**: IP locale, IP pubblico
- **Altro**: Risoluzione, tema, font, utenti, data

### Miglioramenti GPU

- **macOS**: Rilevamento Apple Silicon (M1, M2, etc.) con conteggio core
- **Linux**: Supporto migliorato per GPU discrete e integrate
- **Windows**: Rilevamento GPU tramite WMI

### Miglioramenti Disco

- **Formato migliorato**: Mostra "Usato / Totale (Percentuale%)"
- **Parsing accurato**: Gestione corretta delle unità (GB, TB, etc.)
- **Multi-platform**: Supporto ottimizzato per Linux, macOS e Windows

## Sistemi operativi supportati

### Linux
- Arch Linux
- Ubuntu
- Fedora
- Debian
- Gentoo
- Manjaro
- openSUSE
- CentOS
- Alpine
- E molti altri

### Termux (Android)
- Supporto completo per Termux su Android
- Rilevamento automatico dell'ambiente Termux
- Gestione pacchetti con `pkg`
- Informazioni CPU e GPU specifiche per Android
- Logo dedicato per Termux

### macOS
- Supporto completo per macOS/Darwin
- Rilevamento Apple Silicon ottimizzato
- Informazioni disco accurate

### Windows
- Supporto base per Windows
- Rilevamento GPU tramite WMI
- Informazioni disco tramite WMI

## Sviluppo

### Prerequisiti

- Rust 1.70+
- Cargo

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Release

```bash
cargo build --release
```

## Contribuire

I contributi sono benvenuti! Per favore:

1. Fai un fork del progetto
2. Crea un branch per la tua feature (`git checkout -b feature/AmazingFeature`)
3. Committa le tue modifiche (`git commit -m 'Add some AmazingFeature'`)
4. Pusha al branch (`git push origin feature/AmazingFeature`)
5. Apri una Pull Request

## Licenza

Questo progetto è sotto licenza MIT. Vedi il file `LICENSE` per i dettagli.

## Ringraziamenti

- Ispirato da [fastfetch](https://github.com/fastfetch-cli/fastfetch)
- Loghi ASCII da varie fonti della community
- Community Rust per le eccellenti librerie

## Roadmap

- [ ] Supporto per più package manager
- [ ] Temi personalizzati definiti dall'utente
- [ ] Plugin system per informazioni aggiuntive
- [ ] Configurazione GUI
- [ ] Supporto per più formati di output
- [ ] Plugin system
- [ ] Temi predefiniti
- [ ] Supporto per immagini nei terminali
- [ ] Benchmark e ottimizzazioni
- [ ] Supporto per container
- [ ] API per integrazione con altri tool