export DRONE_RUSTFLAGS := '--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f303"'
target := 'thumbv7em-none-eabihf'
features := ''
name := "f303-blinky" 
release_bin := "target/" + target + "/release/" + name

# Install dependencies
deps:
	rustup target add {{target}}
	rustup component add rust-src
	rustup component add rustfmt
	rustup component add clippy
	rustup component add llvm-tools-preview
	type cargo-objdump >/dev/null || cargo +stable install cargo-binutils
	type drone >/dev/null || cargo install drone

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	drone env {{target}} -- cargo clippy --features "{{features}}"

# Build the binary
build:
	drone env {{target}} -- cargo build --features "{{features}}" --release

# Build the documentation
doc:
	drone env {{target}} -- cargo doc --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	drone env {{target}} -- cargo doc --features "{{features}}" --open

# Run the tests
test:
	drone env -- cargo test --features "std {{features}}"

# Display information from the binary
dump: build
	drone env {{target}} -- cargo objdump --target {{target}} \
		--features "{{features}}" --release --bin {{name}} -- \
		--disassemble --demangle --full-contents -all-headers --syms | pager

# Display the sizes of sections inside the binary
size +args='': build
	drone env {{target}} -- cargo size --target {{target}} \
		--features "{{features}}" --release --bin {{name}} -- {{args}}

# Display the result of macro expansion
expand:
	drone env {{target}} -- cargo rustc --target {{target}} \
		--features "{{features}}" --lib -- -Z unstable-options --pretty=expanded

# Assert the reset signal
reset:
	drone reset

# Write the binary to ROM
flash: build
	drone flash {{release_bin}}

# Run a GDB session
gdb:
	drone gdb {{release_bin}} --reset

# Run a GDB session as a backend for a debugger GUI or an IDE
@gdb-mi:
	drone gdb {{release_bin}} --reset -i=mi -- -nx

# Capture the log output
log:
	drone log --reset :0:1

# Record `heaptrace` file (the target should be running a binary with `heaptrace` feature)
heaptrace:
	truncate -s0 heaptrace
	drone log --reset :0:1 heaptrace:31
