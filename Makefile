psql:
	psql -h localhost -p 5432 -d sample -U postgres

generate:
	diesel migration generate $(name)

run:
	diesel migration run

redo:
	diesel migration redo
