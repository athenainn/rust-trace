#!/bin/sh

NODEMON=/usr/local/bin/nodemon

WATCH_DIRS="."
IGNORES=""
EXTENSIONS="rs"
DELAY="1"

EXEC="bash -c"
APP="rustc --cfg test $1 && ./${1%%.rs}"
APP_ARGS=""

##
# $1 - option name
# $2 - argument list
#
function createItems()
{
    local OPTION=$1
    local LIST=$2
    local RESULT=""

    for i in $LIST; do
	if [ "$RESULT" = "" ]; then
	    RESULT="$OPTION $i"
	else
	    RESULT="$RESULT $OPTION $i"
	fi
    done

    echo $RESULT
}

WATCH_OPTION=$(createItems "--watch" "$WATCH_DIRS")
IGNORE_OPTION=$(createItems "--ignore" "$IGNORES")
EXTENSIONS_OPTION="-e "$(echo "$EXTENSIONS" | sed -e "s/[[:space:]]/,/g")

$NODEMON \
    $WATCH_OPTION \
    $IGNORE_OPTION \
    $EXTENSIONS_OPTION \
    --delay "$DELAY" \
    --exec "$EXEC" \
    "$APP" -- "$APP_ARGS"
