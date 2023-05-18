use std::io::{self, Write};
use std::fs::{self, File};
use std::env;
use std::collections::HashMap;
use std::path::PathBuf;

use reqwest;



fn main() {
    let aqpm_version = env!("CARGO_PKG_VERSION");
    let username = env::var("USERNAME").unwrap_or_else(|_| "user".to_owned());
    print!("Aqpm Package Manager [Version {}]\n(c) 2023 ACHROMATIC LTD. All Rights Reserved.\n\nAqpm\\{}> Enter an app to install (or :l to list apps): ", aqpm_version, username);
    io::stdout().flush().expect("An error occurred.");

    let mut uinp = String::new();
    io::stdin().read_line(&mut uinp).expect("An error occurred.");
    uinp = uinp.to_lowercase().trim().to_owned();

    let mut apps = HashMap::<String, String>::new();
    apps.insert("discord".to_owned(), "https://discord.com/api/download?platform=win".to_owned());
    apps.insert("spotify".to_owned(), "https://github.com/amd64fox/SpotX/releases/download/1.7/Install_Prem.bat".to_owned());
    apps.insert("vscode".to_owned(), "https://code.visualstudio.com/docs/?dv=win".to_owned());
    apps.insert("vscodium".to_owned(), "https://github.com/VSCodium/vscodium/releases/download/1.78.2.23132/VSCodiumUserSetup-x64-1.78.2.23132.exe".to_owned());
    apps.insert("git".to_owned(), "https://github.com/git-for-windows/git/releases/download/v2.40.1.windows.1/Git-2.40.1-64-bit.exe".to_owned());
    apps.insert("vencord".to_owned(), "https://github.com/Vencord/Installer/releases/latest/download/VencordInstaller.exe".to_owned());
    apps.insert("telegram".to_owned(), "https://telegram.org/dl/desktop/win64".to_owned());
    apps.insert("adobephotoshop".to_owned(), "https://dl.malwarewatch.org/software/useful/adobe/Adobe%20Photoshop%202022.iso".to_owned());
    apps.insert("hexeditor".to_owned(), "https://dl.malwarewatch.org/software/useful/HxDEditor.zip".to_owned());
    apps.insert("adobepremierepro".to_owned(), "https://dl.malwarewatch.org/software/useful/adobe/Adobe%20Premiere%20Pro%202022.iso".to_owned());
    apps.insert("chrome".to_owned(), "https://dl.google.com/tag/s/appguid%3D%7B8A69D345-D564-463C-AFF1-A69D9E530F96%7D%26iid%3D%7BBFC76495-1C74-6805-2A44-7B65AC933874%7D%26lang%3Den-GB%26browser%3D5%26usagestats%3D1%26appname%3DGoogle%2520Chrome%26needsadmin%3Dprefers%26ap%3Dx64-stable-statsdef_1%26installdataindex%3Dempty/update2/installers/ChromeSetup.exe".to_owned());
    apps.insert("brave".to_owned(), "https://laptop-updates.brave.com/latest/winx64".to_owned());

    if let Some(download_url) = apps.get(&uinp) {
        print!("Aqpm\\{}> {} selected, do you want to continue? (y/n): ", username, uinp);
        io::stdout().flush().expect("An error occurred.");

        let mut user_confirmation = String::new();
        io::stdin().read_line(&mut user_confirmation).expect("An error occurred.");
        let user_confirmation = user_confirmation.trim().to_lowercase();

        if user_confirmation == "y" {
            println!("Aqpm\\{}> Installing: {}. Standby!", username, uinp);

            let response = reqwest::blocking::get(download_url).expect("Failed to download file");

            let aqtmp_dir = format!("C:\\Users\\{}\\AppData\\Local\\AQTMP", username);
            if !PathBuf::from(&aqtmp_dir).exists() {
                fs::create_dir(&aqtmp_dir).expect("Unable to create directory: C:\\Users\\{username}\\AppData\\Local\\AQTMP");
            }

            let file_path = format!("C:\\Users\\{}\\AppData\\Local\\AQTMP\\{}.exe", username, uinp);
            let mut file = File::create(&file_path).expect("Failed to create file");

            io::copy(&mut response.bytes().unwrap().as_ref(), &mut file).expect("Failed to write to file");

            println!("Aqpm\\{username}> Done. Check: C:\\Users\\{username}\\AppData\\Local\\AQTMP for the app. Auto Open coming soon.");
            io::stdout().flush().expect("An error occurred.");
        } else if user_confirmation == "n" {
            println!("Aqpm\\{}> Aborted.", username);
        }
    } else if uinp == ":l" {
        println!("Aqpm\\{}> All apps available: ", username);
        for (i, (app, _)) in apps.iter().enumerate() {
            if i < apps.len() - 1 {
                print!("{}, ", app);
            } else {
                println!("{}", app);
            }
        }
    }
}
