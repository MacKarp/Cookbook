#!/bin/bash
rm -r ./win
rm Windows.zip
cargo build --target=x86_64-pc-windows-gnu --release
mkdir ./win
cp target/x86_64-pc-windows-gnu/release/*.exe ./win
cp target/x86_64-pc-windows-gnu/release/*.rlib ./win
mkdir -p ./win/gui/ui
cp gui/ui/* ./win/gui/ui
cp $MINGW_PREFIX/bin/*.dll ./win
mkdir -p ./win/share/glib-2.0/schemas
cp $MINGW_PREFIX/share/glib-2.0/schemas/gschemas.compiled ./win/share/glib-2.0/schemas/gschemas.compiled
cp -r $MINGW_PREFIX/share/icons ./win/share/icons
rm ./win/gui/ui/Cookbook.glade
zip -r Windows.zip ./win
echo "Done"
