complete -c grab-xkcd -s t -l timeout -d 'Set a connection timeout' -r
complete -c grab-xkcd -s o -l output -d 'Print output in a format' -r -f -a "json\t''
text\t''"
complete -c grab-xkcd -s n -l num -d 'The comic to load' -r
complete -c grab-xkcd -s s -l save -d 'Save image file to current directory'
complete -c grab-xkcd -s h -l help -d 'Print help'
