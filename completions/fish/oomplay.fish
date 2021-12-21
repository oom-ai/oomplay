complete -c oomplay -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c oomplay -n "__fish_use_subcommand" -f -a "init" -d 'Initialize playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "stop" -d 'Stop playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "completion" -d 'Output shell completion code'
complete -c oomplay -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c oomplay -n "__fish_seen_subcommand_from init" -s c -l config -d 'Config file path' -r
complete -c oomplay -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop" -s c -l config -d 'Config file path' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from help" -s h -l help -d 'Print help information'
