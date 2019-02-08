/*
Navicat MySQL Data Transfer

Source Server         : www.snlan.top
Source Server Version : 80013
Source Host           : www.snlan.top:3306
Source Database       : block

Target Server Type    : MYSQL
Target Server Version : 80013
File Encoding         : 65001

Date: 2019-02-08 12:50:42
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for TbTestModel
-- ----------------------------
DROP TABLE IF EXISTS `TbTestModel`;
CREATE TABLE `TbTestModel` (
  `RoleGuid` varchar(32) NOT NULL,
  `TwoKey` int(11) NOT NULL,
  `CreateTime` varchar(255) DEFAULT NULL,
  `CreateDatetime` date DEFAULT NULL,
  `CreateDate` datetime DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
  `CreateTimestamp` int(11) DEFAULT NULL,
  PRIMARY KEY (`RoleGuid`,`TwoKey`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Records of TbTestModel
-- ----------------------------
INSERT INTO `TbTestModel` VALUES ('0000009b100008004b64fb', '1', '22:00:00', '2019-02-08', '2019-02-06 01:24:38', '1480580000');
INSERT INTO `TbTestModel` VALUES ('0000009b120008004b64fb', '2', '22:00:00', '2019-02-13', '2019-02-13 04:49:50', '1482580000');
INSERT INTO `TbTestModel` VALUES ('0000009b130008004b64fb', '3', '22:00:00', '2019-02-15', '2019-02-24 04:49:56', '1484580000');
