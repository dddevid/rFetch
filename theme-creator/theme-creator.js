// rFetch Theme Creator - Enhanced Version with Dark/Light Mode
class ThemeCreator {
    constructor() {
        this.currentFormat = 'yaml';
        this.activeTab = 'meta';
        this.colors = {
            title: { base: 'cyan', rgb: [0, 255, 255], effects: ['bold'], animation: null },
            subtitle: { base: 'blue', rgb: [0, 0, 255], effects: [], animation: null },
            key: { base: 'yellow', rgb: [255, 255, 0], effects: ['bold'], animation: null },
            value: { base: 'white', rgb: [255, 255, 255], effects: [], animation: null },
            separator: { base: 'white', rgb: [255, 255, 255], effects: [], animation: null },
            logo: { base: 'cyan', rgb: [0, 255, 255], effects: ['glow'], animation: null },
            accent: { base: 'magenta', rgb: [255, 0, 255], effects: [], animation: null }
        };
        
        this.sampleData = {
            os: 'Arch Linux',
            kernel: 'Linux 6.6.8-arch1-1',
            uptime: '2 days, 14 hours, 32 minutes',
            packages: '1,247 (pacman)',
            shell: 'zsh 5.9',
            terminal: 'Alacritty',
            cpu: 'AMD Ryzen 7 5800X (16) @ 3.8GHz',
            gpu: 'NVIDIA GeForce RTX 3070',
            memory: '8.2GB / 32.0GB (26%)',
            disk: '256GB / 1TB (25%)',
            battery: 'N/A',
            date: '2024-01-15 14:30:25'
        };

        this.init();
    }

    init() {
        this.setupThemeToggle();
        this.setupTabNavigation();
        this.createColorControls();
        this.setupEventListeners();
        this.generateTheme();
        this.updatePreview();
        
        // Auto-update preview when any input changes
        this.setupAutoUpdate();
    }

    setupThemeToggle() {
        const themeToggle = document.getElementById('themeToggle');
        const themeIcon = themeToggle.querySelector('.theme-icon');
        
        // Detect system preference
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        
        // Initialize theme
        const savedTheme = localStorage.getItem('theme');
        const initialTheme = savedTheme || (prefersDark ? 'dark' : 'light');
        this.setTheme(initialTheme);
        
        // Theme toggle event
        themeToggle.addEventListener('click', () => {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            this.setTheme(newTheme);
        });
        
        // Listen for system theme changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
            if (!localStorage.getItem('theme')) {
                this.setTheme(e.matches ? 'dark' : 'light');
            }
        });
    }

    setTheme(theme) {
        const themeIcon = document.querySelector('.theme-icon');
        
        if (theme === 'dark') {
            document.documentElement.setAttribute('data-theme', 'dark');
            themeIcon.textContent = '☀️';
        } else {
            document.documentElement.removeAttribute('data-theme');
            themeIcon.textContent = '🌙';
        }
        
        localStorage.setItem('theme', theme);
    }

    setupTabNavigation() {
        const navItems = document.querySelectorAll('.nav-item');
        const tabContents = document.querySelectorAll('.tab-content');

        navItems.forEach(item => {
            item.addEventListener('click', () => {
                const tabId = item.dataset.tab;
                
                // Update active nav item
                navItems.forEach(nav => nav.classList.remove('active'));
                item.classList.add('active');
                
                // Update active tab content
                tabContents.forEach(tab => tab.classList.remove('active'));
                document.getElementById(`${tabId}-tab`).classList.add('active');
                
                this.activeTab = tabId;
            });
        });
    }

    createColorControls() {
        const container = document.getElementById('colorControls');
        const colorNames = Object.keys(this.colors);
        
        colorNames.forEach(colorName => {
            const colorDiv = document.createElement('div');
            colorDiv.className = 'color-control';
            colorDiv.innerHTML = `
                <input type="color" class="color-input" id="${colorName}Color" 
                       value="${this.rgbToHex(this.colors[colorName].rgb)}">
                <div class="color-info">
                    <div class="color-name">${this.capitalizeFirst(colorName)}</div>
                    <div class="color-value">${this.rgbToHex(this.colors[colorName].rgb)}</div>
                    <input type="text" id="${colorName}Base" value="${this.colors[colorName].base}" 
                           placeholder="Color name" style="margin-top: 0.5rem; font-size: 0.75rem; padding: 0.5rem;">
                </div>
            `;
            container.appendChild(colorDiv);
        });
    }

    setupEventListeners() {
        // Format selector
        document.querySelectorAll('.format-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                document.querySelectorAll('.format-btn').forEach(b => b.classList.remove('active'));
                e.target.classList.add('active');
                this.currentFormat = e.target.dataset.format;
                this.generateTheme();
            });
        });

        // Color inputs
        Object.keys(this.colors).forEach(colorName => {
            // Color picker
            const colorPicker = document.getElementById(`${colorName}Color`);
            if (colorPicker) {
                colorPicker.addEventListener('input', (e) => {
                    const rgb = this.hexToRgb(e.target.value);
                    this.colors[colorName].rgb = [rgb.r, rgb.g, rgb.b];
                    
                    // Update color value display
                    const colorValue = colorPicker.parentElement.querySelector('.color-value');
                    if (colorValue) {
                        colorValue.textContent = e.target.value;
                    }
                    
                    this.updatePreview();
                    this.generateTheme();
                });
            }

            // Base color name
            const baseInput = document.getElementById(`${colorName}Base`);
            if (baseInput) {
                baseInput.addEventListener('input', (e) => {
                    this.colors[colorName].base = e.target.value;
                    this.updatePreview();
                    this.generateTheme();
                });
            }
        });

        // Range inputs
        document.querySelectorAll('input[type="range"]').forEach(input => {
            input.addEventListener('input', () => {
                const valueSpan = input.nextElementSibling;
                if (valueSpan && valueSpan.classList.contains('range-value')) {
                    valueSpan.textContent = input.value;
                }
                this.updatePreview();
            });
        });

        // Checkboxes
        document.querySelectorAll('input[type="checkbox"]').forEach(input => {
            input.addEventListener('change', () => {
                this.updatePreview();
            });
        });

        // Text inputs
        document.querySelectorAll('input[type="text"], textarea').forEach(input => {
            input.addEventListener('input', () => {
                this.updatePreview();
            });
        });

        // Select elements
        document.querySelectorAll('select').forEach(select => {
            select.addEventListener('change', () => {
                this.updatePreview();
            });
        });

        // Effect checkboxes with specific handling
        document.querySelectorAll('[data-effect]').forEach(checkbox => {
            checkbox.addEventListener('change', () => {
                this.updatePreview();
            });
        });

        // Glow intensity slider
        const glowSlider = document.getElementById('glowIntensity');
        if (glowSlider) {
            glowSlider.addEventListener('input', () => {
                const valueSpan = glowSlider.nextElementSibling;
                if (valueSpan) {
                    valueSpan.textContent = glowSlider.value;
                }
                this.updatePreview();
            });
        }
    }

    setupAutoUpdate() {
        // Auto-update for all form inputs
        const inputs = document.querySelectorAll('input, select, textarea');
        inputs.forEach(input => {
            const events = input.type === 'checkbox' ? ['change'] : ['input', 'change'];
            events.forEach(event => {
                input.addEventListener(event, () => {
                    this.updatePreview();
                    this.generateTheme();
                });
            });
        });
    }

    updatePreview() {
        const previewContent = document.getElementById('previewContent');
        const logoType = document.getElementById('logoType')?.value || 'auto';
        const separator = document.getElementById('separator')?.value || ': ';
        const showBorders = document.getElementById('showBorders')?.checked || false;
        const showColorBar = document.getElementById('showColorBar')?.checked || false;
        const customAscii = document.getElementById('customAscii')?.value || '';
        const smallLogo = document.getElementById('smallLogo')?.value || '🎨';

        // Terminal prompt
        const prompt = `<div class="terminal-prompt">rfetch</div>`;

        let logoHtml = '';
        if (logoType === 'ascii' && customAscii.trim()) {
            logoHtml = this.formatAsciiArt(customAscii, 'logo');
        } else if (logoType === 'small') {
            logoHtml = `<div style="text-align: center; font-size: 2rem; margin-bottom: 1rem; color: ${this.getColorStyle('logo')};">${smallLogo}</div>`;
        } else if (logoType === 'auto') {
            // Default rFetch logo
            const rFetchLogo = `  ██████╗ ███████╗███████╗████████╗ ██████╗██╗  ██╗
  ██╔══██╗██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
  ██████╔╝█████╗  █████╗     ██║   ██║     ███████║
  ██╔══██╗██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
  ██║  ██║██║     ███████╗   ██║   ╚██████╗██║  ██║
  ╚═╝  ╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝`;
            logoHtml = this.formatAsciiArt(rFetchLogo, 'logo');
        }

        let borderTop = '';
        let borderBottom = '';
        if (showBorders) {
            const borderChar = '─'.repeat(50);
            borderTop = `<div style="color: ${this.getColorStyle('accent')}; margin-bottom: 0.5rem;">${borderChar}</div>`;
            borderBottom = `<div style="color: ${this.getColorStyle('accent')}; margin-top: 0.5rem;">${borderChar}</div>`;
        }

        // Create info lines with proper terminal formatting
        const infoLines = Object.entries(this.sampleData).map(([key, value]) => {
            const keyStyled = this.applyColorAndEffects(this.capitalizeFirst(key), 'key');
            const separatorStyled = this.applyColorAndEffects(separator, 'separator');
            const valueStyled = this.applyColorAndEffects(value, 'value');
            
            return `${keyStyled}${separatorStyled}${valueStyled}`;
        }).join('\n');

        let colorBarHtml = '';
        if (showColorBar) {
            const colors = ['#000000', '#800000', '#008000', '#808000', '#000080', '#800080', '#008080', '#c0c0c0',
                          '#808080', '#ff0000', '#00ff00', '#ffff00', '#0000ff', '#ff00ff', '#00ffff', '#ffffff'];
            const colorBlocks = colors.map(color => `<span style="background: ${color}; color: ${color};">██</span>`).join('');
            colorBarHtml = `\n\n${colorBlocks}`;
        }

        // Simulate terminal output with proper spacing and cursor
        const terminalOutput = `${logoHtml ? logoHtml + '\n' : ''}${borderTop}${infoLines}${borderBottom}${colorBarHtml}`;
        
        previewContent.innerHTML = `
            <div class="terminal-content">
                ${prompt}
                <pre style="margin: 0; font-family: inherit; white-space: pre-wrap;">${terminalOutput}</pre>
                <span class="terminal-cursor"></span>
            </div>
        `;
    }

    formatAsciiArt(ascii, colorType) {
        const colorStyle = this.getColorStyle(colorType);
        return ascii.split('\n').map(line => 
            `<span style="color: ${colorStyle};">${line || ' '}</span>`
        ).join('\n');
    }

    applyColorAndEffects(text, colorType) {
        const color = this.colors[colorType];
        let style = `color: ${this.getColorStyle(colorType)};`;
        
        if (color.effects.includes('bold')) {
            style += ' font-weight: bold;';
        }
        
        if (color.effects.includes('italic')) {
            style += ' font-style: italic;';
        }
        
        if (color.effects.includes('underline')) {
            style += ' text-decoration: underline;';
        }
        
        if (color.effects.includes('glow')) {
            const glowIntensity = document.getElementById('glowIntensity')?.value || 0;
            if (glowIntensity > 0) {
                const glowColor = this.getColorStyle(colorType);
                style += ` text-shadow: 0 0 ${glowIntensity * 5}px ${glowColor}, 0 0 ${glowIntensity * 10}px ${glowColor};`;
            }
        }
        
        // Add transitions if enabled
        const enableTransitions = document.getElementById('enableTransitions')?.checked || false;
        if (enableTransitions) {
            style += ' transition: all 0.3s ease;';
        }
        
        return `<span style="${style}">${text}</span>`;
    }

    getColorStyle(colorType) {
        const color = this.colors[colorType];
        return `rgb(${color.rgb.join(', ')})`;
    }

    generateTheme() {
        const exportArea = document.getElementById('exportArea');
        const themeData = this.getThemeData();
        
        let output = '';
        switch (this.currentFormat) {
            case 'yaml':
                output = this.toYAML(themeData);
                break;
            case 'json':
                output = JSON.stringify(themeData, null, 2);
                break;
            case 'toml':
                output = this.toTOML(themeData);
                break;
        }
        
        exportArea.textContent = output;
    }

    getThemeData() {
        return {
            meta: {
                name: document.getElementById('themeName')?.value || 'my_theme',
                description: document.getElementById('themeDescription')?.value || 'My custom theme',
                author: document.getElementById('themeAuthor')?.value || 'Your Name',
                version: document.getElementById('themeVersion')?.value || '1.0.0'
            },
            colors: Object.fromEntries(
                Object.entries(this.colors).map(([key, value]) => [
                    key,
                    {
                        base: value.base,
                        rgb: value.rgb,
                        effects: value.effects,
                        animation: value.animation
                    }
                ])
            ),
            display: {
                logo_type: document.getElementById('logoType')?.value || 'auto',
                separator: document.getElementById('separator')?.value || ': ',
                padding: parseInt(document.getElementById('padding')?.value) || 2,
                alignment: document.getElementById('alignment')?.value || 'left',
                show_borders: document.getElementById('showBorders')?.checked || false,
                show_color_bar: document.getElementById('showColorBar')?.checked || false,
                show_icons: document.getElementById('showIcons')?.checked || false
            },
            ascii: {
                custom_logo: document.getElementById('customAscii')?.value || '',
                small_logo: document.getElementById('smallLogo')?.value || '🎨'
            },
            effects: {
                transitions: document.getElementById('enableTransitions')?.checked || false,
                shadows: document.getElementById('enableShadows')?.checked || false,
                particles: document.getElementById('enableParticles')?.checked || false,
                glow_intensity: parseFloat(document.getElementById('glowIntensity')?.value) || 0
            }
        };
    }

    toYAML(obj, indent = 0) {
        const spaces = '  '.repeat(indent);
        let yaml = '';
        
        for (const [key, value] of Object.entries(obj)) {
            if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
                yaml += `${spaces}${key}:\n${this.toYAML(value, indent + 1)}`;
            } else if (Array.isArray(value)) {
                yaml += `${spaces}${key}:\n`;
                value.forEach(item => {
                    yaml += `${spaces}  - ${item}\n`;
                });
            } else {
                const valueStr = typeof value === 'string' ? `"${value}"` : value;
                yaml += `${spaces}${key}: ${valueStr}\n`;
            }
        }
        
        return yaml;
    }

    toTOML(obj) {
        let toml = '';
        
        // Meta section
        toml += '[meta]\n';
        for (const [key, value] of Object.entries(obj.meta)) {
            toml += `${key} = "${value}"\n`;
        }
        toml += '\n';
        
        // Colors section
        for (const [colorName, colorData] of Object.entries(obj.colors)) {
            toml += `[colors.${colorName}]\n`;
            toml += `base = "${colorData.base}"\n`;
            toml += `rgb = [${colorData.rgb.join(', ')}]\n`;
            toml += `effects = [${colorData.effects.map(e => `"${e}"`).join(', ')}]\n`;
            toml += `animation = ${colorData.animation || 'null'}\n\n`;
        }
        
        // Display section
        toml += '[display]\n';
        for (const [key, value] of Object.entries(obj.display)) {
            if (typeof value === 'string') {
                toml += `${key} = "${value}"\n`;
            } else {
                toml += `${key} = ${value}\n`;
            }
        }
        toml += '\n';
        
        // ASCII section
        toml += '[ascii]\n';
        toml += `custom_logo = """${obj.ascii.custom_logo}"""\n`;
        toml += `small_logo = "${obj.ascii.small_logo}"\n\n`;
        
        // Effects section
        toml += '[effects]\n';
        for (const [key, value] of Object.entries(obj.effects)) {
            toml += `${key} = ${value}\n`;
        }
        
        return toml;
    }

    // Utility functions
    capitalizeFirst(str) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }

    rgbToHex(rgb) {
        return '#' + rgb.map(x => {
            const hex = x.toString(16);
            return hex.length === 1 ? '0' + hex : hex;
        }).join('');
    }

    hexToRgb(hex) {
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : null;
    }

    showToast(message, type = 'success') {
        const toast = document.getElementById('toast');
        toast.textContent = message;
        toast.className = `toast ${type}`;
        toast.classList.add('show');
        
        setTimeout(() => {
            toast.classList.remove('show');
        }, 3000);
    }
}

// Global functions for buttons
function generateTheme() {
    window.themeCreator.generateTheme();
    window.themeCreator.showToast('Theme updated successfully!');
}

function updatePreview() {
    window.themeCreator.updatePreview();
    window.themeCreator.showToast('Preview updated!');
}

function downloadTheme() {
    const themeData = window.themeCreator.getThemeData();
    const format = window.themeCreator.currentFormat;
    const filename = `${themeData.meta.name}.${format}`;
    
    let content = '';
    switch (format) {
        case 'yaml':
            content = window.themeCreator.toYAML(themeData);
            break;
        case 'json':
            content = JSON.stringify(themeData, null, 2);
            break;
        case 'toml':
            content = window.themeCreator.toTOML(themeData);
            break;
    }
    
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    window.themeCreator.showToast(`Theme downloaded as ${filename}!`);
}

function copyToClipboard() {
    const exportArea = document.getElementById('exportArea');
    navigator.clipboard.writeText(exportArea.textContent).then(() => {
        window.themeCreator.showToast('Theme copied to clipboard!');
    }).catch(() => {
        window.themeCreator.showToast('Failed to copy to clipboard', 'error');
    });
}

// Initialize the theme creator when the page loads
document.addEventListener('DOMContentLoaded', () => {
    window.themeCreator = new ThemeCreator();
});