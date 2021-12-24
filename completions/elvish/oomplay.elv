
use builtin;
use str;

set edit:completion:arg-completer[oomplay] = [@words]{
    fn spaces [n]{
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'oomplay'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'oomplay'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand init 'Initialize playgrounds'
            cand clear 'Clean up playgrounds'
            cand stop 'Stop playgrounds'
            cand completion 'Output shell completion code'
        }
        &'oomplay;init'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand redis 'Redis playground'
            cand postgres 'Postgres playground'
            cand mysql 'MySQL playground'
            cand dynamodb 'DynamoDB playground'
            cand cassandra 'Cassandra'
        }
        &'oomplay;init;redis'= {
            cand -P 'P'
            cand --port 'port'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;postgres'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;mysql'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;dynamodb'= {
            cand -P 'P'
            cand --port 'port'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;cassandra'= {
            cand -P 'P'
            cand --port 'port'
            cand -k 'k'
            cand --keyspace 'keyspace'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand redis 'Redis playground'
            cand postgres 'Postgres playground'
            cand mysql 'MySQL playground'
            cand dynamodb 'DynamoDB playground'
            cand cassandra 'Cassandra'
        }
        &'oomplay;clear;redis'= {
            cand -P 'P'
            cand --port 'port'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;postgres'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;mysql'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;dynamodb'= {
            cand -P 'P'
            cand --port 'port'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;cassandra'= {
            cand -P 'P'
            cand --port 'port'
            cand -k 'k'
            cand --keyspace 'keyspace'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand redis 'Redis playground'
            cand postgres 'Postgres playground'
            cand mysql 'MySQL playground'
            cand dynamodb 'DynamoDB playground'
            cand cassandra 'Cassandra'
        }
        &'oomplay;stop;redis'= {
            cand -P 'P'
            cand --port 'port'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;postgres'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;mysql'= {
            cand -P 'P'
            cand --port 'port'
            cand -u 'u'
            cand --user 'user'
            cand -p 'p'
            cand --password 'password'
            cand -d 'd'
            cand --database 'database'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;dynamodb'= {
            cand -P 'P'
            cand --port 'port'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;cassandra'= {
            cand -P 'P'
            cand --port 'port'
            cand -k 'k'
            cand --keyspace 'keyspace'
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}
