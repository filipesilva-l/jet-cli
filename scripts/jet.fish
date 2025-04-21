function jet
    set -x JET_SHELL true

    set new_dir (/Users/filipe/mine/jet-cli/target/debug/jet-cli $argv)

    if test $status -eq 0
        cd $new_dir
    end
end
