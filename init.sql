CREATE TABLE IF NOT EXISTS `tiny_link`(
    `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `origin_url` varchar(1024) NOT NULL COMMENT'原始链接',
    `tiny_code` varchar(10) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_tiny_code` (`tiny_code`) USING BTREE
    UNIQUE KEY `uk_origin_url` (`origin_url`) USING BTREE
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8 COMMENT ='短链接';