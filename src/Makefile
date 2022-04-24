all:
		yarn tauri build

clean:
		rm -rf src-tauri/target

#pack:

test:
		cd src-tauri; cargo test

doc:
		cd src-tauri; cargo doc --no-deps

run:
		cd src-tauri/target/release; run frontend.exe

profile:
		./src-tauri/target/release/stddev.exe