### My personal telegram bot written in Rust using [Teloxide](https://github.com/teloxide/teloxide).

## Features

- Currently supports Expense Tracker Mode.
- Add multiple persons and add/track/settle due of all person.
- Currently supports single user mode, i.e. Bot will not work for any user other then ID mentioned in `.env` file. (`MYID`).

## Demo

<details>
  <summary>Click to see video demos</summary>
  <br>
  - Start Bot
  <br>
  <img src='/assets/start_bot.gif'></img>
  <br>
  <br>
  - Add Person
  <br>
  <img src='/assets/add_person.gif'></img>
  <br>
  <br>
  - Add Transaction
  <br>
  <img src='/assets/add_transaction.gif'></img>
  <br>
  <br>
  - Settle Transaction
  <br>
  <img src='/assets/settle_due.gif'></img>
  <br>
  <br>
</details>

## Deployment

- As this bot is currently only works in single user mode, This bot needs tobe deployed in your server if you want to use it.
- Deployments steps are described below.

0. Setup Rust toolchain from [here](https://www.rust-lang.org/tools/install) and verify installation.
1. Generate Bot Token using [BotFather](https://t.me/BotFather) from Telegram. Your generated token will be used as `BOT_TOKEN` in `.env` file.
2. Populate `.env` file with appropriate values (Look for `.env.example` in source and create `.env` from that example).
3. Do `cargo build -r` to build this project in release mode.
4. Do `cargo run -r` to start this project.
