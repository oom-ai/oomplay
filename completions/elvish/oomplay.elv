
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
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;postgres'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;mysql'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;dynamodb'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;init;cassandra'= {
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
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;postgres'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;mysql'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;dynamodb'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;clear;cassandra'= {
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
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;postgres'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;mysql'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;dynamodb'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop;cassandra'= {
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
