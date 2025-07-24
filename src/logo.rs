use std::collections::HashMap;

pub fn get_logo(os_name: &str, logo_type: &str) -> Vec<String> {
    match logo_type {
        "none" => vec![],
        "small" => get_small_logo(os_name),
        "ascii" => get_ascii_logo(os_name),
        _ => get_auto_logo(os_name),
    }
}

fn get_auto_logo(os_name: &str) -> Vec<String> {
    let os_lower = os_name.to_lowercase();
    
    if os_lower.contains("termux") {
        get_termux_logo()
    } else if os_lower.contains("arch") {
        get_arch_logo()
    } else if os_lower.contains("ubuntu") {
        get_ubuntu_logo()
    } else if os_lower.contains("fedora") {
        get_fedora_logo()
    } else if os_lower.contains("debian") {
        get_debian_logo()
    } else if os_lower.contains("macos") || os_lower.contains("darwin") {
        get_macos_logo()
    } else if os_lower.contains("ios") {
        get_ios_logo()
    } else if os_lower.contains("windows") {
        get_windows_logo()
    } else if os_lower.contains("gentoo") {
        get_gentoo_logo()
    } else if os_lower.contains("manjaro") {
        get_manjaro_logo()
    } else if os_lower.contains("opensuse") {
        get_opensuse_logo()
    } else if os_lower.contains("centos") {
        get_centos_logo()
    } else if os_lower.contains("alpine") {
        get_alpine_logo()
    } else {
        get_generic_logo()
    }
}

fn get_small_logo(os_name: &str) -> Vec<String> {
    let os_lower = os_name.to_lowercase();
    
    if os_lower.contains("termux") {
        vec!["  📱  ".to_string()]
    } else if os_lower.contains("arch") {
        vec!["  /\\  ".to_string(), " /  \\ ".to_string(), "/____\\".to_string()]
    } else if os_lower.contains("ubuntu") {
        vec!["  ___  ".to_string(), " (   ) ".to_string(), "  \\_/  ".to_string()]
    } else if os_lower.contains("macos") || os_lower.contains("darwin") {
        vec!["   🍎   ".to_string()]
    } else if os_lower.contains("ios") {
        vec!["   📱   ".to_string()]
    } else if os_lower.contains("windows") {
        vec!["  ▢▢  ".to_string(), "  ▢▢  ".to_string()]
    } else {
        vec!["  ●  ".to_string()]
    }
}

fn get_ascii_logo(_os_name: &str) -> Vec<String> {
    vec![
        "  ██████  ".to_string(),
        " ████████ ".to_string(),
        "██████████".to_string(),
        " ████████ ".to_string(),
        "  ██████  ".to_string(),
    ]
}

fn get_arch_logo() -> Vec<String> {
    vec![
        "                   -`                    ".to_string(),
        "                  .o+`                   ".to_string(),
        "                 `ooo/                   ".to_string(),
        "                `+oooo:                  ".to_string(),
        "               `+oooooo:                 ".to_string(),
        "               -+oooooo+:                ".to_string(),
        "             `/:-:++oooo+:               ".to_string(),
        "            `/++++/+++++++:              ".to_string(),
        "           `/++++++++++++++:             ".to_string(),
        "          `/+++ooooooooo++++/`           ".to_string(),
        "         ./ooosssso++osssssso+`          ".to_string(),
        "        .oossssso-````/ossssss+`         ".to_string(),
        "       -osssssso.      :ssssssso.        ".to_string(),
        "      :osssssss/        osssso+++.       ".to_string(),
        "     /ossssssss/        +ssssooo/-       ".to_string(),
        "   `/ossssso+/:-        -:/+osssso+-     ".to_string(),
        "  `+sso+:-`                 `.-/+oso:    ".to_string(),
        " `++:.                           `-/+/   ".to_string(),
        " .`                                 `/   ".to_string(),
    ]
}

fn get_ubuntu_logo() -> Vec<String> {
    vec![
        "            .-/+oossssoo+/-.               ".to_string(),
        "        `:+ssssssssssssssssss+:`           ".to_string(),
        "      -+ssssssssssssssssssyyssss+-         ".to_string(),
        "    .osssssssssssssssssdMMMNysssso.        ".to_string(),
        "   /ssssssssssshdmmNNmmyNMMMMhssssss/      ".to_string(),
        "  +ssssssssshmydMMMMMMMNddddyssssssss+     ".to_string(),
        " /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/    ".to_string(),
        ".ssssssssdMMMNhsssssssssshNMMMdssssssss.   ".to_string(),
        "+sssshhhyNMMNyssssssssssssyNMMMysssssss+   ".to_string(),
        "ossyNMMMNyMMhsssssssssssssshmmmhssssssso   ".to_string(),
        "ossyNMMMNyMMhsssssssssssssshmmmhssssssso   ".to_string(),
        "+sssshhhyNMMNyssssssssssssyNMMMysssssss+   ".to_string(),
        ".ssssssssdMMMNhsssssssssshNMMMdssssssss.   ".to_string(),
        " /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/    ".to_string(),
        "  +sssssssssdmydMMMMMMMMddddyssssssss+     ".to_string(),
        "   /ssssssssssshdmNNNNmyNMMMMhssssss/      ".to_string(),
        "    .osssssssssssssssssdMMMNysssso.        ".to_string(),
        "      -+sssssssssssssssssyyyssss+-         ".to_string(),
        "        `:+ssssssssssssssssss+:`           ".to_string(),
        "            .-/+oossssoo+/-.               ".to_string(),
    ]
}

fn get_fedora_logo() -> Vec<String> {
    vec![
        "             .',;::::;,'.                ".to_string(),
        "         .';;;;;;;;;;;;;,'.              ".to_string(),
        "      .,;;;;;;;;;;;;;;;;;;;,.            ".to_string(),
        "     .,;;;;;;;;;;;;;;;;;;;;;;,.          ".to_string(),
        "    .;;;;;;;;;;;;;;;;;;;;;;;;;,'.        ".to_string(),
        "   .;;;;;;;;;;;;;;;;;;;;;;;;;;;;,.       ".to_string(),
        "   ,;;;;;;;;;;;;;;;;;;;;;;;;;;;;,.       ".to_string(),
        "   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;,       ".to_string(),
        "   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;       ".to_string(),
        "   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;       ".to_string(),
        "   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;,'.       ".to_string(),
        "   .;;;;;;;;;;;;;;;;;;;;;;;;;,'.         ".to_string(),
        "    .;;;;;;;;;;;;;;;;;;;;;,'.            ".to_string(),
        "     .;;;;;;;;;;;;;;;;;,'.               ".to_string(),
        "       .;;;;;;;;;;;;;,'.                 ".to_string(),
        "         .';;;;;;;;;,'.                  ".to_string(),
        "             .',;,'.                     ".to_string(),
    ]
}

fn get_debian_logo() -> Vec<String> {
    vec![
        "       _,met$$$$$gg.          ".to_string(),
        "    ,g$$$$$$$$$$$$$$$P.       ".to_string(),
        "  ,g$$P\"     \"\"\"Y$$.\"^.      ".to_string(),
        " ,$$P'              `$$$.     ".to_string(),
        "',$$P       ,ggs.     `$$b:   ".to_string(),
        "`d$$'     ,$P\"'   .    $$$    ".to_string(),
        " $$P      d$'     ,    $$P    ".to_string(),
        " $$:      $$.   -    ,d$$'    ".to_string(),
        " $$;      Y$b._   _,d$P'      ".to_string(),
        " Y$$.    `.`\"Y$$$$P\"'         ".to_string(),
        " `$$b      \"-.__              ".to_string(),
        "  `Y$$                        ".to_string(),
        "   `Y$$.                      ".to_string(),
        "     `$$b.                    ".to_string(),
        "       `Y$$b.                 ".to_string(),
        "          `\"Y$b._             ".to_string(),
        "              `\"\"\"\"           ".to_string(),
    ]
}

fn get_macos_logo() -> Vec<String> {
    vec![
        "                    'c.          ".to_string(),
        "                 ,xNMM.          ".to_string(),
        "               .OMMMMo           ".to_string(),
        "               OMMM0,            ".to_string(),
        "     .;loddo:' loolloddol;.      ".to_string(),
        "   cKMMMMMMMMMMNWMMMMMMMMMM0:    ".to_string(),
        " .KMMMMMMMMMMMMMMMMMMMMMMMWd.    ".to_string(),
        " XMMMMMMMMMMMMMMMMMMMMMMMX.      ".to_string(),
        ";MMMMMMMMMMMMMMMMMMMMMMMM:       ".to_string(),
        ":MMMMMMMMMMMMMMMMMMMMMMMM:       ".to_string(),
        ".MMMMMMMMMMMMMMMMMMMMMMMMX.      ".to_string(),
        " kMMMMMMMMMMMMMMMMMMMMMMMMWd.    ".to_string(),
        " .XMMMMMMMMMMMMMMMMMMMMMMMMMMk   ".to_string(),
        "  .XMMMMMMMMMMMMMMMMMMMMMMMMK.   ".to_string(),
        "    kMMMMMMMMMMMMMMMMMMMMMMd     ".to_string(),
        "     ;KMMMMMMMWXXWMMMMMMMk.      ".to_string(),
        "       .cooc,.    .,coo:.        ".to_string(),
    ]
}

fn get_ios_logo() -> Vec<String> {
    vec![
        "                 .8888b           ".to_string(),
        "                d88888b           ".to_string(),
        "                888888b           ".to_string(),
        "                Y888888           ".to_string(),
        "                 Y88888           ".to_string(),
        "                 d88888           ".to_string(),
        "               .d888888b          ".to_string(),
        "              .d88888888b         ".to_string(),
        "             .d8888888888b        ".to_string(),
        "            .d888888888888b       ".to_string(),
        "           .d88888888888888b      ".to_string(),
        "          .d8888888888888888b     ".to_string(),
        "         .d888888888888888888b    ".to_string(),
        "        .d88888888888888888888b   ".to_string(),
        "       .d8888888888888888888888b  ".to_string(),
        "      .d888888888888888888888888b ".to_string(),
        "     .d88888888888888888888888888b".to_string(),
        "     Y888888888888888888888888888P".to_string(),
        "      Y8888888888888888888888888P ".to_string(),
        "       Y88888888888888888888888P  ".to_string(),
        "        Y888888888888888888888P   ".to_string(),
        "         Y8888888888888888888P    ".to_string(),
        "          Y88888888888888888P     ".to_string(),
        "           Y888888888888888P      ".to_string(),
        "            Y8888888888888P       ".to_string(),
        "             Y88888888888P        ".to_string(),
        "              Y888888888P         ".to_string(),
        "               Y8888888P          ".to_string(),
        "                Y88888P           ".to_string(),
        "                 Y888P            ".to_string(),
        "                  Y8P             ".to_string(),
        "                   Y              ".to_string(),
    ]
}

fn get_windows_logo() -> Vec<String> {
    vec![
        "                                ..,       ".to_string(),
        "                    ....,,:;+ccllll      ".to_string(),
        "      ...,,+:;  cllllllllllllllllll      ".to_string(),
        ",cclllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "                                         ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "llllllllllllll  lllllllllllllllllll      ".to_string(),
        "`'ccllllllllll  lllllllllllllllllll      ".to_string(),
        "       `' \\*::  :ccllllllllllllllll      ".to_string(),
        "                       ````''*::cll      ".to_string(),
        "                                 ``      ".to_string(),
    ]
}

fn get_gentoo_logo() -> Vec<String> {
    vec![
        "         -/oyddmdhs+:.                ".to_string(),
        "     -odNMMMMMMMMNNmhy+-`             ".to_string(),
        "   -yNMMMMMMMMMMMNNNmmdhy+-           ".to_string(),
        " `omMMMMMMMMMMMMNmdmmmmddhhy/`        ".to_string(),
        " omMMMMMMMMMMMNhhyyyohmdddhhhdo`      ".to_string(),
        ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`    ".to_string(),
        " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.   ".to_string(),
        "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh   ".to_string(),
        "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy   ".to_string(),
        "       /mMMMMMMNNNmmmdddhhhhhmMNhs:   ".to_string(),
        "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`    ".to_string(),
        "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.      ".to_string(),
        " /NMMMMMMMMNNNNmmmdddmNMNdso:`        ".to_string(),
        "+MMMMMMMNNNNNmmmmdmNMNdso/-           ".to_string(),
        "yMMNNNNNNNmmmmmNNMmhs+/-`             ".to_string(),
        "/hMMNNNNNNNNMNdhs++/-`                ".to_string(),
        "`/ohdmmddhys+++/:.`                   ".to_string(),
        "  `-//////:--.                       ".to_string(),
    ]
}

fn get_manjaro_logo() -> Vec<String> {
    vec![
        "██████████████████  ████████     ".to_string(),
        "██████████████████  ████████     ".to_string(),
        "██████████████████  ████████     ".to_string(),
        "██████████████████  ████████     ".to_string(),
        "████████            ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
        "████████  ████████  ████████     ".to_string(),
    ]
}

fn get_opensuse_logo() -> Vec<String> {
    vec![
        "           .;ldkO0000Okdl;.           ".to_string(),
        "       .;d00xl:^''''''^:ok00d;.       ".to_string(),
        "     .d00l'                'o00d.     ".to_string(),
        "   .d0Kd'  Okxol:;,.          :O0d.   ".to_string(),
        "  .OK KKK0kOKKKKKKKKKKOxo:,      lKO.  ".to_string(),
        " ,0K KKKKKKKKKKKKKKK0P^,,,^dx:    ;00, ".to_string(),
        ".OK KKKKKKKKKKKKKKKk'^0KKKKK0d:    lOO.".to_string(),
        ",KK KKKKKKKKKKKKKK0x;,,oOKKKKKKK0d:  ;KK,".to_string(),
        ",KK KKKKKKKKKKKKKK.^KKK0d^0KKKKKKK0d: ;KK,".to_string(),
        ".OK KKKKKKKKKKKKKKKx.^KKKd .dKKKKKKKKOd;lOO.".to_string(),
        " ,0K KKKKKKKKKKKKKKK0^.KK0d:^OKKKKKKKK0d;00, ".to_string(),
        "  .OK KKK0xOKKKKKKKKKKK0.^KKKKKKKKKKKKKKKKKd:lKO.  ".to_string(),
        "   .d0Kd '^OKKKKKKKKKKKK0.^KKKKKKKKKKKKKKKKKKKd:O0d.   ".to_string(),
        "     .d00l' '^O0KKKKKKKKKKK0.^KKKKKKKKKKKKKKKKKK0d:00d.     ".to_string(),
        "       .;d00xl;,,;^OKKKKKKKKKK0.^KKKKKKKKKKKKKKKKK0d:O0d;.       ".to_string(),
        "           .;ldkO0000OKKKKKKKKKK0.^KKKKKKKKKKKKKKKK0d:O0d;.           ".to_string(),
    ]
}

fn get_centos_logo() -> Vec<String> {
    vec![
        "                 ..                    ".to_string(),
        "               .PLTJ.                  ".to_string(),
        "              <><><><>                 ".to_string(),
        "     KKSSV' 4KKK LJ KKKL.'VSSKK        ".to_string(),
        "     KKV' 4KKKKK LJ KKKKAL 'VKK        ".to_string(),
        "     V' ' 'VKKKK LJ KKKKV' ' 'V        ".to_string(),
        "     .4MA.' 'VKK LJ KKV' '.4Mb.        ".to_string(),
        "   . KKKKKA.' 'V LJ V' '.4KKKKK .      ".to_string(),
        " .4D KKKKKKKA.'' LJ ''.4KKKKKKK FA.    ".to_string(),
        "<QDD ++++++++++++  ++++++++++++++ GFD> ".to_string(),
        " 'VD KKKKKKKK'.. LJ ..'KKKKKKKK FV'    ".to_string(),
        "   ' VKKKKK'. .4 LJ K. .'KKKKKV '      ".to_string(),
        "      'VK'. .4KK LJ KKA. .'KV'         ".to_string(),
        "     A. . .4KKKK LJ KKKKA. . .4        ".to_string(),
        "     KKA. 'KKKKK LJ KKKKK' .4KK        ".to_string(),
        "     KKSSA. VKKK LJ KKKV .4SSKK        ".to_string(),
        "              <><><><>                 ".to_string(),
        "               'MKKM'                  ".to_string(),
        "                 ''                    ".to_string(),
    ]
}

fn get_alpine_logo() -> Vec<String> {
    vec![
        "       .hddddddddddddddddddddddh.       ".to_string(),
        "      :dddddddddddddddddddddddddd:      ".to_string(),
        "     /dddddddddddddddddddddddddddd/     ".to_string(),
        "    +dddddddddddddddddddddddddddddd+    ".to_string(),
        "  `sdddddddddddddddddddddddddddddddds`  ".to_string(),
        "  `ydddddddddddd++hdddddddddddddddddy`  ".to_string(),
        "   .hddddddddddd+`  `+ddddddddddddddh.   ".to_string(),
        "    `ydddddddddd:      :dddddddddddy`    ".to_string(),
        "     `sdddddddd+        +ddddddddds`     ".to_string(),
        "       `yddddd:          :dddddy`       ".to_string(),
        "         `sdd+            +dds`         ".to_string(),
        "           `:              :`           ".to_string(),
    ]
}

fn get_termux_logo() -> Vec<String> {
    vec![
        "                                      ".to_string(),
        "    ████████ ████████ ████████       ".to_string(),
        "   ██      ██      ██      ██        ".to_string(),
        "   ██      ██      ██      ██        ".to_string(),
        "   ██      ██      ██      ██        ".to_string(),
        "   ████████ ████████ ████████        ".to_string(),
        "   ██                               ".to_string(),
        "   ██   ████████ ████████ ██   ██    ".to_string(),
        "   ██   ██       ██    ██ ██   ██    ".to_string(),
        "   ██   ████████ ████████ ███████    ".to_string(),
        "   ██   ██       ██  ██   ██   ██    ".to_string(),
        "   ██   ████████ ██   ██  ██   ██    ".to_string(),
        "                                     ".to_string(),
        "        📱 Android Terminal 📱        ".to_string(),
        "                                     ".to_string(),
    ]
}

fn get_generic_logo() -> Vec<String> {
    vec![
        "   _____   ".to_string(),
        "  /     \\  ".to_string(),
        " | () () | ".to_string(),
        "  \\  ^  /  ".to_string(),
        "   |||||   ".to_string(),
        "   |||||   ".to_string(),
    ]
}

pub fn get_color_codes() -> HashMap<String, String> {
    let mut colors = HashMap::new();
    colors.insert("black".to_string(), "\x1b[30m".to_string());
    colors.insert("red".to_string(), "\x1b[31m".to_string());
    colors.insert("green".to_string(), "\x1b[32m".to_string());
    colors.insert("yellow".to_string(), "\x1b[33m".to_string());
    colors.insert("blue".to_string(), "\x1b[34m".to_string());
    colors.insert("magenta".to_string(), "\x1b[35m".to_string());
    colors.insert("cyan".to_string(), "\x1b[36m".to_string());
    colors.insert("white".to_string(), "\x1b[37m".to_string());
    colors.insert("bright_black".to_string(), "\x1b[90m".to_string());
    colors.insert("bright_red".to_string(), "\x1b[91m".to_string());
    colors.insert("bright_green".to_string(), "\x1b[92m".to_string());
    colors.insert("bright_yellow".to_string(), "\x1b[93m".to_string());
    colors.insert("bright_blue".to_string(), "\x1b[94m".to_string());
    colors.insert("bright_magenta".to_string(), "\x1b[95m".to_string());
    colors.insert("bright_cyan".to_string(), "\x1b[96m".to_string());
    colors.insert("bright_white".to_string(), "\x1b[97m".to_string());
    colors.insert("reset".to_string(), "\x1b[0m".to_string());
    colors.insert("bold".to_string(), "\x1b[1m".to_string());
    colors
}