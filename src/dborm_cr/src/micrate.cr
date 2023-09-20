require "micrate"
require "pg"

Micrate::DB.connection_url = "postgresql://postgres:pass@127.0.0.1/postgres"
Micrate::Cli.run
