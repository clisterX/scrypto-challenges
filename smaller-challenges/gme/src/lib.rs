use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
       // Your state variables go here.
       gme_vault: Vault,                // Vault for our token
       xrd_vault: Vault,                // Vault for earned xrd
       price_per_token: Decimal         // Price
   }
 
   impl TokenSale {
       pub fn new(price_per_token: Decimal) -> (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Our GME Token")
                .metadata("symbol", "GME")
                .metadata("team-member-1-ticket-number","4005113449")   // Ticket Nb Teammember 1
                .metadata("team-member-2-ticket-number","4066174309")   // Ticket Nb Teammember 2
                .metadata("team-member-3-ticket-number","4066208769")   // Ticket Nb Teammember 3
                .metadata("team-member-4-ticket-number","4133531439")   // Ticket Nb Teammember 4
                .divisibility(DIVISIBILITY_MAXIMUM)
                .initial_supply(100_000);
            
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "GME Seller Badge")
                .metadata("symbol", "GME SELLER")
                .initial_supply(1);


            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));
            
            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component_address: ComponentAddress = Self {
                gme_vault: Vault::with_bucket(bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            (component_address, seller_badge)
        }
 
       pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purcased_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_vault.put(funds);
            self.gme_vault.take(purcased_amount)
       }
 
       pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_vault.take(amount)
       }
 
       pub fn change_price(&mut self, price: Decimal) {
        self.price_per_token = price
       }
   }
}
