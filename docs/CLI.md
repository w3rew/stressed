# Command-Line Help for `stressed`

This document contains the help content for the `stressed` command-line program.

**Command Overview:**

* [`stressed`↴](#stressed)

## `stressed`

**Usage:** `stressed [OPTIONS] --sampler <sampler_path> --checker <check> <SOLVER_PATH>`

###### **Arguments:**

* `<SOLVER_PATH>` — Path to solver

###### **Options:**

* `-s`, `--sampler <sampler_path>` — Path to sampler
* `-c`, `--checker <check>` — Path to checker: either to reference solver, or to the dedicated checker. See --use-custom-checker for details
* `--use-custom-checker` — Whether to use custom checker. Without this flag, --checker argument is interpreted as path to the reference solver. If --use-custom-checker flag is present, --checker receives *testcase* and, **immediately after**, *the program's answer* and should exit with zero or non-zero exit code
* `--sampler-use-stdin` — Use stdin to supply random seed to sampler. The default behaviour is to specify it as the only argument to the sampler
* `--diff-mode <DIFF_MODE>` — Mode to use for diffs; works only for default checker

  Default value: `line`

  Possible values:
  - `line`:
    Output diff per line
  - `char`:
    Output diff per character
  - `none`:
    Do not output diff at all; instead, just output what the tested program answered. This might be desirable since the reference solver's output is printed anyway

* `--no-progress` — Do not show progress bar
* `-n`, `--niter <NITER>` — Number of samples to try

  Default value: `10000`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
