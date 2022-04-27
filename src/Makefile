.PHONY: all clean pack test doc run profile

all:
	yarn install
	yarn tauri build

clean:
	cd src-tauri; cargo clean
	powershell "if (Test-Path dist) {Remove-Item -Recurse -Force dist}"
	powershell "if (Test-Path node_modules) {Remove-Item -Recurse -Force node_modules}"

pack: all doc
	powershell "if (Test-Path dist) {Remove-Item -Recurse -Force dist}"
	powershell "if (Test-Path node_modules) {Remove-Item -Recurse -Force node_modules}"
	cd ../..; powershell "if (-not (Test-Path xkucha28_xsirov00_xbenci01_xnecka00)) {mkdir xkucha28_xsirov00_xbenci01_xnecka00}"
	cd ../../xkucha28_xsirov00_xbenci01_xnecka00; powershell "if (-not (Test-Path install)) {mkdir install}"
	powershell "Copy-Item -Path src-tauri/target/release/bundle/msi/*.msi -Destination ../../xkucha28_xsirov00_xbenci01_xnecka00/install/"
	cd ..; powershell "if (Test-Path ../xkucha28_xsirov00_xbenci01_xnecka00/doc) {Remove-Item -Recurse -Force ../xkucha28_xsirov00_xbenci01_xnecka00/doc} Move-Item -Path src/src-tauri/target/doc/* -Destination ../xkucha28_xsirov00_xbenci01_xnecka00/doc -Force"
	cd src-tauri; cargo clean
	cd ../../xkucha28_xsirov00_xbenci01_xnecka00; powershell "if (-not (Test-Path repo)) {mkdir repo}"
	cd ..; powershell "Copy-Item -Path ./* -Destination ../xkucha28_xsirov00_xbenci01_xnecka00/repo/ -Recurse -Force"
	cd ..; powershell "Compress-Archive -Path ../xkucha28_xsirov00_xbenci01_xnecka00 -Destination xkucha28_xsirov00_xbenci01_xnecka00.zip"
	cd ../..; powershell "if (Test-Path xkucha28_xsirov00_xbenci01_xnecka00) {Remove-Item -Recurse -Force xkucha28_xsirov00_xbenci01_xnecka00}"

test:
	powershell "if (-Not (Test-Path dist)) {mkdir dist}"
	cd src-tauri; cargo test

doc:
	powershell "if (-Not (Test-Path dist)) {mkdir dist}"
	cd src-tauri; cargo doc --no-deps

run:
	cd src-tauri/target/release; run frontend.exe

profile:
	./src-tauri/target/release/stddev.exe