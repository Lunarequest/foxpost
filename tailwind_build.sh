#!/bin/sh
if ! type "npx" > /dev/null; then
    echo "npx required exiting"
    exit 1
fi

npx tailwindcss -i ./tailwind/input.css -o ./static/css/output.css