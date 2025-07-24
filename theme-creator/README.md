# rFetch Theme Creator

Un'applicazione web moderna per creare temi personalizzati per rFetch utilizzando il nuovo Theme Definition Language (TDL).

## 🌟 Caratteristiche

- **Editor Visuale**: Interfaccia intuitiva per creare temi senza scrivere codice
- **Preview in Tempo Reale**: Vedi immediatamente come apparirà il tuo tema
- **Supporto Multi-Formato**: Esporta in JSON, YAML o TOML
- **Colori Avanzati**: Sistema di colori RGB con effetti neon e animazioni
- **ASCII Art Editor**: Crea loghi ASCII personalizzati
- **Effetti Speciali**: Glow, pulse, rainbow, typewriter e altri effetti
- **Temi Predefiniti**: Esempi di temi come Cyberpunk, Matrix e Synthwave

## 🚀 Come Usare

1. **Apri l'applicazione**: Apri `index.html` nel tuo browser
2. **Personalizza i Colori**: Usa i selettori di colore e gli input RGB
3. **Aggiungi Effetti**: Seleziona effetti come glow, bold, animazioni
4. **Modifica il Display**: Configura separatori, padding, bordi
5. **Crea ASCII Art**: Disegna il tuo logo personalizzato
6. **Preview**: Controlla il risultato nel pannello di anteprima
7. **Esporta**: Scarica il tema nel formato desiderato

## 🎨 Formati Supportati

### YAML (.yaml)
```yaml
meta:
  name: "mio-tema"
  description: "Il mio tema personalizzato"
  
colors:
  title:
    base: "cyan"
    rgb: [0, 255, 255]
    effects: ["bold", "glow"]
```

### JSON (.json)
```json
{
  "meta": {
    "name": "mio-tema",
    "description": "Il mio tema personalizzato"
  },
  "colors": {
    "title": {
      "base": "cyan",
      "rgb": [0, 255, 255],
      "effects": ["bold", "glow"]
    }
  }
}
```

### TOML (.toml)
```toml
[meta]
name = "mio-tema"
description = "Il mio tema personalizzato"

[colors.title]
base = "cyan"
rgb = [0, 255, 255]
effects = ["bold", "glow"]
```

## 🎯 Sezioni del Theme Creator

### 1. Metadati del Tema
- Nome del tema
- Descrizione
- Versione
- Autore

### 2. Controlli Colore
Per ogni elemento (title, subtitle, key, value, separator, logo, accent):
- **Selettore Colore**: Picker visuale
- **Nome Colore**: Nome del colore base
- **RGB**: Valori RGB personalizzati
- **Effetti**: Bold, italic, underline, glow, shadow, blink
- **Animazioni**: Pulse, fade, rainbow, typewriter, slide, bounce

### 3. Impostazioni Display
- **Tipo Logo**: ASCII, small, auto
- **Separatore**: Carattere tra chiave e valore
- **Padding**: Spaziatura interna
- **Bordi**: Mostra/nascondi bordi
- **Barra Colori**: Stile della barra colorata
- **Allineamento**: Left, center, right
- **Spaziatura Righe**: Distanza tra le righe
- **Icone**: Mostra/nascondi icone

### 4. Editor ASCII Art
- Editor di testo per creare loghi ASCII personalizzati
- Logo piccolo per modalità compatta
- Decorazioni per bordi e angoli

### 5. Effetti Speciali
- **Transizioni**: Animazioni fluide
- **Ombre**: Effetti di profondità
- **Intensità Glow**: Controllo dell'intensità del bagliore
- **Effetti Particelle**: Animazioni di particelle

### 6. Preview in Tempo Reale
- Anteprima istantanea del tema
- Dati di esempio del sistema
- Visualizzazione di tutti gli elementi stilizzati

### 7. Export e Download
- Selezione formato (JSON/YAML/TOML)
- Anteprima del codice generato
- Download diretto del file tema
- Copia negli appunti

## 🎨 Temi di Esempio

### Cyberpunk
- Colori neon magenta e cyan
- Effetti glow intensi
- ASCII art futuristico
- Animazioni pulse e rainbow

### Matrix
- Verde brillante monocromatico
- Effetti typewriter
- Stile console digitale
- Bordi a blocchi

### Synthwave
- Palette anni '80
- Magenta, cyan e giallo
- Estetica retro-futuristica
- Gradienti synthwave

## 🛠️ Installazione e Uso con rFetch

1. **Crea il tuo tema** con il Theme Creator
2. **Scarica il file** nel formato desiderato
3. **Salva il file** nella cartella dei temi di rFetch
4. **Usa il tema** con rFetch:

```bash
# Carica tema personalizzato
rfetch --theme /path/to/mio-tema.yaml

# Genera template per iniziare
rfetch --generate-template yaml > nuovo-tema.yaml
```

## 🌈 Effetti Disponibili

### Effetti Colore
- **Bold**: Testo in grassetto
- **Italic**: Testo in corsivo
- **Underline**: Testo sottolineato
- **Glow**: Effetto bagliore neon
- **Shadow**: Ombra del testo
- **Blink**: Testo lampeggiante

### Animazioni
- **Pulse**: Pulsazione del colore
- **Fade**: Dissolvenza in/out
- **Rainbow**: Ciclo di colori arcobaleno
- **Typewriter**: Effetto macchina da scrivere
- **Slide**: Scorrimento del testo
- **Bounce**: Rimbalzo del testo

## 🎯 Tips per Creare Temi Fantastici

1. **Usa Contrasti**: Assicurati che i colori siano leggibili
2. **Non Esagerare**: Troppi effetti possono essere distraenti
3. **Testa su Diversi Terminali**: Alcuni effetti potrebbero non funzionare ovunque
4. **Ispirati**: Guarda i temi di esempio per idee
5. **Condividi**: Condividi i tuoi temi con la community

## 🔧 Tecnologie Utilizzate

- **HTML5**: Struttura dell'applicazione
- **CSS3**: Stili e animazioni neon
- **JavaScript ES6+**: Logica dell'applicazione
- **Theme Definition Language (TDL)**: Linguaggio per definire i temi

## 📝 Licenza

MIT License - Sentiti libero di usare, modificare e distribuire!

---

**Creato con ❤️ per la community rFetch**