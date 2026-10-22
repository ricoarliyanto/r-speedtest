PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;

DROP TABLE IF EXISTS `seaql_migrations`;
CREATE TABLE `seaql_migrations` (
  `version` TEXT NOT NULL PRIMARY KEY,
  `applied_at` INTEGER NOT NULL
);

INSERT INTO `seaql_migrations` (`version`, `applied_at`) VALUES('m20220101_000001_create_table', 1723226505);
INSERT INTO `seaql_migrations` (`version`, `applied_at`) VALUES('m20220101_000002_create_codegen_config', 1778843787);

DROP TABLE IF EXISTS `sys_api_permission`;
CREATE TABLE `sys_api_permission` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `api` TEXT NOT NULL,
  `method` TEXT NOT NULL,
  `apiname` TEXT NOT NULL,
  `logcache` TEXT NOT NULL DEFAULT '0',
  `remark` TEXT,
  `sort` INTEGER NOT NULL,
  `created_at` TEXT NOT NULL,
  UNIQUE (`api`)
);

INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', '0', '但凡', 1, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', '0', NULL, 3, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', '0', NULL, 3, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', '0', NULL, 4, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', '0', NULL, 5, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', '0', NULL, 6, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', '0', NULL, 7, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', '0', NULL, 8, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', '0', NULL, 9, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', '0', NULL, 10, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', '0', NULL, 11, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', '0', NULL, 12, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', '0', NULL, 13, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', '0', NULL, 14, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', '0', NULL, 15, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', '0', NULL, 16, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', '0', NULL, 17, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', '0', NULL, 18, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', '0', NULL, 19, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', '0', NULL, 20, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', '0', NULL, 21, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', '0', NULL, 22, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', '0', NULL, 23, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', '0', NULL, 24, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', '0', NULL, 25, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', '0', NULL, 26, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', '0', NULL, 27, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', '0', NULL, 28, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', '0', NULL, 29, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', '0', NULL, 30, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', '0', NULL, 31, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', '0', NULL, 32, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', '0', NULL, 33, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', '0', NULL, 34, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', '0', NULL, 35, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', '0', NULL, 36, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', '0', NULL, 37, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752189951611904, '/sys/role/add', 'Post', '添加角色', '0', NULL, 38, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190018720768, '/sys/role/del', 'Delete', '删除角色', '0', NULL, 39, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', '0', NULL, 40, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', '0', NULL, 41, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', '0', NULL, 42, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', '0', NULL, 43, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190333293568, '/sys/user/del', 'Delete', '删除用户', '0', NULL, 44, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', '0', NULL, 45, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190459122688, '/sys/user/add', 'Post', '添加用户', '0', NULL, 46, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', '0', NULL, 47, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', '0', NULL, 48, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', '0', NULL, 49, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190727558144, '/sys/dept/add', 'Post', '添加部门', '0', NULL, 50, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', '0', NULL, 51, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', '0', NULL, 52, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', '0', NULL, 53, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', '0', NULL, 54, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', '0', NULL, 55, '2024-08-23 22:15:00');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', '0', NULL, 25, '2024-08-24 00:11:29');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', '0', NULL, 24, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', '0', NULL, 25, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', '0', NULL, 26, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', '0', NULL, 27, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', '0', NULL, 28, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', '0', NULL, 29, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', '0', NULL, 30, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', '0', NULL, 31, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', '0', NULL, 32, '2024-08-24 01:04:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', '0', NULL, 63, '2024-08-27 00:09:07');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', '0', NULL, 52, '2024-08-28 01:41:39');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', '0', NULL, 47, '2024-08-28 22:40:38');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', '0', NULL, 18, '2024-08-29 01:44:13');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', '0', NULL, 19, '2024-08-29 01:44:13');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', '0', NULL, 12, '2024-08-31 21:04:59');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', '0', NULL, 67, '2024-09-01 00:02:35');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', '0', NULL, 36, '2024-09-04 01:16:35');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', '0', NULL, 38, '2024-09-04 01:16:35');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', '0', NULL, 66, '2024-09-12 23:37:39');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', '0', NULL, 66, '2024-09-17 00:59:09');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', '0', NULL, 14, '2024-10-18 01:16:22');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', '0', NULL, 44, '2024-10-18 01:53:45');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', '0', NULL, 57, '2024-10-18 23:39:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842530210816, '/wechat/wxaccounts/del', 'Delete', '删除微信公众号', '0', NULL, 76, '2025-07-08 20:55:13');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842559570944, '/wechat/wxaccounts/edit', 'Put', '编辑微信公众号', '0', NULL, 77, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842567959552, '/wechat/wxaccounts/list', 'Get', '获取微信公众号列表', '0', NULL, 78, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842572153856, '/wechat/wxaccounts/add', 'Post', '添加微信公众号', '0', NULL, 79, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842584736768, '/wechat/wxautoreplies/add', 'Post', '添加自动回复', '0', NULL, 80, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842609902592, '/wechat/wxautoreplies/del', 'Delete', '删除自动回复', '0', NULL, 81, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333842635068416, '/wechat/wxautoreplies/list', 'Get', '获取自动回复列表', '0', NULL, 82, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843008361472, '/wechat/wxautoreplies/edit', 'Put', '编辑自动回复', '0', NULL, 83, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843016750080, '/wechat/wxmenus/list', 'Get', '获取微信菜单列表', '0', NULL, 84, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843071276032, '/wechat/wxmenus/del', 'Delete', '删除微信菜单', '0', NULL, 85, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843075470336, '/wechat/wxmenus/pull_menu', 'Post', '发布菜单到微信', '0', NULL, 86, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843092247552, '/wechat/wxmenus/add', 'Post', '添加微信菜单', '0', NULL, 87, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843100636160, '/wechat/wxmenus/edit', 'Put', '编辑微信菜单', '0', NULL, 88, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843104830464, '/wechat/wxmessages/list', 'Get', '获取微信消息列表', '0', NULL, 89, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843125801984, '/wechat/wxmessages/edit', 'Put', '编辑微信消息', '0', NULL, 90, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843138384896, '/wechat/wxmessages/del', 'Delete', '删除微信消息', '0', NULL, 91, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843150967808, '/wechat/wxmessages/add', 'Post', '添加微信消息', '0', NULL, 92, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843159356416, '/wechat/wxusers/add', 'Post', '添加微信用户', '0', NULL, 93, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843167745024, '/wechat/wxusers/edit', 'Put', '编辑微信用户', '0', NULL, 94, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843180327936, '/wechat/wxusers/del', 'Delete', '删除微信用户', '0', NULL, 95, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7348333843184522240, '/wechat/wxusers/list', 'Get', '获取微信用户列表', '0', NULL, 96, '2025-07-08 20:55:14');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7385368752226735104, '/test/test_api/list', 'Get', '获取列表', '0', NULL, 75, '2025-10-19 01:38:44');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7385368752235123712, '/test/test_api/del', 'Delete', '删除TestApi', '0', NULL, 76, '2025-10-19 01:38:44');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7385368752239318016, '/test/test_api/add', 'Post', '添加TestApi', '0', NULL, 77, '2025-10-19 01:38:44');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7385368752243512320, '/test/test_api/edit', 'Put', '编辑TestApi', '0', NULL, 78, '2025-10-19 01:38:44');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7401293722618336256, '/test/test_api/db_read_write_test', 'Put', 'db_read_write_test', '0', NULL, 76, '2025-12-02 00:18:53');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7401293722630919168, '/test/test_api/db_name_test', 'Put', 'db_name_test', '0', NULL, 77, '2025-12-02 00:18:53');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7401293722647696384, '/test/test_api/db_index_test', 'Put', 'db_index_test', '0', NULL, 81, '2025-12-02 00:18:53');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7401293722651890688, '/test/test_api/db_auto_test', 'Put', 'db_auto_test', '0', NULL, 82, '2025-12-02 00:18:53');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7402694868939478016, '/test/test_data_scope/del', 'Delete', 'Delete', '0', NULL, 83, '2025-12-05 21:06:32');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7402694868947866624, '/test/test_data_scope/list', 'Get', 'list', '0', NULL, 84, '2025-12-05 21:06:32');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7402694868947866625, '/test/test_data_scope/add', 'Post', 'add', '0', NULL, 85, '2025-12-05 21:06:32');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7402694868952060928, '/test/test_data_scope/edit', 'Put', 'edit', '0', NULL, 86, '2025-12-05 21:06:32');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461047623965643776, '/sys/codegen/scan', 'Get', '扫描所有Entity', '0', NULL, 62, '2026-05-15 21:39:33');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461047623990809600, '/sys/codegen/generate', 'Post', '生成代码', '0', NULL, 63, '2026-05-15 21:39:33');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461309194964997120, '/sys/codegen/load_config', 'Get', '加载字段配置', '0', NULL, 37, '2026-05-16 14:58:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461309194973385728, '/sys/codegen/save_config', 'Post', '保存字段配置', '0', NULL, 38, '2026-05-16 14:58:57');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461472538283906048, '/test/test_article/del', 'Delete', '删除TestArticle', '0', NULL, 9, '2026-05-17 01:48:01');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461472538304877568, '/test/test_article/edit', 'Put', '编辑TestArticle', '0', NULL, 10, '2026-05-17 01:48:01');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461472538309071872, '/test/test_article/add', 'Post', '添加TestArticle', '0', NULL, 11, '2026-05-17 01:48:01');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461472538313266176, '/test/test_article/list', 'Get', '获取TestArticle列表', '0', NULL, 12, '2026-05-17 01:48:01');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461476362570601472, '/test/test_category/list', 'Get', '获取TestCategory列表', '0', NULL, 91, '2026-05-17 02:03:12');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461476362583184384, '/test/test_category/del', 'Delete', '删除TestCategory', '0', NULL, 92, '2026-05-17 02:03:12');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461476362591572992, '/test/test_category/edit', 'Put', '编辑TestCategory', '0', NULL, 93, '2026-05-17 02:03:12');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7461476362595767296, '/test/test_category/add', 'Post', '添加TestCategory', '0', NULL, 94, '2026-05-17 02:03:12');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7462535578852955136, '/sys/upload', 'Post', '上传文件', '0', NULL, 79, '2026-05-20 00:12:09');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200110801920, '/game/purchase/list', 'Get', '内购列表', '0', NULL, 1, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200127579136, '/game/purchase/validate', 'Post', '验证内购', '0', NULL, 2, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200135967744, '/game/purchase/confirm', 'Post', '确认内购', '0', NULL, 3, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200140162048, '/game/purchase/refund', 'Post', '退款', '0', NULL, 4, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200144356352, '/game/account/device_login', 'Post', '设备ID登录', '0', NULL, 5, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200148550656, '/game/account/list', 'Get', '账号列表', '0', NULL, 6, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200152744960, '/game/account/email_login', 'Post', '邮箱登录', '0', NULL, 7, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200156939264, '/game/account/username_register', 'Post', '用户名注册', '0', NULL, 8, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200161133568, '/game/account/email_register', 'Post', '邮箱注册', '0', NULL, 9, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200169522176, '/game/account/get', 'Get', '获取账号信息', '0', NULL, 10, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200173716480, '/game/account/update', 'Put', '更新账号资料', '0', NULL, 11, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200177910784, '/game/account/link_device', 'Post', '关联设备ID', '0', NULL, 12, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200186299392, '/game/account/link_custom_id', 'Post', '关联自定义ID', '0', NULL, 13, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953200190493696, '/game/account/username_login', 'Post', '用户名登录', '0', NULL, 14, '2026-05-23 22:05:16');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201851438080, '/game/chat/join_channel', 'Post', '加入频道', '0', NULL, 15, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201855632384, '/game/chat/send_direct', 'Post', '发送私聊消息', '0', NULL, 16, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201859826688, '/game/chat/messages', 'Get', '消息列表', '0', NULL, 17, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201864020992, '/game/chat/leave_channel', 'Post', '离开频道', '0', NULL, 18, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201868215296, '/game/chat/send', 'Post', '发送频道消息', '0', NULL, 19, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201868215297, '/game/chat/create_channel', 'Post', '创建频道', '0', NULL, 20, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201872409600, '/game/group/kick', 'Post', '踢出成员', '0', NULL, 21, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201876603904, '/game/group/list', 'Get', '群组列表', '0', NULL, 22, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201880798208, '/game/group/update', 'Put', '更新群组', '0', NULL, 23, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201884992512, '/game/group/leave', 'Post', '离开群组', '0', NULL, 24, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201889186816, '/game/group/members', 'Get', '群组成员列表', '0', NULL, 25, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953201893381120, '/game/group/create', 'Post', '创建群组', '0', NULL, 26, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202581246976, '/game/group/join', 'Post', '加入群组', '0', NULL, 27, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202585441280, '/game/matchmaker/create_ticket', 'Post', '创建匹配票', '0', NULL, 28, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202589635584, '/game/matchmaker/cancel_ticket', 'Post', '取消匹配', '0', NULL, 29, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202593829888, '/game/matchmaker/get_ticket', 'Get', '查询匹配状态', '0', NULL, 30, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202598024192, '/game/storage/write', 'Post', '写入存储对象', '0', NULL, 31, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202602218496, '/game/storage/delete', 'Delete', '删除存储对象', '0', NULL, 32, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202606412800, '/game/storage/list', 'Get', '存储对象列表', '0', NULL, 33, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202610607104, '/game/storage/read', 'Get', '读取存储对象', '0', NULL, 34, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202614801408, '/game/leaderboard/list', 'Get', '排行榜列表', '0', NULL, 35, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202618995712, '/game/leaderboard/delete', 'Delete', '删除排行榜', '0', NULL, 36, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202623190016, '/game/leaderboard/create', 'Post', '创建排行榜', '0', NULL, 37, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202627384320, '/game/leaderboard/submit_score', 'Post', '提交分数', '0', NULL, 38, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202631578624, '/game/leaderboard/records', 'Get', '获取排行榜记录', '0', NULL, 39, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202635772928, '/game/friend/add', 'Post', '添加好友', '0', NULL, 40, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202639967232, '/game/friend/block', 'Post', '屏蔽用户', '0', NULL, 41, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202644161536, '/game/friend/list', 'Get', '好友列表', '0', NULL, 42, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202648355840, '/game/friend/accept', 'Post', '接受好友请求', '0', NULL, 43, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202652550144, '/game/friend/remove', 'Delete', '删除好友', '0', NULL, 44, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202656744448, '/game/social/login', 'Post', '社交登录', '0', NULL, 45, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202660938752, '/game/social/link', 'Post', '关联社交账号', '0', NULL, 46, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202660938753, '/game/social/list', 'Get', '社交关联列表', '0', NULL, 47, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202665133056, '/game/social/unlink', 'Post', '解除社交关联', '0', NULL, 48, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202669327360, '/game/notification/unread_count', 'Get', '未读数量', '0', NULL, 49, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202677715968, '/game/notification/delete', 'Delete', '删除通知', '0', NULL, 50, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202677715969, '/game/notification/list', 'Get', '通知列表', '0', NULL, 51, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202681910272, '/game/notification/mark_read', 'Post', '标记已读', '0', NULL, 52, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202686104576, '/game/notification/send', 'Post', '发送通知', '0', NULL, 53, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202690298880, '/game/match/list', 'Get', '对战列表', '0', NULL, 54, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202694493184, '/game/match/get', 'Get', '对战详情', '0', NULL, 55, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953202698687488, '/game/match/players', 'Get', '对战成员列表', '0', NULL, 56, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203009065984, '/game/match/delete', 'Delete', '删除对战', '0', NULL, 57, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203013260288, '/game/match/join', 'Post', '加入对战', '0', NULL, 58, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203017454592, '/game/match/create', 'Post', '创建对战', '0', NULL, 59, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203021648896, '/game/match/leave', 'Post', '离开对战', '0', NULL, 60, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203051009024, '/game/wallet/balance', 'Get', '获取余额', '0', NULL, 61, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203055203328, '/game/wallet/ledger', 'Post', '钱包变动', '0', NULL, 62, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203059397632, '/game/wallet/list_ledger', 'Get', '流水列表', '0', NULL, 63, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203063591936, '/game/party/my_party', 'Get', '我的队伍', '0', NULL, 64, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203067786240, '/game/party/create', 'Post', '创建组队', '0', NULL, 65, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203071980544, '/game/party/transfer_leader', 'Post', '转让队长', '0', NULL, 66, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203076174848, '/game/party/kick', 'Post', '踢出成员', '0', NULL, 67, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203080369152, '/game/party/leave', 'Post', '离开组队', '0', NULL, 68, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203084563456, '/game/party/join', 'Post', '加入组队', '0', NULL, 69, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203088757760, '/game/party/get', 'Get', '获取组队信息', '0', NULL, 70, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203092952064, '/game/tournament/list', 'Get', '锦标赛列表', '0', NULL, 71, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203097146368, '/game/tournament/join', 'Post', '加入锦标赛', '0', NULL, 72, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203101340672, '/game/tournament/delete', 'Delete', '删除锦标赛', '0', NULL, 73, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203101340673, '/game/tournament/submit_score', 'Post', '提交锦标赛分数', '0', NULL, 74, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203105534976, '/game/tournament/leave', 'Post', '离开锦标赛', '0', NULL, 75, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203109729280, '/game/tournament/update_status', 'Post', '更新锦标赛状态', '0', NULL, 76, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203113923584, '/game/tournament/participants', 'Get', '锦标赛参与者列表', '0', NULL, 77, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203118117888, '/game/tournament/create', 'Post', '创建锦标赛', '0', NULL, 78, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203122312192, '/game/presence/get', 'Get', '获取在线状态', '0', NULL, 79, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203126506496, '/game/presence/update', 'Post', '更新在线状态', '0', NULL, 80, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203130700800, '/game/script/get', 'Get', '脚本详情', '0', NULL, 81, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203134895104, '/game/script/create', 'Post', '创建脚本', '0', NULL, 82, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203139089408, '/game/script/execute', 'Post', '执行脚本', '0', NULL, 83, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203143283712, '/game/script/list', 'Get', '脚本列表', '0', NULL, 84, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203147478016, '/game/script/update', 'Put', '更新脚本', '0', NULL, 85, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203151672320, '/game/script/publish', 'Post', '发布脚本', '0', NULL, 86, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203185226752, '/game/script/logs', 'Get', '脚本日志', '0', NULL, 87, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7463953203189421056, '/game/script/delete', 'Delete', '删除脚本', '0', NULL, 88, '2026-05-23 22:05:17');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059899028480, '/wechat/wxpay/notify/{account_id}', 'Post', '支付回调通知', '0', NULL, 29, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059911611392, '/wechat/wxpay/query_order', 'Post', '查询订单状态', '0', NULL, 30, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059915805696, '/wechat/wxpay/refunds', 'Get', '获取退款列表', '0', NULL, 31, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059924194304, '/wechat/wxpay/close_order', 'Post', '关闭订单', '0', NULL, 32, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059928388608, '/wechat/wxpay/create_order', 'Post', '创建支付订单', '0', NULL, 33, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059932582912, '/wechat/wxpay/refund', 'Post', '申请退款', '0', NULL, 34, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059940971520, '/wechat/wxpay/orders', 'Get', '获取支付订单列表', '0', NULL, 35, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059945165824, '/wechat/wxmessages/conversation', 'Get', '获取会话消息列表', '0', NULL, 36, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059961943040, '/wechat/wxmessages/reply', 'Post', '手动回复消息', '0', NULL, 39, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279059999691776, '/wechat/wxmenus/sync_menu', 'Post', '从微信同步菜单到本地', '0', NULL, 45, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060016468992, '/wechat/wxmenus/delete_remote_menu', 'Post', '删除微信服务器上的自定义菜单', '0', NULL, 48, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060029051904, '/wechat/official_account/callback', 'Post', '微信公众号消息回调', '0', NULL, 49, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060037440512, '/wechat/official_account/verify', 'Get', '微信公众号验证', '0', NULL, 50, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060041634816, '/wechat/wxtemplates/send', 'Post', '发送模板消息', '0', NULL, 51, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060045829120, '/wechat/wxtemplates/del', 'Delete', '删除微信模板', '0', NULL, 52, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060050023424, '/wechat/wxtemplates/list', 'Get', '获取微信模板列表', '0', NULL, 53, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060058412032, '/wechat/wxtemplates/sync', 'Post', '从微信服务器同步模板', '0', NULL, 54, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060062606336, '/wechat/wxtemplates/add', 'Post', '添加微信模板', '0', NULL, 55, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060066800640, '/wechat/wxtemplates/logs', 'Get', '获取模板发送日志', '0', NULL, 56, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060075189248, '/wechat/wxtemplates/edit', 'Put', '编辑微信模板', '0', NULL, 57, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060100355072, '/sys/mailer/config', 'Get', '获取邮件配置', '0', NULL, 62, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060104549376, '/sys/mailer/send', 'Post', '发送邮件', '0', NULL, 63, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060112937984, '/sys/mailer/log_list', 'Get', '获取邮件日志列表', '0', NULL, 64, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467279060117132288, '/sys/mailer/send_template', 'Post', '发送模板邮件', '0', NULL, 65, '2026-06-02 02:21:03');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467293259341927424, '/wechat/wxmessages/stream', 'Get', 'SSE消息推送', '0', NULL, 5, '2026-06-02 03:17:29');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282072941568, '/wechat/wxmaterials/sync_materials', 'Post', '从微信同步素材到本地', '0', NULL, 136, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282081330176, '/wechat/wxmaterials/upload_news', 'Post', '上传永久图文素材到微信', '0', NULL, 137, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282085524480, '/wechat/wxmaterials/list', 'Get', '获取微信素材列表', '0', NULL, 138, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282089718784, '/wechat/wxmaterials/edit', 'Put', '编辑微信素材', '0', NULL, 139, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282093913088, '/wechat/wxmaterials/del', 'Delete', '删除微信素材', '0', NULL, 140, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282098107392, '/wechat/wxmaterials/upload_temp_media', 'Post', '上传临时素材到微信', '0', NULL, 141, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282102301696, '/wechat/wxmaterials/upload_permanent_media', 'Post', '上传永久素材到微信', '0', NULL, 142, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282106496000, '/wechat/wxmaterials/material_count', 'Post', '获取微信素材计数', '0', NULL, 143, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282110690304, '/wechat/wxmaterials/delete_remote_media', 'Post', '删除微信服务器上的永久素材', '0', NULL, 144, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282114884608, '/wechat/wxmaterials/upload_file', 'Post', '上传文件到服务器本地', '0', NULL, 145, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467489282119078912, '/wechat/wxmaterials/add', 'Post', '添加微信素材', '0', NULL, 146, '2026-06-02 16:16:24');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467596925076673536, '/sys/mailer/template/edit', 'Put', '编辑邮件模板', '0', NULL, 91, '2026-06-02 23:24:08');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467596925085062144, '/sys/mailer/template/del/{id}', 'Delete', '删除邮件模板', '0', NULL, 92, '2026-06-02 23:24:08');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467596925089256448, '/sys/mailer/template/list', 'Get', '获取邮件模板列表', '0', NULL, 93, '2026-06-02 23:24:08');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467596925093450752, '/sys/mailer/template/add', 'Post', '新增邮件模板', '0', NULL, 94, '2026-06-02 23:24:08');
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES(7467601014565016576, '/sys/mailer/template/del', 'Delete', '删除邮件模板', '0', NULL, 6, '2026-06-02 23:40:23');

DROP TABLE IF EXISTS `sys_codegen_config`;
CREATE TABLE `sys_codegen_config` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `module_name` TEXT NOT NULL,
  `table_name` TEXT NOT NULL,
  `config_json` TEXT NOT NULL,
  `created_at` TEXT NOT NULL,
  `updated_at` TEXT NOT NULL,
  UNIQUE (`module_name`, `table_name`)
);

INSERT INTO `sys_codegen_config` (`id`, `module_name`, `table_name`, `config_json`, `created_at`, `updated_at`) VALUES(7461365840546272256, 'test', 'test_api', '[{"field_name":"id","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":null,"show_in_detail":true,"required":true,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"id","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":"InputNumber","show_in_detail":true,"required":null,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"name","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":null,"show_in_detail":true,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"age","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"Textarea","show_in_detail":true,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"email","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":null,"show_in_detail":true,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"created_at","show_in_list":false,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null},{"field_name":"updated_at","show_in_list":false,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null}]', '2026-05-16 18:44:02', '2026-05-16 19:02:49');
INSERT INTO `sys_codegen_config` (`id`, `module_name`, `table_name`, `config_json`, `created_at`, `updated_at`) VALUES(7461390264666330112, 'test', 'test_category', '[{"field_name":"id","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":true,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"name","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"description","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":"InputTextarea","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"sort","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"InputNumber","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":null,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"status","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":"Select","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":null,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"cover","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"Upload","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":null,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"is_active","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"Switch","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"weight","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"InputNumber","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"view_count","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"created_at","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"updated_at","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"sortable":true,"sort_priority":1,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null}]', '2026-05-16 20:21:05', '2026-05-20 03:13:52');
INSERT INTO `sys_codegen_config` (`id`, `module_name`, `table_name`, `config_json`, `created_at`, `updated_at`) VALUES(7461403558273782784, 'test', 'test_article', '[{"field_name":"id","show_in_list":true,"show_in_search":false,"show_in_form":false,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":true,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"category_id","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":"Select","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":"TestCategory","relation_display_field":null},{"field_name":"title","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"content","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":"InputTextarea","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"author","show_in_list":true,"show_in_search":true,"show_in_form":true,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"password","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"InputPassword","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"is_published","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"Switch","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"view_count","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":null,"show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"download_count","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"InputNumber","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"rating","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"InputNumber","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"cover","show_in_list":true,"show_in_search":false,"show_in_form":true,"component":"Upload","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"published_at","show_in_list":false,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"sortable":null,"sort_priority":null,"sort_order":null,"label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"created_at","show_in_list":false,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"sortable":true,"sort_priority":2,"sort_order":"asc","label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null},{"field_name":"updated_at","show_in_list":false,"show_in_search":false,"show_in_form":false,"component":"DatePicker","show_in_detail":true,"sortable":true,"sort_priority":4,"sort_order":"asc","label":null,"required":false,"min_length":null,"max_length":null,"pattern":null,"pattern_message":null,"relation_target":null,"relation_display_field":null}]', '2026-05-16 21:13:54', '2026-05-20 01:38:16');

DROP TABLE IF EXISTS `sys_dept`;
CREATE TABLE `sys_dept` (
  `dept_id` INTEGER NOT NULL PRIMARY KEY,
  `parent_id` INTEGER NOT NULL DEFAULT 0,
  `dept_name` TEXT DEFAULT '',
  `lft` INTEGER NOT NULL DEFAULT 0,
  `rgt` INTEGER NOT NULL DEFAULT 0,
  `depth` INTEGER NOT NULL DEFAULT 0,
  `dept_category` TEXT,
  `order` INTEGER NOT NULL DEFAULT 0,
  `leader` INTEGER,
  `phone` TEXT,
  `email` TEXT,
  `status` TEXT NOT NULL DEFAULT '0',
  `remark` TEXT,
  `created_at` TEXT,
  `updated_at` TEXT,
  `deleted_at` TEXT
);

INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(100, 0, '科技', 1, 26, 1, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(101, 100, '深圳总公司', 2, 13, 2, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(102, 100, '长沙分公司', 14, 21, 2, NULL, 2, NULL, '187554971', 'xxx@qq.com', '0', '长沙的公司', '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(103, 101, '研发部门', 3, 4, 3, NULL, 1, 1, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(104, 101, '市场部门', 5, 6, 3, NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(105, 101, '测试部门', 7, 8, 3, NULL, 3, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(106, 101, '财务部门', 9, 10, 3, NULL, 4, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(107, 101, '运维部门', 11, 12, 3, NULL, 5, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(108, 102, '市场部门', 15, 16, 3, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(109, 102, '财务部门', 17, 18, 3, NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(7223731313261547520, 102, '运维部门', 19, 20, 3, NULL, 3, NULL, NULL, NULL, '0', '长沙运维部门', NULL, NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(7223731922878468096, 100, '广州分公司', 22, 25, 2, NULL, 5, NULL, NULL, NULL, '0', NULL, NULL, NULL, NULL);
INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(7223732104475054080, 7223731922878468096, '小部门', 23, 24, 3, NULL, 3, NULL, NULL, NULL, '0', NULL, NULL, NULL, NULL);

DROP TABLE IF EXISTS `sys_dict_data`;
CREATE TABLE `sys_dict_data` (
  `dict_code` INTEGER NOT NULL PRIMARY KEY,
  `dict_sort` INTEGER NOT NULL DEFAULT 0,
  `dict_label` TEXT NOT NULL DEFAULT '',
  `dict_value` TEXT NOT NULL DEFAULT '',
  `dict_type_id` INTEGER NOT NULL,
  `created_at` TEXT,
  `updated_at` TEXT,
  `remark` TEXT
);

INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(1, 1, '男', '0', 1, '2024-07-03 01:17:32', NULL, '性别男');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(2, 2, '女', '1', 1, '2024-07-03 01:17:32', NULL, '性别女');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(3, 3, '未知', '2', 1, '2024-07-03 01:17:32', NULL, '性别未知');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(4, 1, '显示', '0', 2, '2024-07-03 01:17:32', NULL, '显示菜单a');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(5, 2, '隐藏', '1', 2, '2024-07-03 01:17:32', NULL, '隐藏菜单');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(6, 1, '正常', '0', 3, '2024-07-03 01:17:33', NULL, '正常状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7, 2, '停用', '1', 3, '2024-07-03 01:17:33', NULL, '停用状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(12, 3, '是', 'Y', 6, '2024-07-03 01:17:33', NULL, '系统默认是');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(13, 4, '否', 'N', 6, '2024-07-03 01:17:33', NULL, '系统默认否');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(14, 1, '通知', '1', 7, '2024-07-03 01:17:33', NULL, '通知');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(15, 2, '公告', '2', 7, '2024-07-03 01:17:33', NULL, '公告');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(16, 1, '正常', '0', 8, '2024-07-03 01:17:33', NULL, '正常状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(17, 2, '关闭', '1', 8, '2024-07-03 01:17:33', NULL, '关闭状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(18, 1, '新增', '1', 9, '2024-07-03 01:17:33', NULL, '新增操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(19, 2, '修改', '2', 9, '2024-07-03 01:17:33', NULL, '修改操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(20, 3, '删除', '3', 9, '2024-07-03 01:17:33', NULL, '删除操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(21, 4, '授权', '4', 9, '2024-07-03 01:17:33', NULL, '授权操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(22, 5, '导出', '5', 9, '2024-07-03 01:17:33', NULL, '导出操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(23, 6, '导入', '6', 9, '2024-07-03 01:17:33', NULL, '导入操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(24, 7, '强退', '7', 9, '2024-07-03 01:17:33', NULL, '强退操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(25, 8, '生成代码', '8', 9, '2024-07-03 01:17:33', NULL, '生成操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(26, 9, '清空数据', '9', 9, '2024-07-03 01:17:33', NULL, '清空操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(27, 1, '成功', '0', 10, '2024-07-03 01:17:33', NULL, '正常状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(28, 2, '失败', '1', 10, '2024-07-03 01:17:33', NULL, '停用状态');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(29, 99, '其他', '0', 9, '2024-07-03 01:17:33', NULL, '其他操作');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(30, 0, '密码认证', 'password', 11, '2024-07-03 01:17:33', NULL, '密码认证');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(31, 0, '短信认证', 'sms', 11, '2024-07-03 01:17:33', NULL, '短信认证');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(32, 0, '邮件认证', 'email', 11, '2024-07-03 01:17:33', NULL, '邮件认证');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(33, 0, '小程序认证', 'xcx', 11, '2024-07-03 01:17:33', NULL, '小程序认证');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(34, 0, '三方登录认证', 'social', 11, '2024-07-03 01:17:33', NULL, '三方登录认证');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(35, 0, 'PC', 'pc', 12, '2024-07-03 01:17:33', NULL, 'PC');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(36, 0, '安卓', 'android', 12, '2024-07-03 01:17:33', NULL, '安卓');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(37, 0, 'iOS', 'ios', 12, '2024-07-03 01:17:33', NULL, 'iOS');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(38, 0, '小程序', 'xcx', 12, '2024-07-03 01:17:33', NULL, '小程序');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7229170738212769792, 1, '请求网址', 'geturl', 7229170480917385217, NULL, NULL, 'GET请求网址');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7229171315483217920, 3, '发送邮件', 'sendemail', 7229170480917385217, NULL, NULL, '发送邮件');
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7230797167031619584, 1, '默认', 'default', 7230795611045498880, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7230797303799484416, 3, '系统', 'system', 7230795611045498880, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7233442254918947840, 1, '不记录', '0', 7233442155891430400, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7233442310988403712, 3, '数据库', '2', 7233442155891430400, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7233442350909789184, 2, '文件记录', '1', 7233442155891430400, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7233442400423547904, 5, '数据库和文件', '3', 7233442155891430400, NULL, NULL, NULL);
INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES(7235988025748919296, 3, '调用方法', 'invokefunction', 7229170480917385217, NULL, NULL, '调用方法');

DROP TABLE IF EXISTS `sys_dict_type`;
CREATE TABLE `sys_dict_type` (
  `dict_id` INTEGER NOT NULL PRIMARY KEY,
  `dict_name` TEXT NOT NULL,
  `dict_type` TEXT NOT NULL,
  `order` INTEGER NOT NULL,
  `created_at` TEXT NOT NULL,
  `updated_at` TEXT NOT NULL,
  `remark` TEXT,
  UNIQUE (`dict_type`)
);

INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(1, '用户性别', 'sys_user_sex', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '用户性别列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(2, '菜单状态', 'sys_show_hide', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '菜单状态列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(3, '系统开关', 'sys_normal_disable', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '系统开关列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(6, '系统是否', 'sys_yes_no', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '系统是否列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(7, '通知类型', 'sys_notice_type', 103, '2024-07-03 01:17:32', '2024-08-14 01:06:32', '通知类型列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(8, '通知状态', 'sys_notice_status', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '通知状态列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(9, '操作类型', 'sys_oper_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '操作类型列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(10, '系统状态', 'sys_common_status', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '登录状态列表');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(11, '授权类型', 'sys_grant_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '认证授权类型');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(12, '设备类型', 'sys_device_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '客户端设备类型');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(7229170480917385217, '定时任务', 'sys_job', 1, '2024-08-14 01:02:34', '2024-08-14 01:02:34', '定时任务类型');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(7230795611045498880, '任务分组', 'sys_job_group', 3, '2024-08-18 12:40:15', '2024-08-18 12:40:15', '定时任务分组');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(7233442155891430400, '日志记录', 'sys_logcache', 2, '2024-08-25 19:56:41', '2024-08-25 20:46:41', '日志记录');
INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES(7235877375018177536, '数据权限', 'sys_data_scope', 4, '2024-09-01 13:13:22', '2024-09-01 13:13:22', '数据权限');

DROP TABLE IF EXISTS `sys_job`;
CREATE TABLE `sys_job` (
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `job_id` INTEGER NOT NULL PRIMARY KEY,
  `task_type` TEXT NOT NULL,
  `task_count` INTEGER NOT NULL,
  `run_count` INTEGER NOT NULL,
  `job_name` TEXT NOT NULL,
  `job_params` TEXT,
  `job_group` TEXT NOT NULL,
  `cron_expression` TEXT NOT NULL,
  `status` TEXT NOT NULL,
  `remark` TEXT NOT NULL,
  UNIQUE (`job_id`)
);

INSERT INTO `sys_job` (`created_at`, `updated_at`, `job_id`, `task_type`, `task_count`, `run_count`, `job_name`, `job_params`, `job_group`, `cron_expression`, `status`, `remark`) VALUES('2024-08-16 00:34:21', '2024-08-29 22:01:40', 7227378358836924416, 'geturl', 0, 1737, '请求百度', 'https://www.baidu.com/', 'default', '30 * * * * ?', '1', '66');
INSERT INTO `sys_job` (`created_at`, `updated_at`, `job_id`, `task_type`, `task_count`, `run_count`, `job_name`, `job_params`, `job_group`, `cron_expression`, `status`, `remark`) VALUES('2024-08-16 00:34:21', '2024-08-29 22:01:44', 7227378358836924417, 'geturl', 0, 3479, '请求QQ', 'https://www.qq.com/', 'default', '0/30 * * * * ?', '1', '99');
INSERT INTO `sys_job` (`created_at`, `updated_at`, `job_id`, `task_type`, `task_count`, `run_count`, `job_name`, `job_params`, `job_group`, `cron_expression`, `status`, `remark`) VALUES('2024-09-01 12:35:38', '2024-09-01 22:24:37', 7235988676738454528, 'invokefunction', 0, 13, '清理过期的Token', '{"callfun":"clearuserinfo"}', 'default', '* * 2 * * ?', '1', '晚上两点执行清理过期的Token');

DROP TABLE IF EXISTS `sys_job_log`;
CREATE TABLE `sys_job_log` (
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `id` INTEGER NOT NULL PRIMARY KEY,
  `job_id` INTEGER NOT NULL,
  `run_count` INTEGER NOT NULL,
  `job_message` TEXT,
  `status` TEXT NOT NULL,
  `elapsed_time` TEXT DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (`id`)
);

DROP TABLE IF EXISTS `sys_login_info`;
CREATE TABLE `sys_login_info` (
  `info_id` INTEGER NOT NULL PRIMARY KEY,
  `uid` INTEGER NOT NULL,
  `user_name` TEXT NOT NULL DEFAULT '',
  `device_type` TEXT DEFAULT '',
  `ipaddr` TEXT DEFAULT '',
  `login_location` TEXT DEFAULT '',
  `net_work` TEXT,
  `browser` TEXT DEFAULT '',
  `os` TEXT DEFAULT '',
  `status` TEXT DEFAULT '0',
  `msg` TEXT DEFAULT '',
  `login_time` TEXT
);

CREATE INDEX `idx_sys_login_info_idx_sys_logininfor_lt` ON `sys_login_info` (`login_time`);

CREATE INDEX `idx_sys_login_info_idx_sys_logininfor_s` ON `sys_login_info` (`status`);

DROP TABLE IF EXISTS `sys_mail_log`;
CREATE TABLE `sys_mail_log` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `recipient` TEXT NOT NULL,
  `subject` TEXT NOT NULL,
  `content_text` TEXT,
  `content_html` TEXT,
  `from_addr` TEXT,
  `reply_to` TEXT,
  `status` TEXT NOT NULL,
  `error_message` TEXT,
  `mail_type` TEXT,
  `created_by` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT
);

INSERT INTO `sys_mail_log` (`id`, `recipient`, `subject`, `content_text`, `content_html`, `from_addr`, `reply_to`, `status`, `error_message`, `mail_type`, `created_by`, `created_at`, `updated_at`) VALUES(7467793347218674688, '761907552@qq.com', 'boy', '你好 boy', '你好 boy', '你好 boy', '948547440@qq.com', 'pending', NULL, 'plain', 1, '2026-06-03 04:24:38', NULL);
INSERT INTO `sys_mail_log` (`id`, `recipient`, `subject`, `content_text`, `content_html`, `from_addr`, `reply_to`, `status`, `error_message`, `mail_type`, `created_by`, `created_at`, `updated_at`) VALUES(7467793672268846080, '761907552@qq.com', 'boy', 'boy d', 'boydd', NULL, '948547440@qq.com', 'success', NULL, 'plain', 1, '2026-06-03 04:25:56', NULL);
INSERT INTO `sys_mail_log` (`id`, `recipient`, `subject`, `content_text`, `content_html`, `from_addr`, `reply_to`, `status`, `error_message`, `mail_type`, `created_by`, `created_at`, `updated_at`) VALUES(7467796447153263616, '761907552@qq.com', 'boy', 'boy d', 'boydd', NULL, '948547440@qq.com', 'failed', 'no from sender configured', 'plain', 1, '2026-06-03 04:36:57', NULL);
INSERT INTO `sys_mail_log` (`id`, `recipient`, `subject`, `content_text`, `content_html`, `from_addr`, `reply_to`, `status`, `error_message`, `mail_type`, `created_by`, `created_at`, `updated_at`) VALUES(7467797633671533568, '761907552@qq.com', 'boy', 'boy d', 'boydd', NULL, '948547440@qq.com', 'success', NULL, 'plain', 1, '2026-06-03 04:41:40', NULL);
INSERT INTO `sys_mail_log` (`id`, `recipient`, `subject`, `content_text`, `content_html`, `from_addr`, `reply_to`, `status`, `error_message`, `mail_type`, `created_by`, `created_at`, `updated_at`) VALUES(7467808818831725568, '761907552@qq.com', '你是谁啊', '你是谁啊', '你是谁啊', '激活ing', NULL, 'success', NULL, 'plain', 1, '2026-06-03 05:26:07', NULL);

DROP TABLE IF EXISTS `sys_mail_template`;
CREATE TABLE `sys_mail_template` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT NOT NULL,
  `code` TEXT NOT NULL,
  `subject` TEXT NOT NULL,
  `text_content` TEXT,
  `html_content` TEXT,
  `description` TEXT,
  `status` INTEGER NOT NULL DEFAULT 1,
  `created_at` TEXT DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (`code`)
);

INSERT INTO `sys_mail_template` (`id`, `name`, `code`, `subject`, `text_content`, `html_content`, `description`, `status`, `created_at`, `updated_at`) VALUES(7468353094338647040, '注册验证', '001', '自助验证 {{name}}', '欢迎 {{name}}, 您现在可以登录了。
  使用以下链接验证您的帐户：

{{host}}/api/sys/verify/{{verifyToken}}', '<html>

<body>
  Dear {{name}},
  欢迎使用Rust祺洛系统! 您现在可以登录到您的帐户。
  开始之前，请单击下面的链接验证您的帐户:
  <a href="{{host}}/api/sys/verify/{{verifyToken}}">
    验证您的帐户
  </a>
  <p>Best regards,<br>Rust祺洛</p>
</body>

</html>', NULL, 1, '2026-06-04 17:28:52', '2026-06-04 17:28:52');

DROP TABLE IF EXISTS `sys_menu`;
CREATE TABLE `sys_menu` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT,
  `title` TEXT NOT NULL,
  `i18nkey` TEXT,
  `pid` INTEGER NOT NULL DEFAULT 0,
  `order` INTEGER NOT NULL DEFAULT 0,
  `path` TEXT DEFAULT '',
  `component` TEXT,
  `redirect` TEXT,
  `href` TEXT DEFAULT '',
  `no_cache` TEXT NOT NULL DEFAULT '0',
  `menu_type` TEXT NOT NULL DEFAULT '',
  `hidden` TEXT NOT NULL DEFAULT '0',
  `active_menu` TEXT NOT NULL DEFAULT '0',
  `always_show` TEXT NOT NULL DEFAULT '0',
  `breadcrumb` TEXT NOT NULL DEFAULT '1',
  `affix` TEXT NOT NULL DEFAULT '0',
  `no_tags_view` TEXT NOT NULL DEFAULT '0',
  `can_to` TEXT NOT NULL DEFAULT '0',
  `status` TEXT NOT NULL DEFAULT '1',
  `perms` TEXT,
  `icon` TEXT DEFAULT '#',
  `created_at` TEXT,
  `updated_at` TEXT,
  `remark` TEXT DEFAULT '',
  `deleted_at` TEXT
);

INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(1, 'Dashboard', '首页', NULL, 0, 0, '/dashboard', '#', '/dashboard/analysis', '', '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(2, 'Analysis', '分析页', NULL, 1, 1, 'analysis', 'views/Dashboard/Analysis', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-23 22:51:09', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(3, 'Workplace', '工作台', NULL, 1, 2, 'workplace', 'views/Dashboard/Workplace', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:codepen-outlined', NULL, '2024-08-23 22:50:07', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(4, 'Authorization', '系统管理', NULL, 0, 2, '/authorization', '#', NULL, '', '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(5, 'Department', '部门管理', NULL, 4, 5, 'department', 'views/Authorization/Department/Department', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-11 05:31:40', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(6, 'User', '用户管理', NULL, 4, 2, 'user', 'views/Authorization/User/User', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7, 'Menu', '菜单管理', NULL, 4, 2, 'menu', 'views/Authorization/Menu/Menu', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '0', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-08 00:50:11', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(8, 'Role', '角色管理', NULL, 4, 2, 'role', 'views/Authorization/Role/Role', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7224471859710005249, 'Monitor', '系统监控', NULL, 0, 5, '/motion', '#', NULL, NULL, '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-tdesign:archway', NULL, '2024-10-25 00:42:23', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7225437966700777473, 'SystemLog', '操作日志', NULL, 7224471859710005249, 4, 'systemlog', 'views/Monitor/SystemLog/SystemLog', NULL, NULL, '0', 'C', '0', '0', '0', '1', '0', '0', '0', '0', NULL, 'vi-tdesign:angry', NULL, '2024-08-28 01:49:53', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7225439414641627136, 'LoginInfo', '登陆日志', NULL, 7224471859710005249, 2, 'logininfo', 'views/Monitor/LoginInfo/LoginInfo', NULL, NULL, '0', 'C', '0', '0', '0', '1', '0', '0', '0', '0', NULL, 'vi-tdesign:app', NULL, '2024-08-25 00:53:36', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7226994818660044800, 'UserOnline', '在线用户', NULL, 7224471859710005249, 0, 'useronline', 'views/Monitor/UserOnline/UserOnline', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ep:bell-filled', '2024-08-08 00:57:16', '2024-10-24 01:28:11', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7227715297679118337, 'Job', '定时任务', NULL, 7224471859710005249, 0, 'job', 'views/Monitor/Job/Job', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:apple', '2024-08-10 00:40:11', '2024-08-10 00:40:11', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7228150987080470529, 'DictData', '字典数据', NULL, 4, 100, 'dictdata', 'views/Authorization/DictData/DictData', NULL, '', '0', 'C', '1', '0', '0', '0', '0', '1', '0', '0', NULL, 'vi-ant-design:aliwangwang-outlined', '2024-08-11 05:31:28', '2024-10-25 23:18:20', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7228186533035118593, 'DictType', '字典管理', NULL, 4, 7, 'dicttype', 'views/Authorization/DictType/DictType', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:alert-filled', '2024-08-11 07:52:43', '2024-10-25 23:18:11', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7228454585970593792, 'ServerInfo', '服务监控', NULL, 7224471859710005249, 4, 'serverinfo', 'views/Monitor/ServerInfo/ServerInfo', NULL, '', '1', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:fund-projection-screen-outlined', '2024-08-12 01:37:51', '2024-08-25 00:54:16', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7230621134038896640, 'JobLog', '定时日志', NULL, 7224471859710005249, 0, 'joblog', 'views/Monitor/JobLog/JobLog', NULL, '', '1', 'C', '1', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:baseball', '2024-08-18 01:06:57', '2024-09-01 00:09:39', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7230940389695426560, 'ApiPermission', '权限查询', NULL, 4, 7, 'apipermission', 'views/Authorization/ApiPermission/ApiPermission', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:burger', '2024-08-18 22:15:33', '2024-08-24 01:22:22', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7231673632416502784, 'RoleApi', '角色API', NULL, 4, 1, 'roleapi', 'views/Authorization/RoleApi/RoleApi', NULL, '', '0', 'C', '1', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:basketball', '2024-08-20 22:49:12', '2024-10-24 01:32:15', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7234621748841190400, 'Cache', '缓存管理', NULL, 7224471859710005249, 1, 'cache', 'views/Monitor/Cache/Cache', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:bicycle', '2024-08-29 02:03:58', '2024-08-29 02:03:58', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7402695499167208448, NULL, '测试', NULL, 0, 15, '/test', '#', NULL, '', '0', 'M', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:application', '2025-12-05 21:09:02', '2025-12-05 21:09:02', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7402702074216879104, 'TestDataScope', '测试数据', NULL, 7402695499167208448, 2, 'test/TestDataScope', 'views/Test/TestDataScope/TestDataScope', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:animation-1', '2025-12-05 21:35:10', '2026-06-02 21:38:42', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7461092148004164608, NULL, '工具', NULL, 0, 25, '/tools', '#', NULL, '', '0', 'M', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:tools', '2026-05-16 00:36:28', '2026-05-16 00:37:00', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7461092495128957952, NULL, '代码生成', NULL, 7461092148004164608, 1, 'tools/CodeGen', 'views/Tool/CodeGen/CodeGen', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, NULL, '2026-05-16 00:37:51', '2026-05-16 00:38:09', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7461476724811666432, NULL, '测试分类', NULL, 7402695499167208448, 5, 'test/TestCategory', 'views/Test/TestCategory/TestCategory', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:certificate-1', '2026-05-17 02:04:39', '2026-05-17 02:05:07', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7462558764051108864, NULL, '测试文章', NULL, 7402695499167208448, 10, 'test/TestArticle', 'views/Test/TestArticle/TestArticle', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:article', '2026-05-20 01:44:17', '2026-05-20 01:44:17', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467279900517241856, NULL, '微信', NULL, 0, 6, '/wechat', '#', NULL, '', '0', 'M', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:logo-wechat-stroke', '2026-06-02 02:24:24', '2026-06-02 02:24:36', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467280164657730560, NULL, '公众号', NULL, 7467279900517241856, 1, 'wechat/wechataccounts', 'views/wechat/wechataccounts/wxaccounts', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:chat-message', '2026-06-02 02:25:27', '2026-06-02 02:26:15', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467286094099158016, NULL, '微信用户', NULL, 7467279900517241856, 5, 'wechat/wxusers', 'views/wechat/wxusers/wxusers', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:usergroup-add', '2026-06-02 02:49:00', '2026-06-05 01:57:36', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467286364820509696, NULL, '微信消息', NULL, 7467279900517241856, 7, 'wechat/wxmessages', 'views/wechat/wxmessages/wxmessages', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:message-outlined', '2026-06-02 02:50:05', '2026-06-02 02:50:27', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467288062007874560, NULL, '微信菜单', NULL, 7467279900517241856, 3, 'wechat/wxmenus', 'views/wechat/wxmenus/wxmenus', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:menu-outlined', '2026-06-02 02:56:49', '2026-06-05 01:57:51', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467447239728600064, NULL, '自动回复', NULL, 7467279900517241856, 4, 'wechat/wxautoreplies', 'views/wechat/wxautoreplies/wxautoreplies', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:message-twotone', '2026-06-02 13:29:20', '2026-06-05 01:57:44', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467568513482134528, NULL, '素材管理', NULL, 7467279900517241856, 15, 'wechat/wxmaterials', 'views/wechat/wxmaterials/wxmaterials', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-tdesign:animation', '2026-06-02 21:31:14', '2026-06-02 21:31:30', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES(7467597219751695360, NULL, '邮件模板', NULL, 7224471859710005249, 1, 'monitor/Email', 'views/Monitor/Email/Email', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:chat-line-round', '2026-06-02 23:25:18', '2026-06-02 23:26:12', NULL, NULL);

DROP TABLE IF EXISTS `sys_notice`;
CREATE TABLE `sys_notice` (
  `notice_id` INTEGER NOT NULL PRIMARY KEY,
  `notice_title` TEXT NOT NULL,
  `notice_type` TEXT NOT NULL,
  `notice_content` BLOB,
  `status` TEXT DEFAULT '0',
  `create_dept` INTEGER,
  `create_by` INTEGER,
  `created_at` TEXT,
  `update_by` INTEGER,
  `updated_at` TEXT,
  `remark` TEXT
);

INSERT INTO `sys_notice` (`notice_id`, `notice_title`, `notice_type`, `notice_content`, `status`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `remark`) VALUES(1, '温馨提醒：2018-07-01 新版本发布啦', '2', X'E696B0E78988E69CACE58685E5AEB9', '0', 103, 1, '2024-07-03 01:17:33', NULL, NULL, '管理员');
INSERT INTO `sys_notice` (`notice_id`, `notice_title`, `notice_type`, `notice_content`, `status`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `remark`) VALUES(2, '维护通知：2018-07-01 系统凌晨维护', '1', X'E7BBB4E68AA4E58685E5AEB9', '0', 103, 1, '2024-07-03 01:17:33', NULL, NULL, '管理员');

DROP TABLE IF EXISTS `sys_oper_log`;
CREATE TABLE `sys_oper_log` (
  `oper_id` INTEGER NOT NULL PRIMARY KEY,
  `api_name` TEXT DEFAULT '',
  `method` TEXT DEFAULT '',
  `request_method` TEXT DEFAULT '',
  `oper_name` TEXT DEFAULT '',
  `oper_url` TEXT DEFAULT '',
  `oper_ip` TEXT DEFAULT '',
  `oper_location` TEXT DEFAULT '',
  `oper_param` TEXT DEFAULT '',
  `json_result` TEXT DEFAULT '',
  `status` TEXT DEFAULT '0',
  `error_msg` TEXT DEFAULT '',
  `oper_time` TEXT,
  `cost_time` INTEGER DEFAULT 0
);

CREATE INDEX `idx_sys_oper_log_idx_sys_oper_log_ot` ON `sys_oper_log` (`oper_time`);

CREATE INDEX `idx_sys_oper_log_idx_sys_oper_log_s` ON `sys_oper_log` (`status`);

DROP TABLE IF EXISTS `sys_post`;
CREATE TABLE `sys_post` (
  `post_id` INTEGER NOT NULL PRIMARY KEY,
  `dept_id` INTEGER NOT NULL,
  `post_code` TEXT NOT NULL,
  `post_category` TEXT,
  `post_name` TEXT NOT NULL,
  `post_sort` INTEGER NOT NULL,
  `status` TEXT NOT NULL,
  `create_dept` INTEGER,
  `created_at` TEXT,
  `updated_at` TEXT,
  `remark` TEXT
);

INSERT INTO `sys_post` (`post_id`, `dept_id`, `post_code`, `post_category`, `post_name`, `post_sort`, `status`, `create_dept`, `created_at`, `updated_at`, `remark`) VALUES(1, 103, 'ceo', NULL, '董事长', 1, '0', 103, '2024-07-03 01:17:32', NULL, '');
INSERT INTO `sys_post` (`post_id`, `dept_id`, `post_code`, `post_category`, `post_name`, `post_sort`, `status`, `create_dept`, `created_at`, `updated_at`, `remark`) VALUES(2, 100, 'se', NULL, '项目经理', 2, '0', 103, '2024-07-03 01:17:32', NULL, '');
INSERT INTO `sys_post` (`post_id`, `dept_id`, `post_code`, `post_category`, `post_name`, `post_sort`, `status`, `create_dept`, `created_at`, `updated_at`, `remark`) VALUES(3, 100, 'hr', NULL, '人力资源', 3, '0', 103, '2024-07-03 01:17:32', NULL, '');
INSERT INTO `sys_post` (`post_id`, `dept_id`, `post_code`, `post_category`, `post_name`, `post_sort`, `status`, `create_dept`, `created_at`, `updated_at`, `remark`) VALUES(4, 100, 'user', NULL, '普通员工', 4, '0', 103, '2024-07-03 01:17:32', NULL, '');

DROP TABLE IF EXISTS `sys_role`;
CREATE TABLE `sys_role` (
  `role_id` INTEGER NOT NULL PRIMARY KEY,
  `role_name` TEXT NOT NULL,
  `role_key` TEXT NOT NULL,
  `order` INTEGER NOT NULL,
  `data_scope` TEXT NOT NULL DEFAULT '1',
  `status` TEXT NOT NULL,
  `create_dept` INTEGER,
  `remark` TEXT,
  `created_at` TEXT,
  `updated_at` TEXT,
  `deleted_at` TEXT
);

INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(1, '管理员', 'admin', 2, '1', '0', 103, '超级管理员', '2024-07-03 01:17:32', NULL, NULL);
INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(3, '本部门及以下', 'test1', 5, '4', '0', 103, '部门数据以及部门以下的数据', '2024-07-03 01:17:32', NULL, NULL);
INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(4, '本人的数据权限', 'self', 6, '5', '0', 103, '本人数据', '2024-07-03 01:17:32', NULL, NULL);
INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(7222985345490620417, '自定义数据权限', 'self', 7, '2', '0', NULL, '自定义数据权限', NULL, NULL, NULL);
INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES(7403403929607640064, '本部门权限', 'Dept', 5, '3', '0', NULL, '本部门数据', NULL, NULL, NULL);

DROP TABLE IF EXISTS `sys_role_api`;
CREATE TABLE `sys_role_api` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `role_id` INTEGER NOT NULL,
  `api_id` INTEGER NOT NULL,
  `api` TEXT NOT NULL,
  `method` TEXT NOT NULL,
  `apiname` TEXT NOT NULL,
  `sort` INTEGER NOT NULL
);

INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253093340160, 7222985345490620417, 7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', 1);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253202392064, 7222985345490620417, 7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', 2);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253261112320, 7222985345490620417, 7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', 3);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253307249664, 7222985345490620417, 7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', 4);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253361775616, 7222985345490620417, 7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', 5);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253412107264, 7222985345490620417, 7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', 6);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253462438912, 7222985345490620417, 7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', 7);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253512770560, 7222985345490620417, 7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', 8);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253558907904, 7222985345490620417, 7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', 9);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253609239552, 7222985345490620417, 7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', 10);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253655376896, 7222985345490620417, 7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', 11);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253705708544, 7222985345490620417, 7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', 12);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253751845888, 7222985345490620417, 7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', 13);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253802177536, 7222985345490620417, 7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', 14);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253856703488, 7222985345490620417, 7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', 15);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253898646528, 7222985345490620417, 7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', 16);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715253957366784, 7222985345490620417, 7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', 17);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254011892736, 7222985345490620417, 7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', 18);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254053835776, 7222985345490620417, 7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', 19);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254112556032, 7222985345490620417, 7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', 20);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254162887680, 7222985345490620417, 7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', 21);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254234190848, 7222985345490620417, 7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', 22);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254288716800, 7222985345490620417, 7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', 23);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254339048448, 7222985345490620417, 7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', 24);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254376797184, 7222985345490620417, 7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', 25);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254422934528, 7222985345490620417, 7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', 26);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254469071872, 7222985345490620417, 7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', 27);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254515209216, 7222985345490620417, 7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', 28);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254565540864, 7222985345490620417, 7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', 29);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254620066816, 7222985345490620417, 7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', 30);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254674592768, 7222985345490620417, 7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', 31);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254729118720, 7222985345490620417, 7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', 32);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254771061760, 7222985345490620417, 7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', 33);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254821393408, 7222985345490620417, 7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', 34);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254863336448, 7222985345490620417, 7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', 35);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254909473792, 7222985345490620417, 7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', 36);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715254959805440, 7222985345490620417, 7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', 37);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255005942784, 7222985345490620417, 7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', 38);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255056274432, 7222985345490620417, 7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', 39);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255102411776, 7222985345490620417, 7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', 40);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255148549120, 7222985345490620417, 7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', 41);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255207269376, 7222985345490620417, 7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', 42);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255261795328, 7222985345490620417, 7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', 43);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255307932672, 7222985345490620417, 7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', 44);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255354070016, 7222985345490620417, 7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', 45);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255396013056, 7222985345490620417, 7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', 46);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255450539008, 7222985345490620417, 7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', 47);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255496676352, 7222985345490620417, 7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', 48);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255542813696, 7222985345490620417, 7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', 49);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255597339648, 7222985345490620417, 7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', 50);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255643476992, 7222985345490620417, 7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', 51);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255685420032, 7222985345490620417, 7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', 52);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255727363072, 7222985345490620417, 7232752189951611904, '/sys/role/add', 'Post', '添加角色', 53);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255773500416, 7222985345490620417, 7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', 54);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255819637760, 7222985345490620417, 7232752190018720768, '/sys/role/del', 'Delete', '删除角色', 55);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255861580800, 7222985345490620417, 7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', 56);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255903523840, 7222985345490620417, 7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', 57);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255949661184, 7222985345490620417, 7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', 58);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715255991604224, 7222985345490620417, 7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', 59);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256029352960, 7222985345490620417, 7232752190333293568, '/sys/user/del', 'Delete', '删除用户', 60);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256075490304, 7222985345490620417, 7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', 61);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256113239040, 7222985345490620417, 7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', 62);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256159376384, 7222985345490620417, 7232752190459122688, '/sys/user/add', 'Post', '添加用户', 63);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256205513728, 7222985345490620417, 7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', 64);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256289399808, 7222985345490620417, 7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', 65);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256339731456, 7222985345490620417, 7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', 66);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256385868800, 7222985345490620417, 7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', 67);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256427811840, 7222985345490620417, 7232752190727558144, '/sys/dept/add', 'Post', '添加部门', 68);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256473949184, 7222985345490620417, 7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', 69);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256515892224, 7222985345490620417, 7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', 70);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256578806784, 7222985345490620417, 7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', 71);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256629138432, 7222985345490620417, 7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', 72);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256687858688, 7222985345490620417, 7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', 73);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256733996032, 7222985345490620417, 7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', 74);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256784327680, 7222985345490620417, 7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', 75);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256834659328, 7222985345490620417, 7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', 76);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256884990976, 7222985345490620417, 7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', 77);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256931128320, 7222985345490620417, 7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', 78);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715256968877056, 7222985345490620417, 7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', 79);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257023403008, 7222985345490620417, 7385368752226735104, '/test/test_api/list', 'Get', '获取列表', 80);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257073734656, 7222985345490620417, 7385368752235123712, '/test/test_api/del', 'Delete', '删除TestApi', 81);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257119872000, 7222985345490620417, 7401293722618336256, '/test/test_api/db_read_write_test', 'Put', 'db_read_write_test', 82);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257174397952, 7222985345490620417, 7348333842530210816, '/wechat/wxaccounts/del', 'Delete', '删除WxAccounts', 83);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257212146688, 7222985345490620417, 7385368752239318016, '/test/test_api/add', 'Post', '添加TestApi', 84);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257262478336, 7222985345490620417, 7401293722630919168, '/test/test_api/db_name_test', 'Put', 'db_name_test', 85);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257308615680, 7222985345490620417, 7348333842559570944, '/wechat/wxaccounts/edit', 'Put', '编辑WxAccounts', 86);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257354753024, 7222985345490620417, 7348333842567959552, '/wechat/wxaccounts/list', 'Get', '获取WxAccounts列表', 87);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257400890368, 7222985345490620417, 7385368752243512320, '/test/test_api/edit', 'Put', '编辑TestApi', 88);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257451222016, 7222985345490620417, 7348333842572153856, '/wechat/wxaccounts/add', 'Post', '添加WxAccounts', 89);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257497359360, 7222985345490620417, 7348333842584736768, '/wechat/wxautoreplies/add', 'Post', '添加WxAutoReplies', 90);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257547691008, 7222985345490620417, 7401293722647696384, '/test/test_api/db_index_test', 'Put', 'db_index_test', 91);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257589634048, 7222985345490620417, 7348333842609902592, '/wechat/wxautoreplies/del', 'Delete', '删除WxAutoReplies', 92);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257644160000, 7222985345490620417, 7348333842635068416, '/wechat/wxautoreplies/list', 'Get', '获取WxAutoReplies列表', 93);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257698685952, 7222985345490620417, 7401293722651890688, '/test/test_api/db_auto_test', 'Put', 'db_auto_test', 94);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257749017600, 7222985345490620417, 7348333843008361472, '/wechat/wxautoreplies/edit', 'Put', '编辑WxAutoReplies', 95);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257790960640, 7222985345490620417, 7402694868939478016, '/test/test_data_scope/del', 'Delete', 'Delete', 96);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257832903680, 7222985345490620417, 7402694868947866624, '/test/test_data_scope/list', 'Get', 'list', 97);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257883235328, 7222985345490620417, 7348333843016750080, '/wechat/wxmenus/list', 'Get', '获取WxMenus列表', 98);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257929372672, 7222985345490620417, 7348333843071276032, '/wechat/wxmenus/del', 'Delete', '删除WxMenus', 99);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715257975510016, 7222985345490620417, 7402694868947866625, '/test/test_data_scope/add', 'Post', 'add', 100);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258034230272, 7222985345490620417, 7402694868952060928, '/test/test_data_scope/edit', 'Put', 'edit', 101);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258088756224, 7222985345490620417, 7348333843075470336, '/wechat/wxmenus/pull_menu', 'Get', '创建自定义菜单', 102);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258134893568, 7222985345490620417, 7348333843092247552, '/wechat/wxmenus/add', 'Post', '添加WxMenus', 103);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258185225216, 7222985345490620417, 7348333843100636160, '/wechat/wxmenus/edit', 'Put', '编辑WxMenus', 104);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258231362560, 7222985345490620417, 7348333843104830464, '/wechat/wxmessages/list', 'Get', '获取WxMessages列表', 105);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258281694208, 7222985345490620417, 7348333843125801984, '/wechat/wxmessages/edit', 'Put', '编辑WxMessages', 106);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258332025856, 7222985345490620417, 7348333843138384896, '/wechat/wxmessages/del', 'Delete', '删除WxMessages', 107);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258403329024, 7222985345490620417, 7348333843150967808, '/wechat/wxmessages/add', 'Post', '添加WxMessages', 108);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258453660672, 7222985345490620417, 7348333843159356416, '/wechat/wxusers/add', 'Post', '添加WxUsers', 109);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258495603712, 7222985345490620417, 7348333843167745024, '/wechat/wxusers/edit', 'Put', '编辑WxUsers', 110);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258545935360, 7222985345490620417, 7348333843180327936, '/wechat/wxusers/del', 'Delete', '删除WxUsers', 111);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715258587878400, 7222985345490620417, 7348333843184522240, '/wechat/wxusers/list', 'Get', '获取WxUsers列表', 112);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541250413568, 4, 7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', 1);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541346882560, 4, 7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', 2);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541397214208, 4, 7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', 3);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541430768640, 4, 7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', 4);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541489488896, 4, 7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', 5);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541539820544, 4, 7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', 6);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541585957888, 4, 7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', 7);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541636289536, 4, 7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', 8);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541690815488, 4, 7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', 9);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541736952832, 4, 7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', 10);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541787284480, 4, 7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', 11);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541846004736, 4, 7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', 12);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541896336384, 4, 7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', 13);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541946668032, 4, 7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', 14);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715541992805376, 4, 7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', 15);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542047331328, 4, 7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', 16);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542097662976, 4, 7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', 17);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542152188928, 4, 7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', 18);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542210909184, 4, 7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', 19);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542261240832, 4, 7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', 20);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542307378176, 4, 7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', 21);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542361904128, 4, 7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', 22);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542420624384, 4, 7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', 23);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542466761728, 4, 7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', 24);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542508704768, 4, 7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', 25);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542554842112, 4, 7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', 26);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542605173760, 4, 7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', 27);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542663894016, 4, 7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', 28);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542697448448, 4, 7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', 29);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542743585792, 4, 7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', 30);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542781334528, 4, 7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', 31);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542827471872, 4, 7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', 32);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542894580736, 4, 7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', 33);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542940718080, 4, 7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', 34);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715542991049728, 4, 7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', 35);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543041381376, 4, 7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', 36);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543100101632, 4, 7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', 37);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543150433280, 4, 7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', 38);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543221736448, 4, 7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', 39);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543280456704, 4, 7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', 40);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543330788352, 4, 7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', 41);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543389508608, 4, 7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', 42);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543444034560, 4, 7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', 43);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543485977600, 4, 7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', 44);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543548892160, 4, 7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', 45);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543603418112, 4, 7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', 46);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543666332672, 4, 7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', 47);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543720858624, 4, 7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', 48);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543787967488, 4, 7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', 49);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543842493440, 4, 7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', 50);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543897019392, 4, 7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', 51);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715543955739648, 4, 7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', 52);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544006071296, 4, 7232752189951611904, '/sys/role/add', 'Post', '添加角色', 53);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544052208640, 4, 7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', 54);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544106734592, 4, 7232752190018720768, '/sys/role/del', 'Delete', '删除角色', 55);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544157066240, 4, 7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', 56);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544215786496, 4, 7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', 57);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544261923840, 4, 7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', 58);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544303866880, 4, 7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', 59);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544350004224, 4, 7232752190333293568, '/sys/user/del', 'Delete', '删除用户', 60);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544387752960, 4, 7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', 61);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544433890304, 4, 7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', 62);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544480027648, 4, 7232752190459122688, '/sys/user/add', 'Post', '添加用户', 63);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544534553600, 4, 7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', 64);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544576496640, 4, 7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', 65);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544626828288, 4, 7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', 66);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544677159936, 4, 7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', 67);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544727491584, 4, 7232752190727558144, '/sys/dept/add', 'Post', '添加部门', 68);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544782017536, 4, 7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', 69);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544828154880, 4, 7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', 70);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544870097920, 4, 7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', 71);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544907846656, 4, 7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', 72);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715544958178304, 4, 7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', 73);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545000121344, 4, 7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', 74);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545054647296, 4, 7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', 75);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545109173248, 4, 7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', 76);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545159504896, 4, 7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', 77);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545205642240, 4, 7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', 78);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545268556800, 4, 7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', 79);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545323082752, 4, 7385368752226735104, '/test/test_api/list', 'Get', '获取列表', 80);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545373414400, 4, 7385368752235123712, '/test/test_api/del', 'Delete', '删除TestApi', 81);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545444717568, 4, 7401293722618336256, '/test/test_api/db_read_write_test', 'Put', 'db_read_write_test', 82);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545495049216, 4, 7348333842530210816, '/wechat/wxaccounts/del', 'Delete', '删除WxAccounts', 83);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545549575168, 4, 7385368752239318016, '/test/test_api/add', 'Post', '添加TestApi', 84);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545599906816, 4, 7401293722630919168, '/test/test_api/db_name_test', 'Put', 'db_name_test', 85);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545650238464, 4, 7348333842559570944, '/wechat/wxaccounts/edit', 'Put', '编辑WxAccounts', 86);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545704764416, 4, 7348333842567959552, '/wechat/wxaccounts/list', 'Get', '获取WxAccounts列表', 87);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545755096064, 4, 7385368752243512320, '/test/test_api/edit', 'Put', '编辑TestApi', 88);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545801233408, 4, 7348333842572153856, '/wechat/wxaccounts/add', 'Post', '添加WxAccounts', 89);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545847370752, 4, 7348333842584736768, '/wechat/wxautoreplies/add', 'Post', '添加WxAutoReplies', 90);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545893508096, 4, 7401293722647696384, '/test/test_api/db_index_test', 'Put', 'db_index_test', 91);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545939645440, 4, 7348333842609902592, '/wechat/wxautoreplies/del', 'Delete', '删除WxAutoReplies', 92);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715545989977088, 4, 7348333842635068416, '/wechat/wxautoreplies/list', 'Get', '获取WxAutoReplies列表', 93);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546031920128, 4, 7401293722651890688, '/test/test_api/db_auto_test', 'Put', 'db_auto_test', 94);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546086446080, 4, 7348333843008361472, '/wechat/wxautoreplies/edit', 'Put', '编辑WxAutoReplies', 95);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546136777728, 4, 7402694868939478016, '/test/test_data_scope/del', 'Delete', 'Delete', 96);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546182915072, 4, 7402694868947866624, '/test/test_data_scope/list', 'Get', 'list', 97);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546233246720, 4, 7348333843016750080, '/wechat/wxmenus/list', 'Get', '获取WxMenus列表', 98);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546283578368, 4, 7348333843071276032, '/wechat/wxmenus/del', 'Delete', '删除WxMenus', 99);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546329715712, 4, 7402694868947866625, '/test/test_data_scope/add', 'Post', 'add', 100);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546384241664, 4, 7402694868952060928, '/test/test_data_scope/edit', 'Put', 'edit', 101);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546426184704, 4, 7348333843075470336, '/wechat/wxmenus/pull_menu', 'Get', '创建自定义菜单', 102);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546484904960, 4, 7348333843092247552, '/wechat/wxmenus/add', 'Post', '添加WxMenus', 103);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546535236608, 4, 7348333843100636160, '/wechat/wxmenus/edit', 'Put', '编辑WxMenus', 104);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546585568256, 4, 7348333843104830464, '/wechat/wxmessages/list', 'Get', '获取WxMessages列表', 105);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546635899904, 4, 7348333843125801984, '/wechat/wxmessages/edit', 'Put', '编辑WxMessages', 106);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546690425856, 4, 7348333843138384896, '/wechat/wxmessages/del', 'Delete', '删除WxMessages', 107);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546740757504, 4, 7348333843150967808, '/wechat/wxmessages/add', 'Post', '添加WxMessages', 108);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546791089152, 4, 7348333843159356416, '/wechat/wxusers/add', 'Post', '添加WxUsers', 109);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546841420800, 4, 7348333843167745024, '/wechat/wxusers/edit', 'Put', '编辑WxUsers', 110);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546900141056, 4, 7348333843180327936, '/wechat/wxusers/del', 'Delete', '删除WxUsers', 111);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402715546954667008, 4, 7348333843184522240, '/wechat/wxusers/list', 'Get', '获取WxUsers列表', 112);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415402300416, 3, 7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', 1);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415490380800, 3, 7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', 2);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415574266880, 3, 7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', 3);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415649764352, 3, 7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', 4);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415712678912, 3, 7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', 5);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415758816256, 3, 7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', 6);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415825925120, 3, 7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', 7);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415893033984, 3, 7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', 8);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728415955948544, 3, 7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', 9);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416023057408, 3, 7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', 10);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416077583360, 3, 7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', 11);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416123720704, 3, 7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', 12);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416174052352, 3, 7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', 13);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416228578304, 3, 7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', 14);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416283104256, 3, 7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', 15);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416333435904, 3, 7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', 16);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416379573248, 3, 7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', 17);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416429904896, 3, 7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', 18);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416480236544, 3, 7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', 19);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416530568192, 3, 7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', 20);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416580899840, 3, 7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', 21);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416631231488, 3, 7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', 22);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416681563136, 3, 7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', 23);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416731894784, 3, 7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', 24);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416773837824, 3, 7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', 25);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416824169472, 3, 7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', 26);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416887084032, 3, 7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', 27);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416937415680, 3, 7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', 28);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728416991941632, 3, 7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', 29);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417042273280, 3, 7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', 30);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417088410624, 3, 7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', 31);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417138742272, 3, 7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', 32);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417189073920, 3, 7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', 33);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417226822656, 3, 7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', 34);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417281348608, 3, 7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', 35);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417323291648, 3, 7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', 36);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417369428992, 3, 7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', 37);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417423954944, 3, 7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', 38);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417486869504, 3, 7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', 39);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417541395456, 3, 7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', 40);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417587532800, 3, 7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', 41);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417637864448, 3, 7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', 42);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417688196096, 3, 7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', 43);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417738527744, 3, 7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', 44);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417776276480, 3, 7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', 45);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417826608128, 3, 7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', 46);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417872745472, 3, 7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', 47);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417927271424, 3, 7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', 48);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728417981797376, 3, 7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', 49);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418036323328, 3, 7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', 50);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418095043584, 3, 7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', 51);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418145375232, 3, 7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', 52);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418195706880, 3, 7232752189951611904, '/sys/role/add', 'Post', '添加角色', 53);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418250232832, 3, 7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', 54);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418304758784, 3, 7232752190018720768, '/sys/role/del', 'Delete', '删除角色', 55);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418355090432, 3, 7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', 56);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418405422080, 3, 7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', 57);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418455753728, 3, 7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', 58);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418506085376, 3, 7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', 59);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418552222720, 3, 7232752190333293568, '/sys/user/del', 'Delete', '删除用户', 60);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418602554368, 3, 7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', 61);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418669663232, 3, 7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', 62);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418715800576, 3, 7232752190459122688, '/sys/user/add', 'Post', '添加用户', 63);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418757743616, 3, 7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', 64);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418803880960, 3, 7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', 65);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418854212608, 3, 7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', 66);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418908738560, 3, 7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', 67);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728418963264512, 3, 7232752190727558144, '/sys/dept/add', 'Post', '添加部门', 68);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419017790464, 3, 7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', 69);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419072316416, 3, 7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', 70);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419122648064, 3, 7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', 71);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419177174016, 3, 7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', 72);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419227505664, 3, 7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', 73);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419265254400, 3, 7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', 74);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419328168960, 3, 7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', 75);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419382694912, 3, 7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', 76);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419433026560, 3, 7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', 77);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419479163904, 3, 7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', 78);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419529495552, 3, 7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', 79);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419584021504, 3, 7385368752226735104, '/test/test_api/list', 'Get', '获取列表', 80);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419638547456, 3, 7385368752235123712, '/test/test_api/del', 'Delete', '删除TestApi', 81);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419688879104, 3, 7401293722618336256, '/test/test_api/db_read_write_test', 'Put', 'db_read_write_test', 82);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419751793664, 3, 7348333842530210816, '/wechat/wxaccounts/del', 'Delete', '删除WxAccounts', 83);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419818902528, 3, 7385368752239318016, '/test/test_api/add', 'Post', '添加TestApi', 84);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419873428480, 3, 7401293722630919168, '/test/test_api/db_name_test', 'Put', 'db_name_test', 85);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419940537344, 3, 7348333842559570944, '/wechat/wxaccounts/edit', 'Put', '编辑WxAccounts', 86);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728419999257600, 3, 7348333842567959552, '/wechat/wxaccounts/list', 'Get', '获取WxAccounts列表', 87);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420053783552, 3, 7385368752243512320, '/test/test_api/edit', 'Put', '编辑TestApi', 88);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420099920896, 3, 7348333842572153856, '/wechat/wxaccounts/add', 'Post', '添加WxAccounts', 89);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420150252544, 3, 7348333842584736768, '/wechat/wxautoreplies/add', 'Post', '添加WxAutoReplies', 90);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420192195584, 3, 7401293722647696384, '/test/test_api/db_index_test', 'Put', 'db_index_test', 91);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420242527232, 3, 7348333842609902592, '/wechat/wxautoreplies/del', 'Delete', '删除WxAutoReplies', 92);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420292858880, 3, 7348333842635068416, '/wechat/wxautoreplies/list', 'Get', '获取WxAutoReplies列表', 93);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420351579136, 3, 7401293722651890688, '/test/test_api/db_auto_test', 'Put', 'db_auto_test', 94);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420406105088, 3, 7348333843008361472, '/wechat/wxautoreplies/edit', 'Put', '编辑WxAutoReplies', 95);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420460631040, 3, 7402694868939478016, '/test/test_data_scope/del', 'Delete', 'Delete', 96);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420506768384, 3, 7402694868947866624, '/test/test_data_scope/list', 'Get', 'list', 97);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420548711424, 3, 7348333843016750080, '/wechat/wxmenus/list', 'Get', '获取WxMenus列表', 98);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420624208896, 3, 7348333843071276032, '/wechat/wxmenus/del', 'Delete', '删除WxMenus', 99);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420682929152, 3, 7402694868947866625, '/test/test_data_scope/add', 'Post', 'add', 100);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420733260800, 3, 7402694868952060928, '/test/test_data_scope/edit', 'Put', 'edit', 101);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420796175360, 3, 7348333843075470336, '/wechat/wxmenus/pull_menu', 'Get', '创建自定义菜单', 102);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420854895616, 3, 7348333843092247552, '/wechat/wxmenus/add', 'Post', '添加WxMenus', 103);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420909421568, 3, 7348333843100636160, '/wechat/wxmenus/edit', 'Put', '编辑WxMenus', 104);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728420959753216, 3, 7348333843104830464, '/wechat/wxmessages/list', 'Get', '获取WxMessages列表', 105);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421018473472, 3, 7348333843125801984, '/wechat/wxmessages/edit', 'Put', '编辑WxMessages', 106);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421072999424, 3, 7348333843138384896, '/wechat/wxmessages/del', 'Delete', '删除WxMessages', 107);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421127525376, 3, 7348333843150967808, '/wechat/wxmessages/add', 'Post', '添加WxMessages', 108);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421194634240, 3, 7348333843159356416, '/wechat/wxusers/add', 'Post', '添加WxUsers', 109);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421249160192, 3, 7348333843167745024, '/wechat/wxusers/edit', 'Put', '编辑WxUsers', 110);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421303686144, 3, 7348333843180327936, '/wechat/wxusers/del', 'Delete', '删除WxUsers', 111);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7402728421358212096, 3, 7348333843184522240, '/wechat/wxusers/list', 'Get', '获取WxUsers列表', 112);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405998959793152, 1, 7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', 1);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999001736192, 1, 7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', 2);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999039484928, 1, 7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', 3);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999068845056, 1, 7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', 4);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999094010880, 1, 7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', 5);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999119176704, 1, 7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', 6);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999148536832, 1, 7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', 7);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999182091264, 1, 7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', 8);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999215645696, 1, 7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', 9);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999245005824, 1, 7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', 10);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999274365952, 1, 7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', 11);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999303726080, 1, 7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', 12);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999358252032, 1, 7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', 13);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999391806464, 1, 7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', 14);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999425360896, 1, 7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', 15);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999458915328, 1, 7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', 16);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999488275456, 1, 7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', 17);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999517635584, 1, 7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', 18);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999551190016, 1, 7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', 19);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999584744448, 1, 7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', 20);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999618298880, 1, 7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', 21);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999651853312, 1, 7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', 22);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999677019136, 1, 7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', 23);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999710573568, 1, 7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', 24);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999739933696, 1, 7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', 25);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999769293824, 1, 7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', 26);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999802848256, 1, 7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', 27);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999840596992, 1, 7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', 28);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999878345728, 1, 7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', 29);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999911900160, 1, 7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', 30);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999941260288, 1, 7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', 31);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999966426112, 1, 7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', 32);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403405999999980544, 1, 7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', 33);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000033534976, 1, 7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', 34);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000067089408, 1, 7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', 35);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000100643840, 1, 7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', 36);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000125809664, 1, 7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', 37);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000155169792, 1, 7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', 38);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000188724224, 1, 7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', 39);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000213890048, 1, 7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', 40);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000251638784, 1, 7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', 41);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000280998912, 1, 7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', 42);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000310359040, 1, 7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', 43);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000339719168, 1, 7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', 44);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000373273600, 1, 7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', 45);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000402633728, 1, 7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', 46);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000431993856, 1, 7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', 47);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000461353984, 1, 7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', 48);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000490714112, 1, 7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', 49);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000520074240, 1, 7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', 50);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000553628672, 1, 7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', 51);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000578794496, 1, 7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', 52);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000616543232, 1, 7232752189951611904, '/sys/role/add', 'Post', '添加角色', 53);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000645903360, 1, 7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', 54);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000675263488, 1, 7232752190018720768, '/sys/role/del', 'Delete', '删除角色', 55);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000708817920, 1, 7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', 56);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000738178048, 1, 7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', 57);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000767538176, 1, 7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', 58);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000796898304, 1, 7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', 59);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000838841344, 1, 7232752190333293568, '/sys/user/del', 'Delete', '删除用户', 60);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000868201472, 1, 7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', 61);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000901755904, 1, 7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', 62);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000931116032, 1, 7232752190459122688, '/sys/user/add', 'Post', '添加用户', 63);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000960476160, 1, 7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', 64);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406000994030592, 1, 7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', 65);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001019196416, 1, 7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', 66);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001048556544, 1, 7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', 67);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001077916672, 1, 7232752190727558144, '/sys/dept/add', 'Post', '添加部门', 68);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001107276800, 1, 7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', 69);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001136636928, 1, 7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', 70);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001165997056, 1, 7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', 71);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001199551488, 1, 7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', 72);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001233105920, 1, 7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', 73);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001258271744, 1, 7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', 74);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001291826176, 1, 7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', 75);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001321186304, 1, 7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', 76);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001350546432, 1, 7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', 77);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001375712256, 1, 7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', 78);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001405072384, 1, 7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', 79);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001434432512, 1, 7385368752226735104, '/test/test_api/list', 'Get', '获取列表', 80);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001463792640, 1, 7385368752235123712, '/test/test_api/del', 'Delete', '删除TestApi', 81);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001493152768, 1, 7401293722618336256, '/test/test_api/db_read_write_test', 'Put', 'db_read_write_test', 82);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001522512896, 1, 7348333842530210816, '/wechat/wxaccounts/del', 'Delete', '删除WxAccounts', 83);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001560261632, 1, 7385368752239318016, '/test/test_api/add', 'Post', '添加TestApi', 84);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001589621760, 1, 7401293722630919168, '/test/test_api/db_name_test', 'Put', 'db_name_test', 85);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001623176192, 1, 7348333842559570944, '/wechat/wxaccounts/edit', 'Put', '编辑WxAccounts', 86);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001652536320, 1, 7348333842567959552, '/wechat/wxaccounts/list', 'Get', '获取WxAccounts列表', 87);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001681896448, 1, 7385368752243512320, '/test/test_api/edit', 'Put', '编辑TestApi', 88);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001711256576, 1, 7348333842572153856, '/wechat/wxaccounts/add', 'Post', '添加WxAccounts', 89);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001740616704, 1, 7348333842584736768, '/wechat/wxautoreplies/add', 'Post', '添加WxAutoReplies', 90);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001769976832, 1, 7401293722647696384, '/test/test_api/db_index_test', 'Put', 'db_index_test', 91);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001795142656, 1, 7348333842609902592, '/wechat/wxautoreplies/del', 'Delete', '删除WxAutoReplies', 92);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001858057216, 1, 7348333842635068416, '/wechat/wxautoreplies/list', 'Get', '获取WxAutoReplies列表', 93);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001887417344, 1, 7401293722651890688, '/test/test_api/db_auto_test', 'Put', 'db_auto_test', 94);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001912583168, 1, 7348333843008361472, '/wechat/wxautoreplies/edit', 'Put', '编辑WxAutoReplies', 95);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001937748992, 1, 7402694868939478016, '/test/test_data_scope/del', 'Delete', 'Delete', 96);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406001971303424, 1, 7402694868947866624, '/test/test_data_scope/list', 'Get', 'list', 97);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002004857856, 1, 7348333843016750080, '/wechat/wxmenus/list', 'Get', '获取WxMenus列表', 98);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002042606592, 1, 7348333843071276032, '/wechat/wxmenus/del', 'Delete', '删除WxMenus', 99);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002071966720, 1, 7402694868947866625, '/test/test_data_scope/add', 'Post', 'add', 100);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002113909760, 1, 7402694868952060928, '/test/test_data_scope/edit', 'Put', 'edit', 101);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002139075584, 1, 7348333843075470336, '/wechat/wxmenus/pull_menu', 'Get', '创建自定义菜单', 102);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002168435712, 1, 7348333843092247552, '/wechat/wxmenus/add', 'Post', '添加WxMenus', 103);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002197795840, 1, 7348333843100636160, '/wechat/wxmenus/edit', 'Put', '编辑WxMenus', 104);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002227155968, 1, 7348333843104830464, '/wechat/wxmessages/list', 'Get', '获取WxMessages列表', 105);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002252321792, 1, 7348333843125801984, '/wechat/wxmessages/edit', 'Put', '编辑WxMessages', 106);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002285876224, 1, 7348333843138384896, '/wechat/wxmessages/del', 'Delete', '删除WxMessages', 107);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002319430656, 1, 7348333843150967808, '/wechat/wxmessages/add', 'Post', '添加WxMessages', 108);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002348790784, 1, 7348333843159356416, '/wechat/wxusers/add', 'Post', '添加WxUsers', 109);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002378150912, 1, 7348333843167745024, '/wechat/wxusers/edit', 'Put', '编辑WxUsers', 110);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002411705344, 1, 7348333843180327936, '/wechat/wxusers/del', 'Delete', '删除WxUsers', 111);
INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES(7403406002441065472, 1, 7348333843184522240, '/wechat/wxusers/list', 'Get', '获取WxUsers列表', 112);

DROP TABLE IF EXISTS `sys_role_dept`;
CREATE TABLE `sys_role_dept` (
  `role_id` INTEGER NOT NULL,
  `dept_id` INTEGER NOT NULL,
  PRIMARY KEY (`role_id`, `dept_id`)
);

CREATE INDEX `idx_sys_role_dept_dept_id` ON `sys_role_dept` (`dept_id`);

INSERT INTO `sys_role_dept` (`role_id`, `dept_id`) VALUES(7222985345490620417, 101);
INSERT INTO `sys_role_dept` (`role_id`, `dept_id`) VALUES(7222985345490620417, 103);
INSERT INTO `sys_role_dept` (`role_id`, `dept_id`) VALUES(7222985345490620417, 104);

DROP TABLE IF EXISTS `sys_role_menu`;
CREATE TABLE `sys_role_menu` (
  `role_id` INTEGER NOT NULL,
  `menu_id` INTEGER NOT NULL,
  PRIMARY KEY (`role_id`, `menu_id`)
);

CREATE INDEX `idx_sys_role_menu_menu_id` ON `sys_role_menu` (`menu_id`);

INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 1);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 1);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 1);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 1);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 2);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 2);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 2);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 2);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 3);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 3);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 4);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 4);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 4);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 5);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 5);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 5);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 6);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 6);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 6);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 6);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 7);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 7);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 7);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(1, 8);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 8);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 8);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 7224471859710005249);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 7225439414641627136);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 7228454585970593792);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 7402695499167208448);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 7402695499167208448);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(7222985345490620417, 7402695499167208448);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(3, 7402702074216879104);
INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES(4, 7402702074216879104);

DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `dept_id` INTEGER NOT NULL,
  `role_id` INTEGER NOT NULL,
  `user_name` TEXT NOT NULL,
  `nick_name` TEXT NOT NULL,
  `user_type` TEXT DEFAULT 'sys_user',
  `email` TEXT DEFAULT '',
  `phonenumber` TEXT DEFAULT '',
  `sex` TEXT DEFAULT '0',
  `avatar` TEXT,
  `password` TEXT NOT NULL DEFAULT '',
  `status` TEXT DEFAULT '0',
  `login_ip` TEXT DEFAULT '',
  `login_date` TEXT,
  `create_dept` INTEGER,
  `create_by` INTEGER,
  `created_at` TEXT,
  `update_by` INTEGER,
  `updated_at` TEXT,
  `deleted_at` TEXT,
  `remark` TEXT
);

INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(1, 100, 1, 'admin', '管理员', 'sys_user', 'willmagic@qq.com', '18588719996', '1', 'static/2024-10/25_7255598825434158080.png', '$argon2id$v=19$m=19456,t=2,p=1$M6+FVFGkM/zIAC7VZgGoLQ$zSAK9IIFN28ZsskO5s1eKKNv2g4yGw2b+CKjRa1cLvo', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', NULL, NULL, NULL, '管理员');
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(3, 104, 3, 'test', '测试1', 'sys_user', 'test@qq.com', '15888888889', '0', NULL, '$argon2id$v=19$m=19456,t=2,p=1$mO97IyGb5zSmoGrmB3ET1A$x8CKk5Z33siO3zVtMUm2AjMQsRrfNISa35S+sGmksf4', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 3, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(4, 102, 3, 'test1', '测试q', 'sys_user', 'test1@qq.com', '15888888887', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 4, '2024-07-03 01:17:31', '2024-08-24 00:21:07', NULL);
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(5, 102, 3, 'test2', '测试2', 'sys_user', 'test2@qq.com', '15888888886', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 4, '2024-07-03 01:17:31', NULL, NULL);
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(7222295451336314881, 103, 4, 'test3', '测试3', 'sys_user', 'test3@qq.com', '0588445337', '1', NULL, '$argon2id$v=19$m=19456,t=2,p=1$2ekLWIma6X91vcafwSnWmA$/l1xAacSwe3c/urIwRNTCb3l0hf1zDmUSdpAsm1V/ok', '0', '', '2024-07-03 01:17:31', NULL, NULL, '2024-07-03 01:17:31', NULL, NULL, NULL, NULL);
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(7222299781468655617, 104, 3, 'test4', '测试4', 'sys_user', 'test4@qq.com', '0588445337', '0', NULL, '$argon2id$v=19$m=19456,t=2,p=1$JJh+GICj7Ii7dnexWgNrZQ$h/MU+GhTX5O9SMu7p9S899QUBWLM8QPo3lpFk7KGars', '0', '', '2024-07-03 01:17:31', NULL, NULL, '2024-07-03 01:17:31', NULL, NULL, NULL, '1');
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(7223731680347033600, 7223731313261547520, 7222985345490620417, 'test5', 'test5', 'sys_user', 'test5@qq.com', '0588445337', NULL, NULL, '$argon2id$v=19$m=19456,t=2,p=1$VsofXU4ao39GasijfbQvjw$sASrh2idw664BmrRN8XPguppliyXbFmYJhvclNtSi3w', NULL, '', '2024-07-03 01:17:31', NULL, NULL, '2024-07-03 01:17:31', NULL, NULL, NULL, NULL);
INSERT INTO `sys_user` (`id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES(7402727913407026176, 101, 3, 'test6', 'test7', 'sys_user', 'test7@qq.com', NULL, NULL, NULL, '$argon2id$v=19$m=19456,t=2,p=1$gD8LZFl1wYA5iG8kf97ELw$pl5iL7eE7U5p97o6mu92vbft0VwsjbTa3iATXSKkJRw', '0', '', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);

DROP TABLE IF EXISTS `sys_user_dept`;
CREATE TABLE `sys_user_dept` (
  `user_id` INTEGER NOT NULL,
  `dept_id` INTEGER NOT NULL,
  PRIMARY KEY (`user_id`, `dept_id`)
);

CREATE INDEX `idx_sys_user_dept_dept_id` ON `sys_user_dept` (`dept_id`);

INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(1, 100);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(7402727913407026176, 101);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(1, 103);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(7222295451336314881, 103);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(3, 104);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(7222299781468655617, 104);
INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES(7223731680347033600, 7223731313261547520);

DROP TABLE IF EXISTS `sys_user_post`;
CREATE TABLE `sys_user_post` (
  `user_id` INTEGER NOT NULL,
  `post_id` INTEGER NOT NULL,
  PRIMARY KEY (`user_id`, `post_id`)
);

CREATE INDEX `idx_sys_user_post_post_id` ON `sys_user_post` (`post_id`);

INSERT INTO `sys_user_post` (`user_id`, `post_id`) VALUES(1, 1);

DROP TABLE IF EXISTS `sys_user_role`;
CREATE TABLE `sys_user_role` (
  `user_id` INTEGER NOT NULL,
  `role_id` INTEGER NOT NULL,
  PRIMARY KEY (`user_id`, `role_id`)
);

CREATE INDEX `idx_sys_user_role_role_id` ON `sys_user_role` (`role_id`);

INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(1, 1);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(1, 3);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(3, 3);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(7222299781468655617, 3);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(7402727913407026176, 3);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(4, 4);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(7222295451336314881, 4);
INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES(7223731680347033600, 7222985345490620417);

DROP TABLE IF EXISTS `sys_white_jwt`;
CREATE TABLE `sys_white_jwt` (
  `jid` INTEGER NOT NULL PRIMARY KEY,
  `uid` INTEGER NOT NULL,
  `token_id` INTEGER NOT NULL,
  `info_id` INTEGER NOT NULL,
  `token_expr` INTEGER NOT NULL,
  `created_at` TEXT NOT NULL,
  `update_at` TEXT NOT NULL,
  `deleted_at` TEXT
);

INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7338250336651351040, 1, 7338250336584242176, 7338250336584242177, 1750180018, '2025-06-11 01:06:58', '2025-06-11 01:06:58', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7348334080116560896, 1, 7348334080074617856, 7348334080074617857, 1752584170, '2025-07-08 20:56:10', '2025-07-08 20:56:10', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7350937685579437056, 1, 7350937685495550976, 7350937685495550977, 1753204918, '2025-07-16 01:21:58', '2025-07-16 01:21:58', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7351264641399297024, 1, 7351264641348965376, 7351264641348965377, 1753282870, '2025-07-16 23:01:10', '2025-07-16 23:01:10', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7395848975468631040, 1, 7395848975426688000, 7395848975426688001, 1763912603, '2025-11-16 23:43:24', '2025-11-16 23:43:24', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402412372016403456, 1, 7402412371378869248, 7402412371378869249, 1765477439, '2025-12-05 02:24:00', '2025-12-05 02:24:00', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402695033029039104, 1, 7402695032978707456, 7402695032978707457, 1765544831, '2025-12-05 21:07:11', '2025-12-05 21:07:11', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402709166579422208, 1, 7402709166432621568, 7402709166432621569, 1765548200, '2025-12-05 22:03:21', '2025-12-05 22:03:21', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402714693157950464, 7222295451336314881, 7402714693057287168, 7402714693057287169, 1765549518, '2025-12-05 22:25:19', '2025-12-05 22:25:19', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402715852635542528, 7222295451336314881, 7402715852585210880, 7402715852585210881, 1765549795, '2025-12-05 22:29:55', '2025-12-05 22:29:55', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402728157268055040, 7402727913407026176, 7402728157159003136, 7402728157159003137, 1765552728, '2025-12-05 23:18:49', '2025-12-05 23:18:49', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402749118914663424, 1, 7402749118885303296, 7402749118889497600, 1765557726, '2025-12-06 00:42:06', '2025-12-06 00:42:06', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7402749256496223232, 7222295451336314881, 7402749256399754240, 7402749256399754241, 1765557759, '2025-12-06 00:42:39', '2025-12-06 00:42:39', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7403508110926451712, 1, 7403508110842565632, 7403508110842565633, 1765738684, '2025-12-08 02:58:04', '2025-12-08 02:58:04', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7403508317714027520, 7402727913407026176, 7403508317600781312, 7403508317604975616, 1765738733, '2025-12-08 02:58:53', '2025-12-08 02:58:53', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7408879230387131392, 1, 7408879230340994048, 7408879230340994049, 1767019258, '2025-12-22 22:40:59', '2025-12-22 22:40:59', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7447696444581909504, 1, 7447696444028261376, 7447696444028261377, 1776274003, '2026-04-09 01:26:44', '2026-04-09 01:26:44', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7451645776330920960, 1, 7451645775789855744, 7451645775789855745, 1777215597, '2026-04-19 22:59:58', '2026-04-19 22:59:58', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7461085845278069760, 1, 7461085845202572288, 7461085845202572289, 1779466285, '2026-05-16 00:11:26', '2026-05-16 00:11:26', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7461092002507953152, 1, 7461092002415678464, 7461092002419872768, 1779467753, '2026-05-16 00:35:54', '2026-05-16 00:35:54', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7467279097853285376, 1, 7467279097819730944, 7467279097819730945, 1780942872, '2026-06-02 02:21:12', '2026-06-02 02:21:12', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7467616830538814464, 1, 7467616830505260032, 7467616830505260033, 1781023394, '2026-06-03 00:43:14', '2026-06-03 00:43:14', NULL);
INSERT INTO `sys_white_jwt` (`jid`, `uid`, `token_id`, `info_id`, `token_expr`, `created_at`, `update_at`, `deleted_at`) VALUES(7467626537966015488, 1, 7467626537877935104, 7467626537877935105, 1781025708, '2026-06-03 01:21:48', '2026-06-03 01:21:48', NULL);

DROP TABLE IF EXISTS `test_api`;
CREATE TABLE `test_api` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT NOT NULL,
  `age` INTEGER NOT NULL,
  `email` TEXT NOT NULL,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO `test_api` (`id`, `name`, `age`, `email`, `created_at`, `updated_at`) VALUES(7402976005653959680, '韶颖', 86, 'uclgm3.tny@139.com', '2025-12-06 15:43:40', '2025-12-06 15:43:40');

DROP TABLE IF EXISTS `test_article`;
CREATE TABLE `test_article` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `category_id` INTEGER NOT NULL,
  `title` TEXT NOT NULL,
  `content` TEXT NOT NULL,
  `author` TEXT,
  `password` TEXT NOT NULL,
  `is_published` INTEGER NOT NULL DEFAULT 0,
  `view_count` INTEGER NOT NULL DEFAULT 0,
  `download_count` INTEGER NOT NULL DEFAULT 0,
  `rating` REAL NOT NULL DEFAULT '0',
  `cover` TEXT,
  `published_at` TEXT,
  `created_at` TEXT,
  `updated_at` TEXT
);

CREATE INDEX `idx_test_article_idx_category_id` ON `test_article` (`category_id`);

DROP TABLE IF EXISTS `test_category`;
CREATE TABLE `test_category` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT NOT NULL,
  `description` TEXT,
  `sort` INTEGER NOT NULL DEFAULT 0,
  `status` TEXT NOT NULL DEFAULT 'active',
  `cover` TEXT,
  `is_active` INTEGER NOT NULL DEFAULT 1,
  `weight` INTEGER NOT NULL DEFAULT 0,
  `view_count` INTEGER NOT NULL DEFAULT 0,
  `created_at` TEXT,
  `updated_at` TEXT
);

INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(1, '技术', '技术相关文章', 1, 'active', '', 1, 10, 0, NULL, '2026-05-20 01:50:32');
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(2, '生活', '生活随笔', 2, 'active', 'http://localhost:5001/static/uploads/2026-05/20-7462542739511350272.png', 1, 8, 0, NULL, '2026-05-20 00:40:38');
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(3, '产品', '产品设计相关', 3, 'active', NULL, 1, 5, 0, NULL, NULL);
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(7462560992648401920, '游戏', '游戏', 1, 'active', NULL, 0, 1, 1, NULL, NULL);
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(7462561771895559168, '音乐', '音乐', 5, 'active', NULL, 0, 1, 1, '2026-05-20 01:56:14', '2026-05-20 01:56:14');
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(7462563304137724928, '驾驭', '驾驭', 0, 'active', NULL, 0, 0, 1, '2026-05-20 02:02:19', '2026-05-20 02:02:19');
INSERT INTO `test_category` (`id`, `name`, `description`, `sort`, `status`, `cover`, `is_active`, `weight`, `view_count`, `created_at`, `updated_at`) VALUES(7462563461537371136, '但凡', '但凡', 0, 'active', NULL, 0, 0, 0, '2026-05-20 02:02:57', '2026-05-20 02:02:57');

DROP TABLE IF EXISTS `test_data_scope`;
CREATE TABLE `test_data_scope` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `title` TEXT NOT NULL,
  `content` TEXT,
  `dept_id` INTEGER NOT NULL,
  `owner_id` INTEGER NOT NULL,
  `status` TEXT NOT NULL DEFAULT '0',
  `created_at` TEXT NOT NULL,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `deleted_at` TEXT
);

CREATE INDEX `idx_test_data_scope_idx_test_data_scope_dept_id` ON `test_data_scope` (`dept_id`);

CREATE INDEX `idx_test_data_scope_idx_test_data_scope_owner_id` ON `test_data_scope` (`owner_id`);

INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402711480572744704, '胡说并列于长略微整齐耶耶耶渐渐画渐渐反正', 'cillum anim do nulla magna', 103, 7222295451336314881, '0', '2025-12-05 22:12:33', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402712010544026624, '攥广告此外不像嗯油门却一齐尽管直到', 'incididunt laboris ipsum irure', 72, 35, '0', '2025-12-05 22:14:39', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402712077673862144, '摘擅自共枯萎笔直长椅', 'elit sed velit quis', 103, 7222295451336314881, '0', '2025-12-05 22:14:55', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402712087106851840, '出生写搜一同面对甚至拨多轻松', 'ea', 55, 72, '0', '2025-12-05 22:14:57', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402712096971854848, '耶耶耶统统坐抄勇敢哎呀关于揽', 'adipisicing', 56, 89, '0', '2025-12-05 22:15:00', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402712106258043904, '特地大哎哟朝向以呼必定', 'veniam', 33, 15, '0', '2025-12-05 22:15:02', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402749020059112448, '特地大哎哟朝向以呼必定', 'veniam', 33, 15, '0', '2025-12-06 00:41:43', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402749295490667520, '特地大哎哟朝向以呼必定', 'veniam', 103, 7222295451336314881, '0', '2025-12-06 00:42:48', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7402749517151245312, '哦糟糕嘘除了', 'in consequat exercitation velit', 103, 7222295451336314881, '0', '2025-12-06 00:43:41', '2025-12-08 01:19:17', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7403506317010048000, 'fgdfsg ', 'sdfgsdfg123', 103, 7222295451336314881, '0', '2025-12-08 02:50:56', '2025-12-08 02:56:49', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7403508171047605248, '955', '456', 100, 1, '0', '2025-12-08 02:58:19', '2025-12-08 02:58:19', NULL);
INSERT INTO `test_data_scope` (`id`, `title`, `content`, `dept_id`, `owner_id`, `status`, `created_at`, `updated_at`, `deleted_at`) VALUES(7403508420336063488, '测试6', '测试6的数据', 101, 7402727913407026176, '0', '2025-12-08 02:59:18', '2025-12-08 02:59:18', NULL);

DROP TABLE IF EXISTS `wx_accounts`;
CREATE TABLE `wx_accounts` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `app_id` TEXT NOT NULL,
  `app_secret` TEXT NOT NULL,
  `account_name` TEXT NOT NULL,
  `account_type` INTEGER NOT NULL,
  `original_id` TEXT,
  `wechat_id` TEXT,
  `status` INTEGER,
  `message_mode` INTEGER NOT NULL,
  `access_token` TEXT,
  `token_expires_at` TEXT,
  `server_url` TEXT,
  `token` TEXT,
  `encoding_aes_key` TEXT,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (`app_id`)
);

DROP TABLE IF EXISTS `wx_auto_replies`;
CREATE TABLE `wx_auto_replies` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `reply_type` INTEGER NOT NULL,
  `keyword` TEXT,
  `match_type` INTEGER,
  `message_type` TEXT NOT NULL,
  `content` TEXT,
  `media_id` TEXT,
  `title` TEXT,
  `description` TEXT,
  `pic_url` TEXT,
  `url` TEXT,
  `music_url` TEXT,
  `hq_music_url` TEXT,
  `thumb_media_id` TEXT,
  `status` INTEGER,
  `priority` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_auto_replies_account_id` ON `wx_auto_replies` (`account_id`);

DROP TABLE IF EXISTS `wx_materials`;
CREATE TABLE `wx_materials` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `media_type` TEXT NOT NULL,
  `media_id` TEXT,
  `name` TEXT,
  `url` TEXT,
  `local_path` TEXT,
  `file_size` INTEGER,
  `content_type` TEXT,
  `width` INTEGER,
  `height` INTEGER,
  `duration` INTEGER,
  `description` TEXT,
  `title` TEXT,
  `introduction` TEXT,
  `thumb_media_id` TEXT,
  `thumb_url` TEXT,
  `content_source_url` TEXT,
  `digest` TEXT,
  `author` TEXT,
  `content` TEXT,
  `news_items` TEXT,
  `is_permanent` INTEGER NOT NULL DEFAULT 0,
  `sync_status` INTEGER NOT NULL DEFAULT 0,
  `synced_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_materials_account_id` ON `wx_materials` (`account_id`);

DROP TABLE IF EXISTS `wx_menus`;
CREATE TABLE `wx_menus` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `parent_id` INTEGER,
  `menu_name` TEXT NOT NULL,
  `menu_type` TEXT NOT NULL,
  `menu_key` TEXT,
  `url` TEXT,
  `media_id` TEXT,
  `appid` TEXT,
  `pagepath` TEXT,
  `article_id` TEXT,
  `sort_order` INTEGER,
  `status` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_menus_account_id` ON `wx_menus` (`account_id`);

DROP TABLE IF EXISTS `wx_messages`;
CREATE TABLE `wx_messages` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `openid` TEXT NOT NULL,
  `msg_id` INTEGER,
  `msg_type` TEXT NOT NULL,
  `direction` INTEGER NOT NULL,
  `content` TEXT,
  `media_id` TEXT,
  `pic_url` TEXT,
  `voice_format` TEXT,
  `recognition` TEXT,
  `thumb_media_id` TEXT,
  `msg_title` TEXT,
  `msg_description` TEXT,
  `link_url` TEXT,
  `event_type` TEXT,
  `event_key` TEXT,
  `reply_msg_id` INTEGER,
  `is_auto_reply` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_messages_account_id` ON `wx_messages` (`account_id`);

DROP TABLE IF EXISTS `wx_pay_orders`;
CREATE TABLE `wx_pay_orders` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `out_trade_no` TEXT NOT NULL,
  `transaction_id` TEXT,
  `openid` TEXT NOT NULL,
  `body` TEXT NOT NULL,
  `total_fee` INTEGER NOT NULL,
  `fee_type` TEXT,
  `trade_type` TEXT NOT NULL,
  `spbill_create_ip` TEXT,
  `notify_url` TEXT NOT NULL,
  `prepay_id` TEXT,
  `code_url` TEXT,
  `mch_id` TEXT NOT NULL,
  `trade_state` TEXT,
  `trade_state_desc` TEXT,
  `attach` TEXT,
  `time_expire` TEXT,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_pay_orders_account_id` ON `wx_pay_orders` (`account_id`);

DROP TABLE IF EXISTS `wx_pay_refunds`;
CREATE TABLE `wx_pay_refunds` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `out_trade_no` TEXT NOT NULL,
  `out_refund_no` TEXT NOT NULL,
  `transaction_id` TEXT,
  `refund_id` TEXT,
  `total_fee` INTEGER NOT NULL,
  `refund_fee` INTEGER NOT NULL,
  `refund_reason` TEXT,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_pay_refunds_account_id` ON `wx_pay_refunds` (`account_id`);

DROP TABLE IF EXISTS `wx_template_logs`;
CREATE TABLE `wx_template_logs` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `template_id` TEXT NOT NULL,
  `openid` TEXT NOT NULL,
  `content` TEXT,
  `miniprogram_appid` TEXT,
  `miniprogram_pagepath` TEXT,
  `msg_id` INTEGER,
  `errcode` INTEGER,
  `errmsg` TEXT,
  `status` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_template_logs_account_id` ON `wx_template_logs` (`account_id`);

DROP TABLE IF EXISTS `wx_templates`;
CREATE TABLE `wx_templates` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `template_id` TEXT NOT NULL,
  `title` TEXT,
  `industry` TEXT,
  `content` TEXT,
  `example` TEXT,
  `template_type` TEXT,
  `status` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_templates_account_id` ON `wx_templates` (`account_id`);

DROP TABLE IF EXISTS `wx_users`;
CREATE TABLE `wx_users` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `account_id` INTEGER NOT NULL,
  `openid` TEXT NOT NULL,
  `unionid` TEXT,
  `nickname` TEXT,
  `sex` INTEGER,
  `city` TEXT,
  `country` TEXT,
  `province` TEXT,
  `language` TEXT,
  `headimgurl` TEXT,
  `subscribe_time` TEXT,
  `unsubscribe_time` TEXT,
  `subscribe_status` INTEGER,
  `remark` TEXT,
  `subscribe_scene` TEXT,
  `qr_scene` TEXT,
  `qr_scene_str` TEXT,
  `last_interact_time` TEXT,
  `message_count` INTEGER,
  `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX `idx_wx_users_account_id` ON `wx_users` (`account_id`);

COMMIT;