
use builtin;
use str;

set edit:completion:arg-completer[grab-xkcd] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'grab-xkcd'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'grab-xkcd'= {
            cand -t 'Set a connection timeout'
            cand --timeout 'Set a connection timeout'
            cand -o 'Print output in a format'
            cand --output 'Print output in a format'
            cand -n 'The comic to load'
            cand --num 'The comic to load'
            cand -s 'Save image file to current directory'
            cand --save 'Save image file to current directory'
            cand -h 'Print help'
            cand --help 'Print help'
        }
    ]
    $completions[$command]
}
