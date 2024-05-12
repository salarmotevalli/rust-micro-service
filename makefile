
SHELL           := /bin/bash
.DEFAULT_GOAL   := help
.SILENT:        #don't echo commands as they run
# ==============================================================================
-include .env
DIR             := ${CURDIR}
UNAME           := `uname`
UUID            := `id -u`
GUID            := `id -g`

ifeq (Darwin, $(findstring Darwin, $(shell uname -a)))
    PLATFORM    := OSX
    OPEN        := open
else
    PLATFORM    := Linux
    OPEN        := xdg-open
endif

# Declare Variables
ifdef APP_ENV
    ENV ?= ${APP_ENV}
else
    ENV ?= "local"
endif


.PHONY: help
help:   # List helpful commands
	echo ''
	echo 'Makefile for '
	echo ' build:order                        build order image'
	echo ''

build\:order:
	cd order && docker build -t ${ORDER_APP_NAME}:${ORDER_APP_VERSION} -f Dockerfile . --build-arg ORDER_APP_NAME=${ORDER_APP_NAME}

run\:broker-http:
	cd broker && BROKER_SERVICE_PORT=8080 cargo run --bin http-server

run\:order-http:
	cd order && MONGODB_URI="mongodb://admin:admin@0.0.0.0:27017/" ORDER_SERVICE_PORT=8081 cargo run --bin http-server

run\:order-grpc:
	cd order && MONGODB_URI="mongodb://admin:admin@0.0.0.0:27017/" ORDER_SERVICE_PORT=8081 cargo run --bin grpc-server

