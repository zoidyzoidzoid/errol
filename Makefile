all: data db run

.env:
	cp .env.example .env

data: .env
	initdb data

db: data
	pg_ctl -D data -l logfile start

