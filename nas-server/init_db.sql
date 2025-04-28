-- 初始化数据库表
-- 创建用户表

CREATE TABLE IF NOT EXISTS users ( username TEXT PRIMARY KEY -- 用户名 
 , password TEXT NOT NULL -- 密码 
 , avator TEXT -- 用户头像 (可选) 
 , identity TEXT --用户身份 
 , disksize INT --用户磁盘大小 
 );
INSERT INTO users (username, password, avator, identity) VALUES('root', 'adcd1234', '/avators/default.jpg', 'manager');