CREATE TABLE IF NOT EXISTS `tiny_link`(
    `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `origin_url` longtext NOT NULL COMMENT'原始链接',
    `url_sha1` char(40) NOT NULL  COMMENT  'url哈希',
    `tiny_code` varchar(10) DEFAULT NULL,
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    INDEX (url_sha1),
    UNIQUE KEY `uk_tiny_code` (`tiny_code`) USING BTREE
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8 COMMENT ='短链接';