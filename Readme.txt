重启
/usr/local/pgsql/bin/pg_ctl restart -D /usr/local/pgsql/data


ssh root@121.42.218.61
BSL1qaz@WSX


alter user postgres with password '1988lm';
GRANT ALL PRIVILEGES ON DATABASE jqjd to postgres;

netstat -tunl：查看所有正在监听的端口
netstat  -tunp：查看所有已连接的网络连接状态，并显示其PID及程序名称。