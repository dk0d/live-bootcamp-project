--- Database Schema Definition
---
--- Define your database schema here using the shki Lua API.
--- Run `shki generate` in the project directory to create migrations.
---
--- For IDE support (autocomplete, type checking, hover docs), install the
--- Lua Language Server extension. The .luarc.json is already configured.

local schema = pg.schema("public")
local Table = TableBuilder
local Col = ColumnBuilder

-- Example: Define a users table
schema:table(
	Table.new("user")
		:description("User accounts")
		:column(Col.text("email"):primary_key())
		:column(Col.text("password_hash"):not_null())
		:column(Col.text("two_factor"):default_value("none"):not_null())
)

-- schema:table(
-- 	Table.new("address")
-- 		:description("User accounts")
-- 		:column(Col.uuid("id"))
-- 		:column(Col.uuid("name"))
-- 		:column(Col.text("street"):not_null())
-- 		:primary_key("id", "name") -- composite primary key
-- )

-- Example: Define an enum type (PostgreSQL only)
-- schema:enum_type(
-- 	EnumBuilder.new("status"):description("Record status"):value("active"):value("inactive"):value("archived")
-- )

return schema
