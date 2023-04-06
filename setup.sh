#!/bin/bash

# USER INPUT
echo -e '\033[0;35m🦄 SETUP \033[0m-> postgres \033[0;36musername\033[0m'
echo -n -e '\033[0;32m❯\033[0m '
read username
echo -e '\033[0;35m🦄 SETUP \033[0m-> postgres \033[0;36mpassword\033[0m 🤫'
echo -n -e '\033[0;32m❯\033[0m '
read -s password # hide password
echo # new line after password
echo -e '\033[0;35m🦄 SETUP \033[0m-> postgres \033[0;36mdatabase\033[0m'
echo -n -e '\033[0;32m❯\033[0m '
read database

# TIME START
start=$(date +%s%N)

# ACTUAL SETUP
sudo apt-get install postgresql postgresql-contrib libpq-dev
sudo -u postgres createuser $username
sudo -u postgres createdb $database
sudo -u postgres psql -d postgres -c "ALTER USER \"$username\" WITH PASSWORD '$password';"
sudo -u postgres psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE \"$database\" TO \"$username\";"
cargo install diesel_cli --no-default-features --features postgres
echo -e "DATABASE_URL=postgres://$username:$password@localhost/$database\nDIESEL_CONFIG_FILE=./diesel.toml" > .env
diesel setup
diesel migration run

# TIME END
end=$(date +%s%N)

# CONGRATS 🎉
echo -e "\033[0;35m🦄 SETUP\033[0m completed in \033[0;32m$(($(($end-$start))/1000000)) ms\033[0m 🎉"
