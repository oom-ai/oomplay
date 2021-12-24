complete -c oomplay -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c oomplay -n "__fish_use_subcommand" -f -a "init" -d 'Initialize playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "clear" -d 'Clean up playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "stop" -d 'Stop playgrounds'
complete -c oomplay -n "__fish_use_subcommand" -f -a "completion" -d 'Output shell completion code'
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "redis" -d 'Redis playground'
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "postgres" -d 'Postgres playground'
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "mysql" -d 'MySQL playground'
complete -c oomplay -n "__fish_seen_subcommand_from init; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "dynamodb" -d 'DynamoDB playground'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from redis" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from mysql" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from dynamodb" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from dynamodb" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from init; and __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "redis" -d 'Redis playground'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "postgres" -d 'Postgres playground'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "mysql" -d 'MySQL playground'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "dynamodb" -d 'DynamoDB playground'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from redis" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from mysql" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from dynamodb" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from dynamodb" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from clear; and __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "redis" -d 'Redis playground'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "postgres" -d 'Postgres playground'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "mysql" -d 'MySQL playground'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from redis; and not __fish_seen_subcommand_from postgres; and not __fish_seen_subcommand_from mysql; and not __fish_seen_subcommand_from dynamodb" -f -a "dynamodb" -d 'DynamoDB playground'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from redis" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s u -l user -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s p -l password -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s d -l database -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from mysql" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from dynamodb" -s P -l port -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from dynamodb" -s f -l file -d 'file path containing backends' -r
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from dynamodb" -l version -d 'Print version information'
complete -c oomplay -n "__fish_seen_subcommand_from stop; and __fish_seen_subcommand_from dynamodb" -s h -l help -d 'Print help information'
complete -c oomplay -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
