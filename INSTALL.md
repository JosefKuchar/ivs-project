# Postup kompilace

Postup platí pro platformu Windows x64

## Závislosti

Je třeba nainstalovat následující závislosti

- Rust tooling - https://www.rust-lang.org/tools/install

  - třeba při promptu nainstalovat C++ build tools
  - případně lze nainstalovat C++ build tools manuálně https://visualstudio.microsoft.com/cs/visual-cpp-build-tools/

- Node.js - https://nodejs.org/en/
  - verze v16 LTS či podobná
- Yarn - https://classic.yarnpkg.com/lang/en/docs/install/#windows-stable
- Make - http://gnuwin32.sourceforge.net/packages/make.htm
  - Je třeba manuálně přidat do PATH

## Kompilování

Ve složce `src` stačí spustit `make`

Spustitelné soubory se vytvoří ve složce `src/src-tauri/target/release/`

Instalátor se vytvoří ve složce `src/src-tauri/target/release/bundle/msi`
