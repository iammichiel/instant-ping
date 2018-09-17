-- Your SQL goes here

CREATE TABLE `dashboards` (
  `id_dashboard` VARCHAR(36) NOT NULL,
  `last_access` datetime DEFAULT NULL,
  PRIMARY KEY (`id_dashboard`)
);

CREATE TABLE `certificates` (
  `id_certificate` VARCHAR(36) NOT NULL,
  `domain` varchar(255) NOT NULL,
  `last_check` datetime DEFAULT NULL,
  `dashboard_id` varchar(36)  NOT NULL,
  PRIMARY KEY (`id_certificate`),
  KEY `fk_certificate_dashboard` (`dashboard_id`),
  CONSTRAINT `fk_certificate_dashboard` FOREIGN KEY (`dashboard_id`) REFERENCES `dashboards` (`id_dashboard`) ON DELETE CASCADE ON UPDATE CASCADE
);