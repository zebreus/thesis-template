#/usr/bin/env bash

CHART_NAME="$1"

if test -z "$CHART_NAME"; then
    echo "Please provide a chart name" 1>&2
    exit 1
fi

ORIGINAL_CHART="$CHART_NAME"

if ! test -f "$ORIGINAL_CHART"; then
    echo "$ORIGINAL_CHART does not exist" 1>&2
    exit 1
fi

CHART_DIR=$(dirname $ORIGINAL_CHART)

while read -r line; do
    if [[ $line == *'"url":'* ]]; then
        FILENAME=$CHART_DIR/$(echo $line | grep -Po '"url": "\K[^"]*')
        echo '"values": '
        cat "$FILENAME" | python -c 'import csv, json, sys; print(json.dumps([dict(r) for r in csv.DictReader(sys.stdin, quoting=csv.QUOTE_NONNUMERIC)]))' | jq
    else
        echo "$line"
    fi
done < <(
    cat $ORIGINAL_CHART
    echo ""
)
