CREATE TABLE "Accounts" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"type"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"currency"	TEXT NOT NULL,
	FOREIGN KEY("currency") REFERENCES "Currency"("code")
)

CREATE TABLE "Credits" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER NOT NULL,
	"transaction_id"	INTEGER NOT NULL,
	"balance"	INTEGER NOT NULL DEFAULT 0 CHECK (typeof("balance") = 'integer'),
	FOREIGN KEY("account") REFERENCES "Accounts"("id"),
	FOREIGN KEY("transaction_id") REFERENCES "Transactions"("id") ON DELETE CASCADE
)

CREATE TABLE "Debits" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER NOT NULL,
	"transaction_id"	INTEGER NOT NULL,
	"balance"	INTEGER NOT NULL DEFAULT 0 CHECK (typeof("balance") = 'integer'),
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

CREATE TABLE "Budgets" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"name"	TEXT,
	"open"	TEXT NOT NULL,
	"close"	TEXT NOT NULL,
)

CREATE TABLE "BudgetExample" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER NOT NULL UNIQUE,
	"balance"	INTEGER NOT NULL DEFAULT 0
)


CREATE TABLE "BudgetEntries" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER,
	"budget"	INTEGER,
	"balance"	INTEGER,
	FOREIGN KEY("budget") REFERENCES "Budgets"("id") ON DELETE CASCADE,
	FOREIGN KEY("account") REFERENCES "Accounts"("id"),
	UNIQUE("account", "budget")
)
