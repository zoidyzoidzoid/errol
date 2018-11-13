#!/usr/bin/env bash
sudo -u errol DJANGO_SETTINGS_MODULE=config.settings.staging DJANGO_READ_DOT_ENV_FILE=True virtualenv/bin/gunicorn -w 4 config.wsgi 
