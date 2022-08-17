//! This contract implements (...functionality...) on blockchain.
//!
//! [function_name]: description of function
//! [function_name]: description of function

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseResult,
};
//use near_sdk::json_types::U128;
use near_sdk::ext_contract;
use near_sdk::Promise;

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    fn ft_balance_of(&self, account_id: String) -> Balance;
}

// #[ext_contract(ext_fungible_token)]
// pub trait FungibleTokenContract {
//     fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

//     fn ft_transfer_call(
//         &mut self,
//         receiver_id: AccountId,
//         amount: U128,
//         memo: Option<String>,
//         msg: String,
//     ) -> PromiseOrValue<U128>;

//     /// Returns the total supply of the token in a decimal string representation.
//     fn ft_total_supply(&self) -> U128;

//     /// Returns the balance of the account. If the account doesn't exist must returns `"0"`.
//     fn ft_balance_of(&self, account_id: AccountId) -> U128;
// }

//near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PaymentSplitter {
    _owner: AccountId,
    _payee_count: u128,
    _total_shares: Balance,
    _total_released: Balance,

    _balance_of: Balance,

    _payees2: Vector<u128>,
    _payees: Vector<AccountId>,

    _shares: UnorderedMap<AccountId, Balance>,
    _released: UnorderedMap<AccountId, Balance>,
    _payee_name: UnorderedMap<AccountId, String>,
    _target_chain: UnorderedMap<AccountId, String>,
    _target_wallet: UnorderedMap<AccountId, String>,
    _payee_count_to_payee_address: UnorderedMap<u128, AccountId>,

    _erc20_released: UnorderedMap<AccountId, UnorderedMap<AccountId, Balance>>,
    _erc20_total_released: UnorderedMap<AccountId, Balance>,
}

// mapping(IERC20 => uint256) private _erc20TotalReleased;
// mapping(IERC20 => mapping(address => uint256)) private _erc20Released;

#[near_bindgen]
impl PaymentSplitter {
    /// Contract Initializer
    /// Behaves like a constructor in solidity
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        log!("PaymentSplitter Initialized!");
        Self {
            _owner: env::predecessor_account_id(),
            _payee_count: 0,
            _total_shares: 0,
            _total_released: 0,

            _balance_of: 0,

            _payees: Vector::new(b"a"),
            _payees2: Vector::new(b"b"),

            _shares: UnorderedMap::new(b"c"),
            _released: UnorderedMap::new(b"d"),
            _payee_name: UnorderedMap::new(b"e"),
            _target_chain: UnorderedMap::new(b"f"),
            _target_wallet: UnorderedMap::new(b"g"),
            _payee_count_to_payee_address: UnorderedMap::new(b"h"),

            _erc20_released: UnorderedMap::new(b"i"),
            _erc20_total_released: UnorderedMap::new(b"j"),
        }
    }
    /**
     * Returns the address of the current owner.
     */
    pub fn owner(self) -> AccountId {
        self._owner
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    pub fn renounce_ownership(&mut self) {
        let no_owner: AccountId = "".parse().unwrap();
        self._only_owner();
        self._transfer_ownership(no_owner);
    }
    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        self._only_owner();
        require!(new_owner != "".parse().unwrap(), "Owner is zero address");
        self._transfer_ownership(new_owner);
    }
    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    fn _transfer_ownership(&mut self, new_owner: AccountId) {
        //let oldOwner = &self._owner;
        self._owner = new_owner;
        //&oldOwner
        //emit OwnershipTransferred(oldOwner, newOwner);
    }
    /**
     * @dev Throws if the sender is not the owner.
     */
    fn _only_owner(&self) {
        require!(
            env::current_account_id() == self._owner,
            "Ownable: caller is not the owner"
        );
    }

    /**
     * @dev Provides information about the current execution context, about the
     * sender of the transaction. While generally available
     * via msg.sender this should not be accessed in such a direct
     * manner, since when dealing with meta-transactions the account sending and
     * paying for execution may not be the actual sender (as far as an application
     * is concerned).
     */
    fn _msg_sender() -> AccountId {
        env::predecessor_account_id()
    }

    /**
     * @dev The Ether received will be logged with {PaymentReceived} events. Note that these
     * events are not fully reliable: it's possible for a contract to receive Ether without
     * triggering this function. This only affects the reliability of the events, and not the
     * actual splitting of Ether.
     *
     * To learn more about this see the Solidity documentation for
     * https://solidity.readthedocs.io/en/latest/contracts.html#fallback-function[fallback
     * functions].
     */
    // Events are not yet fully supported *****
    // #[payable]
    // receive() external payable virtual {
    //     emit PaymentReceived(_msgSender(), msg.value);
    // }

    /**
     * @dev Getter for the total shares held by payees.
     */
    pub fn total_shares(&self) -> Balance {
        self._total_shares
    }

    /**
     * @dev Getter for the total amount of Ether already released.
     */
    pub fn total_released(&self) -> Balance {
        self._total_released
    }

    /**
     * @dev Getter for the total amount of `token` already released. `token` should be the
     * address of an IERC20 contract.
     */
    //START HERE AND RENAME SAME NAME FUNCS/////////*******
    pub fn total_released_erc(&self, token: &AccountId) -> Balance {
        self._erc20_total_released.get(token).unwrap()
    }

    /**
     * @dev Getter for the amount of shares held by an account.
     */
    pub fn shares(&self, account: AccountId) -> Balance {
        self._shares.get(&account).unwrap()
    }

    /**
     * @dev Getter for the amount of Ether already released to a payee.
     */
    pub fn released(&self, account: &AccountId) -> Balance {
        self._released.get(account).unwrap()
    }

    /**
     * @dev Getter for the amount of `token` tokens already released to a payee. `token`
     * should be the address of an IERC20 contract.
     */
    pub fn released_erc(&self, token: &AccountId, account: &AccountId) -> Balance {
        match self._erc20_released.get(token) {
            Some(value) => match value.get(account) {
                Some(val) => val,
                None => 0,
            },
            None => 0,
        }
    }

    /**
     * @dev Getter for the address of the payee number `index`.
     */
    pub fn payee(&self, index: u64) -> AccountId {
        self._payees.get(index).unwrap()
    }

    /**
     * @dev Getter for the amount of payee's releasable Ether.
     */
    pub fn releasable(&self, account: &AccountId) -> u128 {
        let total_received: Balance = env::account_balance() + self.total_released();
        self._pending_payment(&account, &total_received, &self.released(&account))
    }

    /**
     * @dev Getter for the amount of payee's releasable `token` tokens. `token`
     * should be the address of an IERC20 contract.
     */
    pub fn releasable_erc(&self, token: AccountId, account: &AccountId) -> u128 {
        let tkn = token.to_string();
        // let mut total_received: Balance =
        //     ext_ft::ext(token).ft_balance_of(env::current_account_id().to_string());

        ext_ft::ext(token)
            .ft_balance_of(env::current_account_id().to_string())
            .then(Self::ext(env::current_account_id()).query_balance_callback());

        let total_received = self._balance_of + self.total_released_erc(&tkn.parse().unwrap());
        self._pending_payment(
            account,
            &total_received,
            &self.released_erc(&tkn.parse().unwrap(), account),
        )
    }
    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_balance_callback(&mut self) -> Balance {
        self._balance_of = match env::promise_result(0) {
            PromiseResult::Successful(value) => {
                near_sdk::serde_json::from_slice::<Balance>(&value).unwrap()
                //value
            }
            _ => {
                log!("There was an error contacting NEP-141");
                return 0;
            }
        };
        self._balance_of
    }

    /**
     * @dev Getter for the amount of payee's full information, including releasable Ether.
     */
    pub fn info(&self, account: AccountId) -> (String, u128, String, String, u128) {
        let total_received: Balance = env::account_balance() + self.total_released();
        (
            self._payee_name.get(&account).unwrap(),
            self._shares.get(&account).unwrap(),
            self._target_chain.get(&account).unwrap(),
            self._target_wallet.get(&account).unwrap(),
            self._pending_payment(&account, &total_received, &self.released(&account)),
        )
    }

    /**
     * @dev Getter for the amount of payee's full information, including releasable Ether.
     */
    pub fn info_by_id(&self, id: u128) -> (String, u128, String, String, u128) {
        let _recipient_x: AccountId = self._payee_count_to_payee_address.get(&id).unwrap();
        let total_received: u128 = env::account_balance() + self.total_released();
        (
            self._payee_name.get(&_recipient_x).unwrap(),
            self._shares.get(&_recipient_x).unwrap(),
            self._target_chain.get(&_recipient_x).unwrap(),
            self._target_wallet.get(&_recipient_x).unwrap(),
            self._pending_payment(
                &_recipient_x,
                &total_received,
                &self.released(&_recipient_x),
            ),
        )
    }

    /**
     * @dev Getter to find the ID number of the last added recipient, which is also
     * the total count of recipients.
     */
    pub fn payee_count(&self) -> u128 {
        self._payee_count
    }

    /**
     * @dev Loop across all recipients, to process payment for them all in one batch.
     * Useful to do before updating "shares" or adding payees, since the release math
     * isn't designed to calculate changing shares.
     */
    pub fn release_all(&mut self) {
        require!(
            self._payee_count > 1,
            "PaymentSplitter: only one or zero accounts available"
        );

        for i in 1..self._payee_count {
            self.release_by_index(&i);
        }
    }

    /**
     * @dev Loop across all recipients, to process payment for them all in one batch.
     * Useful to do before updating "shares" or adding payees, since the release math
     * isn't designed to calculate changing shares.
     */
    pub fn release_by_index(&mut self, _id: &u128) {
        let _recipient_x: AccountId = self._payee_count_to_payee_address.get(_id).unwrap();
        self.release(_recipient_x);
    }

    /**
     * @dev Triggers a transfer to `account` of the amount of Ether they are owed, according
     * to their percentage of the total shares and their previous withdrawals.
     * @dev Replacement for Solidity's `transfer` borrowing logic from "sendValue" function in  OpenZeppelin's "address" library
     */
    #[payable]
    pub fn release(&mut self, account: AccountId) {
        //////////START WORK HERE
        require!(
            self._shares.get(&account).unwrap() > 0,
            "PaymentSplitter: account has no shares"
        );

        let payment: Balance = self.releasable(&account);

        require!(payment != 0, "PaymentSplitter: account is not due payment");
        require!(
            env::account_balance() >= payment,
            "Address: insufficient balance"
        );

        //_released[account] += payment;
        let payment_x: Balance = payment
            + match self._released.get(&account) {
                Some(value) => value,
                None => 0,
            };
        self._released.insert(&account, &payment_x);
        self._total_released += payment;

        //(bool success, ) = account.call{value: payment}("");
        Promise::new(account).transfer(payment);
        //emit PaymentReleased(account, payment);
    }

    /**
     * @dev Triggers a transfer to `account` of the amount of `token` tokens they are owed, according
     * to their percentage of the total shares and their previous withdrawals. `token` must be the
     * address of an IERC20 contract.
     */
    #[payable]
    pub fn release_erc(&mut self, token: AccountId, account: AccountId) {
        let tkn: String = token.to_string();
        // let sender_balance: String = ext_ft::ext(tkn).ft_balance_of(_msgSender().to_string())

        require!(
            self._shares.get(&account).unwrap() > 0,
            "PaymentSplitter: account has no shares"
        );

        let payment: Balance = self.releasable_erc(token, &account);

        require!(payment != 0, "PaymentSplitter: account is not due payment");
        require!(
            env::account_balance() >= payment,
            "Address: insufficient balance"
        );

        //Moved prior because its necessary to confirm the
        //transfer before making a change to contract state
        //wbtc::ft_transfer({"receiver_id": "bob", "amount": "500000000"})
        ext_ft::ext(tkn.parse().unwrap()).ft_transfer(
            account.to_string(),
            payment.to_string(),
            None,
        );
        //_erc20Released[token][account] += payment;/////////
        //_erc20_total_released[token] += payment;/////////////
        // Below code equates to above 2 lines in solidity //
        let mut _erc20_released_inner: UnorderedMap<AccountId, u128> =
            match self._erc20_released.get(&tkn.parse().unwrap()) {
                Some(value) => value,
                None => UnorderedMap::new(b"x"),
            };
        let payment_x: Balance = payment
            + match _erc20_released_inner.get(&account) {
                Some(value) => value,
                None => 0,
            };
        _erc20_released_inner.insert(&account, &payment_x);
        self._erc20_released
            .insert(&tkn.parse().unwrap(), &_erc20_released_inner);

        let payment_y: Balance = payment
            + match self._erc20_total_released.get(&tkn.parse().unwrap()) {
                Some(value) => value,
                None => 0,
            };
        self._erc20_total_released
            .insert(&tkn.parse().unwrap(), &payment_y);
        /////////////////////////////////////////////////////
        //emit ERC20PaymentReleased(token, account, payment);
    }

    /**
     * @dev internal logic for computing the pending payment of an `account`  
     * given the token historical balances and already released amounts.
     */
    #[private] // only callable by env::current_account_id()
    fn _pending_payment(
        &self,
        account: &AccountId,
        total_received: &u128,
        already_released: &u128,
    ) -> u128 {
        (total_received * self._shares.get(account).unwrap()) / self._total_shares
            - already_released
    }

    /**
     * @dev Add a new payee to the contract. Can only be called by the current owner.
     * @param account The address of the payee to add.
     * @param shares_ The number of shares owned by the payee.
     */
    pub fn add_payee(
        &mut self,
        account: AccountId,
        shares_: u128,
        payee_name_: String,
        target_chain_: String,
        target_wallet_: AccountId,
    ) {
        self._only_owner();
        self._add_payee(account, shares_, payee_name_, target_chain_, target_wallet_);
    }

    /**
     * @dev Add a new payee to the contract. Internal function without access restrictions.
     * @param account The address of the payee to add.
     * @param shares_ The number of shares owned by the payee.
     */
    #[private] // only callable by env::current_account_id()
    fn _add_payee(
        &mut self,
        account: AccountId,
        shares_: u128,
        payee_name_: String,
        target_chain_: String,
        target_wallet_: AccountId,
    ) {
        require!(
            account != "".parse().unwrap(),
            "PaymentSplitter: account is the zero address"
        );
        require!(shares_ > 0, "PaymentSplitter: shares are 0");
        require!(
            self._shares.get(&account).unwrap() == 0,
            "PaymentSplitter: account already has shares"
        );

        if self._payee_count > 1 {
            self.release_all(); // must be done first, because otherwise the addition of a new recipient messes with the release math
        }
        self._payee_count += 1;
        self._payees.push(&account);
        self._shares.insert(&account, &shares_);
        self._payee_name.insert(&account, &payee_name_);
        self._target_chain.insert(&account, &target_chain_);
        self._target_wallet
            .insert(&account, &target_wallet_.to_string());
        self._total_shares = self._total_shares + shares_;
        self._payees2.push(&self._payee_count);
        self._payee_count_to_payee_address
            .insert(&self._payee_count, &account);
        //emit PayeeAdded(account, _payee_count, payee_name_, shares_, target_chain_, target_wallet_);
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received 0 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}
