# School Sanction Gambling

by School Sanctioned Gambling team

## Team Members
* Advanced Topic Subteam 1: Advanced AI
	* Garrett
	* Makye
	* Sam
	* Matthew

* Advanced Topic Subteam 2: Online Multiplayer
	* Alex
	* Maria
	* Griffin

## Game Description

School Sanctioned Gambling is a Texas Hold'em Poker Game. It has two modes: Online Multiplayer and Local Play. In Online Multiplayer, players can join a pool and play against other people online. In Local Play, players can face off against a robust AI system to hone their skills or have fun offline.

## Advanced Topic Description

### Advanced AI

There are two difficulties of AI in the game: Easy and Hard. Easy difficulty uses a combination of randomness and basic decision tree logic to decide what moves to make, and is ideal for newer players. Hard difficulty uses Counterfactual Regret Minimization so the AI will decide the optimal move based on current cards held and shown as the game progresses.
 
### Online Multiplayer

The game will use a custom dedicated server to manage online multiplayer networking needs. We use a lobby-based system, where players can create and join lobbies to play against each other. Players can create unique lobby codes to play against friends in pools of 2 to 6 people.

## Midterm Goals

* Working start screen and menu
* Get simple messages going back and forth over the server using some approved external framework
* Playing a whole game against a single easy AI (with AI cards shown)

## Final Goals

* Saving and loading local games
    * 2%: Players can select the starting amount of money at the beginning of a game
	* 2%: Players can still play offline when there is no internet connection
	* 6%: Players can save a game and load it later to continue playing without any loss of state
* Local play with both AI difficulty options
    * Able to play full local games against both AI difficulty modes
	    * 10%: Player can choose to play against any number of Easy or Hard difficulty AIs (up to 6 total players). Matches follow the standard rules of Texas Hold'em and players can continue as long as they have at least the minumum starting bet for a match.
	    * 5%: Easy difficulty should act according to a simple decision tree with the knowledge of its
		own hand and poker hand probabilities, with random variation to avoid predictability. 
        * 20%: Hard difficulty will utilize the algorithms from [this paper](http://modelai.gettysburg.edu/2013/cfr/cfr.pdf) and transform them to apply to Texas Hold'em.
* Fully functional online multiplayer games (2-6 players)
    * 20%: Dedicated server capable of supporting joinable lobbies and displaying available ones to clients (2-6 connections)
	* 10%: Players can create and join private lobbies (2-6 players) via a unique join code. Lobbies should time out after prolonged inactivity or close after everyone leaves. The creator of the lobby starts the game. Subsequent connections are delayed until the end of a match, or refused if the maximum number of players are already in the lobby.
	* 5%: Online matches conform to the standard rules of Texas Hold'em

## Stretch Goals

* AI benchmark and Unfair mode: Hard AI is benchmarked by comparison to a cheating/card counting AI. Players can face off against this cheating AI in "Unfair mode".
* Multiplayer login functionality for leaderboards (wins, losses, and chips won)
