
This project is now managed by [DataCEO](https://github.com/DataCEO) and is relocated to https://github.com/DataCEO/AgentHub-CodeChainServer

AgentHub-CodeChainServer
====================

[![Join the chat at https://gitter.im/DataCEO/AgentHub-CodeChainServer](https://badges.gitter.im/DataCEO/AgentHub-CodeChainServer.svg)](https://gitter.im/DataCEO/AgentHub-CodeChainServer?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

AgentHub-CodeChainServer is a server which collects many CodeChain node's information(best block, pending parcels, log, ...). Also, AgentHub-CodeChainServer serves collected data to CodeChain Dashboard.

Install
--------

You should set up a rust toolchain.

You can install codechain-agent-hub by running `cargo install`

Install Postgres and create schema
-----------------

Ubuntu
```
sudo apt install postgresql postgresql-contrib
sudo -u postgres psql -f create_user_and_db.sql
generate-schema
```

Mac (brew)
```
brew install postgresql
brew services start postgresql
psql postgres -f create_user_and_db.sql
generate-schema
```

Run
----

Just run `codechain-agent-hub` in your shell. 
To safely communicate with the Dashboard, please set the `PASSPHRASE` environment variable. The Dashboard program should use the same passphrase.

When you are using the `PASSPHRASE` you should use SSL over the connection. If you don't use the SSL, the `PASSPHRASE` is open to the internet. 

AgentHub-CodeChainServer will listen 3012 port to communicate with the Dashboard using JSON-RPC.

AgentHub-CodeChainServer will listen 4012 port to communicate with the Agent using JSON-RPC.

AgentHub-CodeChainServer will listen 5012 port to serve CodeChain's log file using HTTP.