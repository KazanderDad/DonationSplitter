// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts v4.4.1 (finance/PaymentSplitter.sol)

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title PaymentSplitter
 * @dev This contract allows to split Ether payments among a group of accounts. The sender does not need to be aware
 * that the Ether will be split in this way, since it is handled transparently by the contract.
 *
 * The split can be in equal parts or in any other arbitrary proportion. The way this is specified is by assigning each
 * account to a number of shares. Of all the Ether that this contract receives, each account will then be able to claim
 * an amount proportional to the percentage of total shares they were assigned. The distribution of shares is set at the
 * time of contract deployment and can't be updated thereafter.
 *
 * `PaymentSplitter` follows a _pull payment_ model. This means that payments are not automatically forwarded to the
 * accounts but kept in this contract, and the actual transfer is triggered as a separate step by calling the {release}
 * function.
 *
 * NOTE: This contract assumes that ERC20 tokens will behave similarly to native tokens (Ether). Rebasing tokens, and
 * tokens that apply fees during transfers, are likely to not be supported as expected. If in doubt, we encourage you
 * to run tests before sending real value to this contract.
 */
contract PaymentSplitter {
    event PayeeAdded(address account, uint256 count, bytes32 name, uint256 shares, bytes32 targetChain, bytes32 targetWallet);
    event PaymentReleased(address to, uint256 amount);
    event ERC20PaymentReleased(IERC20 indexed token, address to, uint256 amount);
    event PaymentReceived(address from, uint256 amount);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);


    uint256 private _totalShares;
    uint256 private _totalReleased;
    uint256 private _payeeCount;

    mapping(address => uint256) private _shares;
    mapping(address => bytes32) private _payeeName;
    mapping(address => bytes32) private _targetChain;
    mapping(address => bytes32) private _targetWallet;
    mapping(address => uint256) private _released;
    address[] private _payees;

    mapping(IERC20 => uint256) private _erc20TotalReleased;
    mapping(IERC20 => mapping(address => uint256)) private _erc20Released;
    
    
    mapping (uint256 => address payable) private _payeeCountToPayeeAddress;  // the value in the _payeeCountToPayeeAddress mapping points to the adress in the main _payees mapping
    uint256[] private _payees2;

    address private _owner;


    /**
     * @dev Creates an instance of `PaymentSplitter` where each account in `payees` is assigned the number of shares at
     * the matching position in the `shares` array.
     *
     * @dev Initializes the contract setting the deployer as the initial owner.
     *
     */
    constructor() payable {
        _transferOwnership(_msgSender());
        _payeeCount = 0;
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view {
        require(owner() == _msgSender(), "Ownable: caller is not the owner");
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() public onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }

    /**
    * @dev Provides information about the current execution context, about the
    * sender of the transaction. While generally available
    * via msg.sender this should not be accessed in such a direct
    * manner, since when dealing with meta-transactions the account sending and
    * paying for execution may not be the actual sender (as far as an application
    * is concerned).
    */
    function _msgSender() internal view returns (address) {
        return msg.sender;
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
    receive() external payable virtual {
        emit PaymentReceived(_msgSender(), msg.value);
    }

    /**
     * @dev Getter for the total shares held by payees.
     */
    function totalShares() public view returns (uint256) {
        return _totalShares;
    }

    /**
     * @dev Getter for the total amount of Ether already released.
     */
    function totalReleased() public view returns (uint256) {
        return _totalReleased;
    }

    /**
     * @dev Getter for the total amount of `token` already released. `token` should be the 
     * address of an IERC20 contract.
     */
    function totalReleased(IERC20 token) public view returns (uint256) {
        return _erc20TotalReleased[token];
    }

    /**
     * @dev Getter for the amount of shares held by an account.
     */
    function shares(address account) public view returns (uint256) {
        return _shares[account];
    }

    /**
     * @dev Getter for the amount of Ether already released to a payee.
     */
    function released(address account) public view returns (uint256) {
        return _released[account];
    }

    /**
     * @dev Getter for the amount of `token` tokens already released to a payee. `token` 
     * should be the address of an IERC20 contract.
     */
    function released(IERC20 token, address account) public view returns (uint256) {
        return _erc20Released[token][account];
    }

    /**
     * @dev Getter for the address of the payee number `index`.
     */
    function payee(uint256 index) public view returns (address) {
        return _payees[index];
    }

    /**
     * @dev Getter for the amount of payee's releasable Ether.
     */
    function releasable(address account) public view returns (uint256) {
        uint256 totalReceived = address(this).balance + totalReleased();
        return _pendingPayment(account, totalReceived, released(account));
    }

    /**
     * @dev Getter for the amount of payee's releasable `token` tokens. `token` 
     * should be the address of an IERC20 contract.
     */
    function releasable(IERC20 token, address account) public view returns (uint256) {
        uint256 totalReceived = token.balanceOf(address(this)) + totalReleased(token);
        return _pendingPayment(account, totalReceived, released(token, account));
    }

    /**
     * @dev Getter for the amount of payee's full information, including releasable Ether.
     */
    function info(address account) public view returns (bytes32, uint256, bytes32, bytes32, uint256) {
        uint256 totalReceived = address(this).balance + totalReleased();
        return (
            _payeeName[account],
            _shares[account],
            _targetChain[account],
            _targetWallet[account],
            _pendingPayment(account, totalReceived, released(account)));
    }

    /**
     * @dev Getter for the amount of payee's full information, including releasable Ether.
     */
    function infoByID(uint256 ID) public view returns (bytes32, uint256, bytes32, bytes32, uint256) {
        address _recipientX = _payeeCountToPayeeAddress[ID];
        uint256 totalReceived = address(this).balance + totalReleased();

        return (
            _payeeName[_recipientX],
            _shares[_recipientX],
            _targetChain[_recipientX],
            _targetWallet[_recipientX],
            _pendingPayment(_recipientX, totalReceived, released(_recipientX)));
    }

    /**
     * @dev Getter to find the ID number of the last added recipient, which is also 
     * the total count of recipients.
     */
    function payeeCount () public view returns (uint256) {
        return _payeeCount;
    }

    /**
     * @dev Loop across all recipients, to process payment for them all in one batch.
     * Useful to do before updating "shares" or adding payees, since the release math 
     * isn't designed to calculate changing shares.
     */
    function releaseAll() public {
        require (_payeeCount > 1, "PaymentSplitter: only one or zero accounts available");
        for (uint256 i=1; i < _payeeCount+1; i++) {
            releaseByIndex(i);
        }
    }

    /**
     * @dev Loop across all recipients, to process payment for them all in one batch.
     * Useful to do before updating "shares" or adding payees, since the release math 
     * isn't designed to calculate changing shares.
     */
    function releaseByIndex(uint256 _id) public {
            address payable _recipientX = _payeeCountToPayeeAddress[_id];
            release(_recipientX);
    }

    /**
     * @dev Triggers a transfer to `account` of the amount of Ether they are owed, according 
     * to their percentage of the total shares and their previous withdrawals.
     * @dev Replacement for Solidity's `transfer` borrowing logic from "sendValue" function in  OpenZeppelin's "address" library
     */
    function release(address payable account) public {
        require(_shares[account] > 0, "PaymentSplitter: account has no shares");

        uint256 payment = releasable(account);

        require(payment != 0, "PaymentSplitter: account is not due payment");
        require(address(this).balance >= payment, "Address: insufficient balance");

        _released[account] += payment;
        _totalReleased += payment;

        (bool success, ) = account.call{value: payment}("");
        require(success, "Address: unable to send value, recipient may have reverted");

        emit PaymentReleased(account, payment);
    }

    /**
     * @dev Triggers a transfer to `account` of the amount of `token` tokens they are owed, according 
     * to their percentage of the total shares and their previous withdrawals. `token` must be the 
     * address of an IERC20 contract.
     */
    function release(IERC20 token, address account) public {
        require(_shares[account] > 0, "PaymentSplitter: account has no shares");

        uint256 payment = releasable(token, account);

        require(payment != 0, "PaymentSplitter: account is not due payment");
        require(address(this).balance >= payment, "Address: insufficient balance");

        _erc20Released[token][account] += payment;
        _erc20TotalReleased[token] += payment;

        SafeERC20.safeTransfer(token, account, payment);
        emit ERC20PaymentReleased(token, account, payment);
    }

    /**
     * @dev internal logic for computing the pending payment of an `account`  
     * given the token historical balances and already released amounts.
     */
    function _pendingPayment(
        address account,
        uint256 totalReceived,
        uint256 alreadyReleased
    ) private view returns (uint256) {
        return (totalReceived * _shares[account]) / _totalShares - alreadyReleased;
    }

    /**
     * @dev Add a new payee to the contract. Can only be called by the current owner.
     * @param account The address of the payee to add.
     * @param shares_ The number of shares owned by the payee. 
     */
    function addPayee(
        address payable account, 
        uint256 shares_, 
        bytes32 payeeName_, 
        bytes32 targetChain_, 
        bytes32 targetWallet_
    ) public onlyOwner {
        _addPayee(account, shares_, payeeName_, targetChain_, targetWallet_);
    }

    /**
     * @dev Add a new payee to the contract. Internal function without access restrictions.
     * @param account The address of the payee to add.
     * @param shares_ The number of shares owned by the payee.
     */
    function _addPayee(
        address payable account, 
        uint256 shares_, 
        bytes32 payeeName_, 
        bytes32 targetChain_, 
        bytes32 targetWallet_
    ) private {
        
        require(account != address(0), "PaymentSplitter: account is the zero address");
        require(shares_ > 0, "PaymentSplitter: shares are 0");
        require(_shares[account] == 0, "PaymentSplitter: account already has shares");

        if (_payeeCount >1) { 
            releaseAll();            // must be done first, because otherwise the addition of a new recipient messes with the release math
        }
        _payeeCount++;
        _payees.push(account);
        _shares[account] = shares_;
        _payeeName[account] = payeeName_;
        _targetChain[account] = targetChain_;
        _targetWallet[account] = targetWallet_;
        _totalShares = _totalShares + shares_;
        _payees2.push(_payeeCount);
        _payeeCountToPayeeAddress[_payeeCount] = account;
        emit PayeeAdded(account, _payeeCount, payeeName_, shares_, targetChain_, targetWallet_);
    }
}
