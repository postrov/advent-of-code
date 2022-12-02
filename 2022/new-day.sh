#!/bin/sh

if [ "$#" != 1 ]; then
  echo "usage: $0 <day>" >&2
  exit 1
fi

DAY_NUM=$1
if ! echo "$DAY_NUM" | grep -Eq '^[0-9]+$'; then
  echo "error: day must be a number" >&2
  exit 1
fi

TARGET_DIR=day-"$DAY_NUM"
if [ -d "$TARGET_DIR" ]; then
  echo "error: $TARGET_DIR already exists" >&2
  exit 1
fi

mkdir "$TARGET_DIR" || (echo "error: could not create $TARGET_DIR" >&2; exit 1)

cp -r day-template/* "$TARGET_DIR"
find "$TARGET_DIR" -iname \*.rs -a -type f | xargs sed -i "s/DAY_TEMPLATE_MOD/day_$1/g"
sed -i "s/DAY_TEMPLATE_PACKAGE/day_$1/g" "$TARGET_DIR"/Cargo.toml

