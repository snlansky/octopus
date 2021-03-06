<!-- TOC depthFrom:1 depthTo:6 withLinks:1 updateOnSave:1 orderedList:0 -->

- [运算类型](#1. 运算类型)
- [缓存策略](#2. 缓存策略)
    - [增加](#增加)
    - [删除](#删除)
    - [更新](#更新)
    - [查询](#查询)
    - [事务](#事务)
    - [容量控制](#容量控制)
- [接口](#接口)
    - [HTTP](#http)
        - [增加一条记录](#增加一条记录)
        - [删除记录](#删除记录)
        - [变更记录](#变更记录)
        - [查询记录](#查询记录)
        - [事务](#事务-1)
    - [gRPC](#grpc)
        - [增加一条记录](#增加一条记录-1)
        - [删除记录](#删除记录-1)
        - [变更记录](#变更记录-1)
        - [查询记录](#查询记录-1)
        - [事务](#事务-2)
        - [运行参数](#运行参数)

<!-- /TOC -->

# 运算类型

连接运算符（只能选一种）：AND OR

操作运算符：eq: =, ne: !=, lt: <, lte: <=, gt: >, gte: >=, in: IN, like: LIKE

_ _ _


# 缓存策略

## 增加

直接写入数据库。

## 删除

先删除缓存数据再删除数据库数据。

## 更新

缓存中不存在的数据则载入并更新，已存在的数据直接更新，更新缓存过期时间，协程异步执行SQL更新语句，协程执行过程通过Zookeeper任务队列控制同步。

## 查询

以主键为条件的查询先查缓存，如果缓存数据不存在则从数据库加载，其它查询直接查数据库。

## 事务

插入操作时，创建缓存数据，设置缓存过期时间；更新操作时，缓存中不存在的数据则载入并更新，已存在的数据直接更新，更新缓存过期时间；删除操作时，删除缓存中已存在的数据。
如果缓存事务执行成功， 异步则执行数据库事务，如果数据库事务失败则回滚同时清除缓存相关数据。
协程异步执行SQL数据库事务语句时，协程执行过程通过Zookeeper任务队列控制同步：
1. 创建事务序列节点；
2. 判断当前节点序号是否小于缓冲区大小，小于缓冲区大小则执行步骤4，否则执行步骤3；
3. 监听序列节点变化事件，当序列节点数量变化时执行步骤2；
4. 执行数据库事务，删除当前序列节点。

## 分布式事务

分布式缓存事务按照缓存事务策略分别执行，如果其中一个事务失败则全部回滚，否则逐一执行缓存事务。
分布式数据库事务按照数据库事务策略分别执行，在每个事务提交前确认事务状态，如果其中一个事务执行SQL语句失败则全部回滚，否则所有事务开始逐一提交。

## 容量控制

使用LRU算法（最近最少使用算法）控制缓存容量。
缓存记录设置过期时间自动清理长期未使用数据。

_ _ _

# 接口

## HTTP

### 增加一条记录
```
// url: /追踪标识/数据路由/对象名
// body: {"values": {"字段名": 字段值, ...}}
```

### 删除记录
```
// url: /追踪标识/数据路由/对象名/主键名,...
// query: operator=AND[OR]&字段名__运算符=字段值,...
```

### 变更记录
```
// url: /追踪标识/数据路由/对象名/主键名,...
// body: {"values": {"字段名": 字段值, ...}, "conditions": {"字段名__运算符": 字段值, ..., "operator": "AND[OR]"}}
```

### 查询记录
- 持久化数据查询
```
// url: /追踪标识/数据路由/对象名/_/[/字段名,...]
// query: operator=AND[OR]&limit=上限值&offset=偏移位置&order=字段名__ASC[DESC],...&字段名__运算符=字段值&...
```

- 启用缓存策略查询
```
// url: /追踪标识/数据路由/对象名/主键名,...[/字段名,...]
// query: operator=AND[OR]&limit=上限值&offset=偏移位置&order=字段名__ASC[DESC],...&字段名__运算符=字段值&...
```


### 事务
```
// url: /追踪标识/数据路由
body: [

    {"orm": "对象名", "pk": "主键名,...", "action": "insert", "body": {"values": {"字段名": 字段值, ...}}},

    {"orm": "对象名", "pk": "主键名,...", "action": "delete", "body": {"字段名__运算符": 字段值, ..., "operator": "AND[OR]"}},

    {"orm": "对象名", "pk": "主键名,...", "action": "update", "body": {"values": {"字段名": 字段值, ...}, "conditions": {"字段名__运算符": 字段值, ...,  operator": "AND[OR]"}}},

    ...
]
```


### 分布式事务
```
// url: /追踪标识
body: [

    {"Db": "数据路由", "orm": "对象名", "pk": "主键名,...", "action": "insert", "body": {"values": {"字段名": 字段值, ...}}},

    {"Db": "数据路由", "orm": "对象名", "pk": "主键名,...", "action": "delete", "body": {"字段名__运算符": 字段值, ..., "operator": "AND[OR]"}},

    {"Db": "数据路由", "orm": "对象名", "pk": "主键名,...", "action": "update", "body": {"values": {"字段名": 字段值, ...}, "conditions": {"字段名__运算符": 字段值, ...,  operator": "AND[OR]"}}},

    ...
]
```

## gRPC
```
// 请求消息体：
message OneReq {
    Uri descrip;
    bytes body;
}
descrip : 对象
body    ：json

// 返回消息体：
message Result {
    Uri.PackType pack;
    Uri.ZipType zip;
    bytes content;
}
pack      int           // 打包类型, 默认msgpack打包
zip       int           // 压缩类型, content长度大于1024字节 打包
content   byte[]        // 消息体

descrip: type Uri struct {
    Trace   string   
    Db      string    
    Orm     string       
    Pk      string<br>             //如果有两个主键RoleGuid、TwoKey，则uri.pk="RoleGuid,TwoKey"<br>
    Columns string<br>
}
```

### 增加一条记录:
```
// descrip :Uri对象
// body    : {"values": {"字段名": 字段值, ...}}
```

### 删除记录
```
// descrip :Uri对象
// body    : {operator: AND[OR], 字段名__运算符: 字段值,...}
```

### 变更记录
```
// descrip :Uri对象
// body    : {"values": {"字段名": 字段值, ...}, "conditions": {"字段名__运算符": 字段值, ..., "operator": "AND[OR]"}}
```

### 查询记录
```
// descrip :Uri对象
// body    : {operator:AND[OR], limit:上限值, offset=偏移位置, order:字段名__ASC[DESC],...:字段名__运算符=字段值&...}
// 注： body为空，则全表查询
```

### 事务
```
// descrip :Uri对象
body: [

    {"orm": "对象名", "pk": "主键名,...", "action": "insert", "body": {"values": {"字段名": 字段值, ...}}},

    {"orm": "对象名", "pk": "主键名,...", "action": "delete", "body": {"字段名__运算符": 字段值, ..., "operator": "AND[OR]"}},

    {"orm": "对象名", "pk": "主键名,...", "action": "update", "body": {"values": {"字段名": 字段值, ...}, "conditions": {"字段名__运算符": 字段值, ...,  operator": "AND[OR]"}}},
    ...
]
```