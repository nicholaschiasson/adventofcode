default: test-all

bootstrap-all YEAR DAY PRACTICE_INPUT="1": (bootstrap "go" YEAR DAY PRACTICE_INPUT) (bootstrap "rust" YEAR DAY PRACTICE_INPUT)

build-all *ARGS: (build "go" ARGS) (build "rust" ARGS)

check-all *ARGS: (check "go" ARGS) (check "rust" ARGS)

clean-all *ARGS: (clean "go" ARGS) (clean "rust" ARGS)

format-all *ARGS: (format "go" ARGS) (format "rust" ARGS)
  nixfmt $(find . -type f -name "*.nix")

lint-all *ARGS: (lint "go" ARGS) (lint "rust" ARGS)

run-all *ARGS: (run "go" ARGS) (run "rust" ARGS)

test-all *ARGS: (test "go" ARGS) (test "rust" ARGS)

bootstrap LANG YEAR DAY PRACTICE_INPUT="1":
  mkdir -p rsrc/inputs/year_{{YEAR}}/day_{{DAY}}/tests
  touch rsrc/inputs/year_{{YEAR}}/day_{{DAY}}/tests/final.txt
  for i in {1..{{PRACTICE_INPUT}}}; do \
    touch rsrc/inputs/year_{{YEAR}}/day_{{DAY}}/tests/practice_${i}.txt; \
  done
  just {{LANG}}/bootstrap {{YEAR}} {{DAY}}

build LANG *ARGS:
  just {{LANG}}/build {{ARGS}}

check LANG *ARGS:
  just {{LANG}}/check {{ARGS}}

clean LANG *ARGS:
  just {{LANG}}/clean {{ARGS}}

format LANG *ARGS:
  just {{LANG}}/format {{ARGS}}

lint LANG *ARGS:
  just {{LANG}}/lint {{ARGS}}

run LANG *ARGS:
  just {{LANG}}/run {{ARGS}}

test LANG *ARGS:
  just {{LANG}}/test {{ARGS}}

watch LANG *ARGS:
  just {{LANG}}/watch {{ARGS}}

