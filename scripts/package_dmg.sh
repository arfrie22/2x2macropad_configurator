[[ -f Macropad-Configurator-Installer.dmg ]] && rm Macropad-Configurator-Installer.dmg

# if current dir ends with "scripts" cd up one dir
[[ $PWD =~ scripts$ ]] &&

echo "cd .." && cd ..

cargo bundle

create-dmg \
  --volname "Macropad Configurator Installer" \
  --background "assets/installer/Background.png" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Macropad Configurator.app" 200 190 \
  --hide-extension "Macropad Configurator.app" \
  --app-drop-link 600 185 \
  "Macropad-Configurator-Installer.dmg" \
  "target/debug/bundle/osx/"