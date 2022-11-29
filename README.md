# rust-parallel

Run commands in parallel, like a simple rust verision of [GNU Parallel](https://www.gnu.org/software/parallel/).

Just starting - more options to come :)

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/rust-parallel.svg
[crates-url]: https://crates.io/crates/rust-parallel

# Usage:
```
$ rust-parallel -h

Run commands in parallel

Usage: rust-parallel [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  Input file or - for stdin.  Defaults to stdin if no inputs are specified

Options:
  -j, --jobs <JOBS>    Maximum number of commands to run in parallel, defauts to num cpus [default: 12]
  -s, --shell-enabled  Use /bin/sh -c shell to run commands
  -h, --help           Print help information
  -V, --version        Print version information
```

# Demos:

Small demo of 5 echo commands:

```
$ cat >./test <<EOL
echo hi
echo there
echo how
echo are
echo you
EOL

$ cat test | rust-parallel -j5
are
hi
there
how
you
```

Using `awk` to form commands:

```
$ head -100 /usr/share/dict/words| awk '{printf "md5 -s %s\n", $1}' | rust-parallel
MD5 ("Abba") = 5fa1e1f6e07a6fea3f2bb098e90a8de2
MD5 ("abaxial") = ac3a53971d52d9ce3277eadf03f13a5e
MD5 ("abaze") = 0b08c52aa63d947b6a5601ee975bc3a4
MD5 ("abaxile") = 21f5fc27d7d34117596e41d8c001087e
MD5 ("abbacomes") = 76640eb0c929bc97d016731bfbe9a4f8
MD5 ("abbacy") = 08aeac72800adc98d2aba540b6195921
MD5 ("Abbadide") = 7add1d6f008790fa6783bc8798d8c803
MD5 ("abb") = ea01e5fd8e4d8832825acdd20eac5104
```

With debug logs enabled:

```
$ cat test | RUST_LOG=debug rust-parallel
2022-11-29T01:22:20.352921Z DEBUG rust_parallel: begin main
2022-11-29T01:22:20.353561Z DEBUG rust_parallel::command_line_args: command_line_args = CommandLineArgs { jobs: 12, shell_enabled: false, inputs: [] }
2022-11-29T01:22:20.353605Z DEBUG rust_parallel::commands: begin spawn_commands
2022-11-29T01:22:20.353636Z DEBUG rust_parallel::commands: begin process_one_input input_name = '-'
2022-11-29T01:22:20.353783Z DEBUG rust_parallel::commands: read line echo hi
2022-11-29T01:22:20.353835Z DEBUG rust_parallel::commands: read line echo there
2022-11-29T01:22:20.353861Z DEBUG rust_parallel::commands: read line echo how
2022-11-29T01:22:20.353879Z DEBUG rust_parallel::commands: read line echo are
2022-11-29T01:22:20.353897Z DEBUG rust_parallel::commands: read line echo you
2022-11-29T01:22:20.353925Z DEBUG rust_parallel::commands: begin run_command command = Command { _input_name: "-", _line_number: 1, command: "echo hi", shell_enabled: false }
2022-11-29T01:22:20.353922Z DEBUG rust_parallel::commands: begin run_command command = Command { _input_name: "-", _line_number: 2, command: "echo there", shell_enabled: false }
2022-11-29T01:22:20.353963Z DEBUG rust_parallel::commands: begin run_command command = Command { _input_name: "-", _line_number: 4, command: "echo are", shell_enabled: false }
2022-11-29T01:22:20.353948Z DEBUG rust_parallel::commands: begin run_command command = Command { _input_name: "-", _line_number: 3, command: "echo how", shell_enabled: false }
2022-11-29T01:22:20.353974Z DEBUG rust_parallel::commands: end process_one_input input_name = '-'
2022-11-29T01:22:20.353980Z DEBUG rust_parallel::commands: begin run_command command = Command { _input_name: "-", _line_number: 5, command: "echo you", shell_enabled: false }
2022-11-29T01:22:20.357172Z DEBUG rust_parallel::commands: end spawn_commands
2022-11-29T01:22:20.357207Z DEBUG rust_parallel: before wait_group.wait wait_group = WaitGroup { count: 5 }
2022-11-29T01:22:20.359083Z DEBUG rust_parallel::commands: got command status = exit status: 0
hi
2022-11-29T01:22:20.359161Z DEBUG rust_parallel::commands: end run_command command = Command { _input_name: "-", _line_number: 1, command: "echo hi", shell_enabled: false }
2022-11-29T01:22:20.360766Z DEBUG rust_parallel::commands: got command status = exit status: 0
are
2022-11-29T01:22:20.360805Z DEBUG rust_parallel::commands: end run_command command = Command { _input_name: "-", _line_number: 4, command: "echo are", shell_enabled: false }
2022-11-29T01:22:20.361892Z DEBUG rust_parallel::commands: got command status = exit status: 0
you
2022-11-29T01:22:20.361925Z DEBUG rust_parallel::commands: end run_command command = Command { _input_name: "-", _line_number: 5, command: "echo you", shell_enabled: false }
2022-11-29T01:22:20.363213Z DEBUG rust_parallel::commands: got command status = exit status: 0
there
2022-11-29T01:22:20.363242Z DEBUG rust_parallel::commands: end run_command command = Command { _input_name: "-", _line_number: 2, command: "echo there", shell_enabled: false }
2022-11-29T01:22:20.364298Z DEBUG rust_parallel::commands: got command status = exit status: 0
how
2022-11-29T01:22:20.364326Z DEBUG rust_parallel::commands: end run_command command = Command { _input_name: "-", _line_number: 3, command: "echo how", shell_enabled: false }
2022-11-29T01:22:20.364378Z DEBUG rust_parallel: end main
```
