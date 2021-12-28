
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
            cand list 'List supported backends'
            cand completion 'Output shell completion code'
        }
        &'oomplay;init'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;stop'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'oomplay;list'= {
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
