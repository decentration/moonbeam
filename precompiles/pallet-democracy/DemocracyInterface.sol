// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/**
 * @title Pallet Democracy Interface
 *
 * The interface through which solidity contracts will interact with pallet-democracy
 *
 * @dev This interface does not exhaustively wrap pallet democracy, rather it wraps the most
 * important parts and the parts that are expected to be most useful to evm contracts.
 * More exhaustive wrapping can be added later if it is desireable and the pallet interface
 * is deemed sufficiently stable.
 */
interface Democracy {
    // First some simple accessors

    /**
     * Get The total number of public proposals past and present
     * @return The total number of public proposals past and present
     */
    function public_prop_count() external view returns (uint256);

    /**
     * Get details about all public porposals.
     * @return (prop index, proposal hash, proposer)
     * TODO The best type for the hash is bytes32. https://ethereum.stackexchange.com/a/92124/9963
     * TODO This is supposed to be a vec. Let's save this one for later.
     */
    // function public_props()
    //     external
    //     view
    //     returns (
    //         uint256,
    //         uint256,
    //         address
    //     );

    /**
     * Get the total amount locked behind a proposal.
     *
     * @dev Unlike the similarly-named Rust function this one only returns the value, not the
     * complete list of backers.
     * @param prop_index The index of the proposal you are interested in
     * @return The amount of tokens locked behind the proposal
     */
    function deposit_of(uint256 prop_index) external view returns (uint256);

    /**
     * Get the index of the lowest unbaked referendum
     * @return The lowest referendum index representing an unbaked referendum.
     */
    function lowest_unbaked() external view returns (uint256);

    /**
     * Get the details about an ongoing referendum.
     *
     * @dev This, along with `finished_referendum_info`, wraps the pallet's `referendum_info`
     * function. It is necessary to split it into two here because Solidity only has c-style enums.
     * @param ref_index The index of the referendum you are interested in
     * @return A tuple including:
     * * The block in which the referendum ended
     * * The proposal hash (Should use bytes20 when it is available)
     * * The baising mechanism 0-SuperMajorityApprove, 1-SuperMajorityAgainst, 3-SimpleMajority
     * * The delay between passing and launching
     * * The total ayevote (including conviction)
     * * The total nay vote (including conviction)
     * * The total turnout (not including conviction)
     */
    function ongoing_referendum_info(uint256 ref_index)
        external
        view
        returns (
            uint256,
            uint256,
            uint256,
            uint256,
            uint256,
            uint256,
            uint256
        );

    /**
     * Get the details about a finished referendum.
     *
     * @dev This, along with `ongoing_referendum_info`, wraps the pallet's `referendum_info`
     * function. It is necessary to split it into two here because Solidity only has c-style enums.
     * @param ref_index The index of the referendum you are interested in
     * @return A tuple including whether the referendum passed, and the block at which it finished.
     */
    function finished_referendum_info(uint256 ref_index)
        external
        view
        returns (bool, uint256);

    // Now the dispatchables

    /**
     * Make a new proposal
     *
     * @param proposal_hash The hash of the proposal you are making
     * @param value The number of tokens to be locked behind this proposal.
     */
    function propose(bytes32 proposal_hash, uint256 value) external;

    /**
     * Signal agreement with a proposal
     *
     * @dev No amount is necessary here. Seconds are always for the same amount that the original
     * proposer locked. You may second multiple times.
     *
     * @param prop_index index of the proposal you want to second
     * @param seconds_upper_bound A number greater than or equal to the current number of seconds.
     * This is necessary for calculating the weight of the call.
     */
    function second(uint256 prop_index, uint256 seconds_upper_bound) external;

    //TODO should we have an alternative `simple_second` where the upper bound is read from storage?

    //TODO in the future we could consider using a solidity struct here for AccountVote
    // like they do in the Rust code. For now I'll just accept each param individually.
    /**
     * Vote in a referendum.
     *
     * @param ref_index index of the referendum you want to vote in
     * @param aye `true` is a vote to enact the proposal; `false` is a vote to keep the status quo.
     * @param vote_amount The number of tokens you are willing to lock if you get your way
     * @param conviction How strongly you want to vote. Higher conviction means longer lock time.
     * This must be an interget in the range 0 to 6
     *
     * @dev This function only supposrts `Standard` votes where you either vote aye xor nay.
     * It does not support `Split` votes where you vote on both sides. If such a need
     * arises, we should add an additional function to this interface called `split_vote`.
     */
    function stardard_vote(
        uint256 ref_index,
        bool aye,
        uint256 vote_amount,
        uint256 conviction
    ) external;

    /** Remove a vote for a referendumx.
     *
     * @dev Locks get complex when votes are removed. See pallet-democracy's docs for details.
     * @param ref_index The index of the referendum you are interested in
     */
    function remove_vote(uint256 ref_index) external;

    /**
     * Delegate voting power to another account.
     *
     * @dev The balance delegated is locked for as long as it is delegated, and thereafter for the
     * time appropriate for the conviction's lock period.
     * @param representative The account to whom the vote shall be delegated.
     * @param conviction The conviction with which yo uare delegating. This conviction is used for
     * _all_ delegated votes.
     * @param amount The number of tokens whose voting power shall be delegated.
     */
    function delegate(
        address representative,
        uint256 conviction,
        uint256 amount
    ) external;

    /**
     * Undelegatehe voting power
     *
     * @dev Tokens may be unlocked once the lock period corresponding to the conviction with which
     * the delegation was issued has elapsed.
     */
    function un_delegate() external;

    /**
     * Unlock tokens that have an expired lock.
     *
     * @param target The account whose tokens should be unlocked. This may be any account.
     */
    function unlock(address target) external;
}

// These are the selectors generated by remix following this advice
// https://ethereum.stackexchange.com/a/73405/9963
// Eventually we will probably want a better way of generating these and copying them to Rust
// {
// "0185921e": "delegate(address,uint256,uint256)",
// "a30305e9": "deposit_of(uint256)",
// "b1fd383f": "finished_referendum_info(uint256)",
// "0388f282": "lowest_unbaked()",
// "8b93d11a": "ongoing_referendum_info(uint256)",
// "7824e7d1": "propose(bytes32,uint256)",
// "56fdf547": "public_prop_count()",
// "2042f50b": "remove_vote(uint256)",
// "c7a76601": "second(uint256,uint256)",
// "35cde7ae": "stardard_vote(uint256,bool,uint256,uint256)",
// "cb37b8ea": "un_delegate()",
// "2f6c493c": "unlock(address)"
// }
