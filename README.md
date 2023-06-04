# Instantiate2 Accounts

This repo explores how Instantiate2 can be used to create "named" contracts. This means an address can instantiate in a deterministic way that is associated with a salt, which is considered the name.

External contracts can confirm this contract's creator in an additional way, if they wish, but that's a novelty. The power of named accounts is in creating "application accounts" for other dApps.

## Example scenario

My game dApp would like to create an account for each player. The game and the player are able to call it. We'd like to be able to "discover" the contract address, if it exists, for a particular user. Knowing the checksum of this contract and the player, we can see if it exists and call it. Furthermore, our dApp can confirm if that contract is calling the game, and treat it as if it's the user. This allows for automation and other interesting aspects inherent in contract accounts/wallets.
