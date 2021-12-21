
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
            cand stop 'Stop playgrounds'
            cand completion 'Output shell completion code'
        }
        &'oomplay;init'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand redis 'Redis store'
            cand postgres 'Postgres store'
            cand mysql 'Mysql store'
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
        &'oomplay;stop'= {
            cand -f 'file path containing backends'
            cand --file 'file path containing backends'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand redis 'Redis store'
            cand postgres 'Postgres store'
            cand mysql 'Mysql store'
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
        &'oomplay;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}
