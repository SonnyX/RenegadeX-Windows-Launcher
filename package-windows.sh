#!/bin/sh
cargo build --target=x86_64-pc-windows-gnu --release
rm RenX-Launcher.zip
zip -j9 RenX-Launcher target/x86_64-pc-windows-gnu/release/RenegadeX-Launcher.exe sciter.dll RenegadeX-Launcher.ini
zip -9 RenX-Launcher -r dom
rm RenX-Launcher.7z
7z a RenX-Launcher.7z RenX-Launcher target/x86_64-pc-windows-gnu/release/RenegadeX-Launcher.exe sciter.dll RenegadeX-Launcher.ini
7z a RenX-Launcher.7z -r dom

