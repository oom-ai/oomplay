complete -c oomplay -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c oomplay -n "__fish_use_subcommand" -f -a "init" -d 'Initialize playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "stop" -d 'Stop playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "list" -d 'List supported playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "completion" -d 'Output shell completion code'
complete -c oomplay -n "__fish_seen_subcommand_from init" -s j -l jobs -d 'Number of parallel jobs, defaults to # of CPUs' -r
complete -c oomplay -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop" -s j -l jobs -d 'Number of parallel jobs, defaults to # of CPUs' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
