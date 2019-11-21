CREATE TABLE "Accounts" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"Type"	INTEGER NOT NULL,
	"Name"	TEXT NOT NULL
)

CREATE TABLE "Credits" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER NOT NULL,
	"transaction_id"	INTEGER NOT NULL,
	"balance"	REAL NOT NULL DEFAULT 0.0,
	FOREIGN KEY("account") REFERENCES "Accounts"("id"),
	FOREIGN KEY("transaction_id") REFERENCES "Transactions"("id") ON DELETE CASCADE
)

CREATE TABLE "Debits" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER NOT NULL,
	"transaction_id"	INTEGER NOT NULL,
	"balance"	REAL NOT NULL DEFAULT 0.0,
	FOREIGN KEY("account") REFERENCES "Accounts"("id"),
	FOREIGN KEY("transaction_id") REFERENCES "Transactions"("id") ON DELETE CASCADE
)

CREATE TABLE "Transactions" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"date"	TEXT NOT NULL,
	"name"	TEXT
)

CREATE TABLE "Currency" (
	"code"	TEXT NOT NULL UNIQUE,
	"numeric_code"	INTEGER NOT NULL UNIQUE,
	"minor_unit"	INTEGER NOT NULL DEFAULT 2,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("code")
)