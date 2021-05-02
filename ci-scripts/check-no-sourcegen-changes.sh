#!/usr/bin/env bash

set -e

SCRIPT=`realpath $0`
SCRIPTPATH=`dirname $SCRIPT`

if ( (!(git diff --exit-code) || !(git diff --cached --exit-code)) 2>&1 > /dev/null ); then
	echo 'Cannot run script; working directory has been modified'
	exit 1
fi

"$SCRIPTPATH"/../sourcegen.sh

if ( (!(git diff --exit-code) || !(git diff --cached --exit-code)) 2>&1 > /dev/null ); then
	echo 'Uncommitted code generation changes detected. Run sourcegen.sh and commit the result to fix.'
	exit 2
fi
