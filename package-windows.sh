#!/bin/sh
cargo build --target=x86_64-pc-windows-gnu --release
rm RenX-Launcher.zip
zip -j RenX-Launcher target/x86_64-pc-windows-gnu/release/RenegadeX-Launcher.exe sciter.dll 
zip RenX-Launcher -r dom
