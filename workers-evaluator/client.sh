#!/bin/bash

case $# in
    0)
	curl -H "Content-Type: application/octet-stream" --data-binary @- https://program.ekez.workers.dev
    ;;
    1)
	curl -H "Content-Type: application/octet-stream" --data-binary @$1 https://program.ekez.workers.dev
	;;
    *)
	echo "usage: $0 <filename>?"
	;;
esac
