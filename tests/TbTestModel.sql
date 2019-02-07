/*
Navicat MySQL Data Transfer

Source Server         : www.snlan.top
Source Server Version : 80013
Source Host           : www.snlan.top:3306
Source Database       : block

Target Server Type    : MYSQL
Target Server Version : 80013
File Encoding         : 65001

Date: 2019-02-08 01:26:29
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
INSERT INTO `TbTestModel` VALUES ('0000009b790008004b64fb', '1', '22:00:00', '2019-02-08', '2019-02-06 01:24:38', '1480580000');
