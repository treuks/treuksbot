# TreuKSBot
**The reu is pronounced like (/ˈ r uː/)** 
--
(WIP!!!) TreuKSBot is **(currently)** a generic purpose bot written in Rust, which uses the twitch-irc-rs library

Sorry if the code is shit, this is my first semi big rust project, and my first time actually using API's

## How to use
At this moment, the bot is **linux only**, because it is hardcoded to a specific config spot, which is currently **~/.config/treuksbot/secret.toml**

You can change the directory in the **main.rs** file to make it work, but i have no guarantee that it will actually work.

## Step 1: Storing the credentials.
If you are on Linux, you can easily make the bot work by following commands below:

1. `cd ~/.config` 
2. `mkdir treuksbot`

At this point you should copy **the example secret file** from the root of the repo, called **`EXAMPLE_SECRET_FILE.toml`** and paste it into the treuksbot folder.

After doing that, you should rename it into secret.toml, and replace the examples with their nonexample counterparts.

##

Currently, you will need the following lines:

| Parameters                          | Type          | Explanation                                                                                       |
| ----------------------------------- | ------------- | ------------------------------------------------------------------------------------------------- |
| [secret]                            | None          | It is a **hash table**                                                                            |
| owner = "\*\*\*\*"                  | String        | You need to put in your name here                                                                 |
| login = "\*\*\*\*"                  | String        | The bot's name needs to go here                                                                   |
| oauth = "\*\*\*\*"                  | String        | **It is very important**, if you don't know where to get one, you should look [at some twitch docs](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth)                                                                                          |
| client_id = "\*\*\*\*"              | String        | You need to insert the **id** of the channel you are hosting for here                             |
| client_secret = "\*\*\*\*"          | String        | You can get the bot's secret [here](https://dev.twitch.tv/)                                       |
| channel_name = "\*\*\*\*"           | String        | Insert the name of the channel you're hosting the bot for here                                    |
| openweather_oauth = "\*\*\*\*"      | String        | Insert the OpenWeatherMap api OAUTH key here                                                      |

##

After doing that, you should compile the bot somewhere, and execute the binary you get.

Required: **A modern version of rust**, **Some kind of way to clone the repo** (git, or perhaps github cli)

You can do so by using the following commands:

## Step 2: Cloning the repository.
**If you use git:**
 `git clone https://github.com/NS2555/treuksbot.git` 

**If you use the Github CLI:**
 `gh repo clone NS2555/treuksbot` 
## Step 3: Compiling the bot.
This step is very easy, due to the Rust™️ Cargo™️

1. `cd treuksbot`
2. `cargo build --release` (You can use `cargo run` if you're just trying to test, it will instantly run the bot after building it)

After it is done compiling, **you can find the binary in `treuksbot/target/release`**

**And you are done!**

