-- --------------------------------------------------------
-- Host:                         127.0.0.1
-- Versión del servidor:         8.4.3 - MySQL Community Server - GPL
-- SO del servidor:              Win64
-- HeidiSQL Versión:             12.8.0.6908
-- --------------------------------------------------------

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET NAMES utf8 */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

-- Volcando estructura para tabla growvent_staltos.airlines
CREATE TABLE IF NOT EXISTS `airlines` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `contraction` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.airlines: ~16 rows (aproximadamente)
INSERT INTO `airlines` (`id`, `name`, `contraction`, `created_at`, `updated_at`) VALUES
	(1, 'Aeroméxico', 'AM', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(2, 'Volaris', 'Y4', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(3, 'Viva Aerobus', 'VB', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(4, 'Interjet', '4O', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(5, 'Aeromar', 'VW', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(6, 'TAR Aerolíneas', 'YQ', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(7, 'Magnicharters', 'UJ', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(8, 'American Airlines', 'AA', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(9, 'United Airlines', 'UA', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(10, 'Delta Airlines', 'DL', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(11, 'Copa Airlines', 'CM', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(12, 'Avianca', 'AV', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(13, 'LATAM Airlines', 'LA', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(14, 'Iberia', 'IB', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(15, 'Air Canada', 'AC', '2025-12-16 21:04:56', '2025-12-16 21:04:56'),
	(16, 'Volaris', 'Y4', '2026-05-03 04:25:27', '2026-05-03 04:25:27');

-- Volcando estructura para tabla growvent_staltos.bank_accounts
CREATE TABLE IF NOT EXISTS `bank_accounts` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `clabe` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `account` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `card` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `owner` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `bank` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.bank_accounts: ~2 rows (aproximadamente)
INSERT INTO `bank_accounts` (`id`, `clabe`, `account`, `card`, `owner`, `bank`, `created_at`, `updated_at`) VALUES
	(1, 'CLABE BANCARIA 1', 'CTA1', 'TARJETA 1', 'Grow Travel', 'BBVA', '2025-12-16 21:15:52', '2025-12-16 21:16:18'),
	(2, 'CLABE BANCARIA 2', 'CTA 2', 'TARJETA 2', 'Grow Travel', 'Santander', '2025-12-16 21:16:11', '2025-12-16 21:16:11');

-- Volcando estructura para tabla growvent_staltos.benefit_transaction_settings
CREATE TABLE IF NOT EXISTS `benefit_transaction_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Egreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `benefit_transaction_settings_group_id_foreign` (`group_id`),
  KEY `benefit_transaction_settings_classification_id_foreign` (`classification_id`),
  KEY `benefit_transaction_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `benefit_transaction_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `benefit_transaction_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `benefit_transaction_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `benefit_transaction_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `benefit_transaction_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.benefit_transaction_settings: ~1 rows (aproximadamente)
INSERT INTO `benefit_transaction_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 30, 3, 4, 2, 'Pago de Prestaciones', 'Egreso', 'Pago de prestaciones semanales', 1, '2025-12-16 21:18:21', '2025-12-16 21:18:23');

-- Volcando estructura para tabla growvent_staltos.bonus_transaction_settings
CREATE TABLE IF NOT EXISTS `bonus_transaction_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Egreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `bonus_transaction_settings_group_id_foreign` (`group_id`),
  KEY `bonus_transaction_settings_classification_id_foreign` (`classification_id`),
  KEY `bonus_transaction_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `bonus_transaction_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `bonus_transaction_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `bonus_transaction_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `bonus_transaction_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `bonus_transaction_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.bonus_transaction_settings: ~1 rows (aproximadamente)
INSERT INTO `bonus_transaction_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 12, 2, 4, 2, 'Pago de Bonos', 'Egreso', 'Pago de bonos semanales', 1, '2025-12-16 21:18:46', '2025-12-16 21:18:48');

-- Volcando estructura para tabla growvent_staltos.cache
CREATE TABLE IF NOT EXISTS `cache` (
  `key` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `expiration` int NOT NULL,
  PRIMARY KEY (`key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.cache: ~1 rows (aproximadamente)
INSERT INTO `cache` (`key`, `value`, `expiration`) VALUES
	('settings_reservation_config', 'O:18:"App\\Models\\Setting":30:{s:13:"\0*\0connection";s:5:"mysql";s:8:"\0*\0table";s:8:"settings";s:13:"\0*\0primaryKey";s:2:"id";s:10:"\0*\0keyType";s:3:"int";s:12:"incrementing";b:1;s:7:"\0*\0with";a:0:{}s:12:"\0*\0withCount";a:0:{}s:19:"preventsLazyLoading";b:0;s:10:"\0*\0perPage";i:15;s:6:"exists";b:1;s:18:"wasRecentlyCreated";b:0;s:28:"\0*\0escapeWhenCastingToString";b:0;s:13:"\0*\0attributes";a:14:{s:2:"id";i:2;s:4:"name";s:11:"Grow Travel";s:7:"address";s:34:"Dirección Principal #510, Jalisco";s:5:"phone";s:16:"+52 55 9110 8340";s:5:"email";s:18:"info@growventa.com";s:5:"terms";s:5426:"EL CLIENTE reconoce que GROW TRAVEL (en lo sucesivo actúa\núnicamente como intermediaria de los prestadores de servicios con los que ha contratado EL CLIENTE y a los que\nse refiere(n) el boleto, recibo y/o voucher y demás documentación que se expide(n) al CLIENTE con motivo de los\nservicios, los cuales estarán sujetos a los términos y condiciones especificados por los prestadores de servicios\ncontratados, incluyendo de manera enunciativa más no limitadas cars por cancelaciones, perdidas de vuelos,\nperdidas de equipaje, entre otros. Los mencionados servicios serán proporcionados por prestadores\nindependientes que no actúan por o en representación de GROW TRAVEL, y en este\nacto, EL CLIENTE se hace expresamente conocedor de esta situación, así´ mismo acepta que ni GROW TRAVEL, ni sus subsidiarias, filiales, controladoras o representantes tienen injerencia alguna\nen la presentación de los servicios que ha contratado. En este acto EL CLIENTE libera a GROW TRAVEL, así´ como a sus accionistas, consejeros, afiliadas, subsidiarias, asesores, factores,\ndependientes, o partes relacionadas, de todo tipo de responsabilidad generada con motivo de cualesquier\nincumplimiento, hecho, acto omisión, accidente o lesión vinculada y/o derivada de los servicios que ha contratado\nEL CLIENTE, incluyendo daños y perjuicios, ya sean directos, indirectos, incidentales, económicos, y/o negligentes,\nasí´ como pérdidas, costos o gastos que pudieren ser generados con motivo o en relación a los servicios\ncontratados a través de Empresa. Toda reservación de hotel, paquetes, vuelos expedición de boletos y demás\ndocumentación genera un honorario por servicio y no son reembolsables bajo ninguna circunstancia en caso de\ncancelación. Para reservar los paquetes, en su mayoría se requiere de un depósito que puede variar de acuerdo al\noperador, destino, o temporada en que se quiere viajar este depósito inicial NO es reembolsable. La reservación de\nlos circuitos que ya estén dentro del período de cars, requerirá´ del pa total de los servicios. Una vez confirmados\nlos servicios estarán sujetos al tiempo límite para pa total que indique el operador, quedando sujetos a sus políticas\nde cars que pueden ir desde $100.00 USD por persona por trámites administrativos; hasta del 50 o 100% del\npaquete; los cuales serán cobrados de acuerdo a cada caso. Los depósitos no son reembolsables en caso de\ncualquier cancelación, según políticas del operador responsable. Los boletos de avión con tarifa económica son: •\nNo reembolsables. • No endosables. • No transferibles (esto quiere decir que no se aceptan cambios de nombres).\nTodas las aerolíneas se reservan el derecho de sobrevender, cambiar horarios, e incluso cancelar o realizar\ncambios que ellos consideren pertinentes sin previo aviso y sin responsabilidad de [Empresa]; esto esta´ autorizado\ngubernamentalmente. Los programas con vuelos tipo cha´rter, inter-asia´ticos e inter-sudamericanos no son\nreembolsables y no permiten cambios de ningún tipo, según políticas del operador. En caso de que tenga que\ninterrumpir su viaje por causas personales, los servicios total o parcialmente no utilizados no serán reembolsables.\nTodos los viajeros deberán contar con la documentación necesaria para realizar su viaje; pasaportes vigentes,\nvisas, siendo de su total responsabilidad los problemas e inconvenientes que puedan surgir por incumplimientos de\neste requerimiento. Los cambios o cancelación de los viajes ocasionados por causas originadas por huracanes,\nciclones, terremotos o cualquier otra, son ajenos a [Empresa], y quedarán sujetos a las políticas y lineamientos de\nla línea aérea, transportista u operador para la cancelación total de viaje o cambios de itinerario. Los reembolsos\nparciales o totales dependerán de la línea aérea, transportista u operador. Recibí de conformidad y de manera\noportuna, profesional y con la calidad necesaria los documentos de viaje, aceptando que corresponden a las\nfechas, nombres de pasajeros y horarios acordados de acuerdo con las características y necesidades del servicio.\nCausales de rescisión y consecuencias jurídicas: EL CLIENTE acepta por falta de pa total o parcial de los servicios\nque ha contratado a través de GROW TRAVEL será´ causa suficiente de cancelación\no rescisión del contrato y de los servicios contratados por EL CLIENTE con los diversos prestadores mencionados\nque pudieran ser de manera enunciativa más no limitativa: líneas aéreas, hoteles, transportistas terrestres,\nmarítimos u operadores, etc. por lo que en este acto EL CLIENTE libera en todo momento a GROW TRAVEL, a sus accionistas, consejeros, afiliados, subsidiarias, asesores, factores\ndependientes o partes relacionadas de cualquier responsabilidad que por su incumplimiento (DEL CLIENTE)\npudiera corresponder. "GROW TRAVEL”\ncon domicilio en DIRECCION PRINCIPAL #510, JALISCO En cumplimiento a la Ley Federal de Protección de Datos Personales en\nPosesión de los Particulares (la “Ley”), y su reglamento, hacen de su conocimiento que: "GROW TRAVEL” protegemos y salvaguardamos sus datos\npersonales para evitar el daño, pérdida, destrucción, robo, extravío, alteración, así como el tratamiento no\nautorizado de sus datos personales. En esta sección, encontrarás los términos y condiciones aplicables a cada\npromoción publicada dentro de nuestro sitio.";s:18:"reservation_prefix";s:2:"GT";s:18:"reservation_format";s:20:"[IDENTIFICADOR]-[ID]";s:25:"reservation_id_pad_length";i:5;s:17:"profit_percentage";s:5:"20.00";s:23:"profit_include_in_total";i:1;s:16:"enable_birthdate";i:1;s:10:"created_at";s:19:"2025-12-16 16:15:31";s:10:"updated_at";s:19:"2026-03-17 01:53:54";}s:11:"\0*\0original";a:14:{s:2:"id";i:2;s:4:"name";s:11:"Grow Travel";s:7:"address";s:34:"Dirección Principal #510, Jalisco";s:5:"phone";s:16:"+52 55 9110 8340";s:5:"email";s:18:"info@growventa.com";s:5:"terms";s:5426:"EL CLIENTE reconoce que GROW TRAVEL (en lo sucesivo actúa\núnicamente como intermediaria de los prestadores de servicios con los que ha contratado EL CLIENTE y a los que\nse refiere(n) el boleto, recibo y/o voucher y demás documentación que se expide(n) al CLIENTE con motivo de los\nservicios, los cuales estarán sujetos a los términos y condiciones especificados por los prestadores de servicios\ncontratados, incluyendo de manera enunciativa más no limitadas cars por cancelaciones, perdidas de vuelos,\nperdidas de equipaje, entre otros. Los mencionados servicios serán proporcionados por prestadores\nindependientes que no actúan por o en representación de GROW TRAVEL, y en este\nacto, EL CLIENTE se hace expresamente conocedor de esta situación, así´ mismo acepta que ni GROW TRAVEL, ni sus subsidiarias, filiales, controladoras o representantes tienen injerencia alguna\nen la presentación de los servicios que ha contratado. En este acto EL CLIENTE libera a GROW TRAVEL, así´ como a sus accionistas, consejeros, afiliadas, subsidiarias, asesores, factores,\ndependientes, o partes relacionadas, de todo tipo de responsabilidad generada con motivo de cualesquier\nincumplimiento, hecho, acto omisión, accidente o lesión vinculada y/o derivada de los servicios que ha contratado\nEL CLIENTE, incluyendo daños y perjuicios, ya sean directos, indirectos, incidentales, económicos, y/o negligentes,\nasí´ como pérdidas, costos o gastos que pudieren ser generados con motivo o en relación a los servicios\ncontratados a través de Empresa. Toda reservación de hotel, paquetes, vuelos expedición de boletos y demás\ndocumentación genera un honorario por servicio y no son reembolsables bajo ninguna circunstancia en caso de\ncancelación. Para reservar los paquetes, en su mayoría se requiere de un depósito que puede variar de acuerdo al\noperador, destino, o temporada en que se quiere viajar este depósito inicial NO es reembolsable. La reservación de\nlos circuitos que ya estén dentro del período de cars, requerirá´ del pa total de los servicios. Una vez confirmados\nlos servicios estarán sujetos al tiempo límite para pa total que indique el operador, quedando sujetos a sus políticas\nde cars que pueden ir desde $100.00 USD por persona por trámites administrativos; hasta del 50 o 100% del\npaquete; los cuales serán cobrados de acuerdo a cada caso. Los depósitos no son reembolsables en caso de\ncualquier cancelación, según políticas del operador responsable. Los boletos de avión con tarifa económica son: •\nNo reembolsables. • No endosables. • No transferibles (esto quiere decir que no se aceptan cambios de nombres).\nTodas las aerolíneas se reservan el derecho de sobrevender, cambiar horarios, e incluso cancelar o realizar\ncambios que ellos consideren pertinentes sin previo aviso y sin responsabilidad de [Empresa]; esto esta´ autorizado\ngubernamentalmente. Los programas con vuelos tipo cha´rter, inter-asia´ticos e inter-sudamericanos no son\nreembolsables y no permiten cambios de ningún tipo, según políticas del operador. En caso de que tenga que\ninterrumpir su viaje por causas personales, los servicios total o parcialmente no utilizados no serán reembolsables.\nTodos los viajeros deberán contar con la documentación necesaria para realizar su viaje; pasaportes vigentes,\nvisas, siendo de su total responsabilidad los problemas e inconvenientes que puedan surgir por incumplimientos de\neste requerimiento. Los cambios o cancelación de los viajes ocasionados por causas originadas por huracanes,\nciclones, terremotos o cualquier otra, son ajenos a [Empresa], y quedarán sujetos a las políticas y lineamientos de\nla línea aérea, transportista u operador para la cancelación total de viaje o cambios de itinerario. Los reembolsos\nparciales o totales dependerán de la línea aérea, transportista u operador. Recibí de conformidad y de manera\noportuna, profesional y con la calidad necesaria los documentos de viaje, aceptando que corresponden a las\nfechas, nombres de pasajeros y horarios acordados de acuerdo con las características y necesidades del servicio.\nCausales de rescisión y consecuencias jurídicas: EL CLIENTE acepta por falta de pa total o parcial de los servicios\nque ha contratado a través de GROW TRAVEL será´ causa suficiente de cancelación\no rescisión del contrato y de los servicios contratados por EL CLIENTE con los diversos prestadores mencionados\nque pudieran ser de manera enunciativa más no limitativa: líneas aéreas, hoteles, transportistas terrestres,\nmarítimos u operadores, etc. por lo que en este acto EL CLIENTE libera en todo momento a GROW TRAVEL, a sus accionistas, consejeros, afiliados, subsidiarias, asesores, factores\ndependientes o partes relacionadas de cualquier responsabilidad que por su incumplimiento (DEL CLIENTE)\npudiera corresponder. "GROW TRAVEL”\ncon domicilio en DIRECCION PRINCIPAL #510, JALISCO En cumplimiento a la Ley Federal de Protección de Datos Personales en\nPosesión de los Particulares (la “Ley”), y su reglamento, hacen de su conocimiento que: "GROW TRAVEL” protegemos y salvaguardamos sus datos\npersonales para evitar el daño, pérdida, destrucción, robo, extravío, alteración, así como el tratamiento no\nautorizado de sus datos personales. En esta sección, encontrarás los términos y condiciones aplicables a cada\npromoción publicada dentro de nuestro sitio.";s:18:"reservation_prefix";s:2:"GT";s:18:"reservation_format";s:20:"[IDENTIFICADOR]-[ID]";s:25:"reservation_id_pad_length";i:5;s:17:"profit_percentage";s:5:"20.00";s:23:"profit_include_in_total";i:1;s:16:"enable_birthdate";i:1;s:10:"created_at";s:19:"2025-12-16 16:15:31";s:10:"updated_at";s:19:"2026-03-17 01:53:54";}s:10:"\0*\0changes";a:0:{}s:8:"\0*\0casts";a:2:{s:23:"profit_include_in_total";s:7:"boolean";s:16:"enable_birthdate";s:7:"boolean";}s:17:"\0*\0classCastCache";a:0:{}s:21:"\0*\0attributeCastCache";a:0:{}s:13:"\0*\0dateFormat";N;s:10:"\0*\0appends";a:1:{i:0;s:3:"key";}s:19:"\0*\0dispatchesEvents";a:0:{}s:14:"\0*\0observables";a:0:{}s:12:"\0*\0relations";a:0:{}s:10:"\0*\0touches";a:0:{}s:10:"timestamps";b:1;s:13:"usesUniqueIds";b:0;s:9:"\0*\0hidden";a:0:{}s:10:"\0*\0visible";a:0:{}s:11:"\0*\0fillable";a:11:{i:0;s:4:"name";i:1;s:7:"address";i:2;s:5:"phone";i:3;s:5:"email";i:4;s:5:"terms";i:5;s:18:"reservation_prefix";i:6;s:18:"reservation_format";i:7;s:25:"reservation_id_pad_length";i:8;s:17:"profit_percentage";i:9;s:23:"profit_include_in_total";i:10;s:16:"enable_birthdate";}s:10:"\0*\0guarded";a:1:{i:0;s:1:"*";}}', 1788390869);

-- Volcando estructura para tabla growvent_staltos.cache_locks
CREATE TABLE IF NOT EXISTS `cache_locks` (
  `key` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `owner` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `expiration` int NOT NULL,
  PRIMARY KEY (`key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.cache_locks: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.channels
CREATE TABLE IF NOT EXISTS `channels` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.channels: ~4 rows (aproximadamente)
INSERT INTO `channels` (`id`, `name`, `created_at`, `updated_at`) VALUES
	(1, 'Redes sociales', '2025-12-16 21:05:02', '2025-12-16 21:05:02'),
	(2, 'Llamada', '2025-12-16 21:05:02', '2025-12-16 21:05:02'),
	(3, 'Agencia', '2025-12-16 21:05:02', '2025-12-16 21:05:02'),
	(4, 'Referido', '2025-12-16 21:05:02', '2025-12-16 21:05:02');

-- Volcando estructura para tabla growvent_staltos.classifications
CREATE TABLE IF NOT EXISTS `classifications` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.classifications: ~10 rows (aproximadamente)
INSERT INTO `classifications` (`id`, `name`, `created_at`, `updated_at`) VALUES
	(1, 'Gastos Fijos', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(2, 'Gastos Variables', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(3, 'Gastos Administrativos', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(4, 'Gastos Financieros', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(5, 'Gastos Operativos', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(6, 'Gastos No Operativos', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(7, 'Gastos De Mantenimiento', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(8, 'Ingresos', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(9, 'Obligaciones Fiscales', '2025-12-16 21:05:13', '2025-12-16 21:05:13'),
	(10, 'Gastos De Venta', '2025-12-16 21:05:13', '2025-12-16 21:05:13');

-- Volcando estructura para tabla growvent_staltos.comments
CREATE TABLE IF NOT EXISTS `comments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `lead_id` bigint unsigned DEFAULT NULL,
  `lead_group_id` bigint unsigned DEFAULT NULL,
  `user_id` bigint unsigned DEFAULT NULL,
  `file_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `comments_lead_id_foreign` (`lead_id`),
  KEY `comments_user_id_foreign` (`user_id`),
  KEY `comments_lead_group_id_foreign` (`lead_group_id`),
  CONSTRAINT `comments_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `comments_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `comments_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.comments: ~6 rows (aproximadamente)
INSERT INTO `comments` (`id`, `description`, `lead_id`, `lead_group_id`, `user_id`, `file_name`, `file_path`, `created_at`, `updated_at`) VALUES
	(1, 'El cliente vino a solicitar su cancelación, pero después lo convencimos de agregar un paquete adicional para llevar a sus niños', 7, NULL, 12, NULL, NULL, '2025-12-16 22:45:45', '2025-12-16 22:45:45'),
	(2, 'Gracias por la aclaración', 7, NULL, 10, 'sample-local-pdf.pdf', 'https://growventa-public.s3.amazonaws.com/growtravel/m9lwgo1HQKG2jsm41zpq8WET513dr0RNicTLRW4f.pdf', '2025-12-16 22:46:21', '2025-12-16 22:46:21'),
	(3, 'Vine Eleazar y me dijo que no podía dar el anticipo hasta el 1 de febrero', 10, NULL, 10, NULL, NULL, '2026-01-29 22:49:47', '2026-01-29 22:49:47'),
	(4, 'Oye Martha vino el cliente y preguntó por su pase de abordar, se lo puedes enviar', 7, NULL, 10, NULL, NULL, '2026-01-30 21:35:43', '2026-01-30 21:35:43'),
	(5, 'Oye vino el cliente y me pidio cambiar la cotización', 18, NULL, 10, NULL, NULL, '2026-03-26 22:04:42', '2026-03-26 22:04:42'),
	(6, 'Vino el cliente y pidio agregarle una noche mas', 24, NULL, 10, NULL, NULL, '2026-05-09 06:54:14', '2026-05-09 06:54:14');

-- Volcando estructura para tabla growvent_staltos.commission_transaction_settings
CREATE TABLE IF NOT EXISTS `commission_transaction_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Egreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `commission_transaction_settings_group_id_foreign` (`group_id`),
  KEY `commission_transaction_settings_classification_id_foreign` (`classification_id`),
  KEY `commission_transaction_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `commission_transaction_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `commission_transaction_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `commission_transaction_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `commission_transaction_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `commission_transaction_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.commission_transaction_settings: ~1 rows (aproximadamente)
INSERT INTO `commission_transaction_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 12, 2, 4, 2, 'Pago de Comisión', 'Egreso', 'Pago de comisiones semanales', 1, '2025-12-16 21:17:55', '2025-12-16 21:17:56');

-- Volcando estructura para tabla growvent_staltos.currencies
CREATE TABLE IF NOT EXISTS `currencies` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `currency` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `currencies_currency_unique` (`currency`)
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.currencies: ~50 rows (aproximadamente)
INSERT INTO `currencies` (`id`, `currency`, `description`, `created_at`, `updated_at`) VALUES
	(1, 'USD', 'Dólar estadounidense', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(2, 'EUR', 'Euro', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(3, 'JPY', 'Yen japonés', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(4, 'GBP', 'Libra esterlina', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(5, 'AUD', 'Dólar australiano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(6, 'CAD', 'Dólar canadiense', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(7, 'CHF', 'Franco suizo', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(8, 'CNY', 'Yuan chino', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(9, 'NZD', 'Dólar neozelandés', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(10, 'SEK', 'Corona sueca', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(11, 'MXN', 'Peso mexicano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(12, 'SGD', 'Dólar de Singapur', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(13, 'HKD', 'Dólar de Hong Kong', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(14, 'NOK', 'Corona noruega', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(15, 'KRW', 'Won surcoreano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(16, 'TRY', 'Lira turca', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(17, 'INR', 'Rupia india', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(18, 'RUB', 'Rublo ruso', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(19, 'ZAR', 'Rand sudafricano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(20, 'BRL', 'Real brasileño', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(21, 'DKK', 'Corona danesa', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(22, 'PLN', 'Zloty polaco', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(23, 'THB', 'Baht tailandés', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(24, 'MYR', 'Ringgit malasio', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(25, 'IDR', 'Rupia indonesia', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(26, 'CZK', 'Corona checa', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(27, 'HUF', 'Florín húngaro', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(28, 'ILS', 'Nuevo shekel israelí', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(29, 'PHP', 'Peso filipino', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(30, 'TWD', 'Nuevo dólar taiwanés', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(31, 'SAR', 'Riyal saudí', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(32, 'AED', 'Dírham de los Emiratos Árabes Unidos', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(33, 'COP', 'Peso colombiano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(34, 'ARS', 'Peso argentino', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(35, 'CLP', 'Peso chileno', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(36, 'PKR', 'Rupia pakistaní', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(37, 'VND', 'Dong vietnamita', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(38, 'EGP', 'Libra egipcia', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(39, 'BDT', 'Taka bangladesí', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(40, 'NGN', 'Naira nigeriana', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(41, 'KZT', 'Tengue kazajo', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(42, 'QAR', 'Riyal catarí', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(43, 'KWD', 'Dinar kuwaití', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(44, 'UAH', 'Grivna ucraniana', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(45, 'PEN', 'Sol peruano', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(46, 'LKR', 'Rupia de Sri Lanka', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(47, 'DZD', 'Dinar argelino', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(48, 'IQD', 'Dinar iraquí', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(49, 'OMR', 'Rial omaní', '2025-12-16 21:05:18', '2025-12-16 21:05:18'),
	(50, 'BHD', 'Dinar bareiní', '2025-12-16 21:05:18', '2025-12-16 21:05:18');

-- Volcando estructura para tabla growvent_staltos.customers
CREATE TABLE IF NOT EXISTS `customers` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `public_token` char(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `phone` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `birthdate` date DEFAULT NULL,
  `credit_days` int NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `customers_public_token_unique` (`public_token`),
  KEY `customers_classification_id_foreign` (`classification_id`),
  CONSTRAINT `customers_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `customer_classifications` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.customers: ~12 rows (aproximadamente)
INSERT INTO `customers` (`id`, `public_token`, `name`, `phone`, `email`, `birthdate`, `credit_days`, `created_at`, `updated_at`, `classification_id`) VALUES
	(1, '682bf8be-e0f6-4372-965a-b1e55f1be85e', 'Leonardo', '5555555555', NULL, NULL, 0, '2025-12-16 21:22:28', '2025-12-16 21:22:28', 3),
	(2, 'cf6319d8-f020-49be-9bcd-b03b74ceea9c', 'Jose', '4444444444', NULL, NULL, 0, '2025-12-16 21:22:45', '2025-12-16 21:22:50', 4),
	(3, '02324eb3-3844-46c6-9299-3326d4490146', 'Maria', '9999999999', NULL, NULL, 0, '2025-12-16 21:23:03', '2025-12-16 21:23:03', 1),
	(4, '03dc3d36-6c02-45f8-84a5-62f579387ce3', 'MARCO ANTONIO', '44 4444 4444', NULL, NULL, 0, '2025-12-16 21:30:54', '2025-12-16 21:30:54', NULL),
	(5, '48b2f328-d4c6-4fa6-b147-3551625da1cd', 'PEDRO', '987654321', NULL, NULL, 0, '2025-12-16 22:32:52', '2025-12-16 22:32:52', NULL),
	(6, '98a0f6a9-a527-4a4a-8fb3-3c989361fbe2', 'LUPITA', '987654321', NULL, NULL, 0, '2025-12-16 22:38:10', '2025-12-16 22:38:10', NULL),
	(7, 'f6be47da-db7d-4bc9-8443-9a2c0e0d13ef', 'Eleazar', '9876543210', NULL, NULL, 0, '2026-01-29 22:43:50', '2026-01-29 22:43:50', NULL),
	(8, '43c9be04-bb83-49fd-aa59-2a888310dbad', 'Juanita', '32115489', NULL, NULL, 0, '2026-01-29 23:04:04', '2026-01-29 23:04:04', NULL),
	(9, '070fa814-0a41-4e94-ae77-eccbad2d64fd', 'prueba', '6141043837', NULL, NULL, 0, '2026-03-22 00:11:06', '2026-03-22 00:11:06', NULL),
	(10, '342fd8e7-059b-4c1a-8c66-499258c79d42', 'Pedro Rodriguez', '33333333', NULL, NULL, 0, '2026-03-26 21:49:20', '2026-03-26 21:49:20', NULL),
	(12, '34793da5-73cf-44e6-844c-c44e3e0b675a', 'Leonardo Merchand', '99999999', NULL, NULL, 0, '2026-08-25 00:17:01', '2026-08-25 00:17:01', NULL),
	(13, '19905636-1f69-43f1-a4be-c7391f085a88', 'Adrian Petur', '9984933273', NULL, NULL, 0, '2026-08-25 19:50:27', '2026-08-25 19:50:27', NULL),
	(14, 'b2c41b5a-48bb-4ed2-af98-51a7afdff64e', 'Chema', '3781056808', NULL, NULL, 0, '2026-09-03 03:26:04', '2026-09-03 03:26:04', NULL);

-- Volcando estructura para tabla growvent_staltos.customer_classifications
CREATE TABLE IF NOT EXISTS `customer_classifications` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.customer_classifications: ~4 rows (aproximadamente)
INSERT INTO `customer_classifications` (`id`, `name`, `description`, `created_at`, `updated_at`) VALUES
	(1, 'VIP', 'Clientes VIP con máxima prioridad y valor comercial', '2025-12-16 21:05:32', '2026-01-29 23:28:23'),
	(2, 'Bueno', 'Clientes de alta prioridad con gran valor comercial', '2025-12-16 21:05:32', '2026-01-29 23:28:28'),
	(3, 'Mas o menos', 'Clientes de prioridad media con valor comercial moderado', '2025-12-16 21:05:32', '2026-01-29 23:28:36'),
	(4, 'Mal cliente', 'Clientes de baja prioridad o nuevos en el sistema', '2025-12-16 21:05:32', '2026-01-29 23:28:42');

-- Volcando estructura para tabla growvent_staltos.customer_megatravel_interests
CREATE TABLE IF NOT EXISTS `customer_megatravel_interests` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `customer_id` bigint unsigned DEFAULT NULL,
  `mt_clv` int NOT NULL,
  `mt_code` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `price_from` decimal(10,2) NOT NULL,
  `currency` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL,
  `imp_from` decimal(10,2) DEFAULT NULL,
  `detail_url` text COLLATE utf8mb4_unicode_ci,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `captured_at` timestamp NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `customer_megatravel_interests_mt_clv_index` (`mt_clv`),
  KEY `customer_megatravel_interests_customer_id_mt_clv_index` (`customer_id`,`mt_clv`),
  CONSTRAINT `customer_megatravel_interests_customer_id_foreign` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.customer_megatravel_interests: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.destinations
CREATE TABLE IF NOT EXISTS `destinations` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `contraction` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=123 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.destinations: ~122 rows (aproximadamente)
INSERT INTO `destinations` (`id`, `name`, `contraction`, `created_at`, `updated_at`) VALUES
	(1, 'Acapulco (Guerrero)', 'ACA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(2, 'Aguascalientes (Aguascalientes)', 'AGU', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(3, 'Akumal (Quintana Roo)', 'AKU', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(4, 'Álamos (Sonora)', 'ALS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(5, 'Bernal (Querétaro)', 'BER', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(6, 'Campeche (Campeche)', 'CPE', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(7, 'Cancún (Quintana Roo)', 'CUN', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(8, 'Celaya (Guanajuato)', 'CYA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(9, 'Chihuahua (Chihuahua)', 'CUU', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(10, 'Cholula (Puebla)', 'CHL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(11, 'Ciudad de México (CDMX)', 'MEX', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(12, 'Ciudad Juárez (Chihuahua)', 'CJS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(13, 'Ciudad Obregón (Sonora)', 'CEN', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(14, 'Coatzacoalcos (Veracruz)', 'COA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(15, 'Cozumel (Quintana Roo)', 'CZM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(16, 'Creel (Chihuahua)', 'CRE', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(17, 'Durango (Durango)', 'DGO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(18, 'Ensenada (Baja California)', 'ENS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(19, 'Guanajuato (Guanajuato)', 'GTO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(20, 'Guadalajara (Jalisco)', 'GDL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(21, 'Guaymas (Sonora)', 'GYM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(22, 'Hermosillo (Sonora)', 'HMO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(23, 'Holbox (Quintana Roo)', 'HOL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(24, 'Huatulco (Oaxaca)', 'HUX', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(25, 'Ixtapa (Guerrero)', 'ZIH', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(26, 'Jalcomulco (Veracruz)', 'JAL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(27, 'La Paz (Baja California Sur)', 'LAP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(28, 'Loreto (Baja California Sur)', 'LTO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(29, 'Los Cabos (Baja California Sur)', 'SJD', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(30, 'Los Mochis (Sinaloa)', 'LMM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(31, 'Mahahual (Quintana Roo)', 'MAH', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(32, 'Manzanillo (Colima)', 'ZLO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(33, 'Mazatlán (Sinaloa)', 'MZT', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(34, 'Mérida (Yucatán)', 'MID', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(35, 'Mexicali (Baja California)', 'MXL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(36, 'Mineral de Pozos (Guanajuato)', 'POZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(37, 'Mitla (Oaxaca)', 'MIT', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(38, 'Monterrey (Nuevo León)', 'MTY', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(39, 'Morelia (Michoacán)', 'MLM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(40, 'Orizaba (Veracruz)', 'ORIZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(41, 'Pachuca (Hidalgo)', 'PAC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(42, 'Palenque (Chiapas)', 'PQM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(43, 'Playa del Carmen (Quintana Roo)', 'PCM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(44, 'Puebla (Puebla)', 'PBC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(45, 'Puerto Aventuras (Quintana Roo)', 'PAV', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(46, 'Puerto Escondido (Oaxaca)', 'PXM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(47, 'Puerto Morelos (Quintana Roo)', 'PMO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(48, 'Puerto Peñasco (Sonora)', 'PPE', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(49, 'Puerto Vallarta (Jalisco)', 'PVR', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(50, 'Querétaro (Querétaro)', 'QRO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(51, 'Riviera Maya (Quintana Roo)', 'RVM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(52, 'Riviera Nayarit (Nayarit)', 'RNY', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(53, 'Rosarito (Baja California)', 'ROS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(54, 'Salamanca (Guanajuato)', 'SLM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(55, 'Saltillo (Coahuila)', 'SLW', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(56, 'San Blas (Nayarit)', 'SNB', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(57, 'San Carlos (Sonora)', 'SCA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(58, 'San Cristóbal de las Casas (Chiapas)', 'SCC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(59, 'San José del Cabo (Baja California Sur)', 'SJC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(60, 'San Juan Teotihuacán (Estado de México)', 'SJT', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(61, 'San Luis Potosí (San Luis Potosí)', 'SLP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(62, 'San Miguel de Allende (Guanajuato)', 'SMA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(63, 'Sayulita (Nayarit)', 'SYT', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(64, 'Tampico (Tamaulipas)', 'TAM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(65, 'Tapachula (Chiapas)', 'TAP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(66, 'Taxco (Guerrero)', 'TXC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(67, 'Tecate (Baja California)', 'TEC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(68, 'Tepoztlán (Morelos)', 'TEP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(69, 'Tepic (Nayarit)', 'TPQ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(70, 'Tijuana (Baja California)', 'TIJ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(71, 'Toluca (Estado de México)', 'TLC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(72, 'Tonalá (Chiapas)', 'TON', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(73, 'Torreón (Coahuila)', 'TRC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(74, 'Tula (Hidalgo)', 'TUL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(75, 'Tulum (Quintana Roo)', 'TUY', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(76, 'Tuxtla Gutiérrez (Chiapas)', 'TGZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(77, 'Uruapan (Michoacán)', 'UPN', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(78, 'Valle de Bravo (Estado de México)', 'VDB', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(79, 'Valladolid (Yucatán)', 'VLL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(80, 'Veracruz (Veracruz)', 'VER', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(81, 'Villahermosa (Tabasco)', 'VSA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(82, 'Xalapa (Veracruz)', 'XAL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(83, 'Zacatecas (Zacatecas)', 'ZCL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(84, 'Zihuatanejo (Guerrero)', 'ZIH', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(85, 'Real de Catorce (San Luis Potosí)', 'RTC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(86, 'Pátzcuaro (Michoacán)', 'PTZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(87, 'Bacalar (Quintana Roo)', 'BAC', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(88, 'Isla Mujeres (Quintana Roo)', 'ISL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(89, 'Tequisquiapan (Querétaro)', 'TEQ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(90, 'Coscomatepec (Veracruz)', 'COS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(91, 'Chignahuapan (Puebla)', 'CHP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(92, 'Huauchinango (Puebla)', 'HUA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(93, 'Papantla (Veracruz)', 'PAP', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(94, 'Comala (Colima)', 'CMA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(95, 'Tapalpa (Jalisco)', 'TPL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(96, 'Xilitla (San Luis Potosí)', 'XIL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(97, 'Todos Santos (Baja California Sur)', 'TOD', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(98, 'Izamal (Yucatán)', 'IZA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(99, 'Huamantla (Tlaxcala)', 'HMT', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(100, 'Mazunte (Oaxaca)', 'MAZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(101, 'Nueva York (Estados Unidos)', 'JFK', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(102, 'Los Ángeles (Estados Unidos)', 'LAX', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(103, 'Miami (Estados Unidos)', 'MIA', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(104, 'Orlando (Estados Unidos)', 'ORL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(105, 'Las Vegas (Estados Unidos)', 'LAS', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(106, 'Chicago (Estados Unidos)', 'CHI', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(107, 'Houston (Estados Unidos)', 'HOU', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(108, 'Dallas (Estados Unidos)', 'DAL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(109, 'San Francisco (Estados Unidos)', 'SFO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(110, 'Toronto (Canadá)', 'YYZ', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(111, 'Vancouver (Canadá)', 'YVR', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(112, 'Montreal (Canadá)', 'YUL', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(113, 'Madrid (España)', 'MAD', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(114, 'Barcelona (España)', 'BCN', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(115, 'París (Francia)', 'PAR', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(116, 'Londres (Reino Unido)', 'LON', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(117, 'Roma (Italia)', 'ROM', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(118, 'Tokio (Japón)', 'TYO', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(119, 'Buenos Aires (Argentina)', 'BUE', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(120, 'Bogotá (Colombia)', 'BOG', '2025-12-16 21:05:40', '2025-12-16 21:05:40'),
	(121, 'Las coloradas', 'LAS', '2026-01-29 22:46:38', '2026-01-29 22:46:38'),
	(122, 'La Paz, BCS', 'LAP', '2026-05-03 04:25:02', '2026-05-03 04:25:02');

-- Volcando estructura para tabla growvent_staltos.events
CREATE TABLE IF NOT EXISTS `events` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `start_date` date NOT NULL,
  `end_date` date NOT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=42 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.events: ~41 rows (aproximadamente)
INSERT INTO `events` (`id`, `name`, `start_date`, `end_date`, `notes`, `created_at`, `updated_at`) VALUES
	(1, 'Seminario 2024', '2026-04-29', '2026-05-02', 'Actualización de procesos internos', '2025-06-30 20:05:53', '2025-09-16 20:05:53'),
	(2, 'Entrenamiento 2025', '2025-11-30', '2025-12-02', 'Evento de integración de equipos', '2025-09-01 20:05:53', '2025-10-09 20:05:53'),
	(3, 'Conferencia 2025', '2025-09-10', '2025-09-13', 'Actualización de procesos internos', '2025-08-10 20:05:53', '2025-10-08 20:05:53'),
	(4, 'Presentación 2024', '2025-10-08', '2025-10-09', 'Revisión de objetivos anuales', '2025-09-04 20:05:53', '2025-10-15 20:05:53'),
	(5, 'Capacitación 2025', '2026-01-31', '2026-02-03', 'Conferencia de liderazgo', '2025-11-29 21:05:53', '2025-11-26 21:05:53'),
	(6, 'Capacitación 2024', '2026-03-19', '2026-03-25', 'Conferencia de liderazgo', '2025-10-15 20:05:53', '2025-10-24 20:05:53'),
	(7, 'Capacitación 2025', '2026-05-13', '2026-05-15', 'Taller de productividad', '2025-08-20 20:05:53', '2025-11-14 21:05:53'),
	(8, 'Workshop 2024', '2026-05-31', '2026-06-03', 'Capacitación en nuevas tecnologías', '2025-09-06 20:05:53', '2025-11-27 21:05:53'),
	(9, 'Capacitación 2024', '2026-02-21', '2026-02-26', 'Sesión de retroalimentación', '2025-08-18 20:05:53', '2025-10-13 20:05:53'),
	(10, 'Capacitación 2024', '2026-04-27', '2026-05-01', 'Capacitación en nuevas tecnologías', '2025-10-27 20:05:53', '2025-09-14 20:05:53'),
	(11, 'Reunión 2025', '2026-01-29', '2026-02-05', 'Presentación de nuevos productos', '2025-06-09 20:05:53', '2025-09-13 20:05:53'),
	(12, 'Capacitación 2024', '2026-04-26', '2026-04-30', 'Sesión de retroalimentación', '2025-10-09 20:05:53', '2025-10-24 20:05:53'),
	(13, 'Presentación 2025', '2026-08-02', '2026-08-07', 'Evento de integración de equipos', '2025-10-14 20:05:53', '2025-10-11 20:05:53'),
	(14, 'Seminario 2025', '2025-12-31', '2026-01-04', 'Planificación estratégica', '2025-10-21 20:05:53', '2025-12-08 21:05:53'),
	(15, 'Reunión 2025', '2025-10-03', '2025-10-09', 'Conferencia de liderazgo', '2025-07-11 20:05:53', '2025-09-23 20:05:53'),
	(16, 'Reunión 2024', '2026-09-16', '2026-09-18', 'Capacitación en nuevas tecnologías', '2025-11-28 21:05:53', '2025-11-17 21:05:53'),
	(17, 'Entrenamiento 2025', '2026-05-25', '2026-06-01', 'Revisión de objetivos anuales', '2025-08-02 20:05:53', '2025-10-08 20:05:53'),
	(18, 'Workshop 2025', '2026-10-17', '2026-10-17', 'Actualización de procesos internos', '2025-11-10 21:05:53', '2025-10-16 20:05:53'),
	(19, 'Presentación 2024', '2026-01-22', '2026-01-22', 'Taller de productividad', '2025-11-28 21:05:53', '2025-09-30 20:05:53'),
	(20, 'Entrenamiento 2024', '2025-12-30', '2026-01-01', 'Planificación estratégica', '2025-12-04 21:05:53', '2025-11-25 21:05:53'),
	(21, 'Convención 2024', '2026-08-06', '2026-08-11', 'Revisión de objetivos anuales', '2025-09-20 20:05:53', '2025-12-08 21:05:53'),
	(22, 'Entrenamiento 2024', '2026-03-01', '2026-03-07', 'Taller de productividad', '2025-08-06 20:05:53', '2025-10-21 20:05:53'),
	(23, 'Presentación 2024', '2026-04-30', '2026-05-04', 'Actualización de procesos internos', '2025-07-15 20:05:53', '2025-12-08 21:05:53'),
	(24, 'Taller 2024', '2026-05-12', '2026-05-19', 'Revisión de objetivos anuales', '2025-11-14 21:05:53', '2025-10-17 20:05:53'),
	(25, 'Seminario 2025', '2026-01-21', '2026-01-23', 'Programa de desarrollo profesional', '2025-07-04 20:05:53', '2025-10-07 20:05:53'),
	(26, 'Convención 2024', '2026-07-05', '2026-07-09', 'Presentación de nuevos productos', '2025-06-11 20:05:53', '2025-09-10 20:05:53'),
	(27, 'Convención 2024', '2026-03-03', '2026-03-08', 'Capacitación en nuevas tecnologías', '2025-09-05 20:05:53', '2025-09-28 20:05:53'),
	(28, 'Entrenamiento 2024', '2025-10-02', '2025-10-03', 'Sesión de retroalimentación', '2025-08-08 20:05:53', '2025-09-29 20:05:53'),
	(29, 'Taller 2024', '2025-12-26', '2025-12-26', 'Taller de productividad', '2025-06-07 20:05:53', '2025-10-25 20:05:53'),
	(30, 'Capacitación 2025', '2026-01-27', '2026-01-30', 'Programa de desarrollo profesional', '2025-09-11 20:05:53', '2025-09-14 20:05:53'),
	(31, 'Presentación Programado', '2026-03-02', '2026-03-02', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(32, 'Reunión Programado', '2026-02-15', '2026-02-15', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(33, 'Conferencia Programado', '2026-01-12', '2026-01-13', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(34, 'Capacitación Programado', '2026-01-11', '2026-01-11', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(35, 'Conferencia Programado', '2026-02-13', '2026-02-13', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(36, 'Seminario Programado', '2025-12-20', '2025-12-21', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(37, 'Entrenamiento Programado', '2025-12-18', '2025-12-21', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(38, 'Presentación Programado', '2026-01-07', '2026-01-10', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(39, 'Entrenamiento Programado', '2026-01-21', '2026-01-23', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(40, 'Capacitación Programado', '2026-03-01', '2026-03-04', NULL, '2025-12-16 21:05:53', '2025-12-16 21:05:53'),
	(41, 'Capacitacion general', '2026-02-08', '2026-02-15', 'Todos vamos a estar en capacitación para viajes internacionales', '2026-01-29 23:25:34', '2026-01-29 23:25:34');

-- Volcando estructura para tabla growvent_staltos.failed_jobs
CREATE TABLE IF NOT EXISTS `failed_jobs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `uuid` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `connection` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `queue` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `exception` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `failed_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `failed_jobs_uuid_unique` (`uuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.failed_jobs: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.files
CREATE TABLE IF NOT EXISTS `files` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `user_id` bigint unsigned DEFAULT NULL,
  `lead_id` bigint unsigned DEFAULT NULL,
  `lead_group_id` bigint unsigned DEFAULT NULL,
  `public` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `files_lead_id_foreign` (`lead_id`),
  KEY `files_user_id_foreign` (`user_id`),
  KEY `files_lead_group_id_foreign` (`lead_group_id`),
  CONSTRAINT `files_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `files_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE SET NULL,
  CONSTRAINT `files_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.files: ~4 rows (aproximadamente)
INSERT INTO `files` (`id`, `name`, `file_path`, `user_id`, `lead_id`, `lead_group_id`, `public`, `created_at`, `updated_at`) VALUES
	(1, 'sample-local-pdf.pdf', 'https://growventa-public.s3.amazonaws.com/growtravel/IzvBXZzSaeby7rpbcAB1hhjHtI6UpG5T5YJuAP7U.pdf', 10, 7, NULL, 0, '2025-12-16 22:46:03', '2025-12-16 22:46:03'),
	(3, 'Cotizacion_Angel_GV-20260126-0001.pdf', 'https://growventa-public.s3.amazonaws.com/growtravel/zt0dxIfKbarn9DSe3nqGjQK3DP4ui0i7i7CuFTFK.pdf', 10, 10, NULL, 0, '2026-01-29 22:49:01', '2026-01-29 22:49:01'),
	(4, 'documento-2.pdf', 'https://growventa-public.s3.amazonaws.com/growtravel/58hUMr7LMHqR40isI57Hfwn6GCzyk1sBtmCPRpcU.pdf', 10, 18, NULL, 0, '2026-03-26 22:04:52', '2026-03-26 22:04:52'),
	(5, 'result.pdf', 'https://growventa-public.s3.amazonaws.com/growtravel/XznpYisgbnDjKEhHJAqZSUct795YNn9paLhwTKy2.pdf', 10, 24, NULL, 0, '2026-05-09 06:54:36', '2026-05-09 06:54:36');

-- Volcando estructura para tabla growvent_staltos.groups
CREATE TABLE IF NOT EXISTS `groups` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=31 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.groups: ~30 rows (aproximadamente)
INSERT INTO `groups` (`id`, `name`, `created_at`, `updated_at`) VALUES
	(1, 'Gasolina', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(2, 'Desayuno', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(3, 'Gastos No Operativos', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(4, 'Publicidad', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(5, 'Gastos Fijos', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(6, 'Cafetería', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(7, 'Grupo', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(8, 'Nómina', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(9, 'Utilidades', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(10, 'Artículos de limpieza', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(11, 'Gastos administrativos', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(12, 'Comisión', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(13, 'Especiales', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(14, 'Jorge', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(15, 'Viáticos', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(16, 'Papelería', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(17, 'Teléfono', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(18, 'Consulta médica', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(19, 'IMSS', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(20, 'Software', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(21, 'Impuestos', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(22, 'Capacitación', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(23, 'Gastos de venta', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(24, 'Luz', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(25, 'Comisión bancaria', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(26, 'Renta', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(27, 'Agua', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(28, 'Celular', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(29, 'Viáticos grupo', '2025-12-16 21:05:59', '2025-12-16 21:05:59'),
	(30, 'Prestaciones', '2025-12-16 21:05:59', '2025-12-16 21:05:59');

-- Volcando estructura para tabla growvent_staltos.group_products
CREATE TABLE IF NOT EXISTS `group_products` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_group_id` bigint unsigned NOT NULL,
  `group_service_id` bigint unsigned NOT NULL,
  `departure_id` bigint unsigned DEFAULT NULL,
  `arrival_id` bigint unsigned DEFAULT NULL,
  `airline_id` bigint unsigned DEFAULT NULL,
  `hotel_id` bigint unsigned DEFAULT NULL,
  `occupation_id` bigint unsigned DEFAULT NULL,
  `service_id` bigint unsigned DEFAULT NULL,
  `guests` int NOT NULL DEFAULT '1',
  `description` text COLLATE utf8mb4_unicode_ci,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `profit` decimal(15,2) NOT NULL DEFAULT '0.00',
  `flight_datetime` datetime DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `group_products_lead_group_id_foreign` (`lead_group_id`),
  KEY `group_products_group_service_id_foreign` (`group_service_id`),
  KEY `group_products_departure_id_foreign` (`departure_id`),
  KEY `group_products_arrival_id_foreign` (`arrival_id`),
  KEY `group_products_airline_id_foreign` (`airline_id`),
  KEY `group_products_hotel_id_foreign` (`hotel_id`),
  KEY `group_products_occupation_id_foreign` (`occupation_id`),
  KEY `group_products_service_id_foreign` (`service_id`),
  CONSTRAINT `group_products_airline_id_foreign` FOREIGN KEY (`airline_id`) REFERENCES `airlines` (`id`) ON DELETE SET NULL,
  CONSTRAINT `group_products_arrival_id_foreign` FOREIGN KEY (`arrival_id`) REFERENCES `destinations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `group_products_departure_id_foreign` FOREIGN KEY (`departure_id`) REFERENCES `destinations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `group_products_group_service_id_foreign` FOREIGN KEY (`group_service_id`) REFERENCES `group_services` (`id`) ON DELETE CASCADE,
  CONSTRAINT `group_products_hotel_id_foreign` FOREIGN KEY (`hotel_id`) REFERENCES `hotels` (`id`) ON DELETE SET NULL,
  CONSTRAINT `group_products_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE CASCADE,
  CONSTRAINT `group_products_occupation_id_foreign` FOREIGN KEY (`occupation_id`) REFERENCES `occupations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `group_products_service_id_foreign` FOREIGN KEY (`service_id`) REFERENCES `services` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.group_products: ~7 rows (aproximadamente)
INSERT INTO `group_products` (`id`, `lead_group_id`, `group_service_id`, `departure_id`, `arrival_id`, `airline_id`, `hotel_id`, `occupation_id`, `service_id`, `guests`, `description`, `total`, `profit`, `flight_datetime`, `created_at`, `updated_at`) VALUES
	(4, 2, 4, NULL, 49, NULL, 82, 2, NULL, 2, NULL, 4500.00, 1250.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(5, 2, 5, NULL, 49, NULL, NULL, NULL, 1, 1, 'Paseo en Yate', 3500.00, 1400.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(6, 2, 6, 20, 49, 2, NULL, NULL, NULL, 2, NULL, 3250.00, 750.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(7, 3, 7, NULL, 104, NULL, NULL, NULL, 2, 1, 'DISNEY + UNIVERSAL STUDIOS', 3000.00, 1000.00, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(8, 4, 8, 34, 11, 2, NULL, NULL, NULL, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(9, 4, 9, NULL, 11, NULL, NULL, NULL, 2, 1, 'Centro histórico', 1500.00, 750.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(10, 5, 10, NULL, 76, NULL, NULL, NULL, 2, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36');

-- Volcando estructura para tabla growvent_staltos.group_services
CREATE TABLE IF NOT EXISTS `group_services` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_group_id` bigint unsigned NOT NULL,
  `service_category_id` bigint unsigned DEFAULT NULL,
  `round_trip` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `group_services_lead_group_id_foreign` (`lead_group_id`),
  KEY `group_services_service_category_id_foreign` (`service_category_id`),
  CONSTRAINT `group_services_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE CASCADE,
  CONSTRAINT `group_services_service_category_id_foreign` FOREIGN KEY (`service_category_id`) REFERENCES `service_categories` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.group_services: ~7 rows (aproximadamente)
INSERT INTO `group_services` (`id`, `lead_group_id`, `service_category_id`, `round_trip`, `created_at`, `updated_at`) VALUES
	(4, 2, 2, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(5, 2, 3, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(6, 2, 1, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(7, 3, 3, 0, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(8, 4, 1, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(9, 4, 3, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(10, 5, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36');

-- Volcando estructura para tabla growvent_staltos.hotels
CREATE TABLE IF NOT EXISTS `hotels` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `contraction` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=110 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.hotels: ~50 rows (aproximadamente)
INSERT INTO `hotels` (`id`, `name`, `contraction`, `created_at`, `updated_at`) VALUES
	(60, 'Grupo Posadas', 'POS', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(61, 'City Express Hotels', 'CEX', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(62, 'Grupo Brisas', 'GBR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(63, 'Grupo Presidente', 'GPR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(64, 'Hoteles Misión', 'HM', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(65, 'Hotel Lucerna', 'LUC', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(66, 'HS Hotsson', 'HSH', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(67, 'Emporio Hotels', 'EMP', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(68, 'Camino Real', 'CMR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(69, 'Krystal Hotels', 'KHS', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(70, 'Marriott International', 'MAR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(71, 'Hilton Hotels & Resorts', 'HLT', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(72, 'InterContinental Hotels Group (IHG)', 'IHG', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(73, 'Wyndham Hotels & Resorts', 'WYN', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(74, 'Choice Hotels', 'CHO', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(75, 'Accor', 'ACC', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(76, 'Hyatt', 'HYT', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(77, 'Best Western', 'BWR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(78, 'Hard Rock Hotels', 'HRH', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(79, 'Four Seasons Hotels', 'FSH', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(80, 'Fairmont', 'FRM', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(81, 'Barceló', 'BAR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(82, 'Riu Hotels & Resorts', 'RIU', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(83, 'Iberostar', 'IBR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(84, 'Meliá Hotels International', 'MEL', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(85, 'Palladium Hotel Group', 'PAL', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(86, 'Sandos Hotels & Resorts', 'SND', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(87, 'AMResorts', 'AMR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(88, 'Princess Hotels & Resorts', 'PRI', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(89, 'Bahia Principe', 'BHP', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(90, 'H10 Hotels (Ocean)', 'H10', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(91, 'The Ritz-Carlton', 'RIT', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(92, 'Aloft (Marca de Marriott)', 'ALO', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(93, 'Park Hyatt', 'PHA', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(94, 'Dreams Resorts', 'DRM', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(95, 'Secrets Resorts', 'SEC', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(96, 'Nickelodeon Hotels & Resorts', 'NIC', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(97, 'Karisma Hotels & Resorts', 'KAR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(98, 'NH Hotel Group', 'NHH', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(99, 'Omni Hotels & Resorts', 'OMN', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(100, 'Rosewood Hotels & Resorts', 'RWD', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(101, 'JW Marriott', 'JWM', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(102, 'St. Regis', 'STR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(103, 'Westin', 'WES', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(104, 'Sheraton', 'SHE', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(105, 'Renaissance Hotels', 'REN', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(106, 'Le Méridien', 'LMD', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(107, 'Hyatt Regency', 'HYR', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(108, 'Delta Hotels (by Marriott)', 'DLH', '2025-12-16 21:06:05', '2025-12-16 21:06:05'),
	(109, 'Hampton by Hilton', 'HPN', '2025-12-16 21:06:05', '2025-12-16 21:06:05');

-- Volcando estructura para tabla growvent_staltos.jobs
CREATE TABLE IF NOT EXISTS `jobs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `queue` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `attempts` tinyint unsigned NOT NULL,
  `reserved_at` int unsigned DEFAULT NULL,
  `available_at` int unsigned NOT NULL,
  `created_at` int unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `jobs_queue_index` (`queue`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.jobs: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.job_batches
CREATE TABLE IF NOT EXISTS `job_batches` (
  `id` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `total_jobs` int NOT NULL,
  `pending_jobs` int NOT NULL,
  `failed_jobs` int NOT NULL,
  `failed_job_ids` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `options` mediumtext COLLATE utf8mb4_unicode_ci,
  `cancelled_at` int DEFAULT NULL,
  `created_at` int NOT NULL,
  `finished_at` int DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.job_batches: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.leads
CREATE TABLE IF NOT EXISTS `leads` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `public_token` char(36) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `survey` tinyint(1) NOT NULL DEFAULT '0',
  `pos_sale` tinyint(1) NOT NULL DEFAULT '0',
  `user_id` bigint unsigned DEFAULT NULL,
  `lead_group_id` bigint unsigned DEFAULT NULL,
  `customer_id` bigint unsigned DEFAULT NULL,
  `prelead_id` bigint unsigned DEFAULT NULL,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `cost` decimal(15,2) NOT NULL DEFAULT '0.00',
  `profit` decimal(15,2) NOT NULL DEFAULT '0.00',
  `toPay` decimal(15,2) NOT NULL DEFAULT '0.00',
  `stage_id` bigint unsigned DEFAULT '1',
  `service_type_id` bigint unsigned DEFAULT NULL,
  `channel_id` bigint unsigned DEFAULT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `operator` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `currency` text COLLATE utf8mb4_unicode_ci,
  `due_date` date DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `leads_public_token_unique` (`public_token`),
  KEY `leads_user_id_foreign` (`user_id`),
  KEY `leads_customer_id_foreign` (`customer_id`),
  KEY `leads_prelead_id_foreign` (`prelead_id`),
  KEY `leads_stage_id_foreign` (`stage_id`),
  KEY `leads_service_type_id_foreign` (`service_type_id`),
  KEY `leads_lead_group_id_foreign` (`lead_group_id`),
  KEY `leads_channel_id_foreign` (`channel_id`),
  CONSTRAINT `leads_channel_id_foreign` FOREIGN KEY (`channel_id`) REFERENCES `channels` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_customer_id_foreign` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_prelead_id_foreign` FOREIGN KEY (`prelead_id`) REFERENCES `preleads` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_service_type_id_foreign` FOREIGN KEY (`service_type_id`) REFERENCES `service_types` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_stage_id_foreign` FOREIGN KEY (`stage_id`) REFERENCES `stages` (`id`) ON DELETE SET NULL,
  CONSTRAINT `leads_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=36 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.leads: ~34 rows (aproximadamente)
INSERT INTO `leads` (`id`, `public_token`, `survey`, `pos_sale`, `user_id`, `lead_group_id`, `customer_id`, `prelead_id`, `total`, `cost`, `profit`, `toPay`, `stage_id`, `service_type_id`, `channel_id`, `description`, `operator`, `currency`, `due_date`, `created_at`, `updated_at`) VALUES
	(2, '33076ee1-b30f-4033-9e3d-c87c28c222a6', 0, 0, 10, 2, 3, NULL, 14650.00, 11250.00, 3400.00, 0.00, 5, 1, 1, 'VIAJE TODO INCLUIDO VALLARTA 2026', 'OP TRAVEL', 'MXN', '2026-09-30', '2025-12-16 21:27:13', '2025-12-16 22:44:48'),
	(3, 'a94480d7-10f1-40cb-ac5d-ff81199838d6', 0, 0, 10, 2, 2, NULL, 14650.00, 11250.00, 3400.00, 0.00, 3, 1, 1, 'VIAJE TODO INCLUIDO VALLARTA 2026', 'OP TRAVEL', 'MXN', '2026-09-30', '2025-12-16 21:27:13', '2025-12-16 22:44:25'),
	(4, '63511b6b-b1f8-49fb-8519-833426877b9b', 0, 0, 10, 2, 1, NULL, 14650.00, 11250.00, 3400.00, 14650.00, 1, 1, 1, 'VIAJE TODO INCLUIDO VALLARTA 2026', 'OP TRAVEL', 'MXN', '2026-09-30', '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(5, 'cf635c15-8926-4c8a-9bba-a8e0dec00ffd', 0, 0, 11, NULL, 5, 2, 15000.00, 12000.00, 3000.00, 14000.00, 2, 1, 3, 'Viaje a Chihuahua', 'INTERNO', 'MXN', '2026-01-31', '2025-12-16 22:34:27', '2025-12-16 22:39:52'),
	(6, '2976843f-9931-454d-95bc-14759ec94cb6', 0, 0, 12, NULL, 2, NULL, 8500.00, 7000.00, 1500.00, 8500.00, 1, 6, 2, 'Estancia en Guadalajara 7 días', 'PROPIO', 'MXN', '2026-01-15', '2025-12-16 22:39:05', '2025-12-16 22:39:05'),
	(7, 'b2862f7e-c926-4870-b9ef-334333da1c5c', 0, 0, 12, 3, 6, NULL, 4000.00, 3000.00, 1000.00, 0.00, 5, 1, 1, 'ORLANDO + DISNEY', 'MEGA TRAVEL', 'USD', '2026-01-31', '2025-12-16 22:42:03', '2026-01-29 22:57:17'),
	(8, '1182b17d-bde3-446e-a21d-5836370b2b37', 0, 0, 12, 3, 4, NULL, 4000.00, 3000.00, 1000.00, 3000.00, 2, 1, 1, 'ORLANDO + DISNEY', 'MEGA TRAVEL', 'USD', '2026-01-31', '2025-12-16 22:42:03', '2025-12-16 22:43:24'),
	(9, 'ef3db7b3-bfaf-4d63-9f22-73ace2818e09', 0, 0, 12, 3, 5, NULL, 4000.00, 3000.00, 1000.00, 4000.00, 1, 1, 1, 'ORLANDO + DISNEY', 'MEGA TRAVEL', 'USD', '2026-01-31', '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(10, '001456f7-bc2d-45d1-9c34-fc6835381b3b', 0, 0, 10, NULL, 7, 4, 11500.00, 9000.00, 2500.00, 5000.00, 2, 11, 3, 'Paquete ida y vuelta a las colaradas', NULL, 'MXN', '2026-02-28', '2026-01-29 22:47:20', '2026-01-30 21:39:16'),
	(11, '53140a89-792c-4a8f-b3db-9a02855b4176', 0, 0, 10, 4, 7, NULL, 8750.00, 6500.00, 2250.00, 0.00, 3, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-01-29 23:16:46'),
	(12, 'f05b646a-207e-402f-9801-6c400d6ccf73', 0, 0, 10, 4, 6, NULL, 8750.00, 6500.00, 2250.00, 6000.00, 2, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-03-26 22:25:38'),
	(13, 'e576e966-8358-48b9-9b34-5191d9d605d7', 0, 0, 10, NULL, 5, NULL, 6500.00, 5000.00, 1500.00, 0.00, 3, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-03-17 00:10:11'),
	(14, '5211b65f-eef3-4788-8c55-1d26a1488686', 0, 0, 10, NULL, 8, NULL, 13250.00, 9750.00, 3500.00, 12145.83, 2, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-03-18 22:53:30'),
	(15, 'b9b6edd8-4e11-493e-91d9-1bf91583623c', 0, 0, 10, 4, 3, NULL, 8750.00, 6500.00, 2250.00, 8750.00, 1, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(16, 'd8923b24-e8e6-4cb7-9823-5276d1edbf6a', 0, 0, 10, 4, 1, NULL, 8750.00, 6500.00, 2250.00, 8750.00, 1, 5, 1, 'Viaje a la CDMX', 'Mega Travel', 'MXN', '2026-02-28', '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(17, 'eeef7e9b-1b71-47c2-9deb-44a80239f7b3', 0, 0, 10, NULL, 2, NULL, 708.00, 590.00, 118.00, 263.20, 2, 2, 4, 'Viaje a Estados Unidos', 'Mega Travel', 'USD', '2026-04-16', '2026-01-29 23:35:59', '2026-03-21 23:52:32'),
	(18, '5435238a-c036-4e17-b320-f539ba94500f', 0, 0, 10, NULL, 2, NULL, 7200.00, 6000.00, 1200.00, 6171.43, 2, 2, 3, 'Vuelos Rivera Maya', 'Prueba', 'MXN', '2026-03-31', '2026-03-17 00:18:50', '2026-03-17 22:12:04'),
	(19, 'ce13c67e-31df-490b-b008-3730628e5b66', 0, 0, 10, NULL, 10, 5, 10500.00, 8000.00, 2500.00, 0.00, 5, 1, 3, 'Paquete a Vallarta', 'Agencia', 'MXN', '2026-04-30', '2026-03-26 21:56:00', '2026-05-03 04:19:20'),
	(20, 'aca6878f-c700-4187-b2ef-31673fb39d0c', 0, 0, 10, 5, 2, NULL, 10000.00, 8000.00, 2000.00, 0.00, 5, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-03-26 22:30:44'),
	(21, 'fa1fd676-26ee-45e8-b18d-6c898e98828a', 0, 0, 10, NULL, 10, NULL, 10000.00, 8000.00, 2000.00, 8200.00, 2, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-05-03 04:56:17'),
	(22, '1da96e7f-5478-4ae0-bad1-56764a3c2aaf', 0, 0, 10, 5, 10, NULL, 10000.00, 8000.00, 2000.00, 9950.00, 2, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-05-09 06:51:38'),
	(23, '199fded9-fc86-4f75-a397-6cf002a24097', 0, 0, 10, 5, 8, NULL, 10000.00, 8000.00, 2000.00, 10000.00, 1, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(24, '2081af6c-9981-49e8-a061-f02f9b3d9fa2', 0, 0, 10, 5, 7, NULL, 10000.00, 8000.00, 2000.00, 10000.00, 1, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(25, '3a612b65-307d-42bc-bae7-680473be0340', 0, 0, 10, 5, 6, NULL, 10000.00, 8000.00, 2000.00, 10000.00, 1, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(26, '9286189c-7700-4227-9bc6-bd13b694d1db', 0, 0, 10, 5, 5, NULL, 10000.00, 8000.00, 2000.00, 10000.00, 1, 1, 1, 'Viaje Chiapas', NULL, 'MXN', '2026-05-15', '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(27, 'f5c65d30-5b6c-46d5-8597-f9d8e24ed73d', 0, 0, 10, NULL, 4, NULL, 32000.00, 32000.00, 0.00, 0.00, 5, 3, 1, 'MARCO ANTONIO CANCUN', 'NEFERTARI', 'MXN', '2026-08-01', '2026-04-20 18:54:29', '2026-05-03 04:56:03'),
	(28, 'c00aaf03-0332-485b-a7e2-0b89c384bb8d', 0, 0, 10, NULL, NULL, NULL, 42000.00, 35000.00, 7000.00, 0.00, 5, 3, 3, 'Viaje a La Paz', 'CT', 'MXN', '2026-05-05', '2026-05-03 04:32:39', '2026-05-03 04:54:37'),
	(29, '05d31586-f380-4041-8851-e12f18ec2e7e', 0, 0, 10, NULL, 8, NULL, 1800.00, 1500.00, 300.00, 0.00, 3, 8, 3, 'TOYOTA RAIZE 2025', NULL, 'MXN', '2026-05-15', '2026-05-14 20:25:58', '2026-05-23 01:49:31'),
	(30, '62e16404-279e-4427-832c-48fb2fa07442', 0, 0, 10, NULL, 12, NULL, 10.00, 10.00, 0.00, 10.00, 1, 1, 4, 'Quiere', NULL, 'MXN', '2026-08-24', '2026-08-25 00:17:42', '2026-08-25 00:17:42'),
	(31, '22508f21-08e3-4f96-b0bd-a48cfe86b560', 0, 0, 10, NULL, 13, NULL, 100.00, 100.00, 0.00, 0.00, 3, 5, 3, 'Tour Isla Mujeres', NULL, 'USD', '2026-08-31', '2026-08-25 19:54:10', '2026-08-25 19:56:32'),
	(32, 'ce8fc6e0-fdbc-4ec3-b344-f3afabfc5ede', 0, 1, 10, NULL, 8, NULL, 1000.00, 0.00, 0.00, 1000.00, 1, NULL, NULL, 'Prueba', NULL, 'MXN', NULL, '2026-09-02 05:49:17', '2026-09-02 05:49:17'),
	(33, '7872cc97-f2f1-42f5-ab5d-4a1c204bd717', 0, 1, 10, NULL, 8, NULL, 2000.00, 0.00, 0.00, 1000.00, 2, NULL, NULL, 'Prueba', NULL, 'MXN', NULL, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(34, '6102e39a-f8a8-4a40-b3b2-48ecaf6e4a4d', 0, 1, 10, NULL, 10, NULL, 2000.00, 0.00, 0.00, 1500.00, 2, NULL, NULL, 'Test', NULL, 'MXN', NULL, '2026-09-03 03:16:37', '2026-09-03 03:16:37'),
	(35, 'ab7fdf67-7de6-4220-b2cc-534f7faa5183', 0, 1, 10, NULL, 14, NULL, 500.00, 0.00, 0.00, 250.00, 2, NULL, NULL, NULL, NULL, 'MXN', NULL, '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.lead_groups
CREATE TABLE IF NOT EXISTS `lead_groups` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `contraction` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `operator` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `currency` text COLLATE utf8mb4_unicode_ci,
  `service_type_id` bigint unsigned DEFAULT NULL,
  `due_date` date DEFAULT NULL,
  `profit` decimal(15,2) NOT NULL DEFAULT '0.00',
  `channel_id` bigint unsigned DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `lead_groups_service_type_id_foreign` (`service_type_id`),
  KEY `lead_groups_channel_id_foreign` (`channel_id`),
  CONSTRAINT `lead_groups_channel_id_foreign` FOREIGN KEY (`channel_id`) REFERENCES `channels` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_groups_service_type_id_foreign` FOREIGN KEY (`service_type_id`) REFERENCES `service_types` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.lead_groups: ~4 rows (aproximadamente)
INSERT INTO `lead_groups` (`id`, `name`, `contraction`, `created_at`, `updated_at`, `description`, `operator`, `currency`, `service_type_id`, `due_date`, `profit`, `channel_id`) VALUES
	(2, 'VALLARTA 2026', '202512-VAL', '2025-12-16 21:27:13', '2025-12-16 21:27:13', 'VIAJE TODO INCLUIDO VALLARTA 2026', 'OP TRAVEL', 'MXN', 1, '2026-09-30', 3400.00, 1),
	(3, 'ENERO USA', '202512-ENE', '2025-12-16 22:42:03', '2025-12-16 22:42:03', 'ORLANDO + DISNEY', 'MEGA TRAVEL', 'USD', 1, '2026-01-31', 1000.00, 1),
	(4, 'VIAJE FIN DE AÑO COLEGIO', '202601-VIA', '2026-01-29 23:05:26', '2026-01-29 23:05:26', 'Viaje a la CDMX', 'Mega Travel', 'MXN', 5, '2026-02-28', 2250.00, 1),
	(5, 'Chiapas', '202603-CHI', '2026-03-26 22:29:36', '2026-03-26 22:29:36', 'Viaje Chiapas', NULL, 'MXN', 1, '2026-05-15', 2000.00, 1);

-- Volcando estructura para tabla growvent_staltos.lead_products
CREATE TABLE IF NOT EXISTS `lead_products` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned NOT NULL,
  `lead_service_id` bigint unsigned NOT NULL,
  `departure_id` bigint unsigned DEFAULT NULL,
  `arrival_id` bigint unsigned DEFAULT NULL,
  `airline_id` bigint unsigned DEFAULT NULL,
  `hotel_id` bigint unsigned DEFAULT NULL,
  `occupation_id` bigint unsigned DEFAULT NULL,
  `service_id` bigint unsigned DEFAULT NULL,
  `quantity` decimal(10,2) NOT NULL DEFAULT '1.00',
  `unit_price` decimal(15,2) DEFAULT NULL,
  `discount` decimal(15,2) NOT NULL DEFAULT '0.00',
  `guests` int NOT NULL DEFAULT '1',
  `description` text COLLATE utf8mb4_unicode_ci,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `profit` decimal(15,2) NOT NULL DEFAULT '0.00',
  `flight_datetime` datetime DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `lead_products_lead_id_foreign` (`lead_id`),
  KEY `lead_products_lead_service_id_foreign` (`lead_service_id`),
  KEY `lead_products_departure_id_foreign` (`departure_id`),
  KEY `lead_products_arrival_id_foreign` (`arrival_id`),
  KEY `lead_products_airline_id_foreign` (`airline_id`),
  KEY `lead_products_hotel_id_foreign` (`hotel_id`),
  KEY `lead_products_occupation_id_foreign` (`occupation_id`),
  KEY `lead_products_service_id_foreign` (`service_id`),
  CONSTRAINT `lead_products_airline_id_foreign` FOREIGN KEY (`airline_id`) REFERENCES `airlines` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_products_arrival_id_foreign` FOREIGN KEY (`arrival_id`) REFERENCES `destinations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_products_departure_id_foreign` FOREIGN KEY (`departure_id`) REFERENCES `destinations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_products_hotel_id_foreign` FOREIGN KEY (`hotel_id`) REFERENCES `hotels` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_products_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `lead_products_lead_service_id_foreign` FOREIGN KEY (`lead_service_id`) REFERENCES `lead_services` (`id`) ON DELETE CASCADE,
  CONSTRAINT `lead_products_occupation_id_foreign` FOREIGN KEY (`occupation_id`) REFERENCES `occupations` (`id`) ON DELETE SET NULL,
  CONSTRAINT `lead_products_service_id_foreign` FOREIGN KEY (`service_id`) REFERENCES `services` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=108 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.lead_products: ~43 rows (aproximadamente)
INSERT INTO `lead_products` (`id`, `lead_id`, `lead_service_id`, `departure_id`, `arrival_id`, `airline_id`, `hotel_id`, `occupation_id`, `service_id`, `quantity`, `unit_price`, `discount`, `guests`, `description`, `total`, `profit`, `flight_datetime`, `created_at`, `updated_at`) VALUES
	(47, 2, 1, NULL, 49, NULL, 82, 2, NULL, 1.00, NULL, 0.00, 2, NULL, 4500.00, 1250.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(48, 2, 2, NULL, 49, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Paseo en Yate', 3500.00, 1400.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(49, 2, 3, 20, 49, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 3250.00, 750.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(50, 3, 4, NULL, 49, NULL, 82, 2, NULL, 1.00, NULL, 0.00, 2, NULL, 4500.00, 1250.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(51, 3, 5, NULL, 49, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Paseo en Yate', 3500.00, 1400.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(52, 3, 6, 20, 49, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 3250.00, 750.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(53, 4, 7, NULL, 49, NULL, 82, 2, NULL, 1.00, NULL, 0.00, 2, NULL, 4500.00, 1250.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(54, 4, 8, NULL, 49, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Paseo en Yate', 3500.00, 1400.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(55, 4, 9, 20, 49, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 3250.00, 750.00, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(56, 5, 10, NULL, 9, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Personalizado', 12000.00, 3000.00, NULL, '2025-12-16 22:34:27', '2025-12-16 22:34:27'),
	(57, 6, 11, NULL, 20, NULL, 107, 1, NULL, 1.00, NULL, 0.00, 1, NULL, 7000.00, 1500.00, NULL, '2025-12-16 22:39:05', '2025-12-16 22:39:05'),
	(58, 7, 12, NULL, 104, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'DISNEY + UNIVERSAL STUDIOS', 3000.00, 1000.00, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(59, 8, 13, NULL, 104, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'DISNEY + UNIVERSAL STUDIOS', 3000.00, 1000.00, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(60, 9, 14, NULL, 104, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'DISNEY + UNIVERSAL STUDIOS', 3000.00, 1000.00, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(62, 10, 16, NULL, 121, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Viaje de ida y vuelta a las coloradas', 8000.00, 2000.00, NULL, '2026-01-29 22:51:22', '2026-01-29 22:51:22'),
	(63, 10, 16, NULL, 121, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Servicio adicional', 1000.00, 500.00, NULL, '2026-01-29 22:51:22', '2026-01-29 22:51:22'),
	(64, 11, 17, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(65, 11, 18, NULL, 11, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Centro histórico', 1500.00, 750.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(66, 12, 19, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(67, 12, 20, NULL, 11, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Centro histórico', 1500.00, 750.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(72, 15, 25, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(73, 15, 26, NULL, 11, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Centro histórico', 1500.00, 750.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(74, 16, 27, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(75, 16, 28, NULL, 11, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Centro histórico', 1500.00, 750.00, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(76, 14, 29, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 3, NULL, 7500.00, 2500.00, NULL, '2026-01-29 23:06:58', '2026-01-29 23:06:58'),
	(77, 14, 30, NULL, 11, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'Centro histórico', 2250.00, 1000.00, NULL, '2026-01-29 23:06:58', '2026-01-29 23:06:58'),
	(78, 13, 31, 34, 11, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 5000.00, 1500.00, NULL, '2026-01-29 23:07:24', '2026-01-29 23:07:24'),
	(79, 17, 32, NULL, 102, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'X', 590.00, 118.00, NULL, '2026-01-29 23:35:59', '2026-01-29 23:35:59'),
	(80, 18, 33, 20, 7, 2, NULL, NULL, NULL, 1.00, NULL, 0.00, 2, NULL, 6000.00, 1200.00, '2026-04-07 10:15:00', '2026-03-17 00:18:50', '2026-03-17 00:18:50'),
	(82, 20, 35, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(84, 22, 37, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(85, 23, 38, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(86, 24, 39, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(87, 25, 40, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(88, 26, 41, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(90, 27, 43, 38, 7, 3, NULL, NULL, NULL, 1.00, NULL, 0.00, 3, NULL, 0.00, 0.00, NULL, '2026-04-20 18:54:29', '2026-04-20 18:54:29'),
	(91, 27, 44, NULL, 7, NULL, 83, 2, NULL, 1.00, NULL, 0.00, 3, NULL, 32000.00, 0.00, NULL, '2026-04-20 18:54:29', '2026-04-20 18:54:29'),
	(92, 19, 45, NULL, 49, NULL, NULL, NULL, 3, 1.00, NULL, 0.00, 1, '2 noches 3 días todo incluido', 8000.00, 2500.00, NULL, '2026-05-03 04:19:20', '2026-05-03 04:19:20'),
	(98, 28, 50, NULL, 122, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Semana Libre en La Paz', 35000.00, 7000.00, NULL, '2026-05-03 04:47:23', '2026-05-03 04:47:23'),
	(99, 21, 51, NULL, 76, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, '3 días', 8000.00, 2000.00, NULL, '2026-05-03 04:56:17', '2026-05-03 04:56:17'),
	(100, 29, 52, NULL, 122, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'TOYOTA RAIZE', 1500.00, 300.00, NULL, '2026-05-14 20:25:58', '2026-05-14 20:25:58'),
	(101, 30, 53, NULL, 7, NULL, NULL, NULL, 1, 1.00, NULL, 0.00, 1, 'Cupon Promocional', 10.00, 0.00, NULL, '2026-08-25 00:17:42', '2026-08-25 00:17:42'),
	(102, 31, 54, NULL, 7, NULL, NULL, NULL, 2, 1.00, NULL, 0.00, 1, 'En lancha 5 personas', 100.00, 0.00, NULL, '2026-08-25 19:54:10', '2026-08-25 19:54:10'),
	(103, 32, 55, NULL, NULL, NULL, NULL, NULL, NULL, 1.00, 1000.00, 0.00, 1, 'Prueba', 1000.00, 0.00, NULL, '2026-09-02 05:49:17', '2026-09-02 05:49:17'),
	(104, 33, 56, NULL, NULL, NULL, NULL, NULL, NULL, 1.00, 1000.00, 0.00, 1, 'Testing', 1000.00, 0.00, NULL, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(105, 33, 57, NULL, NULL, NULL, NULL, NULL, NULL, 2.00, 500.00, 0.00, 1, 'Cuanto más', 1000.00, 0.00, NULL, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(106, 34, 58, NULL, NULL, NULL, NULL, NULL, NULL, 2.00, 1000.00, 0.00, 1, 'Tour', 2000.00, 0.00, NULL, '2026-09-03 03:16:37', '2026-09-03 03:16:37'),
	(107, 35, 59, NULL, NULL, NULL, NULL, NULL, NULL, 1.00, 500.00, 0.00, 1, 'Testing', 500.00, 0.00, NULL, '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.lead_profits
CREATE TABLE IF NOT EXISTS `lead_profits` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned NOT NULL,
  `profit_distribution_id` bigint unsigned DEFAULT NULL,
  `profit` decimal(10,2) NOT NULL DEFAULT '0.00',
  `operator` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `lead_profits_lead_id_index` (`lead_id`),
  KEY `lead_profits_profit_distribution_id_index` (`profit_distribution_id`),
  CONSTRAINT `lead_profits_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `lead_profits_profit_distribution_id_foreign` FOREIGN KEY (`profit_distribution_id`) REFERENCES `profit_distributions` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.lead_profits: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.lead_services
CREATE TABLE IF NOT EXISTS `lead_services` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned NOT NULL,
  `service_category_id` bigint unsigned DEFAULT NULL,
  `round_trip` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `lead_services_lead_id_foreign` (`lead_id`),
  KEY `lead_services_service_category_id_foreign` (`service_category_id`),
  CONSTRAINT `lead_services_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `lead_services_service_category_id_foreign` FOREIGN KEY (`service_category_id`) REFERENCES `service_categories` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=60 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.lead_services: ~42 rows (aproximadamente)
INSERT INTO `lead_services` (`id`, `lead_id`, `service_category_id`, `round_trip`, `created_at`, `updated_at`) VALUES
	(1, 2, 2, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(2, 2, 3, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(3, 2, 1, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(4, 3, 2, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(5, 3, 3, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(6, 3, 1, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(7, 4, 2, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(8, 4, 3, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(9, 4, 1, 0, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(10, 5, 3, 0, '2025-12-16 22:34:27', '2025-12-16 22:34:27'),
	(11, 6, 2, 0, '2025-12-16 22:39:05', '2025-12-16 22:39:05'),
	(12, 7, 3, 0, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(13, 8, 3, 0, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(14, 9, 3, 0, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(16, 10, 4, 0, '2026-01-29 22:51:22', '2026-01-29 22:51:22'),
	(17, 11, 1, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(18, 11, 3, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(19, 12, 1, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(20, 12, 3, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(25, 15, 1, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(26, 15, 3, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(27, 16, 1, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(28, 16, 3, 0, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(29, 14, 1, 0, '2026-01-29 23:06:58', '2026-01-29 23:06:58'),
	(30, 14, 3, 0, '2026-01-29 23:06:58', '2026-01-29 23:06:58'),
	(31, 13, 1, 0, '2026-01-29 23:07:24', '2026-01-29 23:07:24'),
	(32, 17, 3, 0, '2026-01-29 23:35:59', '2026-01-29 23:35:59'),
	(33, 18, 1, 0, '2026-03-17 00:18:50', '2026-03-17 00:18:50'),
	(35, 20, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(37, 22, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(38, 23, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(39, 24, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(40, 25, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(41, 26, 3, 0, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(43, 27, 1, 0, '2026-04-20 18:54:29', '2026-04-20 18:54:29'),
	(44, 27, 2, 0, '2026-04-20 18:54:29', '2026-04-20 18:54:29'),
	(45, 19, 4, 0, '2026-05-03 04:19:20', '2026-05-03 04:19:20'),
	(50, 28, 4, 0, '2026-05-03 04:47:23', '2026-05-03 04:47:23'),
	(51, 21, 3, 0, '2026-05-03 04:56:17', '2026-05-03 04:56:17'),
	(52, 29, 3, 0, '2026-05-14 20:25:58', '2026-05-14 20:25:58'),
	(53, 30, 3, 0, '2026-08-25 00:17:42', '2026-08-25 00:17:42'),
	(54, 31, 4, 0, '2026-08-25 19:54:10', '2026-08-25 19:54:10'),
	(55, 32, 3, 0, '2026-09-02 05:49:17', '2026-09-02 05:49:17'),
	(56, 33, 3, 0, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(57, 33, 3, 0, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(58, 34, 3, 0, '2026-09-03 03:16:37', '2026-09-03 03:16:37'),
	(59, 35, 3, 0, '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.megatravel_categories
CREATE TABLE IF NOT EXISTS `megatravel_categories` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `slug` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `megatravel_categories_name_unique` (`name`),
  UNIQUE KEY `megatravel_categories_slug_unique` (`slug`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.megatravel_categories: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.megatravel_offers
CREATE TABLE IF NOT EXISTS `megatravel_offers` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `mt_clv` int NOT NULL,
  `mt_code` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `price_from` decimal(10,2) NOT NULL,
  `currency` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'USD',
  `imp_from` decimal(10,2) DEFAULT NULL,
  `detail_url` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `image_url` text COLLATE utf8mb4_unicode_ci,
  `description` text COLLATE utf8mb4_unicode_ci,
  `active` tinyint(1) NOT NULL DEFAULT '1',
  `source` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'megatravel',
  `last_synced_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `megatravel_offers_mt_clv_unique` (`mt_clv`),
  KEY `megatravel_offers_active_index` (`active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.megatravel_offers: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.megatravel_offer_category
CREATE TABLE IF NOT EXISTS `megatravel_offer_category` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `megatravel_offer_id` bigint unsigned NOT NULL,
  `megatravel_category_id` bigint unsigned NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `offer_category_unique` (`megatravel_offer_id`,`megatravel_category_id`),
  KEY `megatravel_offer_category_megatravel_category_id_foreign` (`megatravel_category_id`),
  CONSTRAINT `megatravel_offer_category_megatravel_category_id_foreign` FOREIGN KEY (`megatravel_category_id`) REFERENCES `megatravel_categories` (`id`) ON DELETE CASCADE,
  CONSTRAINT `megatravel_offer_category_megatravel_offer_id_foreign` FOREIGN KEY (`megatravel_offer_id`) REFERENCES `megatravel_offers` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.megatravel_offer_category: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.migrations
CREATE TABLE IF NOT EXISTS `migrations` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `migration` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `batch` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=111 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.migrations: ~106 rows (aproximadamente)
INSERT INTO `migrations` (`id`, `migration`, `batch`) VALUES
	(1, '0001_01_01_000000_create_users_table', 1),
	(2, '0001_01_01_000001_create_cache_table', 1),
	(3, '0001_01_01_000002_create_jobs_table', 1),
	(4, '2025_03_16_012040_create_personal_access_tokens_table', 1),
	(5, '2025_03_16_012040_create_roles_table', 1),
	(6, '2025_03_16_012040_create_user_roles_table', 1),
	(7, '2025_03_16_140400_create_subsidiaries_table', 1),
	(8, '2025_03_16_140401_create_periods_table', 1),
	(9, '2025_03_16_140402_create_classifications_table', 1),
	(10, '2025_03_16_140403_create_groups_table', 1),
	(11, '2025_03_16_140500_create_transactions_table', 1),
	(12, '2025_03_16_140601_create_stages_table', 1),
	(13, '2025_03_16_140603_create_service_types_table', 1),
	(14, '2025_03_16_140604_create_customers_table', 1),
	(15, '2025_03_16_140605_create_preleads_table', 1),
	(16, '2025_03_16_140700_create_leads_table', 1),
	(17, '2025_03_16_140800_create_payments_table', 1),
	(18, '2025_03_16_140900_create_reminders_table', 1),
	(19, '2025_03_16_140901_create_comments_table', 1),
	(20, '2025_03_16_140902_create_files_table', 1),
	(21, '2025_03_16_140903_create_tags_table', 1),
	(22, '2025_03_16_140904_create_service_categories_table', 1),
	(23, '2025_03_16_140905_create_lead_services_table', 1),
	(24, '2025_04_04_140900_create_destinations_table copy', 1),
	(25, '2025_04_04_140901_create_airlines_table', 1),
	(26, '2025_04_04_140902_create_hotels_table', 1),
	(27, '2025_04_04_140903_create_occupations_table', 1),
	(28, '2025_04_04_140904_create_bank_accounts_table', 1),
	(29, '2025_04_04_140905_create_services_table', 1),
	(30, '2025_04_04_140906_create_lead_products_table', 1),
	(31, '2025_05_31_140900_create_currencies_table', 1),
	(32, '2025_06_01_000000_create_settings_table', 1),
	(33, '2025_06_02_000000_create_lead_groups_table', 1),
	(34, '2025_06_02_000001_add_lead_group_id_to_leads_table', 1),
	(35, '2025_06_02_000002_create_group_services_table', 1),
	(36, '2025_06_02_000003_create_group_products_table', 1),
	(37, '2025_06_03_000000_create_supplier_payments_table', 1),
	(38, '2025_06_13_000001_add_description_to_payments_table', 1),
	(39, '2025_06_13_000002_add_hr_fields_to_users_table', 1),
	(40, '2025_06_13_000002_update_inventario_role_to_colaborador', 1),
	(41, '2025_06_13_000003_add_position_to_users_table', 1),
	(42, '2025_06_14_000000_create_payroll_payments_table', 1),
	(43, '2025_06_14_000000_create_vacations_table', 1),
	(44, '2025_06_15_000000_add_fields_to_lead_groups_table', 1),
	(45, '2025_06_24_000000_create_profit_distributions_table', 1),
	(46, '2025_06_24_000001_create_lead_profits_table', 1),
	(47, '2025_06_24_000001_create_payroll_transaction_settings_table', 1),
	(48, '2025_06_24_000002_create_commission_transaction_settings_table', 1),
	(49, '2025_06_24_000003_add_exchange_rate_to_payments_table', 1),
	(50, '2025_06_24_000004_add_exchange_rate_to_supplier_payments_table', 1),
	(51, '2025_06_24_170747_create_retained_profit_settings_table', 1),
	(52, '2025_06_24_205503_add_commission_to_payroll_payments_table', 1),
	(53, '2025_06_24_205613_modify_vacations_status_fields', 1),
	(54, '2025_06_25_000001_create_supplier_group_payments_table', 1),
	(55, '2025_06_25_000002_add_supplier_group_payment_id_to_supplier_payments_table', 1),
	(56, '2025_08_15_000001_create_channels_table', 1),
	(57, '2025_08_15_000002_add_channel_id_to_leads_table', 1),
	(58, '2025_08_16_000000_add_benefit_and_bonus_to_payroll_payments_table', 1),
	(59, '2025_08_16_000001_create_benefit_transaction_settings_table', 1),
	(60, '2025_08_16_000002_create_bonus_transaction_settings_table', 1),
	(61, '2025_08_18_183213_create_events_table', 1),
	(62, '2025_08_28_045546_add_channel_id_to_lead_groups_table', 1),
	(63, '2025_09_05_000001_add_operator_to_lead_profits_table', 1),
	(64, '2025_09_05_175000_create_customer_classifications_table', 1),
	(65, '2025_09_05_175001_add_classification_id_to_customers_table', 1),
	(66, '2025_09_05_184000_add_lead_group_id_to_files_table', 1),
	(67, '2025_09_05_184001_add_lead_group_id_to_reminders_table', 1),
	(68, '2025_09_05_184002_add_lead_group_id_to_comments_table', 1),
	(69, '2025_09_11_045034_add_reservation_config_to_settings_table', 1),
	(70, '2025_09_13_000001_add_profit_percentage_to_settings_table', 1),
	(71, '2025_09_13_000001_create_payment_settings_table', 1),
	(72, '2025_09_13_000002_add_profit_include_in_total_to_settings_table', 1),
	(73, '2025_09_13_000002_create_payment_supplier_settings_table', 1),
	(74, '2025_11_16_175507_add_file_path_to_payments_table', 1),
	(75, '2025_11_16_193918_add_operator_to_leads_table', 1),
	(76, '2025_11_16_193922_add_operator_to_lead_groups_table', 1),
	(77, '2025_11_16_234443_add_foreign_keys_to_transactions_table', 1),
	(78, '2025_11_17_013447_add_profit_to_lead_products_table', 1),
	(79, '2025_11_17_013453_add_profit_to_group_products_table', 1),
	(80, '2025_12_14_181030_create_megatravel_offers_table', 1),
	(81, '2025_12_14_181037_create_customer_megatravel_interests_table', 1),
	(82, '2025_12_14_190000_add_fields_to_megatravel_offers_table', 1),
	(83, '2025_12_14_190001_create_megatravel_categories_table', 1),
	(84, '2025_12_14_190002_create_megatravel_offer_category_table', 1),
	(85, '2026_03_13_000001_add_enable_birthdate_to_settings_table', 1),
	(86, '2026_03_13_000002_add_flight_datetime_to_lead_products_table', 1),
	(87, '2026_03_13_000003_add_flight_datetime_to_group_products_table', 1),
	(88, '2026_03_14_180429_add_public_token_to_customers_table', 1),
	(89, '2026_03_14_180429_add_public_token_to_leads_table', 1),
	(90, '2026_03_14_180430_populate_public_token_existing_records', 1),
	(91, '2026_03_14_200000_create_payment_requests_table', 1),
	(92, '2026_03_14_200001_create_payment_attempts_table', 1),
	(93, '2026_03_14_203714_alter_payment_attempts_checkout_url_to_text', 1),
	(94, '2026_03_15_000001_create_payment_installments_table', 1),
	(95, '2026_03_15_000002_add_payment_installment_id_to_reminders_table', 1),
	(96, '2026_03_15_000003_add_payment_installment_id_to_payment_requests_table', 1),
	(97, '2026_03_15_000004_add_payment_intent_id_to_payment_attempts_table', 1),
	(98, '2026_03_31_000001_add_round_trip_to_lead_services_table', 2),
	(99, '2026_03_31_000002_add_round_trip_to_group_services_table', 2),
	(100, '2026_04_01_000001_add_hidden_to_roles_table', 2),
	(101, '2026_04_01_000002_create_role_screens_table', 2),
	(102, '2026_04_02_000001_add_public_to_files_table', 2),
	(103, '2026_04_05_000001_add_survey_to_leads_table', 2),
	(104, '2026_04_05_000002_create_surveys_table', 2),
	(105, '2026_04_05_000003_create_survey_questions_table', 2),
	(106, '2026_04_05_000004_create_survey_answers_table', 2),
	(107, '2026_09_01_000001_add_pos_sale_to_leads_table', 3),
	(108, '2026_09_01_000002_add_quantity_unit_price_discount_to_lead_products_table', 3),
	(109, '2026_09_01_000003_create_pos_custom_field_definitions_table', 3),
	(110, '2026_09_01_000004_create_pos_custom_field_values_table', 3);

-- Volcando estructura para tabla growvent_staltos.occupations
CREATE TABLE IF NOT EXISTS `occupations` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `adults` int NOT NULL DEFAULT '0',
  `kids` int NOT NULL DEFAULT '0',
  `order` int NOT NULL DEFAULT '0',
  `contraction` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.occupations: ~10 rows (aproximadamente)
INSERT INTO `occupations` (`id`, `name`, `adults`, `kids`, `order`, `contraction`, `created_at`, `updated_at`) VALUES
	(1, 'Single', 1, 0, 0, 'SGL', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(2, 'Doble', 2, 0, 1, 'DBL', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(3, 'Doble con 1 niño', 2, 1, 2, 'DBL1K', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(4, 'Doble con 2 niños', 2, 2, 3, 'DBL2K', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(5, 'Triple', 3, 0, 4, 'TPL', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(6, 'Triple con 1 niño', 3, 1, 5, 'TPL1K', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(7, 'Cuádruple', 4, 0, 6, 'CUA', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(8, 'Familiar', 2, 3, 7, 'FAM', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(9, 'Suite', 2, 2, 8, 'STE', '2025-12-16 21:06:21', '2025-12-16 21:06:21'),
	(10, 'Suite Familiar', 4, 2, 9, 'STF', '2025-12-16 21:06:21', '2025-12-16 21:06:21');

-- Volcando estructura para tabla growvent_staltos.password_reset_tokens
CREATE TABLE IF NOT EXISTS `password_reset_tokens` (
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `token` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.password_reset_tokens: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.payments
CREATE TABLE IF NOT EXISTS `payments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `currency` varchar(3) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `exchange_rate` decimal(10,4) NOT NULL DEFAULT '1.0000',
  `total_mxn` decimal(15,2) DEFAULT NULL,
  `type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `reference` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `paid` tinyint(1) NOT NULL DEFAULT '0',
  `lead_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payments_lead_id_foreign` (`lead_id`),
  CONSTRAINT `payments_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payments: ~24 rows (aproximadamente)
INSERT INTO `payments` (`id`, `total`, `currency`, `exchange_rate`, `total_mxn`, `type`, `reference`, `file_path`, `description`, `paid`, `lead_id`, `created_at`, `updated_at`) VALUES
	(1, 500.00, 'MXN', 1.0000, 500.00, 'Tarjeta', '123456', NULL, 'Apartado', 0, 3, '2025-12-16 22:39:26', '2025-12-16 22:39:26'),
	(2, 1000.00, 'MXN', 1.0000, 1000.00, 'Transferencia', '987654', NULL, 'Apartado', 1, 5, '2025-12-16 22:39:41', '2025-12-16 22:39:52'),
	(3, 1000.00, 'USD', 18.0000, 18000.00, 'Efectivo', '951357', NULL, 'ABONO', 1, 8, '2025-12-16 22:42:27', '2025-12-16 22:43:24'),
	(4, 1000.00, 'USD', 18.6000, 18600.00, 'Efectivo', '3578951', NULL, 'ANTICIPO', 0, 7, '2025-12-16 22:43:59', '2025-12-16 22:43:59'),
	(5, 14650.00, 'MXN', 1.0000, 14650.00, 'Efectivo', '98521456', NULL, 'LIQUIDACION', 1, 3, '2025-12-16 22:44:25', '2025-12-16 22:44:25'),
	(6, 2500.00, 'MXN', 1.0000, 2500.00, 'Efectivo', NULL, 'https://growventa-public.s3.amazonaws.com/growtravel/AeDYkXd5Gq7sxZlCIgHtMKPHNnz7dboKUZ9iDwWH.pdf', 'Abono para confirmar la reservación', 1, 10, '2026-01-29 22:52:51', '2026-01-29 22:54:05'),
	(7, 2000.00, 'MXN', 1.0000, 2000.00, 'Efectivo', NULL, NULL, 'Segundo abono', 1, 10, '2026-01-29 22:55:15', '2026-01-29 22:55:19'),
	(8, 8750.00, 'MXN', 1.0000, 8750.00, 'Efectivo', NULL, NULL, 'Liquidacion', 1, 11, '2026-01-29 23:16:46', '2026-01-29 23:16:46'),
	(9, 2000.00, 'MXN', 1.0000, 2000.00, 'Efectivo', NULL, NULL, 'Abono a reserva COCO-000010', 1, 10, '2026-01-30 21:39:13', '2026-01-30 21:39:16'),
	(10, 141.60, 'USD', 17.6701, 2502.09, 'Stripe', 'cs_test_a1ILHOThaX47Rof7Nn4vCsZZHARCSHfNWvkvFv7ALyeA4GETNXVD2M0NIM', NULL, 'Pago digital (stripe) – intento #1', 1, 17, '2026-03-16 23:58:07', '2026-03-16 23:58:09'),
	(11, 141.60, 'USD', 17.0812, 2418.70, 'Paypal', '1MU19647G8659084N', NULL, 'Pago digital (paypal) – intento #2', 1, 17, '2026-03-17 00:01:16', '2026-03-17 00:01:16'),
	(12, 6500.00, 'MXN', 1.0000, 6500.00, 'Stripe', 'cs_test_a1brKtTu4iqttBpsQOGie3se4JvIYYeghSwlmLTxU1ezIxGTmUBDzzmqoV', NULL, 'Pago digital (stripe) – intento #3', 1, 13, '2026-03-17 00:10:11', '2026-03-17 00:10:11'),
	(13, 1028.57, 'MXN', 1.0000, 1028.57, 'Stripe', 'cs_test_a1vHcC4vP2IlUqDjsESvKWFKxFyopITLnzhjY4aCYX3Nrnqm8wJQZh4mLl', NULL, 'Pago digital (stripe) – intento #4', 1, 18, '2026-03-17 22:12:04', '2026-03-17 22:12:04'),
	(14, 1104.17, 'MXN', 1.0000, 1104.17, 'Stripe', 'cs_test_a1UQsztl4PLRuDo4GwgjpBjGSahspXevAfGZQeVl5BisZXcrTIIKvzzBNj', NULL, 'Pago digital (stripe) – intento #6', 1, 14, '2026-03-18 22:53:30', '2026-03-18 22:53:30'),
	(15, 141.60, 'USD', 17.6968, 2505.87, 'Stripe', 'cs_test_a1ARPSA6jkGDcMrqyGOmCzSiqotoQhs6A7pnP7lRoDzfu3CjxYncx3J20y', NULL, 'Pago digital (stripe) – intento #7', 1, 17, '2026-03-18 22:54:57', '2026-03-18 22:55:00'),
	(16, 20.00, 'USD', 20.0000, 400.00, 'Efectivo', 'TEST', NULL, 'TEST', 1, 17, '2026-03-21 23:52:32', '2026-03-21 23:52:32'),
	(17, 1750.00, 'MXN', 1.0000, 1750.00, 'Stripe', 'cs_test_a1vKpLuGsX6u66XWryg875xcqRd3OUZfut3R1ME931hyCZERlMRF0hKV81', NULL, 'Pago digital (stripe) – intento #10', 1, 12, '2026-03-26 22:15:14', '2026-03-26 22:15:14'),
	(18, 1000.00, 'MXN', 1.0000, 1000.00, 'Efectivo', NULL, NULL, 'Segundo abono', 1, 12, '2026-03-26 22:25:38', '2026-03-26 22:25:38'),
	(19, 1800.00, 'MXN', 1.0000, 1800.00, 'Efectivo', NULL, 'https://growventa-public.s3.amazonaws.com/growtravel/MrrI47NOerxPO8OoGEwnIHW5EVUh12I0sK2bvHA0.pdf', 'Abono 1', 1, 21, '2026-03-26 22:34:51', '2026-03-26 22:35:33'),
	(20, 8000.00, 'MXN', 1.0000, 8000.00, 'Efectivo', NULL, NULL, NULL, 1, 27, '2026-04-20 18:58:11', '2026-04-20 18:58:18'),
	(21, 50.00, 'MXN', 1.0000, 50.00, 'Efectivo', NULL, NULL, NULL, 1, 22, '2026-05-09 06:51:38', '2026-05-09 06:51:38'),
	(22, 1800.00, 'MXN', 1.0000, 1800.00, 'Stripe', 'cs_test_a1ilH6cdCXMXvpdQQJB6cC7FvR9kYa2wh9rPmccDlreeTTxErdKunbXOk4', NULL, 'Pago digital (stripe) – intento #11', 1, 29, '2026-05-23 01:49:31', '2026-05-23 01:49:31'),
	(23, 50.00, 'USD', 19.2400, 962.00, 'Efectivo', 'Pago tarjeta', NULL, 'Primer abono', 1, 31, '2026-08-25 19:55:27', '2026-08-25 19:55:27'),
	(24, 50.00, 'USD', 19.2400, 962.00, 'Efectivo', 'Pago tarjeta', NULL, 'Liquidacion', 1, 31, '2026-08-25 19:56:32', '2026-08-25 19:56:32'),
	(25, 1000.00, 'MXN', 1.0000, 1000.00, 'Efectivo', NULL, NULL, NULL, 1, 33, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(26, 500.00, 'MXN', 1.0000, 500.00, 'Transferencia', NULL, NULL, NULL, 1, 34, '2026-09-03 03:16:37', '2026-09-03 03:16:37'),
	(27, 250.00, 'MXN', 1.0000, 250.00, 'Efectivo', NULL, NULL, NULL, 1, 35, '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.payment_attempts
CREATE TABLE IF NOT EXISTS `payment_attempts` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `payment_request_id` bigint unsigned NOT NULL,
  `provider` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  `provider_reference` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `payment_intent_id` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `provider_response` text COLLATE utf8mb4_unicode_ci,
  `checkout_url` text COLLATE utf8mb4_unicode_ci,
  `status` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'pending',
  `amount` decimal(15,2) NOT NULL,
  `currency` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payment_attempts_payment_request_id_foreign` (`payment_request_id`),
  KEY `payment_attempts_provider_reference_index` (`provider_reference`),
  KEY `payment_attempts_payment_intent_id_index` (`payment_intent_id`),
  CONSTRAINT `payment_attempts_payment_request_id_foreign` FOREIGN KEY (`payment_request_id`) REFERENCES `payment_requests` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payment_attempts: ~12 rows (aproximadamente)
INSERT INTO `payment_attempts` (`id`, `payment_request_id`, `provider`, `provider_reference`, `payment_intent_id`, `provider_response`, `checkout_url`, `status`, `amount`, `currency`, `created_at`, `updated_at`) VALUES
	(1, 1, 'stripe', 'cs_test_a1ILHOThaX47Rof7Nn4vCsZZHARCSHfNWvkvFv7ALyeA4GETNXVD2M0NIM', 'pi_3TBhH7DAViW7oZaY1pddB154', '{\n  "id": "cs_test_a1ILHOThaX47Rof7Nn4vCsZZHARCSHfNWvkvFv7ALyeA4GETNXVD2M0NIM",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 14160,\n  "amount_total": 14160,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/7a6cb93b-9ab4-4451-8a7d-a6ceaf68cefa/cancelado",\n  "client_reference_id": "1",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1773691066,\n  "currency": "usd",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1773777466,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "1",\n    "request_token": "7a6cb93b-9ab4-4451-8a7d-a6ceaf68cefa"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/7a6cb93b-9ab4-4451-8a7d-a6ceaf68cefa/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1ILHOThaX47Rof7Nn4vCsZZHARCSHfNWvkvFv7ALyeA4GETNXVD2M0NIM#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1ILHOThaX47Rof7Nn4vCsZZHARCSHfNWvkvFv7ALyeA4GETNXVD2M0NIM#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 141.60, 'USD', '2026-03-16 23:57:45', '2026-03-16 23:58:07'),
	(2, 2, 'paypal', '1MU19647G8659084N', '1GB615170S832213B', '{"id":"1MU19647G8659084N","status":"CREATED","links":[{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/1MU19647G8659084N","rel":"self","method":"GET"},{"href":"https://www.sandbox.paypal.com/checkoutnow?token=1MU19647G8659084N","rel":"approve","method":"GET"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/1MU19647G8659084N","rel":"update","method":"PATCH"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/1MU19647G8659084N/capture","rel":"capture","method":"POST"}]}', 'https://www.sandbox.paypal.com/checkoutnow?token=1MU19647G8659084N', 'succeeded', 141.60, 'USD', '2026-03-16 23:59:39', '2026-03-17 00:01:16'),
	(3, 3, 'stripe', 'cs_test_a1brKtTu4iqttBpsQOGie3se4JvIYYeghSwlmLTxU1ezIxGTmUBDzzmqoV', 'pi_3TBhSmDAViW7oZaY0mbphO2N', '{\n  "id": "cs_test_a1brKtTu4iqttBpsQOGie3se4JvIYYeghSwlmLTxU1ezIxGTmUBDzzmqoV",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 650000,\n  "amount_total": 650000,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/48b2f328-d4c6-4fa6-b147-3551625da1cd/reservacion/e576e966-8358-48b9-9b34-5191d9d605d7/pago/869b3cdf-3d83-42db-94d4-c31cf8d2dabc/cancelado",\n  "client_reference_id": "3",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1773691792,\n  "currency": "mxn",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1773778192,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "3",\n    "request_token": "869b3cdf-3d83-42db-94d4-c31cf8d2dabc"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/48b2f328-d4c6-4fa6-b147-3551625da1cd/reservacion/e576e966-8358-48b9-9b34-5191d9d605d7/pago/869b3cdf-3d83-42db-94d4-c31cf8d2dabc/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1brKtTu4iqttBpsQOGie3se4JvIYYeghSwlmLTxU1ezIxGTmUBDzzmqoV#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1brKtTu4iqttBpsQOGie3se4JvIYYeghSwlmLTxU1ezIxGTmUBDzzmqoV#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 6500.00, 'MXN', '2026-03-17 00:09:51', '2026-03-17 00:10:11'),
	(4, 4, 'stripe', 'cs_test_a1vHcC4vP2IlUqDjsESvKWFKxFyopITLnzhjY4aCYX3Nrnqm8wJQZh4mLl', 'pi_3TC263DAViW7oZaY0m64zpG2', '{\n  "id": "cs_test_a1vHcC4vP2IlUqDjsESvKWFKxFyopITLnzhjY4aCYX3Nrnqm8wJQZh4mLl",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 102857,\n  "amount_total": 102857,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/5435238a-c036-4e17-b320-f539ba94500f/pago/5f52a091-b3ed-4ab4-96a0-8df89bf46df5/cancelado",\n  "client_reference_id": "4",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1773771082,\n  "currency": "mxn",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1773857482,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "4",\n    "request_token": "5f52a091-b3ed-4ab4-96a0-8df89bf46df5"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/5435238a-c036-4e17-b320-f539ba94500f/pago/5f52a091-b3ed-4ab4-96a0-8df89bf46df5/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1vHcC4vP2IlUqDjsESvKWFKxFyopITLnzhjY4aCYX3Nrnqm8wJQZh4mLl#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1vHcC4vP2IlUqDjsESvKWFKxFyopITLnzhjY4aCYX3Nrnqm8wJQZh4mLl#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 1028.57, 'MXN', '2026-03-17 22:11:20', '2026-03-17 22:12:04'),
	(5, 5, 'paypal', '9E210863KG134532K', NULL, '{"id":"9E210863KG134532K","status":"CREATED","links":[{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/9E210863KG134532K","rel":"self","method":"GET"},{"href":"https://www.sandbox.paypal.com/checkoutnow?token=9E210863KG134532K","rel":"approve","method":"GET"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/9E210863KG134532K","rel":"update","method":"PATCH"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/9E210863KG134532K/capture","rel":"capture","method":"POST"}]}', 'https://www.sandbox.paypal.com/checkoutnow?token=9E210863KG134532K', 'processing', 1028.57, 'MXN', '2026-03-17 22:13:18', '2026-03-17 22:13:19'),
	(6, 6, 'stripe', 'cs_test_a1UQsztl4PLRuDo4GwgjpBjGSahspXevAfGZQeVl5BisZXcrTIIKvzzBNj', 'pi_3TCPDhDAViW7oZaY0czSEqaj', '{\n  "id": "cs_test_a1UQsztl4PLRuDo4GwgjpBjGSahspXevAfGZQeVl5BisZXcrTIIKvzzBNj",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 110417,\n  "amount_total": 110417,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/43c9be04-bb83-49fd-aa59-2a888310dbad/reservacion/5211b65f-eef3-4788-8c55-1d26a1488686/pago/a8011bb2-4ef3-4d76-8c0d-a1e8843b22e7/cancelado",\n  "client_reference_id": "6",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1773859991,\n  "currency": "mxn",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1773946391,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "6",\n    "request_token": "a8011bb2-4ef3-4d76-8c0d-a1e8843b22e7"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/43c9be04-bb83-49fd-aa59-2a888310dbad/reservacion/5211b65f-eef3-4788-8c55-1d26a1488686/pago/a8011bb2-4ef3-4d76-8c0d-a1e8843b22e7/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1UQsztl4PLRuDo4GwgjpBjGSahspXevAfGZQeVl5BisZXcrTIIKvzzBNj#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1UQsztl4PLRuDo4GwgjpBjGSahspXevAfGZQeVl5BisZXcrTIIKvzzBNj#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 1104.17, 'MXN', '2026-03-18 22:53:10', '2026-03-18 22:53:30'),
	(7, 7, 'stripe', 'cs_test_a1ARPSA6jkGDcMrqyGOmCzSiqotoQhs6A7pnP7lRoDzfu3CjxYncx3J20y', 'pi_3TCPF5DAViW7oZaY0HJqeFOY', '{\n  "id": "cs_test_a1ARPSA6jkGDcMrqyGOmCzSiqotoQhs6A7pnP7lRoDzfu3CjxYncx3J20y",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 14160,\n  "amount_total": 14160,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/5b50f9b5-d2ca-4531-acc5-3ffa51a05ad3/cancelado",\n  "client_reference_id": "7",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1773860073,\n  "currency": "usd",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1773946473,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "7",\n    "request_token": "5b50f9b5-d2ca-4531-acc5-3ffa51a05ad3"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/5b50f9b5-d2ca-4531-acc5-3ffa51a05ad3/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1ARPSA6jkGDcMrqyGOmCzSiqotoQhs6A7pnP7lRoDzfu3CjxYncx3J20y#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1ARPSA6jkGDcMrqyGOmCzSiqotoQhs6A7pnP7lRoDzfu3CjxYncx3J20y#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 141.60, 'USD', '2026-03-18 22:54:32', '2026-03-18 22:54:57'),
	(8, 8, 'stripe', 'cs_test_a1ujcaFPuHEFX0tcSY2FZMrOvfgN8WJSi8igvcrxVLBkZY3WyM6qz6VDRH', NULL, '{\n  "id": "cs_test_a1ujcaFPuHEFX0tcSY2FZMrOvfgN8WJSi8igvcrxVLBkZY3WyM6qz6VDRH",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 14160,\n  "amount_total": 14160,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/3a4e2051-66e2-44e1-91ab-c98479d4fed4/cancelado",\n  "client_reference_id": "8",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1774122622,\n  "currency": "usd",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1774209022,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "8",\n    "request_token": "3a4e2051-66e2-44e1-91ab-c98479d4fed4"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/cf6319d8-f020-49be-9bcd-b03b74ceea9c/reservacion/eeef7e9b-1b71-47c2-9deb-44a80239f7b3/pago/3a4e2051-66e2-44e1-91ab-c98479d4fed4/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1ujcaFPuHEFX0tcSY2FZMrOvfgN8WJSi8igvcrxVLBkZY3WyM6qz6VDRH#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1ujcaFPuHEFX0tcSY2FZMrOvfgN8WJSi8igvcrxVLBkZY3WyM6qz6VDRH#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'processing', 141.60, 'USD', '2026-03-21 23:50:21', '2026-03-21 23:50:22'),
	(9, 9, 'paypal', '6B764835P1063184W', NULL, '{"id":"6B764835P1063184W","status":"CREATED","links":[{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/6B764835P1063184W","rel":"self","method":"GET"},{"href":"https://www.sandbox.paypal.com/checkoutnow?token=6B764835P1063184W","rel":"approve","method":"GET"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/6B764835P1063184W","rel":"update","method":"PATCH"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/6B764835P1063184W/capture","rel":"capture","method":"POST"}]}', 'https://www.sandbox.paypal.com/checkoutnow?token=6B764835P1063184W', 'processing', 141.60, 'USD', '2026-03-22 00:13:47', '2026-03-22 00:13:48'),
	(10, 10, 'stripe', 'cs_test_a1vKpLuGsX6u66XWryg875xcqRd3OUZfut3R1ME931hyCZERlMRF0hKV81', 'pi_3TFIR3DAViW7oZaY0E1yurtr', '{\n  "id": "cs_test_a1vKpLuGsX6u66XWryg875xcqRd3OUZfut3R1ME931hyCZERlMRF0hKV81",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 175000,\n  "amount_total": 175000,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/98a0f6a9-a527-4a4a-8fb3-3c989361fbe2/reservacion/f05b646a-207e-402f-9801-6c400d6ccf73/pago/53d8e6b1-84fa-486e-bab6-17ce59b5b7cd/cancelado",\n  "client_reference_id": "10",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1774548894,\n  "currency": "mxn",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1774635294,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "metadata": {\n    "attempt_id": "10",\n    "request_token": "53d8e6b1-84fa-486e-bab6-17ce59b5b7cd"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/98a0f6a9-a527-4a4a-8fb3-3c989361fbe2/reservacion/f05b646a-207e-402f-9801-6c400d6ccf73/pago/53d8e6b1-84fa-486e-bab6-17ce59b5b7cd/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1vKpLuGsX6u66XWryg875xcqRd3OUZfut3R1ME931hyCZERlMRF0hKV81#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1vKpLuGsX6u66XWryg875xcqRd3OUZfut3R1ME931hyCZERlMRF0hKV81#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdkdWxOYHwnPyd1blpxYHZxWjA0UUR8XDBBRFNsUjJqX2RcPX9QdG0zRHNONlFdX3NhXVRBMDNRdHFLT3BoYVR3Mk4wbU1vUUlQd19DYUJnf3ZKcE03Yn91QEJUZjEwdj00PXI2cn9CVVZBNTVhZ3BjTEFcSScpJ2N3amhWYHdzYHcnP3F3cGApJ2dkZm5id2pwa2FGamlqdyc%2FJyZjMGMwYzAnKSdpZHxqcHFRfHVgJz8ndmxrYmlgWmxxYGgnKSdga2RnaWBVaWRmYG1qaWFgd3YnP3F3cGB4JSUl', 'succeeded', 1750.00, 'MXN', '2026-03-26 22:14:53', '2026-03-26 22:15:14'),
	(11, 11, 'stripe', 'cs_test_a1ilH6cdCXMXvpdQQJB6cC7FvR9kYa2wh9rPmccDlreeTTxErdKunbXOk4', 'pi_3Ta0wfDAViW7oZaY1QkdzahM', '{\n  "id": "cs_test_a1ilH6cdCXMXvpdQQJB6cC7FvR9kYa2wh9rPmccDlreeTTxErdKunbXOk4",\n  "object": "checkout.session",\n  "adaptive_pricing": {\n    "enabled": true\n  },\n  "after_expiration": null,\n  "allow_promotion_codes": null,\n  "amount_subtotal": 180000,\n  "amount_total": 180000,\n  "automatic_tax": {\n    "enabled": false,\n    "liability": null,\n    "provider": null,\n    "status": null\n  },\n  "billing_address_collection": null,\n  "branding_settings": {\n    "background_color": "#f5f5f5",\n    "border_style": "rounded",\n    "button_color": "#ff8500",\n    "display_name": "Grow Travel",\n    "font_family": "default",\n    "icon": {\n      "file": "file_1TAyYCDAViW7oZaY1mUzIIZ9",\n      "type": "file"\n    },\n    "logo": {\n      "file": "file_1TAyYCDAViW7oZaYjAeuIz2Y",\n      "type": "file"\n    }\n  },\n  "cancel_url": "https://demo.travel.growventa.com/portal/43c9be04-bb83-49fd-aa59-2a888310dbad/reservacion/05d31586-f380-4041-8851-e12f18ec2e7e/pago/a7b51a29-69fc-4d2a-b087-c51eae15ccff/cancelado",\n  "client_reference_id": "11",\n  "client_secret": null,\n  "collected_information": null,\n  "consent": null,\n  "consent_collection": null,\n  "created": 1779486548,\n  "currency": "mxn",\n  "currency_conversion": null,\n  "custom_fields": [],\n  "custom_text": {\n    "after_submit": null,\n    "shipping_address": null,\n    "submit": null,\n    "terms_of_service_acceptance": null\n  },\n  "customer": null,\n  "customer_account": null,\n  "customer_creation": "if_required",\n  "customer_details": null,\n  "customer_email": null,\n  "discounts": [],\n  "expires_at": 1779572948,\n  "integration_identifier": null,\n  "invoice": null,\n  "invoice_creation": {\n    "enabled": false,\n    "invoice_data": {\n      "account_tax_ids": null,\n      "custom_fields": null,\n      "description": null,\n      "footer": null,\n      "issuer": null,\n      "metadata": {},\n      "rendering_options": null\n    }\n  },\n  "livemode": false,\n  "locale": null,\n  "managed_payments": {\n    "enabled": false\n  },\n  "metadata": {\n    "attempt_id": "11",\n    "request_token": "a7b51a29-69fc-4d2a-b087-c51eae15ccff"\n  },\n  "mode": "payment",\n  "origin_context": null,\n  "payment_intent": null,\n  "payment_link": null,\n  "payment_method_collection": "if_required",\n  "payment_method_configuration_details": null,\n  "payment_method_options": {\n    "card": {\n      "request_three_d_secure": "automatic"\n    }\n  },\n  "payment_method_types": [\n    "card"\n  ],\n  "payment_status": "unpaid",\n  "permissions": null,\n  "phone_number_collection": {\n    "enabled": false\n  },\n  "recovered_from": null,\n  "saved_payment_method_options": null,\n  "setup_intent": null,\n  "shipping_address_collection": null,\n  "shipping_cost": null,\n  "shipping_options": [],\n  "status": "open",\n  "submit_type": null,\n  "subscription": null,\n  "success_url": "https://demo.travel.growventa.com/portal/43c9be04-bb83-49fd-aa59-2a888310dbad/reservacion/05d31586-f380-4041-8851-e12f18ec2e7e/pago/a7b51a29-69fc-4d2a-b087-c51eae15ccff/exito?session_id={CHECKOUT_SESSION_ID}",\n  "total_details": {\n    "amount_discount": 0,\n    "amount_shipping": 0,\n    "amount_tax": 0\n  },\n  "ui_mode": "hosted",\n  "url": "https://checkout.stripe.com/c/pay/cs_test_a1ilH6cdCXMXvpdQQJB6cC7FvR9kYa2wh9rPmccDlreeTTxErdKunbXOk4#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdicGRmZGhqaWBTZHdsZGtxJz8nZmprcXdqaScpJ2R1bE5gfCc%2FJ3VuWnFgdnFaMDRRRHxcMEFEU2xSMmpfZFw9f1B0bTNEc042UV1fc2FdVEEwM1F0cUtPcGhhVHcyTjBtTW9RSVB3X0NhQmd%2FdkpwTTdif3VAQlRmMTB2PTQ9cjZyf0JVVkE1NWFncGNMQVxJJyknY3dqaFZgd3Ngdyc%2FcXdwYCknZ2RmbmJ3anBrYUZqaWp3Jz8nJmMwYzBjMCcpJ2lkfGpwcVF8dWAnPyd2bGtiaWBabHFgaCcpJ2BrZGdpYFVpZGZgbWppYWB3dic%2FcXdwYHgl",\n  "wallet_options": null\n}', 'https://checkout.stripe.com/c/pay/cs_test_a1ilH6cdCXMXvpdQQJB6cC7FvR9kYa2wh9rPmccDlreeTTxErdKunbXOk4#fidnandhYHdWcXxpYCc%2FJ2FgY2RwaXEnKSdicGRmZGhqaWBTZHdsZGtxJz8nZmprcXdqaScpJ2R1bE5gfCc%2FJ3VuWnFgdnFaMDRRRHxcMEFEU2xSMmpfZFw9f1B0bTNEc042UV1fc2FdVEEwM1F0cUtPcGhhVHcyTjBtTW9RSVB3X0NhQmd%2FdkpwTTdif3VAQlRmMTB2PTQ9cjZyf0JVVkE1NWFncGNMQVxJJyknY3dqaFZgd3Ngdyc%2FcXdwYCknZ2RmbmJ3anBrYUZqaWp3Jz8nJmMwYzBjMCcpJ2lkfGpwcVF8dWAnPyd2bGtiaWBabHFgaCcpJ2BrZGdpYFVpZGZgbWppYWB3dic%2FcXdwYHgl', 'succeeded', 1800.00, 'MXN', '2026-05-23 01:49:07', '2026-05-23 01:49:31'),
	(12, 12, 'paypal', '27663041RK131015X', NULL, '{"id":"27663041RK131015X","status":"CREATED","links":[{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/27663041RK131015X","rel":"self","method":"GET"},{"href":"https://www.sandbox.paypal.com/checkoutnow?token=27663041RK131015X","rel":"approve","method":"GET"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/27663041RK131015X","rel":"update","method":"PATCH"},{"href":"https://api.sandbox.paypal.com/v2/checkout/orders/27663041RK131015X/capture","rel":"capture","method":"POST"}]}', 'https://www.sandbox.paypal.com/checkoutnow?token=27663041RK131015X', 'processing', 100.00, 'USD', '2026-08-25 19:54:36', '2026-08-25 19:54:37');

-- Volcando estructura para tabla growvent_staltos.payment_installments
CREATE TABLE IF NOT EXISTS `payment_installments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned NOT NULL,
  `user_id` bigint unsigned DEFAULT NULL,
  `amount` decimal(15,2) NOT NULL,
  `currency` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `due_date` date NOT NULL,
  `status` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'pending',
  `payment_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payment_installments_lead_id_foreign` (`lead_id`),
  KEY `payment_installments_user_id_foreign` (`user_id`),
  KEY `payment_installments_payment_id_foreign` (`payment_id`),
  CONSTRAINT `payment_installments_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `payment_installments_payment_id_foreign` FOREIGN KEY (`payment_id`) REFERENCES `payments` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_installments_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=35 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payment_installments: ~33 rows (aproximadamente)
INSERT INTO `payment_installments` (`id`, `lead_id`, `user_id`, `amount`, `currency`, `due_date`, `status`, `payment_id`, `created_at`, `updated_at`) VALUES
	(1, 17, 10, 141.60, 'USD', '2026-03-23', 'paid', 10, '2026-03-16 23:50:46', '2026-03-16 23:58:07'),
	(2, 17, 10, 141.60, 'USD', '2026-03-30', 'paid', 11, '2026-03-16 23:50:46', '2026-03-17 00:01:16'),
	(3, 17, 10, 141.60, 'USD', '2026-04-06', 'paid', 15, '2026-03-16 23:50:46', '2026-03-18 22:54:57'),
	(4, 17, 10, 141.60, 'USD', '2026-04-13', 'pending', NULL, '2026-03-16 23:50:46', '2026-03-16 23:50:46'),
	(5, 17, 10, 141.60, 'USD', '2026-04-16', 'pending', NULL, '2026-03-16 23:50:46', '2026-03-16 23:50:46'),
	(6, 18, 10, 1028.57, 'MXN', '2026-03-24', 'paid', 13, '2026-03-17 22:10:11', '2026-03-17 22:12:04'),
	(7, 18, 10, 1028.57, 'MXN', '2026-03-31', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(8, 18, 10, 1028.57, 'MXN', '2026-04-07', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(9, 18, 10, 1028.57, 'MXN', '2026-04-14', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(10, 18, 10, 1028.57, 'MXN', '2026-04-21', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(11, 18, 10, 1028.57, 'MXN', '2026-04-28', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(12, 18, 10, 1028.58, 'MXN', '2026-04-30', 'pending', NULL, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(14, 14, 10, 1104.17, 'MXN', '2026-04-02', 'paid', 14, '2026-03-18 22:52:20', '2026-03-18 22:53:30'),
	(15, 14, 10, 1104.17, 'MXN', '2026-04-17', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(16, 14, 10, 1104.17, 'MXN', '2026-05-02', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(17, 14, 10, 1104.17, 'MXN', '2026-05-17', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(18, 14, 10, 1104.17, 'MXN', '2026-06-01', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(19, 14, 10, 1104.17, 'MXN', '2026-06-16', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(20, 14, 10, 1104.17, 'MXN', '2026-07-01', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(21, 14, 10, 1104.17, 'MXN', '2026-07-16', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(22, 14, 10, 1104.17, 'MXN', '2026-07-31', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(23, 14, 10, 1104.17, 'MXN', '2026-08-15', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(24, 14, 10, 1104.17, 'MXN', '2026-08-30', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(25, 14, 10, 1104.13, 'MXN', '2026-08-31', 'pending', NULL, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(26, 12, 10, 1750.00, 'MXN', '2026-04-02', 'paid', 17, '2026-03-26 22:10:39', '2026-03-26 22:15:14'),
	(27, 12, 10, 1750.00, 'MXN', '2026-04-09', 'pending', NULL, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(28, 12, 10, 1750.00, 'MXN', '2026-04-16', 'pending', NULL, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(29, 12, 10, 1750.00, 'MXN', '2026-04-23', 'pending', NULL, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(30, 12, 10, 1750.00, 'MXN', '2026-04-30', 'pending', NULL, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(31, 27, 10, 8000.00, 'MXN', '2026-05-20', 'pending', NULL, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(32, 27, 10, 8000.00, 'MXN', '2026-06-19', 'pending', NULL, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(33, 27, 10, 8000.00, 'MXN', '2026-07-19', 'pending', NULL, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(34, 27, 10, 8000.00, 'MXN', '2026-08-01', 'pending', NULL, '2026-04-20 18:55:55', '2026-04-20 18:55:55');

-- Volcando estructura para tabla growvent_staltos.payment_requests
CREATE TABLE IF NOT EXISTS `payment_requests` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `public_token` char(36) COLLATE utf8mb4_unicode_ci NOT NULL,
  `lead_id` bigint unsigned NOT NULL,
  `customer_id` bigint unsigned DEFAULT NULL,
  `payment_installment_id` bigint unsigned DEFAULT NULL,
  `amount` decimal(15,2) NOT NULL,
  `currency` varchar(10) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `provider` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  `origin` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'portal',
  `status` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'pending',
  `expires_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `payment_requests_public_token_unique` (`public_token`),
  KEY `payment_requests_lead_id_foreign` (`lead_id`),
  KEY `payment_requests_customer_id_foreign` (`customer_id`),
  KEY `payment_requests_payment_installment_id_foreign` (`payment_installment_id`),
  CONSTRAINT `payment_requests_customer_id_foreign` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_requests_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `payment_requests_payment_installment_id_foreign` FOREIGN KEY (`payment_installment_id`) REFERENCES `payment_installments` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payment_requests: ~12 rows (aproximadamente)
INSERT INTO `payment_requests` (`id`, `public_token`, `lead_id`, `customer_id`, `payment_installment_id`, `amount`, `currency`, `provider`, `origin`, `status`, `expires_at`, `created_at`, `updated_at`) VALUES
	(1, '7a6cb93b-9ab4-4451-8a7d-a6ceaf68cefa', 17, 2, 1, 141.60, 'USD', 'stripe', 'portal', 'completed', '2026-03-17 23:57:45', '2026-03-16 23:57:45', '2026-03-16 23:58:07'),
	(2, 'efee7134-b171-4524-97ce-d9e89057e1f5', 17, 2, 2, 141.60, 'USD', 'paypal', 'portal', 'completed', '2026-03-17 23:59:39', '2026-03-16 23:59:39', '2026-03-17 00:01:16'),
	(3, '869b3cdf-3d83-42db-94d4-c31cf8d2dabc', 13, 5, NULL, 6500.00, 'MXN', 'stripe', 'portal', 'completed', '2026-03-18 00:09:51', '2026-03-17 00:09:51', '2026-03-17 00:10:11'),
	(4, '5f52a091-b3ed-4ab4-96a0-8df89bf46df5', 18, 2, 6, 1028.57, 'MXN', 'stripe', 'portal', 'completed', '2026-03-18 22:11:20', '2026-03-17 22:11:20', '2026-03-17 22:12:04'),
	(5, 'befd488f-166a-4ac3-ac6e-6af143436faf', 18, 2, 7, 1028.57, 'MXN', 'paypal', 'portal', 'processing', '2026-03-18 22:13:18', '2026-03-17 22:13:18', '2026-03-17 22:13:19'),
	(6, 'a8011bb2-4ef3-4d76-8c0d-a1e8843b22e7', 14, 8, 14, 1104.17, 'MXN', 'stripe', 'portal', 'completed', '2026-03-19 22:53:10', '2026-03-18 22:53:10', '2026-03-18 22:53:30'),
	(7, '5b50f9b5-d2ca-4531-acc5-3ffa51a05ad3', 17, 2, 3, 141.60, 'USD', 'stripe', 'portal', 'completed', '2026-03-19 22:54:32', '2026-03-18 22:54:32', '2026-03-18 22:54:57'),
	(8, '3a4e2051-66e2-44e1-91ab-c98479d4fed4', 17, 2, 4, 141.60, 'USD', 'stripe', 'portal', 'processing', '2026-03-22 23:50:21', '2026-03-21 23:50:21', '2026-03-21 23:50:22'),
	(9, 'b45d992d-c78a-4c7b-982c-0ac60705a697', 17, 2, 4, 141.60, 'USD', 'paypal', 'portal', 'processing', '2026-03-23 00:13:47', '2026-03-22 00:13:47', '2026-03-22 00:13:48'),
	(10, '53d8e6b1-84fa-486e-bab6-17ce59b5b7cd', 12, 6, 26, 1750.00, 'MXN', 'stripe', 'portal', 'completed', '2026-03-27 22:14:53', '2026-03-26 22:14:53', '2026-03-26 22:15:14'),
	(11, 'a7b51a29-69fc-4d2a-b087-c51eae15ccff', 29, 8, NULL, 1800.00, 'MXN', 'stripe', 'portal', 'completed', '2026-05-24 01:49:07', '2026-05-23 01:49:07', '2026-05-23 01:49:31'),
	(12, '6ac06231-b4ee-49d9-987d-e22d33c14203', 31, 13, NULL, 100.00, 'USD', 'paypal', 'portal', 'processing', '2026-08-26 19:54:36', '2026-08-25 19:54:36', '2026-08-25 19:54:37');

-- Volcando estructura para tabla growvent_staltos.payment_settings
CREATE TABLE IF NOT EXISTS `payment_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Ingreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payment_settings_group_id_foreign` (`group_id`),
  KEY `payment_settings_classification_id_foreign` (`classification_id`),
  KEY `payment_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `payment_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `payment_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payment_settings: ~1 rows (aproximadamente)
INSERT INTO `payment_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 9, 8, 4, 1, 'Pago de Cliente', 'Ingreso', 'Pago de clientes', 1, '2025-12-16 21:19:19', '2025-12-16 21:19:57');

-- Volcando estructura para tabla growvent_staltos.payment_supplier_settings
CREATE TABLE IF NOT EXISTS `payment_supplier_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Egreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payment_supplier_settings_group_id_foreign` (`group_id`),
  KEY `payment_supplier_settings_classification_id_foreign` (`classification_id`),
  KEY `payment_supplier_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `payment_supplier_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `payment_supplier_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_supplier_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_supplier_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payment_supplier_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payment_supplier_settings: ~1 rows (aproximadamente)
INSERT INTO `payment_supplier_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 23, 10, 4, 1, 'Pago a Proveedor', 'Egreso', 'Pago a proveedores configuración de automatización', 1, '2025-12-16 21:19:52', '2025-12-16 21:19:54');

-- Volcando estructura para tabla growvent_staltos.payroll_payments
CREATE TABLE IF NOT EXISTS `payroll_payments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `week_start` date NOT NULL,
  `week_end` date NOT NULL,
  `amount` decimal(15,2) NOT NULL DEFAULT '0.00',
  `commission` decimal(15,2) NOT NULL DEFAULT '0.00',
  `benefit` decimal(15,2) NOT NULL DEFAULT '0.00',
  `bonus` decimal(15,2) NOT NULL DEFAULT '0.00',
  `paid_at` date DEFAULT NULL,
  `payment_date` date NOT NULL COMMENT 'Fecha en que se registró el pago',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payroll_payments_user_id_foreign` (`user_id`),
  KEY `payroll_payments_period_id_foreign` (`period_id`),
  CONSTRAINT `payroll_payments_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payroll_payments_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payroll_payments: ~18 rows (aproximadamente)
INSERT INTO `payroll_payments` (`id`, `user_id`, `period_id`, `week_start`, `week_end`, `amount`, `commission`, `benefit`, `bonus`, `paid_at`, `payment_date`, `created_at`, `updated_at`) VALUES
	(1, 10, NULL, '2025-12-01', '2025-12-07', 1500.00, 500.00, 200.00, 50.00, NULL, '2025-12-15', '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(2, 11, NULL, '2025-12-01', '2025-12-07', 1000.00, 750.00, 200.00, 100.00, NULL, '2025-12-15', '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(3, 12, NULL, '2025-12-01', '2025-12-07', 1150.00, 450.00, 200.00, 100.00, NULL, '2025-12-15', '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(4, 10, NULL, '2026-01-01', '2026-01-15', 1500.00, 0.00, 0.00, 0.00, NULL, '2026-01-15', '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(5, 11, NULL, '2026-01-01', '2026-01-15', 1000.00, 500.00, 0.00, 0.00, NULL, '2026-01-15', '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(6, 12, NULL, '2026-01-01', '2026-01-15', 1250.00, 0.00, 250.00, 0.00, NULL, '2026-01-15', '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(7, 10, NULL, '2026-01-01', '2026-01-15', 1500.00, 0.00, 0.00, 0.00, NULL, '2026-01-17', '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(8, 11, NULL, '2026-01-01', '2026-01-15', 1000.00, 850.00, 0.00, 0.00, NULL, '2026-01-17', '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(9, 12, NULL, '2026-01-01', '2026-01-15', 1250.00, 0.00, 0.00, 300.00, NULL, '2026-01-17', '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(10, 10, NULL, '2026-03-16', '2026-03-22', 1500.00, 0.00, 0.00, 0.00, NULL, '2026-03-21', '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(11, 11, NULL, '2026-03-16', '2026-03-22', 1000.00, 0.00, 0.00, 0.00, NULL, '2026-03-21', '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(12, 12, NULL, '2026-03-16', '2026-03-22', 1250.00, 0.00, 0.00, 0.00, NULL, '2026-03-21', '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(13, 10, NULL, '2026-03-16', '2026-03-22', 1500.00, 0.00, 0.00, 500.00, NULL, '2026-03-21', '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(14, 12, NULL, '2026-03-16', '2026-03-22', 1250.00, 1000.00, 0.00, 0.00, NULL, '2026-03-21', '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(15, 10, NULL, '2026-04-13', '2026-04-19', 1500.00, 0.00, 0.00, 0.00, NULL, '2026-04-18', '2026-04-20 23:39:05', '2026-04-20 23:39:05'),
	(16, 12, NULL, '2026-04-13', '2026-04-19', 1250.00, 0.00, 0.00, 0.00, NULL, '2026-04-18', '2026-04-20 23:39:05', '2026-04-20 23:39:05'),
	(17, 10, NULL, '2026-08-17', '2026-08-23', 1500.00, 500.00, 0.00, 300.00, NULL, '2026-08-22', '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(18, 12, NULL, '2026-08-17', '2026-08-23', 1250.00, 0.00, 240.00, 0.00, NULL, '2026-08-22', '2026-08-25 20:02:03', '2026-08-25 20:02:03');

-- Volcando estructura para tabla growvent_staltos.payroll_transaction_settings
CREATE TABLE IF NOT EXISTS `payroll_transaction_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `concept` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Concepto a usar en la transacción',
  `transaction_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Egreso' COMMENT 'Tipo de transacción (Egreso, Ingreso)',
  `description` text COLLATE utf8mb4_unicode_ci COMMENT 'Descripción del concepto',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Si esta configuración está activa',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `payroll_transaction_settings_group_id_foreign` (`group_id`),
  KEY `payroll_transaction_settings_classification_id_foreign` (`classification_id`),
  KEY `payroll_transaction_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `payroll_transaction_settings_period_id_foreign` (`period_id`),
  CONSTRAINT `payroll_transaction_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payroll_transaction_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payroll_transaction_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `payroll_transaction_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.payroll_transaction_settings: ~1 rows (aproximadamente)
INSERT INTO `payroll_transaction_settings` (`id`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `concept`, `transaction_type`, `description`, `is_active`, `created_at`, `updated_at`) VALUES
	(1, 8, 3, 4, 2, 'Pago de nómina', 'Egreso', 'Configuración para registro de nóminas semanales', 1, '2025-12-16 21:17:24', '2025-12-16 21:17:27');

-- Volcando estructura para tabla growvent_staltos.periods
CREATE TABLE IF NOT EXISTS `periods` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.periods: ~5 rows (aproximadamente)
INSERT INTO `periods` (`id`, `name`, `created_at`, `updated_at`) VALUES
	(1, 'Ocasional', '2025-12-16 21:06:39', '2025-12-16 21:06:39'),
	(2, 'Semanal', '2025-12-16 21:06:39', '2025-12-16 21:06:39'),
	(3, 'Mensual', '2025-12-16 21:06:39', '2025-12-16 21:06:39'),
	(4, 'Bimestral', '2025-12-16 21:06:39', '2025-12-16 21:06:39'),
	(5, 'Anual', '2025-12-16 21:06:39', '2025-12-16 21:06:39');

-- Volcando estructura para tabla growvent_staltos.personal_access_tokens
CREATE TABLE IF NOT EXISTS `personal_access_tokens` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `tokenable_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `tokenable_id` bigint unsigned NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `token` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL,
  `abilities` text COLLATE utf8mb4_unicode_ci,
  `last_used_at` timestamp NULL DEFAULT NULL,
  `expires_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `personal_access_tokens_token_unique` (`token`),
  KEY `personal_access_tokens_tokenable_type_tokenable_id_index` (`tokenable_type`,`tokenable_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.personal_access_tokens: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.pos_custom_field_definitions
CREATE TABLE IF NOT EXISTS `pos_custom_field_definitions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `code` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `field_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `options` json DEFAULT NULL,
  `is_required` tinyint(1) NOT NULL DEFAULT '0',
  `is_active` tinyint(1) NOT NULL DEFAULT '1',
  `target_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `sort_order` int NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `pos_custom_field_definitions_code_unique` (`code`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.pos_custom_field_definitions: ~2 rows (aproximadamente)
INSERT INTO `pos_custom_field_definitions` (`id`, `name`, `code`, `field_type`, `options`, `is_required`, `is_active`, `target_type`, `sort_order`, `created_at`, `updated_at`) VALUES
	(4, 'HOTEL', 'hotel', 'text', NULL, 1, 1, 'lead', 0, '2026-09-03 03:19:31', '2026-09-03 03:19:31'),
	(5, 'Excursion', 'Exc', 'text', NULL, 0, 1, 'lead', 1, '2026-09-03 03:19:52', '2026-09-03 03:19:52'),
	(6, 'Adultos', 'Ad', 'number', NULL, 1, 1, 'lead_product', 0, '2026-09-03 03:20:37', '2026-09-03 03:20:37');

-- Volcando estructura para tabla growvent_staltos.pos_custom_field_values
CREATE TABLE IF NOT EXISTS `pos_custom_field_values` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `definition_id` bigint unsigned NOT NULL,
  `lead_id` bigint unsigned DEFAULT NULL,
  `lead_product_id` bigint unsigned DEFAULT NULL,
  `value` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `pos_custom_field_values_definition_id_foreign` (`definition_id`),
  KEY `pos_custom_field_values_lead_product_id_foreign` (`lead_product_id`),
  KEY `pos_custom_field_values_lead_id_lead_product_id_index` (`lead_id`,`lead_product_id`),
  CONSTRAINT `pos_custom_field_values_definition_id_foreign` FOREIGN KEY (`definition_id`) REFERENCES `pos_custom_field_definitions` (`id`) ON DELETE CASCADE,
  CONSTRAINT `pos_custom_field_values_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `pos_custom_field_values_lead_product_id_foreign` FOREIGN KEY (`lead_product_id`) REFERENCES `lead_products` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.pos_custom_field_values: ~2 rows (aproximadamente)
INSERT INTO `pos_custom_field_values` (`id`, `definition_id`, `lead_id`, `lead_product_id`, `value`, `created_at`, `updated_at`) VALUES
	(7, 4, 35, NULL, 'Prueba', '2026-09-03 03:26:23', '2026-09-03 03:26:23'),
	(8, 5, 35, NULL, 'TEST', '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.preleads
CREATE TABLE IF NOT EXISTS `preleads` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `customer_id` bigint unsigned DEFAULT NULL,
  `user_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `preleads_customer_id_foreign` (`customer_id`),
  KEY `preleads_user_id_foreign` (`user_id`),
  CONSTRAINT `preleads_customer_id_foreign` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`id`) ON DELETE SET NULL,
  CONSTRAINT `preleads_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.preleads: ~6 rows (aproximadamente)
INSERT INTO `preleads` (`id`, `name`, `description`, `customer_id`, `user_id`, `created_at`, `updated_at`) VALUES
	(1, 'MARCO ANTONIO', 'Buscaba cotización Cancun', 4, 10, '2025-12-16 22:32:18', '2025-12-16 22:32:18'),
	(2, 'PEDRO', 'Viaje a Chihuahua', 5, 11, '2025-12-16 22:33:02', '2025-12-16 22:33:02'),
	(3, 'LUPITA', 'Luna de Miel', 6, 12, '2025-12-16 22:38:14', '2025-12-16 22:38:14'),
	(4, 'Eleazar', 'Paquete ida y vuelta a las colaradas', 7, 10, '2026-01-29 22:44:03', '2026-01-29 22:44:03'),
	(5, 'Pedro Rodriguez', 'Paquete a Vallarta', 10, 10, '2026-03-26 21:49:30', '2026-03-26 21:49:30'),
	(6, 'prueba', 'AGENCY', 9, 10, '2026-05-03 04:30:21', '2026-05-03 04:55:10');

-- Volcando estructura para tabla growvent_staltos.profit_distributions
CREATE TABLE IF NOT EXISTS `profit_distributions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `total_profit` decimal(12,2) NOT NULL DEFAULT '0.00',
  `total_dividend` decimal(12,2) NOT NULL DEFAULT '0.00',
  `remaining_profit` decimal(12,2) NOT NULL DEFAULT '0.00',
  `distribution_date` date NOT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `profit_distributions_distribution_date_index` (`distribution_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.profit_distributions: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.reminders
CREATE TABLE IF NOT EXISTS `reminders` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned DEFAULT NULL,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `date` date NOT NULL,
  `finished` tinyint(1) NOT NULL DEFAULT '0',
  `lead_id` bigint unsigned DEFAULT NULL,
  `lead_group_id` bigint unsigned DEFAULT NULL,
  `payment_installment_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `reminders_user_id_foreign` (`user_id`),
  KEY `reminders_lead_id_foreign` (`lead_id`),
  KEY `reminders_lead_group_id_foreign` (`lead_group_id`),
  KEY `reminders_payment_installment_id_foreign` (`payment_installment_id`),
  CONSTRAINT `reminders_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `reminders_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE,
  CONSTRAINT `reminders_payment_installment_id_foreign` FOREIGN KEY (`payment_installment_id`) REFERENCES `payment_installments` (`id`) ON DELETE CASCADE,
  CONSTRAINT `reminders_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=76 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.reminders: ~64 rows (aproximadamente)
INSERT INTO `reminders` (`id`, `user_id`, `description`, `date`, `finished`, `lead_id`, `lead_group_id`, `payment_installment_id`, `created_at`, `updated_at`) VALUES
	(2, 10, 'Fecha límite para que se líquide el grupo', '2026-09-30', 0, NULL, 2, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(3, 10, 'Fecha límite para que el cliente liquide la orden', '2026-09-30', 0, 2, NULL, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(4, 10, 'Fecha límite para que el cliente liquide la orden', '2026-09-30', 0, 3, NULL, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(5, 10, 'Fecha límite para que el cliente liquide la orden', '2026-09-30', 0, 4, NULL, NULL, '2025-12-16 21:27:13', '2025-12-16 21:27:13'),
	(6, 11, 'Fecha límite para que el cliente liquide la orden', '2026-01-31', 0, 5, NULL, NULL, '2025-12-16 22:34:27', '2025-12-16 22:34:27'),
	(7, 12, 'Fecha límite para que el cliente liquide la orden', '2026-01-15', 0, 6, NULL, NULL, '2025-12-16 22:39:05', '2025-12-16 22:39:05'),
	(8, 12, 'Fecha límite para que se líquide el grupo', '2026-01-31', 0, NULL, 3, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(10, 12, 'Fecha límite para que el cliente liquide la orden', '2026-01-31', 0, 8, NULL, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(11, 12, 'Fecha límite para que el cliente liquide la orden', '2026-01-31', 0, 9, NULL, NULL, '2025-12-16 22:42:03', '2025-12-16 22:42:03'),
	(13, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 10, NULL, NULL, '2026-01-29 22:47:20', '2026-01-29 22:47:20'),
	(15, 10, 'Fecha límite para que se líquide el grupo', '2026-02-28', 0, NULL, 4, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(16, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 11, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(17, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 12, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(18, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 13, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(19, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 14, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(20, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 15, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(21, 10, 'Fecha límite para que el cliente liquide la orden', '2026-02-28', 0, 16, NULL, NULL, '2026-01-29 23:05:26', '2026-01-29 23:05:26'),
	(22, 10, 'Fecha límite para que el cliente liquide la orden', '2026-04-16', 0, 17, NULL, NULL, '2026-01-29 23:35:59', '2026-01-29 23:35:59'),
	(23, 10, 'Mensaje de buenos días al cliente cuando salga su vuelo', '2026-02-01', 0, 7, NULL, NULL, '2026-01-30 21:34:29', '2026-01-30 21:34:29'),
	(27, 10, 'Parcialidad de pago: $141.60 USD', '2026-04-13', 0, 17, NULL, 4, '2026-03-16 23:50:46', '2026-03-16 23:50:46'),
	(28, 10, 'Parcialidad de pago: $141.60 USD', '2026-04-16', 0, 17, NULL, 5, '2026-03-16 23:50:46', '2026-03-16 23:50:46'),
	(29, 10, 'Fecha límite para que el cliente liquide la orden', '2026-03-31', 0, 18, NULL, NULL, '2026-03-17 00:18:50', '2026-03-17 00:18:50'),
	(31, 10, 'Parcialidad de pago: $1,028.57 MXN', '2026-03-31', 0, 18, NULL, 7, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(32, 10, 'Parcialidad de pago: $1,028.57 MXN', '2026-04-07', 0, 18, NULL, 8, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(33, 10, 'Parcialidad de pago: $1,028.57 MXN', '2026-04-14', 0, 18, NULL, 9, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(34, 10, 'Parcialidad de pago: $1,028.57 MXN', '2026-04-21', 0, 18, NULL, 10, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(35, 10, 'Parcialidad de pago: $1,028.57 MXN', '2026-04-28', 0, 18, NULL, 11, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(36, 10, 'Parcialidad de pago: $1,028.58 MXN', '2026-04-30', 0, 18, NULL, 12, '2026-03-17 22:10:11', '2026-03-17 22:10:11'),
	(39, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-04-17', 0, 14, NULL, 15, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(40, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-05-02', 0, 14, NULL, 16, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(41, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-05-17', 0, 14, NULL, 17, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(42, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-06-01', 0, 14, NULL, 18, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(43, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-06-16', 0, 14, NULL, 19, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(44, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-07-01', 0, 14, NULL, 20, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(45, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-07-16', 0, 14, NULL, 21, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(46, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-07-31', 0, 14, NULL, 22, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(47, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-08-15', 0, 14, NULL, 23, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(48, 10, 'Parcialidad de pago: $1,104.17 MXN', '2026-08-30', 0, 14, NULL, 24, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(49, 10, 'Parcialidad de pago: $1,104.13 MXN', '2026-08-31', 0, 14, NULL, 25, '2026-03-18 22:52:20', '2026-03-18 22:52:20'),
	(50, 10, 'Fecha límite para que el cliente liquide la orden', '2026-04-30', 0, 19, NULL, NULL, '2026-03-26 21:56:00', '2026-03-26 21:56:00'),
	(51, 10, 'Le voy a mandar la confirmación del vuelo', '2026-04-01', 0, 18, NULL, NULL, '2026-03-26 22:04:05', '2026-03-26 22:04:05'),
	(53, 10, 'Parcialidad de pago: $1,750.00 MXN', '2026-04-09', 0, 12, NULL, 27, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(54, 10, 'Parcialidad de pago: $1,750.00 MXN', '2026-04-16', 0, 12, NULL, 28, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(55, 10, 'Parcialidad de pago: $1,750.00 MXN', '2026-04-23', 0, 12, NULL, 29, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(56, 10, 'Parcialidad de pago: $1,750.00 MXN', '2026-04-30', 0, 12, NULL, 30, '2026-03-26 22:10:39', '2026-03-26 22:10:39'),
	(57, 10, 'Facturar PPD $1750', '2026-03-31', 0, 12, NULL, NULL, '2026-03-26 22:19:19', '2026-03-26 22:19:19'),
	(58, 10, 'Fecha límite para que se líquide el grupo', '2026-05-15', 0, NULL, 5, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(59, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 20, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(60, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 21, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(61, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 22, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(62, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 23, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(63, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 24, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(64, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 25, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(65, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 26, NULL, NULL, '2026-03-26 22:29:36', '2026-03-26 22:29:36'),
	(66, 10, 'Fecha límite para que el cliente liquide la orden', '2026-08-01', 0, 27, NULL, NULL, '2026-04-20 18:54:29', '2026-04-20 18:54:29'),
	(67, 10, 'Parcialidad de pago: $8,000.00 MXN', '2026-05-20', 0, 27, NULL, 31, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(68, 10, 'Parcialidad de pago: $8,000.00 MXN', '2026-06-19', 0, 27, NULL, 32, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(69, 10, 'Parcialidad de pago: $8,000.00 MXN', '2026-07-19', 0, 27, NULL, 33, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(70, 10, 'Parcialidad de pago: $8,000.00 MXN', '2026-08-01', 0, 27, NULL, 34, '2026-04-20 18:55:55', '2026-04-20 18:55:55'),
	(71, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-05', 0, 28, NULL, NULL, '2026-05-03 04:32:39', '2026-05-03 04:32:39'),
	(72, 10, 'Llamar cliente', '2026-05-10', 0, 24, NULL, NULL, '2026-05-09 06:54:01', '2026-05-09 06:54:01'),
	(73, 10, 'Fecha límite para que el cliente liquide la orden', '2026-05-15', 0, 29, NULL, NULL, '2026-05-14 20:25:58', '2026-05-14 20:25:58'),
	(74, 10, 'Fecha límite para que el cliente liquide la orden', '2026-08-24', 0, 30, NULL, NULL, '2026-08-25 00:17:42', '2026-08-25 00:17:42'),
	(75, 10, 'Fecha límite para que el cliente liquide la orden', '2026-08-31', 0, 31, NULL, NULL, '2026-08-25 19:54:10', '2026-08-25 19:54:10');

-- Volcando estructura para procedimiento growvent_staltos.reset_all_autoincrement
DELIMITER //
CREATE PROCEDURE `reset_all_autoincrement`()
BEGIN
 DECLARE done INT DEFAULT FALSE;
  DECLARE tbl_name VARCHAR(255);
  DECLARE max_id INT;
  DECLARE cur CURSOR FOR
    SELECT table_name
    FROM information_schema.columns
    WHERE table_schema = DATABASE()
      AND column_name = 'id';
  DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

  OPEN cur;
  read_loop: LOOP
    FETCH cur INTO tbl_name;
    IF done THEN
      LEAVE read_loop;
    END IF;

    SET @sql = CONCAT('SELECT MAX(id) FROM `', tbl_name, '`');
    PREPARE stmt FROM @sql;
    EXECUTE stmt;
    DEALLOCATE PREPARE stmt;
    -- Aquí deberías guardar el resultado en una variable, pero MySQL no lo permite directamente en procedimientos.
    -- Por eso, lo más seguro es generar el comando como texto:
    SET @alter = CONCAT('ALTER TABLE `', tbl_name, '` AUTO_INCREMENT = (SELECT IFNULL(MAX(id),0)+1 FROM `', tbl_name, '`);');
    SELECT @alter;
  END LOOP;
  CLOSE cur;
END//
DELIMITER ;

-- Volcando estructura para tabla growvent_staltos.retained_profit_settings
CREATE TABLE IF NOT EXISTS `retained_profit_settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `description_template` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'Utilidad retenida - Distribución #{distribution_id}',
  `is_active` tinyint(1) NOT NULL DEFAULT '1',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `retained_profit_settings_group_id_foreign` (`group_id`),
  KEY `retained_profit_settings_classification_id_foreign` (`classification_id`),
  KEY `retained_profit_settings_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `retained_profit_settings_period_id_foreign` (`period_id`),
  KEY `retained_profit_settings_is_active_index` (`is_active`),
  CONSTRAINT `retained_profit_settings_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `retained_profit_settings_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `retained_profit_settings_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `retained_profit_settings_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.retained_profit_settings: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.roles
CREATE TABLE IF NOT EXISTS `roles` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `hidden` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.roles: ~6 rows (aproximadamente)
INSERT INTO `roles` (`id`, `name`, `hidden`, `created_at`, `updated_at`) VALUES
	(1, 'admin', 0, '2025-12-16 21:06:50', '2025-12-16 21:06:50'),
	(2, 'ventas', 0, '2025-12-16 21:06:50', '2025-12-16 21:06:50'),
	(4, 'colaborador', 0, '2025-12-16 21:06:50', '2025-12-16 21:06:50'),
	(6, 'rh', 0, '2025-12-16 21:06:50', '2025-12-16 21:06:50'),
	(7, 'asistente admin', 0, '2025-12-16 21:06:50', '2025-12-16 21:06:50'),
	(8, 'Vendedor POS', 0, '2026-09-02 01:42:27', '2026-09-02 01:42:27');

-- Volcando estructura para tabla growvent_staltos.role_screens
CREATE TABLE IF NOT EXISTS `role_screens` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `role_id` bigint unsigned NOT NULL,
  `screen` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `role_screens_role_id_screen_unique` (`role_id`,`screen`),
  KEY `role_screens_role_id_foreign` (`role_id`)
) ENGINE=InnoDB AUTO_INCREMENT=135 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.role_screens: ~90 rows (aproximadamente)
INSERT INTO `role_screens` (`id`, `role_id`, `screen`, `created_at`, `updated_at`) VALUES
	(1, 2, '/crm', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(2, 2, '/preleads', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(3, 2, '/leads', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(4, 2, '/lead-groups', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(5, 2, '/customers', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(6, 2, '/sellers', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(7, 2, '/sells', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(8, 2, '/myServices', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(9, 2, '/destinations', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(10, 2, '/airlines', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(11, 2, '/hotels', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(12, 2, '/services', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(13, 2, '/service_types', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(14, 2, '/megatravel', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(15, 2, '/hr/calendar', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(16, 2, '/my-vacations', '2026-07-02 04:02:03', '2026-07-02 04:02:03'),
	(17, 4, '/hr/calendar', '2026-07-02 04:02:47', '2026-07-02 04:02:47'),
	(18, 4, '/my-vacations', '2026-07-02 04:02:47', '2026-07-02 04:02:47'),
	(19, 6, '/hr', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(20, 6, '/hr/collaborators', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(21, 6, '/hr/vacations', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(22, 6, '/payroll', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(23, 6, '/hr/events', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(24, 6, '/hr/calendar', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(25, 6, '/my-vacations', '2026-07-02 04:03:37', '2026-07-02 04:03:37'),
	(26, 7, '/crm', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(27, 7, '/sellers', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(28, 7, '/sells', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(29, 7, '/payment/pending', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(30, 7, '/payment', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(31, 7, '/supplier-payments', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(32, 7, '/payment/statement', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(33, 7, '/payment/report', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(34, 7, '/admin', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(35, 7, '/reports', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(36, 7, '/transactions', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(37, 7, '/hr/calendar', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(38, 7, '/my-vacations', '2026-07-02 04:04:09', '2026-07-02 04:04:09'),
	(83, 8, '/pos', '2026-09-02 01:42:27', '2026-09-02 01:42:27'),
	(84, 8, '/pos/sales', '2026-09-02 01:42:27', '2026-09-02 01:42:27'),
	(85, 8, '/pos/closeout', '2026-09-02 01:42:27', '2026-09-02 01:42:27'),
	(86, 8, '/customers', '2026-09-02 01:42:27', '2026-09-02 01:42:27'),
	(87, 8, '/portal', '2026-09-02 01:42:27', '2026-09-02 01:42:27'),
	(88, 1, '/settings', '2026-09-02 04:45:49', '2026-09-02 04:45:49'),
	(89, 1, '/roles', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(90, 1, '/crm', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(91, 1, '/preleads', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(92, 1, '/leads', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(93, 1, '/lead-groups', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(94, 1, '/customers', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(95, 1, '/sells', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(96, 1, '/sellers', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(97, 1, '/payment', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(98, 1, '/payment/pending', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(99, 1, '/supplier-payments', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(100, 1, '/payment/statement', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(101, 1, '/payment/report', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(102, 1, '/admin', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(103, 1, '/myServices', '2026-09-02 04:45:50', '2026-09-02 04:45:50'),
	(104, 1, '/transactions', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(105, 1, '/reports', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(106, 1, '/hr', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(107, 1, '/hr/collaborators', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(108, 1, '/payroll', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(109, 1, '/hr/vacations', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(110, 1, '/hr/events', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(111, 1, '/destinations', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(112, 1, '/airlines', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(113, 1, '/hotels', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(114, 1, '/services', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(115, 1, '/service_types', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(116, 1, '/megatravel', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(117, 1, '/admin/lists', '2026-09-02 04:45:51', '2026-09-02 04:45:51'),
	(118, 1, '/periods', '2026-09-02 04:45:52', '2026-09-02 04:45:52'),
	(119, 1, '/subsidiaries', '2026-09-02 04:45:52', '2026-09-02 04:45:52'),
	(120, 1, '/groups', '2026-09-02 04:45:52', '2026-09-02 04:45:52'),
	(121, 1, '/classifications', '2026-09-02 04:45:52', '2026-09-02 04:45:52'),
	(122, 1, '/channels', '2026-09-02 04:45:52', '2026-09-02 04:45:52'),
	(123, 1, '/customer-classifications', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(124, 1, '/currencies', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(125, 1, '/business', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(126, 1, '/bank', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(127, 1, '/utilities', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(128, 1, '/hr/calendar', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(129, 1, '/my-vacations', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(130, 1, '/pos', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(131, 1, '/pos/new', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(132, 1, '/pos/closeout', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(133, 1, '/pos/sales', '2026-09-02 04:45:53', '2026-09-02 04:45:53'),
	(134, 1, '/pos/custom-fields', '2026-09-02 04:45:53', '2026-09-02 04:45:53');

-- Volcando estructura para tabla growvent_staltos.services
CREATE TABLE IF NOT EXISTS `services` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.services: ~3 rows (aproximadamente)
INSERT INTO `services` (`id`, `name`, `description`, `created_at`, `updated_at`) VALUES
	(1, 'Servicios Especiales Grow Travel', 'No entran en ninguna otra clasificación', '2025-12-16 21:20:38', '2025-12-16 21:20:38'),
	(2, 'TOUR', 'Personalizado', '2025-12-16 22:34:17', '2025-12-16 22:34:17'),
	(3, 'Viaje a Vallarta', 'Personalizado', '2026-03-26 21:52:17', '2026-03-26 21:52:17');

-- Volcando estructura para tabla growvent_staltos.service_categories
CREATE TABLE IF NOT EXISTS `service_categories` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `tablename` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.service_categories: ~4 rows (aproximadamente)
INSERT INTO `service_categories` (`id`, `name`, `tablename`, `created_at`, `updated_at`) VALUES
	(1, 'Vuelos', 'airlines', '2025-12-16 21:07:02', '2025-12-16 21:07:02'),
	(2, 'Hospedaje', 'hotels', '2025-12-16 21:07:03', '2025-12-16 21:07:03'),
	(3, 'Otros servicios', 'services', '2025-12-16 21:07:03', '2025-12-16 21:07:03'),
	(4, 'Paquete especial', 'services', '2026-01-29 22:46:24', '2026-01-29 22:46:24');

-- Volcando estructura para tabla growvent_staltos.service_types
CREATE TABLE IF NOT EXISTS `service_types` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.service_types: ~11 rows (aproximadamente)
INSERT INTO `service_types` (`id`, `name`, `created_at`, `updated_at`) VALUES
	(1, 'Nacional', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(2, 'Internacional', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(3, 'Paquete', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(4, 'Vuelos', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(5, 'Tours', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(6, 'Hospedaje', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(7, 'Cruceros', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(8, 'Autos', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(9, 'Traslados', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(10, 'Otros', '2025-12-16 21:07:09', '2025-12-16 21:07:09'),
	(11, 'Proyecto de viaje', '2026-01-29 22:45:10', '2026-01-29 22:45:10');

-- Volcando estructura para tabla growvent_staltos.sessions
CREATE TABLE IF NOT EXISTS `sessions` (
  `id` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `user_id` bigint unsigned DEFAULT NULL,
  `ip_address` varchar(45) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `user_agent` text COLLATE utf8mb4_unicode_ci,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `last_activity` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `sessions_user_id_index` (`user_id`),
  KEY `sessions_last_activity_index` (`last_activity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.sessions: ~21 rows (aproximadamente)
INSERT INTO `sessions` (`id`, `user_id`, `ip_address`, `user_agent`, `payload`, `last_activity`) VALUES
	('1nhHrP7gMQsx0SliK9OELwRlEzpejerycdmi24lK', 10, '192.168.0.100', 'Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Mobile Safari/537.36', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoidFNVMnJET0ZoZUxoaVVyYTF0ZkVmcGx0cDJyN2dzdm1hMkFRaHE1dCI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjU6Imh0dHA6Ly8xOTIuMTY4LjAuMTExOjgwMDAiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX1zOjUwOiJsb2dpbl93ZWJfNTliYTM2YWRkYzJiMmY5NDAxNTgwZjAxNGM3ZjU4ZWE0ZTMwOTg5ZCI7aToxMDt9', 1788389500),
	('20Xvh2C84gkZcBKpgYsVLl4geyaiLgWlubRMblvB', NULL, '104.64.208.243', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiZ1dMQWkxVE5GeGc3Q3B3bm9hdUJmQ0xrVVlYQzBmbFJOMGYyRTlENiI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788148218),
	('cufB8V6blVxiMcvfSQ2DVt08TDuWN1ev2lNPcAi1', 10, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiMTF2ZUtYb25Oa1FTN0NkMzNVNnlHRHBxOTQ5SXV0bmZ1aUJQTEg3aSI7czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjE6Imh0dHA6Ly9sb2NhbGhvc3Q6ODAwMCI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788381434),
	('EOLq6eGzStwFh78wonYKB8CVwcBNMIaCahBamUeS', NULL, '192.168.0.111', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/152.0.0.0 Safari/537.36', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoid24wRFRFb3VkV1RlbkFPMWNXU2pUYXR0a2hyR2Zic1VOcmZDY3F3SyI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjU6Imh0dHA6Ly8xOTIuMTY4LjAuMTExOjgwMDAiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX19', 1788381983),
	('fSdeHrBEUxSmEzsq6LVBgrmKXxjO8N1ViIrcWK3e', 10, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiMm4wNVZmNzQ3QzlYSEgwVGt1ME9PdUhoa3A5aWlzNGdoRmRTODhGNyI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6Mzc6Imh0dHA6Ly9sb2NhbGhvc3Qvc3RhbHRvc19hZG1pbi9wdWJsaWMiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX1zOjUwOiJsb2dpbl93ZWJfNTliYTM2YWRkYzJiMmY5NDAxNTgwZjAxNGM3ZjU4ZWE0ZTMwOTg5ZCI7aToxMDt9', 1788296064),
	('GdwyxlmOU1SCasBT6rrjbWVmbhhD7tckY0Hhdwty', NULL, '187.184.243.35', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoibFVpaFJNNkJWWkhpTk50TFR3THZIN0d1WDB3YW8xM3g4NnV4d0pxeiI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788215545),
	('I333oHs23uQTpxSSQPvGnNqWEljSpjkKbNarqXC3', 10, '192.168.0.111', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiREcwM2NYVUZwWmkwc2J6RldTWUNKTFIwem1OOXZ4aTJVMDBzc21aQyI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6NDU6Imh0dHA6Ly8xOTIuMTY4LjAuMTExOjgwMDAvc2FuY3R1bS9jc3JmLWNvb2tpZSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fXM6NTA6ImxvZ2luX3dlYl81OWJhMzZhZGRjMmIyZjk0MDE1ODBmMDE0YzdmNThlYTRlMzA5ODlkIjtpOjEwO30=', 1788387828),
	('I4gdMrE6H82f63h3aTvDwjCkASHRxf5i5lTzkZTW', NULL, '188.166.42.18', 'Mozilla/5.0 (compatible; ForestEngine/1.0; +https://forestengine.net/)', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoibGtGcDJqRERuYXdhampFMFpoS1lTOVhaS2JWUE94NTJBMm11NWFqWCI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788166254),
	('kh1FSHfzjBb0T1JzxbNsPgJdhiRBQXU7rwFHMhEd', NULL, '104.64.208.243', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiWDQxWVdGelNydVNITDhEeU1PR1pUbDFpbjAzMnM2RVpKSVFKVGpidSI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788149894),
	('KMlrFGfb7nak5HbErz0rg8cPiIBkOMUkvDtuC32G', NULL, '66.132.224.83', 'Mozilla/5.0 (compatible; CensysInspect/1.1; +https://about.censys.io/)', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiVksyMDVKWjZJREE5Z3F4bklqdFRSWFl0Mm5zdHc4R2FOZm5hREtQWCI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788265001),
	('LvvKnNoC4qRRGWc75QEcuYsyPRJ10qBCnXqs9drb', 10, '192.168.0.252', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36 Edg/151.0.4129.101', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiSHRodVIyckNnS1R3cEdwTm01N1J0dlQ5QWJSck9WcHVGRVJWQlZjWCI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjU6Imh0dHA6Ly8xOTIuMTY4LjAuMTExOjgwMDAiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX1zOjUwOiJsb2dpbl93ZWJfNTliYTM2YWRkYzJiMmY5NDAxNTgwZjAxNGM3ZjU4ZWE0ZTMwOTg5ZCI7aToxMDt9', 1788387161),
	('lzTeihVek0rbhw3QdY9pG4FQggMipqinio9D376q', NULL, '34.237.50.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiUDVVSE0xMGRLNm5QSDlxSkhocEZvVGpWTFBKRm11NGlBdGtWMEtmQSI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788192717),
	('m1CxMxyq1tMkFXC4pwkkTBkC60CNK9quS8McFGip', 10, '192.168.0.111', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiZHZrbmJzenNrSHJkM2FZVzJ3Q1cwcWZrRktuT01XVEJZRGdOVEo3TSI7czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjU6Imh0dHA6Ly8xOTIuMTY4LjAuMTExOjgwMDAiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX19', 1788447895),
	('o8QkRtc6HxxaHLsoiaz7wmGRJVzkT3J61IDCP6wq', 10, '187.150.91.101', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoibXpnY2VNbHJONTR6b3BnQVJieGo2UjNtZnE0RnFKNzRnNUUxRTBiZiI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6NTM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbS9zYW5jdHVtL2NzcmYtY29va2llIjt9czo2OiJfZmxhc2giO2E6Mjp7czozOiJvbGQiO2E6MDp7fXM6MzoibmV3IjthOjA6e319czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7fQ==', 1788231818),
	('oohn6GIF4iubA6Qnn91UBnlFe62rhllklGeA7oqd', 10, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiY2ZuamlTaVlyVkhDd3JsaEJyRnc4aE9CaXk3RzNzajdkQ0NGV1ZoSiI7czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6Mzc6Imh0dHA6Ly9sb2NhbGhvc3Qvc3RhbHRvc19hZG1pbi9wdWJsaWMiO31zOjY6Il9mbGFzaCI7YToyOntzOjM6Im9sZCI7YTowOnt9czozOiJuZXciO2E6MDp7fX19', 1788318604),
	('R5migmBgChcf9uhS05C2rN38AGu7G4dkIoBDefR4', 10, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiMWRwcUdoQ0lzWGJHSkdKb1puclV5NVB3S2p2NFZ5dU5USDNOeHdqOSI7czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MjE6Imh0dHA6Ly9sb2NhbGhvc3Q6ODAwMCI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788381689),
	('SHBisME9Bhrmx7BXRfurnhSIH8X7zKSvoDw2LE9o', 10, '187.252.204.179', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:154.0) Gecko/20100101 Firefox/154.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoib1JQWkNuUzhoYjRwaG1QZklaRG5CN0RiNVRhU3hGUkk2ZVVkSHNwNCI7czo1MDoibG9naW5fd2ViXzU5YmEzNmFkZGMyYjJmOTQwMTU4MGYwMTRjN2Y1OGVhNGUzMDk4OWQiO2k6MTA7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788278555),
	('TodAnf8bmxGT7T629cmjsc3i7OjDRkj4ZzIDCR1G', NULL, '34.204.184.214', 'Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiYmpFZjdyT2pLNnlzcmo3SWlQUFFyb1lhdUY2aGl2N1BudUExazhRVCI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788098324),
	('upiB7P3fJYRRuq7FrHYZrlxhryrs25SB39XLSwih', NULL, '40.77.167.235', 'Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm) Chrome/116.0.1938.76 Safari/537.36', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoialF3TmkyUHFYSm9yWHVsTExObXJRRDZibENQbGV4NmtSeHg5bzFJTSI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788175409),
	('vbkz5pYCqygb8INbbJZAfXj6cxmJVkuOdFOVztKw', NULL, '188.166.57.96', 'Mozilla/5.0 (compatible; ForestEngine/1.0; +https://forestengine.net/)', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoiN2J1Y1JDNjZFcEFXaVAzVmNCN1FLdjJjRzdwYUtTb1JqdGM5dWwyYiI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788176424),
	('xdLZWHkj1H13736iH2EeeNjk64GSSo4b7GDUAWGC', NULL, '187.184.243.35', 'WhatsApp/2.2632.100 W', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoidUgwOFA3MExna3p3MVpWUnc4U0NjeUpibTA5STBDS2RMM0VpOTlURSI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788215591),
	('xnezMuahBY2srmQYisrsncGFxgoaJbafTmNM2wX7', NULL, '211.237.0.216', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36', 'YTozOntzOjY6Il90b2tlbiI7czo0MDoielU3UlpVWmIzU2J4Y1M0SFZnSHFZSFMzSGl6VFNlOU5GbnJMVFNxSSI7czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6MzM6Imh0dHBzOi8vZGVtby50cmF2ZWwuZ3Jvd3ZlbnRhLmNvbSI7fXM6NjoiX2ZsYXNoIjthOjI6e3M6Mzoib2xkIjthOjA6e31zOjM6Im5ldyI7YTowOnt9fX0=', 1788293382);

-- Volcando estructura para tabla growvent_staltos.settings
CREATE TABLE IF NOT EXISTS `settings` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `address` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `phone` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `terms` text COLLATE utf8mb4_unicode_ci,
  `reservation_prefix` varchar(32) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `reservation_format` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '[YYYY][MM][DD]-[ID]',
  `reservation_id_pad_length` tinyint unsigned NOT NULL DEFAULT '1',
  `profit_percentage` decimal(5,2) NOT NULL DEFAULT '0.00',
  `profit_include_in_total` tinyint(1) NOT NULL DEFAULT '1',
  `enable_birthdate` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `settings_email_unique` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.settings: ~1 rows (aproximadamente)
INSERT INTO `settings` (`id`, `name`, `address`, `phone`, `email`, `terms`, `reservation_prefix`, `reservation_format`, `reservation_id_pad_length`, `profit_percentage`, `profit_include_in_total`, `enable_birthdate`, `created_at`, `updated_at`) VALUES
	(2, 'Grow Travel', 'Dirección Principal #510, Jalisco', '+52 55 9110 8340', 'info@growventa.com', 'EL CLIENTE reconoce que GROW TRAVEL (en lo sucesivo actúa\núnicamente como intermediaria de los prestadores de servicios con los que ha contratado EL CLIENTE y a los que\nse refiere(n) el boleto, recibo y/o voucher y demás documentación que se expide(n) al CLIENTE con motivo de los\nservicios, los cuales estarán sujetos a los términos y condiciones especificados por los prestadores de servicios\ncontratados, incluyendo de manera enunciativa más no limitadas cars por cancelaciones, perdidas de vuelos,\nperdidas de equipaje, entre otros. Los mencionados servicios serán proporcionados por prestadores\nindependientes que no actúan por o en representación de GROW TRAVEL, y en este\nacto, EL CLIENTE se hace expresamente conocedor de esta situación, así´ mismo acepta que ni GROW TRAVEL, ni sus subsidiarias, filiales, controladoras o representantes tienen injerencia alguna\nen la presentación de los servicios que ha contratado. En este acto EL CLIENTE libera a GROW TRAVEL, así´ como a sus accionistas, consejeros, afiliadas, subsidiarias, asesores, factores,\ndependientes, o partes relacionadas, de todo tipo de responsabilidad generada con motivo de cualesquier\nincumplimiento, hecho, acto omisión, accidente o lesión vinculada y/o derivada de los servicios que ha contratado\nEL CLIENTE, incluyendo daños y perjuicios, ya sean directos, indirectos, incidentales, económicos, y/o negligentes,\nasí´ como pérdidas, costos o gastos que pudieren ser generados con motivo o en relación a los servicios\ncontratados a través de Empresa. Toda reservación de hotel, paquetes, vuelos expedición de boletos y demás\ndocumentación genera un honorario por servicio y no son reembolsables bajo ninguna circunstancia en caso de\ncancelación. Para reservar los paquetes, en su mayoría se requiere de un depósito que puede variar de acuerdo al\noperador, destino, o temporada en que se quiere viajar este depósito inicial NO es reembolsable. La reservación de\nlos circuitos que ya estén dentro del período de cars, requerirá´ del pa total de los servicios. Una vez confirmados\nlos servicios estarán sujetos al tiempo límite para pa total que indique el operador, quedando sujetos a sus políticas\nde cars que pueden ir desde $100.00 USD por persona por trámites administrativos; hasta del 50 o 100% del\npaquete; los cuales serán cobrados de acuerdo a cada caso. Los depósitos no son reembolsables en caso de\ncualquier cancelación, según políticas del operador responsable. Los boletos de avión con tarifa económica son: •\nNo reembolsables. • No endosables. • No transferibles (esto quiere decir que no se aceptan cambios de nombres).\nTodas las aerolíneas se reservan el derecho de sobrevender, cambiar horarios, e incluso cancelar o realizar\ncambios que ellos consideren pertinentes sin previo aviso y sin responsabilidad de [Empresa]; esto esta´ autorizado\ngubernamentalmente. Los programas con vuelos tipo cha´rter, inter-asia´ticos e inter-sudamericanos no son\nreembolsables y no permiten cambios de ningún tipo, según políticas del operador. En caso de que tenga que\ninterrumpir su viaje por causas personales, los servicios total o parcialmente no utilizados no serán reembolsables.\nTodos los viajeros deberán contar con la documentación necesaria para realizar su viaje; pasaportes vigentes,\nvisas, siendo de su total responsabilidad los problemas e inconvenientes que puedan surgir por incumplimientos de\neste requerimiento. Los cambios o cancelación de los viajes ocasionados por causas originadas por huracanes,\nciclones, terremotos o cualquier otra, son ajenos a [Empresa], y quedarán sujetos a las políticas y lineamientos de\nla línea aérea, transportista u operador para la cancelación total de viaje o cambios de itinerario. Los reembolsos\nparciales o totales dependerán de la línea aérea, transportista u operador. Recibí de conformidad y de manera\noportuna, profesional y con la calidad necesaria los documentos de viaje, aceptando que corresponden a las\nfechas, nombres de pasajeros y horarios acordados de acuerdo con las características y necesidades del servicio.\nCausales de rescisión y consecuencias jurídicas: EL CLIENTE acepta por falta de pa total o parcial de los servicios\nque ha contratado a través de GROW TRAVEL será´ causa suficiente de cancelación\no rescisión del contrato y de los servicios contratados por EL CLIENTE con los diversos prestadores mencionados\nque pudieran ser de manera enunciativa más no limitativa: líneas aéreas, hoteles, transportistas terrestres,\nmarítimos u operadores, etc. por lo que en este acto EL CLIENTE libera en todo momento a GROW TRAVEL, a sus accionistas, consejeros, afiliados, subsidiarias, asesores, factores\ndependientes o partes relacionadas de cualquier responsabilidad que por su incumplimiento (DEL CLIENTE)\npudiera corresponder. "GROW TRAVEL”\ncon domicilio en DIRECCION PRINCIPAL #510, JALISCO En cumplimiento a la Ley Federal de Protección de Datos Personales en\nPosesión de los Particulares (la “Ley”), y su reglamento, hacen de su conocimiento que: "GROW TRAVEL” protegemos y salvaguardamos sus datos\npersonales para evitar el daño, pérdida, destrucción, robo, extravío, alteración, así como el tratamiento no\nautorizado de sus datos personales. En esta sección, encontrarás los términos y condiciones aplicables a cada\npromoción publicada dentro de nuestro sitio.', 'GT', '[IDENTIFICADOR]-[ID]', 5, 20.00, 1, 1, '2025-12-16 21:15:31', '2026-03-17 06:53:54');

-- Volcando estructura para tabla growvent_staltos.stages
CREATE TABLE IF NOT EXISTS `stages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `color` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.stages: ~5 rows (aproximadamente)
INSERT INTO `stages` (`id`, `name`, `color`, `created_at`, `updated_at`) VALUES
	(1, 'Solicitud', 'blue', '2025-12-16 21:07:49', '2025-12-16 21:07:49'),
	(2, 'Cobranza', '#3C3C3B', '2025-12-16 21:07:49', '2025-12-16 21:07:49'),
	(3, 'Pagado', 'green', '2025-12-16 21:07:49', '2025-12-16 21:07:49'),
	(4, 'Cerrado', '#3b3b3b', '2025-12-16 21:07:49', '2025-12-16 21:07:49'),
	(5, 'Cancelado', 'red', '2025-12-16 21:07:49', '2025-12-16 21:07:49');

-- Volcando estructura para tabla growvent_staltos.subsidiaries
CREATE TABLE IF NOT EXISTS `subsidiaries` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `address` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.subsidiaries: ~3 rows (aproximadamente)
INSERT INTO `subsidiaries` (`id`, `name`, `address`, `created_at`, `updated_at`) VALUES
	(4, 'Todas', 'NA', '2025-12-16 21:08:03', '2025-12-16 21:08:03'),
	(5, 'Mérida', 'Address 1', '2025-12-16 21:08:03', '2025-12-16 21:12:36'),
	(6, 'Cancún', 'Address 2', '2025-12-16 21:08:03', '2025-12-16 21:12:42');

-- Volcando estructura para tabla growvent_staltos.supplier_group_payments
CREATE TABLE IF NOT EXISTS `supplier_group_payments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_group_id` bigint unsigned DEFAULT NULL,
  `supplier_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `currency` varchar(3) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `exchange_rate` decimal(10,4) NOT NULL DEFAULT '1.0000',
  `total_mxn` decimal(15,2) DEFAULT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `canceled` tinyint(1) NOT NULL DEFAULT '0',
  `paid` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `supplier_group_payments_lead_group_id_foreign` (`lead_group_id`),
  CONSTRAINT `supplier_group_payments_lead_group_id_foreign` FOREIGN KEY (`lead_group_id`) REFERENCES `lead_groups` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.supplier_group_payments: ~2 rows (aproximadamente)
INSERT INTO `supplier_group_payments` (`id`, `lead_group_id`, `supplier_name`, `description`, `total`, `currency`, `exchange_rate`, `total_mxn`, `file_path`, `canceled`, `paid`, `created_at`, `updated_at`) VALUES
	(1, 3, 'HOTEL ORLANDO', 'APARTADO DE HABITACIONES DE HOTEL', 1200.00, 'USD', 18.5000, 22200.00, NULL, 0, 1, '2025-12-16 22:43:09', '2025-12-16 22:43:18'),
	(2, 4, 'Volaris', 'Vuelos del viaje', 8000.00, 'MXN', 1.0000, 8000.00, NULL, 0, 1, '2026-01-29 23:08:08', '2026-01-29 23:08:49');

-- Volcando estructura para tabla growvent_staltos.supplier_payments
CREATE TABLE IF NOT EXISTS `supplier_payments` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned DEFAULT NULL,
  `supplier_group_payment_id` bigint unsigned DEFAULT NULL,
  `supplier_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `total` decimal(15,2) NOT NULL DEFAULT '0.00',
  `currency` varchar(3) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'MXN',
  `exchange_rate` decimal(10,4) NOT NULL DEFAULT '1.0000',
  `total_mxn` decimal(15,2) DEFAULT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `canceled` tinyint(1) NOT NULL DEFAULT '0',
  `paid` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `supplier_payments_lead_id_foreign` (`lead_id`),
  KEY `supplier_payments_supplier_group_payment_id_foreign` (`supplier_group_payment_id`),
  CONSTRAINT `supplier_payments_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE SET NULL,
  CONSTRAINT `supplier_payments_supplier_group_payment_id_foreign` FOREIGN KEY (`supplier_group_payment_id`) REFERENCES `supplier_group_payments` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=22 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.supplier_payments: ~14 rows (aproximadamente)
INSERT INTO `supplier_payments` (`id`, `lead_id`, `supplier_group_payment_id`, `supplier_name`, `description`, `total`, `currency`, `exchange_rate`, `total_mxn`, `file_path`, `canceled`, `paid`, `created_at`, `updated_at`) VALUES
	(8, 7, 1, 'HOTEL ORLANDO', 'APARTADO DE HABITACIONES DE HOTEL', 400.00, 'USD', 18.5000, 7400.00, NULL, 0, 1, '2025-12-16 22:43:09', '2025-12-16 22:43:18'),
	(9, 8, 1, 'HOTEL ORLANDO', 'APARTADO DE HABITACIONES DE HOTEL', 400.00, 'USD', 18.5000, 7400.00, NULL, 0, 1, '2025-12-16 22:43:09', '2025-12-16 22:43:18'),
	(10, 9, 1, 'HOTEL ORLANDO', 'APARTADO DE HABITACIONES DE HOTEL', 400.00, 'USD', 18.5000, 7400.00, NULL, 0, 1, '2025-12-16 22:43:09', '2025-12-16 22:43:18'),
	(11, 10, NULL, 'Hotel', 'Anticipo para confirmar reserva de hotel', 2000.00, 'MXN', 1.0000, 2000.00, 'https://growventa-public.s3.amazonaws.com/growtravel/lKVrbG0dAeylrDESTrJy4WvjSLL4vt8yRbwoOb3B.pdf', 0, 1, '2026-01-29 22:57:53', '2026-01-29 22:58:30'),
	(12, 8, NULL, 'Volaris', 'Confirmación de asientos', 20.00, 'USD', 18.5000, 370.00, NULL, 0, 1, '2026-01-29 22:59:07', '2026-01-29 22:59:12'),
	(13, 11, 2, 'Volaris', 'Vuelos del viaje', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 1, '2026-01-29 23:08:08', '2026-01-29 23:08:49'),
	(14, 12, 2, 'Volaris', 'Vuelos del viaje', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 1, '2026-01-29 23:08:08', '2026-01-29 23:08:49'),
	(15, 15, 2, 'Volaris', 'Vuelos del viaje', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 1, '2026-01-29 23:08:08', '2026-01-29 23:08:49'),
	(16, 16, 2, 'Volaris', 'Vuelos del viaje', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 1, '2026-01-29 23:08:08', '2026-01-29 23:08:49'),
	(17, 21, NULL, 'Volaris', 'Anticipo del vuelo', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 1, '2026-03-26 22:36:14', '2026-03-26 22:36:26'),
	(18, 18, NULL, 'Mega Travel', 'Liquidacion', 5000.00, 'MXN', 1.0000, 5000.00, NULL, 0, 1, '2026-03-26 22:37:02', '2026-03-26 22:37:05'),
	(19, 27, NULL, 'NEFERTARI', 'ABONO', 2000.00, 'MXN', 1.0000, 2000.00, NULL, 0, 0, '2026-04-20 19:03:55', '2026-04-20 19:03:55'),
	(20, 16, NULL, 'CHOYA TOURS', 'Viaje a La Paz BCS', 25000.00, 'MXN', 1.0000, 25000.00, NULL, 0, 0, '2026-05-03 04:23:06', '2026-05-03 04:23:06'),
	(21, 31, NULL, 'Tour', 'Servicios', 50.00, 'USD', 19.2400, 962.00, NULL, 0, 1, '2026-08-25 19:59:59', '2026-08-25 20:00:07');

-- Volcando estructura para tabla growvent_staltos.surveys
CREATE TABLE IF NOT EXISTS `surveys` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `version` smallint unsigned NOT NULL DEFAULT '1',
  `active` tinyint(1) NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.surveys: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.survey_answers
CREATE TABLE IF NOT EXISTS `survey_answers` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `lead_id` bigint unsigned NOT NULL,
  `survey_id` bigint unsigned NOT NULL,
  `survey_question_id` bigint unsigned NOT NULL,
  `answer` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_lead_question` (`lead_id`,`survey_question_id`),
  KEY `survey_answers_survey_id_foreign` (`survey_id`),
  KEY `survey_answers_survey_question_id_foreign` (`survey_question_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.survey_answers: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.survey_questions
CREATE TABLE IF NOT EXISTS `survey_questions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `survey_id` bigint unsigned NOT NULL,
  `question` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `order` smallint unsigned NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `survey_questions_survey_id_foreign` (`survey_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.survey_questions: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.tags
CREATE TABLE IF NOT EXISTS `tags` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `color` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `lead_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `tags_lead_id_foreign` (`lead_id`),
  CONSTRAINT `tags_lead_id_foreign` FOREIGN KEY (`lead_id`) REFERENCES `leads` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.tags: ~0 rows (aproximadamente)

-- Volcando estructura para tabla growvent_staltos.transactions
CREATE TABLE IF NOT EXISTS `transactions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `date` date NOT NULL,
  `description` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `total` decimal(15,2) NOT NULL,
  `group_id` bigint unsigned DEFAULT NULL,
  `classification_id` bigint unsigned DEFAULT NULL,
  `subsidiary_id` bigint unsigned DEFAULT NULL,
  `period_id` bigint unsigned DEFAULT NULL,
  `payroll_payment_id` bigint unsigned DEFAULT NULL,
  `payment_id` bigint unsigned DEFAULT NULL,
  `supplier_payment_id` bigint unsigned DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `transactions_group_id_foreign` (`group_id`),
  KEY `transactions_classification_id_foreign` (`classification_id`),
  KEY `transactions_subsidiary_id_foreign` (`subsidiary_id`),
  KEY `transactions_period_id_foreign` (`period_id`),
  KEY `transactions_payroll_payment_id_foreign` (`payroll_payment_id`),
  KEY `transactions_payment_id_foreign` (`payment_id`),
  KEY `transactions_supplier_payment_id_foreign` (`supplier_payment_id`),
  CONSTRAINT `transactions_classification_id_foreign` FOREIGN KEY (`classification_id`) REFERENCES `classifications` (`id`) ON DELETE SET NULL,
  CONSTRAINT `transactions_group_id_foreign` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE SET NULL,
  CONSTRAINT `transactions_payment_id_foreign` FOREIGN KEY (`payment_id`) REFERENCES `payments` (`id`) ON DELETE CASCADE,
  CONSTRAINT `transactions_payroll_payment_id_foreign` FOREIGN KEY (`payroll_payment_id`) REFERENCES `payroll_payments` (`id`) ON DELETE CASCADE,
  CONSTRAINT `transactions_period_id_foreign` FOREIGN KEY (`period_id`) REFERENCES `periods` (`id`) ON DELETE SET NULL,
  CONSTRAINT `transactions_subsidiary_id_foreign` FOREIGN KEY (`subsidiary_id`) REFERENCES `subsidiaries` (`id`) ON DELETE SET NULL,
  CONSTRAINT `transactions_supplier_payment_id_foreign` FOREIGN KEY (`supplier_payment_id`) REFERENCES `supplier_payments` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=1852 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.transactions: ~74 rows (aproximadamente)
INSERT INTO `transactions` (`id`, `date`, `description`, `type`, `total`, `group_id`, `classification_id`, `subsidiary_id`, `period_id`, `payroll_payment_id`, `payment_id`, `supplier_payment_id`, `created_at`, `updated_at`) VALUES
	(1775, '2025-12-16', 'Pago de Cliente -  Reservación: GW-2025-0005 Descripción: Apartado - (pago_id:2)', 'Ingreso', 1000.00, 9, 8, 4, 1, NULL, 2, NULL, '2025-12-16 22:39:52', '2025-12-16 22:39:52'),
	(1776, '2025-12-16', 'Pago a Proveedor - Reservación: GW-2025-0007 Descripción: APARTADO DE HABITACIONES DE HOTEL Proveedor: HOTEL ORLANDO (pago_proveedor_id:8)', 'Egreso', -7400.00, 23, 10, 4, 1, NULL, NULL, 8, '2025-12-16 22:43:18', '2025-12-16 22:43:18'),
	(1777, '2025-12-16', 'Pago a Proveedor - Reservación: GW-2025-0008 Descripción: APARTADO DE HABITACIONES DE HOTEL Proveedor: HOTEL ORLANDO (pago_proveedor_id:9)', 'Egreso', -7400.00, 23, 10, 4, 1, NULL, NULL, 9, '2025-12-16 22:43:18', '2025-12-16 22:43:18'),
	(1778, '2025-12-16', 'Pago a Proveedor - Reservación: GW-2025-0009 Descripción: APARTADO DE HABITACIONES DE HOTEL Proveedor: HOTEL ORLANDO (pago_proveedor_id:10)', 'Egreso', -7400.00, 23, 10, 4, 1, NULL, NULL, 10, '2025-12-16 22:43:18', '2025-12-16 22:43:18'),
	(1779, '2025-12-16', 'Pago de Cliente -  Reservación: GW-2025-0008 Descripción: ABONO - (pago_id:3)', 'Ingreso', 18000.00, 9, 8, 4, 1, NULL, 3, NULL, '2025-12-16 22:43:24', '2025-12-16 22:43:24'),
	(1780, '2025-12-16', 'Pago de Cliente -  Reservación: GW-2025-0003 Descripción: LIQUIDACION - (pago_id:5)', 'Ingreso', 14650.00, 9, 8, 4, 1, NULL, 5, NULL, '2025-12-16 22:44:25', '2025-12-16 22:44:25'),
	(1781, '2025-12-15', 'Pago de nómina para Admin - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 1, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1782, '2025-12-15', 'Comisión para Admin - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -500.00, 12, 2, 4, 2, 1, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1783, '2025-12-15', 'Prestaciones para Admin - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -200.00, 30, 3, 4, 2, 1, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1784, '2025-12-15', 'Bonos para Admin - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -50.00, 12, 2, 4, 2, 1, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1785, '2025-12-15', 'Pago de nómina para Jose - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -1000.00, 8, 3, 4, 2, 2, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1786, '2025-12-15', 'Comisión para Jose - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -750.00, 12, 2, 4, 2, 2, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1787, '2025-12-15', 'Prestaciones para Jose - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -200.00, 30, 3, 4, 2, 2, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1788, '2025-12-15', 'Bonos para Jose - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -100.00, 12, 2, 4, 2, 2, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1789, '2025-12-15', 'Pago de nómina para Martha - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -1150.00, 8, 3, 4, 2, 3, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1790, '2025-12-15', 'Comisión para Martha - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -450.00, 12, 2, 4, 2, 3, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1791, '2025-12-15', 'Prestaciones para Martha - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -200.00, 30, 3, 4, 2, 3, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1792, '2025-12-15', 'Bonos para Martha - 2025-12-01 00:00:00 al 2025-12-07 00:00:00', 'Egreso', -100.00, 12, 2, 4, 2, 3, NULL, NULL, '2025-12-16 22:47:36', '2025-12-16 22:47:36'),
	(1793, '2026-01-29', 'Pago de Cliente -  Reservación: GW-2026-0010 Descripción: Abono para confirmar la reservación - (pago_id:6)', 'Ingreso', 2500.00, 9, 8, 4, 1, NULL, 6, NULL, '2026-01-29 22:53:22', '2026-01-29 22:53:22'),
	(1794, '2026-01-29', 'Pago de Cliente -  Reservación: GW-2026-0010 Descripción: Segundo abono - (pago_id:7)', 'Ingreso', 2000.00, 9, 8, 4, 1, NULL, 7, NULL, '2026-01-29 22:55:19', '2026-01-29 22:55:19'),
	(1795, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2026-0010 Descripción: Anticipo para confirmar reserva de hotel Proveedor: Hotel (pago_proveedor_id:11)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 11, '2026-01-29 22:58:15', '2026-01-29 22:58:15'),
	(1796, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2025-0008 Descripción: Confirmación de asientos Proveedor: Volaris (pago_proveedor_id:12)', 'Egreso', -370.00, 23, 10, 4, 1, NULL, NULL, 12, '2026-01-29 22:59:12', '2026-01-29 22:59:12'),
	(1797, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2026-0011 Descripción: Vuelos del viaje Proveedor: Volaris (pago_proveedor_id:13)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 13, '2026-01-29 23:08:49', '2026-01-29 23:08:49'),
	(1798, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2026-0012 Descripción: Vuelos del viaje Proveedor: Volaris (pago_proveedor_id:14)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 14, '2026-01-29 23:08:49', '2026-01-29 23:08:49'),
	(1799, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2026-0015 Descripción: Vuelos del viaje Proveedor: Volaris (pago_proveedor_id:15)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 15, '2026-01-29 23:08:49', '2026-01-29 23:08:49'),
	(1800, '2026-01-29', 'Pago a Proveedor - Reservación: GW-2026-0016 Descripción: Vuelos del viaje Proveedor: Volaris (pago_proveedor_id:16)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 16, '2026-01-29 23:08:49', '2026-01-29 23:08:49'),
	(1801, '2026-01-29', 'Pago de Cliente -  Reservación: GW-2026-0011 Descripción: Liquidacion - (pago_id:8)', 'Ingreso', 8750.00, 9, 8, 4, 1, NULL, 8, NULL, '2026-01-29 23:16:46', '2026-01-29 23:16:46'),
	(1802, '2026-01-29', 'Pago semanal de limpieza', 'Egreso', -1000.00, 5, 5, 5, 2, NULL, NULL, NULL, '2026-01-29 23:20:35', '2026-01-29 23:20:42'),
	(1803, '2026-01-15', 'Pago de nómina para Admin - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 4, NULL, NULL, '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(1804, '2026-01-15', 'Pago de nómina para Jose - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1000.00, 8, 3, 4, 2, 5, NULL, NULL, '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(1805, '2026-01-15', 'Comisión para Jose - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -500.00, 12, 2, 4, 2, 5, NULL, NULL, '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(1806, '2026-01-15', 'Pago de nómina para Martha - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 6, NULL, NULL, '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(1807, '2026-01-15', 'Prestaciones para Martha - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -250.00, 30, 3, 4, 2, 6, NULL, NULL, '2026-01-29 23:22:42', '2026-01-29 23:22:42'),
	(1808, '2026-01-30', 'Pago de Cliente -  Reservación: COCO-00010 Descripción: Abono a reserva COCO-000010 - (pago_id:9)', 'Ingreso', 2000.00, 9, 8, 4, 1, NULL, 9, NULL, '2026-01-30 21:39:16', '2026-01-30 21:39:16'),
	(1809, '2026-01-30', 'Pago a la sra limpieza', 'Egreso', -1000.00, 5, 3, 4, 2, NULL, NULL, NULL, '2026-01-30 21:42:19', '2026-01-30 21:42:19'),
	(1810, '2026-01-17', 'Pago de nómina para Admin - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 7, NULL, NULL, '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(1811, '2026-01-17', 'Pago de nómina para Jose - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1000.00, 8, 3, 4, 2, 8, NULL, NULL, '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(1812, '2026-01-17', 'Comisión para Jose - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -850.00, 12, 2, 4, 2, 8, NULL, NULL, '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(1813, '2026-01-17', 'Pago de nómina para Martha - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 9, NULL, NULL, '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(1814, '2026-01-17', 'Bonos para Martha - 2026-01-01 00:00:00 al 2026-01-15 00:00:00', 'Egreso', -300.00, 12, 2, 4, 2, 9, NULL, NULL, '2026-01-30 21:43:55', '2026-01-30 21:43:55'),
	(1815, '2026-03-16', 'Pago de Cliente -  Reservación: GT-00017 Descripción: Pago digital (stripe) – intento #1 - (pago_id:10)', 'Ingreso', 141.60, 9, 8, 4, 1, NULL, 10, NULL, '2026-03-16 23:58:07', '2026-03-16 23:58:07'),
	(1816, '2026-03-16', 'Pago de Cliente -  Reservación: GT-00017 Descripción: Pago digital (paypal) – intento #2 - (pago_id:11)', 'Ingreso', 2418.70, 9, 8, 4, 1, NULL, 11, NULL, '2026-03-17 00:01:16', '2026-03-17 00:01:16'),
	(1817, '2026-03-16', 'Pago de Cliente -  Reservación: GT-00013 Descripción: Pago digital (stripe) – intento #3 - (pago_id:12)', 'Ingreso', 6500.00, 9, 8, 4, 1, NULL, 12, NULL, '2026-03-17 00:10:11', '2026-03-17 00:10:11'),
	(1818, '2026-03-17', 'Pago de Cliente -  Reservación: GT-00018 Descripción: Pago digital (stripe) – intento #4 - (pago_id:13)', 'Ingreso', 1028.57, 9, 8, 4, 1, NULL, 13, NULL, '2026-03-17 22:12:04', '2026-03-17 22:12:04'),
	(1819, '2026-03-18', 'Pago de Cliente -  Reservación: GT-00014 Descripción: Pago digital (stripe) – intento #6 - (pago_id:14)', 'Ingreso', 1104.17, 9, 8, 4, 1, NULL, 14, NULL, '2026-03-18 22:53:30', '2026-03-18 22:53:30'),
	(1820, '2026-03-18', 'Pago de Cliente -  Reservación: GT-00017 Descripción: Pago digital (stripe) – intento #7 - (pago_id:15)', 'Ingreso', 141.60, 9, 8, 4, 1, NULL, 15, NULL, '2026-03-18 22:54:57', '2026-03-18 22:54:57'),
	(1821, '2026-03-21', 'Pago de Cliente -  Reservación: GT-00017 Descripción: TEST - (pago_id:16)', 'Ingreso', 400.00, 9, 8, 4, 1, NULL, 16, NULL, '2026-03-21 23:52:32', '2026-03-21 23:52:32'),
	(1822, '2026-03-21', 'GASOLINA', 'Egreso', -1000.00, 1, 1, 5, 1, NULL, NULL, NULL, '2026-03-22 00:01:10', '2026-03-22 00:01:10'),
	(1823, '2026-03-21', 'Pago de nómina para Admin - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 10, NULL, NULL, '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(1824, '2026-03-21', 'Pago de nómina para Jose - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1000.00, 8, 3, 4, 2, 11, NULL, NULL, '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(1825, '2026-03-21', 'Pago de nómina para Martha - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 12, NULL, NULL, '2026-03-22 00:16:23', '2026-03-22 00:16:23'),
	(1826, '2026-03-26', 'Pago de Cliente -  Reservación: GT-00012 Descripción: Pago digital (stripe) – intento #10 - (pago_id:17)', 'Ingreso', 1750.00, 9, 8, 4, 1, NULL, 17, NULL, '2026-03-26 22:15:14', '2026-03-26 22:15:14'),
	(1827, '2026-03-26', 'Pago de Cliente -  Reservación: GT-00012 Descripción: Segundo abono - (pago_id:18)', 'Ingreso', 1000.00, 9, 8, 4, 1, NULL, 18, NULL, '2026-03-26 22:25:38', '2026-03-26 22:25:38'),
	(1828, '2026-03-26', 'Pago de Cliente -  Reservación: GT-00021 Descripción: Abono 1 - (pago_id:19)', 'Ingreso', 1800.00, 9, 8, 4, 1, NULL, 19, NULL, '2026-03-26 22:35:15', '2026-03-26 22:35:15'),
	(1829, '2026-03-26', 'Pago a Proveedor - Reservación: GT-00021 Descripción: Anticipo del vuelo Proveedor: Volaris (pago_proveedor_id:17)', 'Egreso', -2000.00, 23, 10, 4, 1, NULL, NULL, 17, '2026-03-26 22:36:26', '2026-03-26 22:36:26'),
	(1830, '2026-03-26', 'Pago a Proveedor - Reservación: GT-00018 Descripción: Liquidacion Proveedor: Mega Travel (pago_proveedor_id:18)', 'Egreso', -5000.00, 23, 10, 4, 1, NULL, NULL, 18, '2026-03-26 22:37:05', '2026-03-26 22:37:05'),
	(1831, '2026-03-26', 'Pague pinol', 'Egreso', -500.00, 11, 1, 4, 1, NULL, NULL, NULL, '2026-03-26 22:38:17', '2026-03-26 22:38:17'),
	(1832, '2026-03-21', 'Pago de nómina para Admin - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 13, NULL, NULL, '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(1833, '2026-03-21', 'Bonos para Admin - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -500.00, 12, 2, 4, 2, 13, NULL, NULL, '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(1834, '2026-03-21', 'Pago de nómina para Martha - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 14, NULL, NULL, '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(1835, '2026-03-21', 'Comisión para Martha - 2026-03-16 00:00:00 al 2026-03-22 00:00:00', 'Egreso', -1000.00, 12, 2, 4, 2, 14, NULL, NULL, '2026-03-26 22:43:20', '2026-03-26 22:43:20'),
	(1836, '2026-04-20', 'Pago de Cliente -  Reservación: GT-00027 Descripción:  - (pago_id:20)', 'Ingreso', 8000.00, 9, 8, 4, 1, NULL, 20, NULL, '2026-04-20 18:58:18', '2026-04-20 18:58:18'),
	(1837, '2026-04-18', 'Pago de nómina para Admin - 2026-04-13 00:00:00 al 2026-04-19 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 15, NULL, NULL, '2026-04-20 23:39:05', '2026-04-20 23:39:05'),
	(1838, '2026-04-18', 'Pago de nómina para Martha - 2026-04-13 00:00:00 al 2026-04-19 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 16, NULL, NULL, '2026-04-20 23:39:05', '2026-04-20 23:39:05'),
	(1839, '2026-05-09', 'Pago de Cliente -  Reservación: GT-00022 Descripción:  - (pago_id:21)', 'Ingreso', 50.00, 9, 8, 4, 1, NULL, 21, NULL, '2026-05-09 06:51:38', '2026-05-09 06:51:38'),
	(1840, '2026-05-22', 'Pago de Cliente -  Reservación: GT-00029 Descripción: Pago digital (stripe) – intento #11 - (pago_id:22)', 'Ingreso', 1800.00, 9, 8, 4, 1, NULL, 22, NULL, '2026-05-23 01:49:31', '2026-05-23 01:49:31'),
	(1841, '2026-08-25', 'Pago de Cliente -  Reservación: GT-00031 Descripción: Primer abono - (pago_id:23)', 'Ingreso', 962.00, 9, 8, 4, 1, NULL, 23, NULL, '2026-08-25 19:55:27', '2026-08-25 19:55:27'),
	(1842, '2026-08-25', 'Pago de Cliente -  Reservación: GT-00031 Descripción: Liquidacion - (pago_id:24)', 'Ingreso', 962.00, 9, 8, 4, 1, NULL, 24, NULL, '2026-08-25 19:56:32', '2026-08-25 19:56:32'),
	(1843, '2026-08-25', 'Pago a Proveedor - Reservación: GT-00031 Descripción: Servicios Proveedor: Tour (pago_proveedor_id:21)', 'Egreso', -962.00, 23, 10, 4, 1, NULL, NULL, 21, '2026-08-25 20:00:07', '2026-08-25 20:00:07'),
	(1844, '2026-08-22', 'Pago de nómina para Admin - 2026-08-17 00:00:00 al 2026-08-23 00:00:00', 'Egreso', -1500.00, 8, 3, 4, 2, 17, NULL, NULL, '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(1845, '2026-08-22', 'Comisión para Admin - 2026-08-17 00:00:00 al 2026-08-23 00:00:00', 'Egreso', -500.00, 12, 2, 4, 2, 17, NULL, NULL, '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(1846, '2026-08-22', 'Bonos para Admin - 2026-08-17 00:00:00 al 2026-08-23 00:00:00', 'Egreso', -300.00, 12, 2, 4, 2, 17, NULL, NULL, '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(1847, '2026-08-22', 'Pago de nómina para Martha - 2026-08-17 00:00:00 al 2026-08-23 00:00:00', 'Egreso', -1250.00, 8, 3, 4, 2, 18, NULL, NULL, '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(1848, '2026-08-22', 'Prestaciones para Martha - 2026-08-17 00:00:00 al 2026-08-23 00:00:00', 'Egreso', -240.00, 30, 3, 4, 2, 18, NULL, NULL, '2026-08-25 20:02:03', '2026-08-25 20:02:03'),
	(1849, '2026-09-02', 'Pago de Cliente -  Reservación: GT-00033 Descripción:  - (pago_id:25)', 'Ingreso', 1000.00, 9, 8, 4, 1, NULL, 25, NULL, '2026-09-02 05:53:56', '2026-09-02 05:53:56'),
	(1850, '2026-09-02', 'Pago de Cliente -  Reservación: GT-00034 Descripción:  - (pago_id:26)', 'Ingreso', 500.00, 9, 8, 4, 1, NULL, 26, NULL, '2026-09-03 03:16:37', '2026-09-03 03:16:37'),
	(1851, '2026-09-02', 'Pago de Cliente -  Reservación: GT-00035 Descripción:  - (pago_id:27)', 'Ingreso', 250.00, 9, 8, 4, 1, NULL, 27, NULL, '2026-09-03 03:26:23', '2026-09-03 03:26:23');

-- Volcando estructura para tabla growvent_staltos.users
CREATE TABLE IF NOT EXISTS `users` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email_verified_at` timestamp NULL DEFAULT NULL,
  `password` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `phone` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `commission` decimal(5,2) DEFAULT NULL,
  `birthdate` date DEFAULT NULL,
  `start_date` date DEFAULT NULL,
  `position` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `weekly_salary` decimal(10,2) DEFAULT NULL,
  `remember_token` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `users_email_unique` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.users: ~4 rows (aproximadamente)
INSERT INTO `users` (`id`, `name`, `email`, `email_verified_at`, `password`, `phone`, `commission`, `birthdate`, `start_date`, `position`, `weekly_salary`, `remember_token`, `created_at`, `updated_at`) VALUES
	(10, 'Admin', 'admin@growtravel.com', NULL, '$2y$12$GrwdzToXFR34wlWXr1CrvumJuW/LZFeVbfdOUDePDwZWuOknNDIPO', NULL, NULL, '2026-01-03', NULL, NULL, 1500.00, 'ZC4PwGgFdfDK01TDtdHvvEVj8TZdvrlHHR5w41JkiAb66kVJHlYsopbpVv1M', '2025-12-16 21:08:51', '2026-01-29 23:24:11'),
	(11, 'Jose', 'jose@growtravel.com', NULL, '$2y$12$QJ75OPZ9BP3Hcd/YWKM/8.cQc.QpgDId18CrmXi9/rYMHB7foulRy', NULL, NULL, '2026-01-25', NULL, NULL, 1000.00, NULL, '2025-12-16 22:32:03', '2026-01-29 23:24:17'),
	(12, 'Martha', 'martha@growtravel.com', NULL, '$2y$12$9yS9I2IxGOWBiYIlABggeuf5KKPc2baBrvisK.Pu9PiKN1qwN4pcC', NULL, NULL, '2026-06-30', NULL, NULL, 1250.00, NULL, '2025-12-16 22:36:38', '2026-01-29 23:24:28'),
	(13, 'PEDRO', 'pedro@gmail.com', NULL, '$2y$12$GHdlTxc0hbbJAqAofl6g5u.ChUHBQC0nEeuD5QCddoPDzOHU5mvgq', NULL, NULL, NULL, NULL, NULL, NULL, 'YKi5mg40BtBAfC5eWV9LwF6sxw3wbDjpYA6lmHmcipI2U8XCRPve9vBNHf38', '2026-03-22 00:02:28', '2026-03-22 00:02:28');

-- Volcando estructura para tabla growvent_staltos.user_roles
CREATE TABLE IF NOT EXISTS `user_roles` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `role_id` bigint unsigned NOT NULL,
  `user_id` bigint unsigned NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `user_roles_role_id_foreign` (`role_id`),
  KEY `user_roles_user_id_foreign` (`user_id`),
  CONSTRAINT `user_roles_role_id_foreign` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE,
  CONSTRAINT `user_roles_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=55 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.user_roles: ~7 rows (aproximadamente)
INSERT INTO `user_roles` (`id`, `role_id`, `user_id`, `created_at`, `updated_at`) VALUES
	(40, 1, 10, '2025-12-16 16:10:22', '2025-12-16 16:10:22'),
	(42, 4, 10, '2025-12-16 16:10:55', '2025-12-16 16:10:55'),
	(44, 6, 10, '2025-12-16 16:11:13', '2025-12-16 16:11:13'),
	(48, 2, 12, '2025-12-16 22:36:47', '2025-12-16 22:36:47'),
	(49, 4, 12, '2025-12-16 22:36:47', '2025-12-16 22:36:47'),
	(50, 2, 13, '2026-03-22 00:03:01', '2026-03-22 00:03:01'),
	(51, 2, 11, '2026-03-26 22:41:13', '2026-03-26 22:41:13'),
	(54, 8, 10, '2026-09-01 18:11:25', '2026-09-01 18:11:26');

-- Volcando estructura para tabla growvent_staltos.vacations
CREATE TABLE IF NOT EXISTS `vacations` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `start_date` date NOT NULL,
  `end_date` date NOT NULL,
  `accepted` tinyint(1) NOT NULL DEFAULT '0',
  `refused` tinyint(1) NOT NULL DEFAULT '0',
  `reviewed_at` timestamp NULL DEFAULT NULL,
  `reviewed_by` bigint unsigned DEFAULT NULL,
  `notes` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `vacations_user_id_foreign` (`user_id`),
  KEY `vacations_reviewed_by_foreign` (`reviewed_by`),
  CONSTRAINT `vacations_reviewed_by_foreign` FOREIGN KEY (`reviewed_by`) REFERENCES `users` (`id`) ON DELETE SET NULL,
  CONSTRAINT `vacations_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Volcando datos para la tabla growvent_staltos.vacations: ~3 rows (aproximadamente)
INSERT INTO `vacations` (`id`, `user_id`, `start_date`, `end_date`, `accepted`, `refused`, `reviewed_at`, `reviewed_by`, `notes`, `created_at`, `updated_at`) VALUES
	(9, 12, '2025-12-24', '2026-01-01', 0, 1, '2026-01-29 23:26:23', 10, 'Fin de año', '2025-12-16 22:37:06', '2026-01-29 23:26:23'),
	(10, 11, '2025-12-22', '2025-12-28', 1, 0, '2025-12-16 22:48:05', 10, 'Periodo navideño', '2025-12-16 22:48:00', '2025-12-16 22:48:05'),
	(11, 10, '2026-02-01', '2026-02-08', 0, 1, '2026-01-29 23:26:33', 10, 'Viaje personal', '2026-01-29 23:26:05', '2026-01-29 23:26:33');

/*!40103 SET TIME_ZONE=IFNULL(@OLD_TIME_ZONE, 'system') */;
/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IFNULL(@OLD_FOREIGN_KEY_CHECKS, 1) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40111 SET SQL_NOTES=IFNULL(@OLD_SQL_NOTES, 1) */;
