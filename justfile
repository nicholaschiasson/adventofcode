bootstrap YEAR DAY:
  cp template/lib.rs src/year_{{YEAR}}/day_{{DAY}}.rs
  mkdir -p rsrc/inputs/year_{{YEAR}}/day_{{DAY}}/tests
  touch rsrc/inputs/year_{{YEAR}}/day_{{DAY}}/tests/{practice_01,final}.txt

build *ARGS:
  cargo build {{ARGS}}

check *ARGS:
  cargo check {{ARGS}}

clean *ARGS:
  cargo clean {{ARGS}}

format *ARGS:
  nixfmt $(find . -type f -name "*.nix")
  cargo fmt {{ARGS}}

lint *ARGS:
  cargo clippy {{ARGS}}

run *ARGS:
  cargo run {{ARGS}}

test *ARGS:
  cargo test {{ARGS}}

watch *ARGS:
  cargo watch {{ARGS}}
