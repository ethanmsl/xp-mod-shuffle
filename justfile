# Justfile (Convenience Command Runner)


# local vars

# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'

# home_dir := env_var('HOME')
local_root := justfile_directory()
invocd_from := invocation_directory()
invoc_is_root := if invocd_from == local_root { "true" } else { "false" }
froze_sha_regex := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
## ANSI Color Codes for use with echo command
GRN := '\033[0;32m' # Green
BLU := '\033[0;34m' # Blue
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown
CYN := '\033[0;36m' # Cyan
NC := '\033[0m'     # No Color

# Default, lists commands.
_default:
        @ just --list --unsorted

# Linting, formatting, typo checking, etc.
check:
    -cargo clippy
    -cargo fmt
    -typos --exclude 'data/*'
    -committed

# Watch a file: compile & run on changes.
watch file_to_run:
    cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'

# Initialize repository.
init: && deps 
    cargo build
    cargo doc

# Clean up cargo build artifacts.
[confirm]
clean:
    cargo clean

# List dependencies. (This command has dependencies.)
deps:
    @echo "{{CYN}}List of external dependencies for this command runner:"
    xsv table ext_dependencies.csv






# ######################################################################## #

# Freeze! For your safety.
freeze FILE:
	mv -iv {{FILE}} FROZE_{{sha256(FILE)}}_FROZE-{{FILE}} | rg {{FILE}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
thaw FILE:
	echo {{FILE}} | sd '{{froze_sha_regex}}' '' | xargs mv -iv {{FILE}}

# Find local file(s) through the ice.
arctic_recon ICELESS_NAME:
	fd --max-depth 1 '{{froze_sha_regex}}{{ICELESS_NAME}}' | rg {{ICELESS_NAME}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha FILE:
	echo {{sha256_file(FILE)}}

# Example function for syntax reference
_example_file_exists_test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}


# ######################################################################## #

# # Benchmark cli-app wrapped code using Hyperfine.
# hyperf reps:
#     @echo "{{ PRP }}Release{{ NC }}:--------------------------------------------------------"
#     hyperfine --warmup 3 'target/release/cli_mod_shuffle {{reps}}'
#     @echo "{{ PRP }}Debug{{ NC }}:----------------------------------------------------------"
#     hyperfine --warmup 3 'target/debug/cli_mod_shuffle {{reps}}'

# # generate a `.env` file to place your database path in.
# gen-env:
#     @echo "{{CYN}}The {{GRN}}.env DATABASE_URL value{{CYN}}will populate your database path when needed.  Please edit the file to manually specify."
#     @echo {{ if path_exists(".env") == "true" { `echo "\(.env file already exists\)"` } else { `cp 'template.env' '.env'; echo "\(.env file created\)"`} }}
