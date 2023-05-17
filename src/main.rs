use std::io::{self, Write};
use std::fs::File;
use std::env;
use std::collections::HashMap;
use reqwest;


fn main() {
    let aqpmversion = env!("CARGO_PKG_VERSION");
    let username = env::var("USERNAME").unwrap_or("user".to_owned());
    print!("Aqpm Package Manager [Version {}]\n(c) 2023 ACHROMATIC LTD. All Rights Reserved.\n\nAqpm\\{}> Enter an app to install: ", aqpmversion, username);
    io::stdout().flush().expect("An error occurred.");

    let mut uinp = String::new();

    io::stdin()
        .read_line(&mut uinp)
        .expect("An error occurred.");

    uinp = uinp.to_lowercase();

    let mut apps = HashMap::new();
    apps.insert("discord", "https://discord.com/api/download?platform=win".to_owned());
    apps.insert("spotify", "https://github.com/amd64fox/SpotX/releases/download/1.7/Install_Prem.bat".to_owned());
    apps.insert("vscode", "https://code.visualstudio.com/docs/?dv=win".to_owned());
    apps.insert("vscodium", "https://github.com/VSCodium/vscodium/releases/download/1.78.2.23132/VSCodiumUserSetup-x64-1.78.2.23132.exe".to_owned());
    apps.insert("git", "https://github.com/git-for-windows/git/releases/download/v2.40.1.windows.1/Git-2.40.1-64-bit.exe".to_owned());
    apps.insert("vencord", "https://github.com/Vencord/Installer/releases/latest/download/VencordInstaller.exe".to_owned());
    apps.insert("telegram", "https://telegram.org/dl/desktop/win64".to_owned());
    apps.insert("adobephotoshop", "https://dl.malwarewatch.org/software/useful/adobe/Adobe%20Photoshop%202022.iso".to_owned());
    apps.insert("adobepremierepro", "https://dl.malwarewatch.org/software/useful/adobe/Adobe%20Premiere%20Pro%202022.iso".to_owned());
    apps.insert("chrome", "https://dl.google.com/tag/s/appguid%3D%7B8A69D345-D564-463C-AFF1-A69D9E530F96%7D%26iid%3D%7BBFC76495-1C74-6805-2A44-7B65AC933874%7D%26lang%3Den-GB%26browser%3D5%26usagestats%3D1%26appname%3DGoogle%2520Chrome%26needsadmin%3Dprefers%26ap%3Dx64-stable-statsdef_1%26installdataindex%3Dempty/update2/installers/ChromeSetup.exe".to_owned());
    apps.insert("brave", "https://laptop-updates.brave.com/latest/winx64".to_owned());

    if let Some(download_url) = apps.get(uinp.trim()) {
        print!("Aqpm\\{}> {} selected, do you want to continue? (y/n): ",username, uinp.trim());
        io::stdout().flush().expect("An error occurred.");

        let mut userconfirmation = String::new();

        io::stdin()
            .read_line(&mut userconfirmation)
            .expect("An error occurred.");

        if userconfirmation.trim().to_lowercase() == "y" {
            print!("Aqpm\\{}> Installing: {}. Standby | ",username, uinp.trim());

            let response = reqwest::blocking::get(download_url)
                .expect("Failed to download file");

            let mut file = File::create(format!("{}.exe", uinp.trim()))
                .expect("Failed to create file");

            io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)
                .expect("Failed to write to file");

            println!("Done.");
        } else {
            println!("Aqpm\\{}> Installation stopped.",username);
        }
    } else {
        println!("Aqpm\\{}> Invalid input.",username);
    }
}
