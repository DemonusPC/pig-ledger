# Anatomy of a Transaction

A `Transaction` is a single recorded movement between two or more accounts. 

Despite being made up of entries, which are smaller units, the api will always deal with with transactions as if it was a single unit. This matches directly with the SQL operations, since adding entries and transactions will happen in a single `database transaction`.

## Contents
A single transaction is made out of:
- id 
- date
- name
- set of `Entry` **pairs**

The first 3 fields are straightforward. The id is a unique identifier for this transaction. The date is an ISO timestamp of when the transaction happened. And a name is a simple 255 character string that should contain a simple description of the transaction. 

The set of Entry pairs is important.

## Entry
An `Entry` is the smallest unit of movement in PIG ledger. It represents a singular movement from a single account. It has two types:
- Debit
- Credit

An entry doesn't live on its own. It has to have a corresponding Entry and be correlated to a transaction. For every single debit entry there has to be a credit entry.

For Asset Accounts a `debit` means an increase in the balance of that account, and a `credit` means a decrease in the balance of that account. 

## Simple Transactions

The simplest transactions are transactions between two accounts.
For example we want to transfer `100 MEMES` between accounts `A` and `B`, both of which hold the currency `MEMES`. Such a transaction would look like this:
```
A -> 100 MEMES -> B
```
Inside the PIG ledger the system would store 2 entries:
```
A -> 100 MEMES // A credit of a 100 MEMES
B <- 100 MEMES // A debit of a 100 MEMES
```
To put everything together we would get
```
Transaction
id: 420
date: 1969-01-09T12:00:00.657468100Z
name: "Maymay transfer"
entries:
  - A -> 100 MEMES
  - B -> 100 MEMES
```


