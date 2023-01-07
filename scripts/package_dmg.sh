[[ -f Macropad-Configurator-Installer.dmg ]] && rm Macropad-Configurator-Installer.dmg

create-dmg \
  --volname "Macropad Configurator Installer" \
  --background "assets/installer/Background.tiff" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Macropad Configurator.app" 200 190 \
  --hide-extension "Macropad Configurator.app" \
  --app-drop-link 600 185 \
  "Macropad-Configurator-Installer.dmg" \
  "target/debug/bundle/osx/"