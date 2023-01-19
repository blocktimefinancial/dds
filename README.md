# dds
Dividend Distribution Service

So we've learned a few things so far.  We can't query the "classic" blockchain to get the accounts for an asset.  This is how we currently make dividend and PIK distributions.  So we're going to explore a more claimable balance type model where we place the accounts/amounts of the asset holder into the smart contract, place a distribution amount of an asset into the contract and let accounts in the list withdraw a pro-rata amount.