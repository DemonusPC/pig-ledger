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

CREATE TABLE "BudgetEntries" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"account"	INTEGER,
	"budget"	INTEGER,
	"balance"	INTEGER,
	FOREIGN KEY("budget") REFERENCES "Budgets"("id") ON DELETE CASCADE,
	FOREIGN KEY("account") REFERENCES "Accounts"("id"),
	UNIQUE("account", "budget")
)

CREATE TABLE "AccountsV2" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"type"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"balance" INTEGER NOT NULL DEFAULT 0,
	"currency"	TEXT NOT NULL,
	FOREIGN KEY("currency") REFERENCES "Currency"("code")
)

INSERT INTO "main"."AccountsV2"
("id", "type", "name", "currency")
VALUES (0, 0, 'Assets', 'XXX');

INSERT INTO "main"."AccountsV2"
("id", "type", "name", "currency")
VALUES (1, 1, 'Liabilities', 'XXX');

INSERT INTO "main"."AccountsV2"
("id", "type", "name", "currency")
VALUES (2, 2, 'Equities', 'XXX');

INSERT INTO "main"."AccountsV2"
("id", "type", "name", "currency")
VALUES (3, 3, 'Revenue', 'XXX');

INSERT INTO "main"."AccountsV2"
("id", "type", "name", "currency")
VALUES (4, 4, 'Expenses', 'XXX');
