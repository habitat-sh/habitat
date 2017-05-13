# Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
#
# The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
# and the party accessing this file ("Licensee") apply to Licensee's use of
# the Software until such time that the Software is made available under an
# open source license such as the Apache 2.0 License.

# This is a bash completion file for the Habitat `hab` command. It requires
# a "newer" version of bash-completion, so if you see an error such as
# "_get_comp_words_by_ref: command not found", try sourcing the Habitat
# bash-completion package via:
#   source "`hab pkg path core/bash-completion`/etc/profile.d/bash_completion.sh"

# bash_completion for hab
_hab()
{
    local cur prev
    _get_comp_words_by_ref cur prev

    COMPREPLY=()
    cur=${COMP_WORDS[COMP_CWORD]}
    prev=${COMP_WORDS[COMP_CWORD-1]}
    len=${#COMP_WORDS[@]}
    if [ $len -gt 2 ]
    then
        minus2=${COMP_WORDS[COMP_CWORD-2]}
    fi

    if [ $COMP_CWORD -eq 1 ]; then
        case $prev in
            hab)
                COMPREPLY=( $( compgen -W "apply artifact config file help install origin pkg ring svc setup start studio sup user" -- "$cur" ) )
                ;;
                   esac
    elif [ $COMP_CWORD -eq 2 ]; then
        case "$prev" in
            artifact)
                cmds=( "hash help sign upload verify" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            cli)
                cmds=( "help setup" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            config)
                cmds=( "apply help" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            file)
                cmds=( "help upload" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            origin)
                cmds=( "help key" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            pkg)
                cmds=( "binlink build exec export help install path" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            ring)
                cmds=( "help key" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            svc)
                cmds=( "help key" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            studio)
                cmds=( "build enter help new rm run version" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            sup)
                cmds=( "bash config help sh start" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;
            user)
                cmds=( "help key" )
                COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                ;;


        esac
    elif [ $COMP_CWORD -eq 3 ]; then
        case "$minus2" in
            origin)
                case "$prev" in
                    key) #hab origin key
                        cmds=( "download export generate help import upload" )
                        COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                    ;;
                esac
            ;;
            ring) # hab ring key
                case "$prev" in
                    key)
                        cmds=( "export generate help import" )
                        COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                    ;;
                esac
            ;;
            svc) # hab svc key
                case "$prev" in
                    key)
                        cmds=( "generate help" )
                        COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                    ;;
                esac
            ;;
            user) # hab user key
                case "$prev" in
                    key)
                        cmds=( "generate help" )
                        COMPREPLY=( $( compgen -W "$cmds" -- "$cur") )
                    ;;
                esac
            ;;
        esac
    fi
}
complete -F _hab hab
