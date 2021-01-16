#!/usr/bin/env bash

if [ "x$WRITE_NGINX_CONFIG" == xtrue ]; then
	if [ "x$NGINX_CONFIG_NAME" == x ]; then
		NGINX_CONFIG_NAME=anyorigin
		if [ "x$TAG" == x ]; then
			NGINX_CONFIG_NAME="$NGINX_CONFIG_NAME.conf"
		else
			NGINX_CONFIG_NAME="$NGINX_CONFIG_NAME-$TAG.conf"
		fi
	fi
	mkdir -p "$NGINX_CONFIG_DIR"
	FILE="$NGINX_CONFIG_DIR/$NGINX_CONFIG_NAME"
	sleep "${DELAY:-0}"
	cat /anyorigin.conf.in | sed -e "s={{NGINX_HOST}}=$NGINX_HOST=g; s={{NGINX_PORT}}=$NGINX_PORT=g; s={{HOST}}=$HOST=g; s={{PORT}}=$PORT=g" > "$FILE"
	trap "rm -r $FILE" EXIT
fi

anyorigin --address "$HOST:$PORT" --log-level "$LOG_LEVEL"









