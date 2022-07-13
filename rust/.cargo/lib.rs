//! This contract implements (...functionality...) on blockchain.
//!
//! [function_name]: description of function
//! [function_name]: description of function

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, UnorderedMap};
use near_sdk::json_types::U128;

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PaymentSplitter {
    _owner: AccountId,
    _payeeCount: u128,
    _totalShares: u128,
    _totalReleased: u128,

    _payees2: Vec<u128>,
    _payees: Vec<AccountId>,
    _targetChain: Vec<AccountId>,

    _shares: UnorderedMap<AccountId, u128>,
    _released: UnorderedMap<AccountId, u128>,
    _payeeName: UnorderedMap<AccountId, u128>,
    _targetWallet: UnorderedMap<AccountId, u128>,
    _payeeCountToPayeeAddress: UnorderedMap<u128, AccountId>,

    _erc20TotalReleased: UnorderedMap<AccountId, u128>,
    _erc20Released: UnorderedMap<AccountId, u128>,
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
            _payeeCount = 0,
            _payees: Vec::new(),
            _shares: UnorderedMap::new(),
            _owner: env::predecessor_account_id(),
        }
    }
    /**
    * Returns the address of the current owner.
    */
    pub fn owner() -> AccountId {
        _owner
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
     pub fn renounceOwnership() {
        _onlyOwner();
        _transferOwnership(0 as AccountId);
    }
    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    pub fn transferOwnership(newOwner: AccountId) {
        _onlyOwner();
        require!(newOwner != (0 as AccountId), "Owner is zero address");
        _transferOwnership(newOwner);
    }
    /**
    * @dev Transfers ownership of the contract to a new account (`newOwner`).
    * Internal function without access restriction.
    */
    fn _transferOwnership(&mut self, newOwner: &AccountId) {
        let oldOwner: AccountId = self._owner;
        self._owner = newOwner;
        //emit OwnershipTransferred(oldOwner, newOwner);
    }
    /**
    * @dev Throws if the sender is not the owner.
    */
    fn _onlyOwner(&self) {
        require!(env::predecessor_account_id() == self._owner, "Ownable: caller is not the owner");
    }
    /**
    * @dev Provides information about the current execution context, about the
    * sender of the transaction. While generally available
    * via msg.sender this should not be accessed in such a direct
    * manner, since when dealing with meta-transactions the account sending and
    * paying for execution may not be the actual sender (as far as an application
    * is concerned).
    */
    fn _msgSender() -> AccountId {
        env::current_account_id()
    }

    /**
     * @dev Add a new payee to the contract. Internal function without access restrictions.
     * @param account The address of the payee to add.
     * @param shares_ The number of shares owned by the payee.
     */
    #[private]
    fn _addPayee(
        &mut self,
        account: &AccountId,
        shares_: &u128,
        payeeName_: &String,
        targetChain_: &String,
        targetWallet_: &AccountId
    ) {
        require!(account != (0 as AccountId), "PaymentSplitter: account is the zero address");
        require!(shares_ > 0, "PaymentSplitter: shares are 0");
        require!(self._shares.get(account) == 0, "PaymentSplitter: account already has shares");

        if self._payeeCount > 1 { 
            releaseAll();// must be done first, because otherwise the addition of a new recipient messes with the release math
        }
        self._payeeCount++;
        self._payees.push(account);
        self._shares.insert(account, shares_);
        self._payeeName.insert(account, payeeName_);
        self._targetChain.insert(account, targetChain_);
        self._targetWallet.insert(account, targetWallet_);
        self._totalShares = self._totalShares + shares_;
        self._payees2.push(_payeeCount);
        self._payeeCountToPayeeAddress.push(self._payeeCount, account);
        //emit PayeeAdded(account, _payeeCount, payeeName_, shares_, targetChain_, targetWallet_);
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
