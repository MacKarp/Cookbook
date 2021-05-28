#!/bin/bash
rm -r ./Linux
cargo build --release
mkdir -p ./Linux/gui/ui
cp gui/Cookbook.desktop ./Linux/Cookbook.desktop
cp target/release/gui ./Linux/cookbook_gui
cp target/release/*.rlib ./Linux
cp gui/ui/* ./Linux/gui/ui
rm ./Linux/gui/ui/Cookbook.glade
zip -r Linux.zip ./Linux
echo "Done"
