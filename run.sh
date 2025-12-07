#!/bin/sh

DURATION=60
END_TIME=$(($(date +%s) + $DURATION))
TEMP_DIR=/tmp/grasp
RESULT=${1:-result.txt}
if [ -n "$1" ]; then shift 1; fi
PARAMS=${@:-"677 195 0.0152"}


rm -rf $TEMP_DIR
mkdir -p $TEMP_DIR

for i in $(seq $(nproc)); do
    TEMP_RESULT=$TEMP_DIR/result_$i.txt
    while [ $(date +%s) -lt $END_TIME ]; do
        # TODO: Think a better way to run other instances.
        ./target/release/graphs-algorithms $PARAMS >> $TEMP_RESULT
    done &
done

wait

cat $TEMP_DIR/result_*.txt > $RESULT
