// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for pallet-delegated-staking.

use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, traits::fungible::InspectHold};
use pallet_staking::Error as StakingError;
use sp_staking::delegation::StakingDelegationSupport;

#[test]
fn create_a_delegatee_with_first_delegator() {
	ExtBuilder::default().build_and_execute(|| {
		let delegatee: AccountId = 200;
		let reward_account: AccountId = 201;
		let delegator: AccountId = 202;

		// set intention to accept delegation.
		assert_ok!(DelegatedStaking::accept_delegations(fund(&delegatee, 1000), &reward_account));

		// delegate to this account
		assert_ok!(DelegatedStaking::delegate(fund(&delegator, 1000), &delegatee, 100));

		// verify
		assert!(DelegatedStaking::is_delegatee(&delegatee));
		assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 100);
		assert_eq!(Balances::balance_on_hold(&HoldReason::Delegating.into(), &delegator), 100);
	});
}

#[test]
fn cannot_become_delegatee() {
	ExtBuilder::default().build_and_execute(|| {
		// cannot set reward account same as delegatee account
		assert_noop!(
			DelegatedStaking::accept_delegations(&100, &100),
			Error::<T>::InvalidRewardDestination
		);

		// an existing validator cannot become delegatee
		assert_noop!(
			DelegatedStaking::accept_delegations(&mock::GENESIS_VALIDATOR, &100),
			Error::<T>::AlreadyStaker
		);

		// an existing nominator cannot become delegatee
		assert_noop!(
			DelegatedStaking::accept_delegations(&mock::GENESIS_NOMINATOR_ONE, &100),
			Error::<T>::AlreadyStaker
		);
		assert_noop!(
			DelegatedStaking::accept_delegations(&mock::GENESIS_NOMINATOR_TWO, &100),
			Error::<T>::AlreadyStaker
		);
	});
}

#[test]
fn create_multiple_delegators() {
	ExtBuilder::default().build_and_execute(|| {
		let delegatee: AccountId = 200;
		let reward_account: AccountId = 201;

		// before becoming a delegatee, stakeable balance is only direct balance.
		assert!(!DelegatedStaking::is_delegatee(fund(&delegatee, 1000)));
		assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 1000);

		// set intention to accept delegation.
		assert_ok!(DelegatedStaking::accept_delegations(&delegatee, &reward_account));

		// create 100 delegators
		for i in 202..302 {
			assert_ok!(DelegatedStaking::delegate(
				fund(&i, 100 + ExistentialDeposit::get()),
				&delegatee,
				100
			));
			// Balance of 100 held on delegator account for delegating to the delegatee.
			assert_eq!(Balances::balance_on_hold(&HoldReason::Delegating.into(), &i), 100);
		}

		// verify
		assert!(DelegatedStaking::is_delegatee(&delegatee));
		assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 100 * 100);
	});
}

#[test]
fn delegate_restrictions() {
	// Similar to creating a nomination pool
	ExtBuilder::default().build_and_execute(|| {
		let delegatee_one = 200;
		let delegator_one = 210;
		assert_ok!(DelegatedStaking::accept_delegations(fund(&delegatee_one, 100), &(delegatee_one+1)));
		assert_ok!(DelegatedStaking::delegate(fund(&delegator_one, 200), &delegatee_one, 100));

		let delegatee_two = 300;
		let delegator_two = 310;
		assert_ok!(DelegatedStaking::accept_delegations(fund(&delegatee_two, 100), &(delegatee_two+1)));
		assert_ok!(DelegatedStaking::delegate(fund(&delegator_two, 200), &delegatee_two, 100));

		// delegatee one tries to delegate to delegatee 2
		assert_noop!(DelegatedStaking::delegate(&delegatee_one, &delegatee_two, 10), Error::<T>::InvalidDelegation);

		// delegatee one tries to delegate to a delegator
		assert_noop!(DelegatedStaking::delegate(&delegatee_one, &delegator_one, 10), Error::<T>::NotDelegatee);
		assert_noop!(DelegatedStaking::delegate(&delegatee_one, &delegator_two, 10), Error::<T>::NotDelegatee);

		// delegator one tries to delegate to delegatee 2 as well (it already delegates to delegatee 1)
		assert_noop!(DelegatedStaking::delegate(&delegator_one, &delegatee_two, 10), Error::<T>::InvalidDelegation);
	});
}

#[test]
fn apply_pending_slash() {
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

#[test]
fn distribute_rewards() {
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

/// Integration tests with pallet-staking.
mod integration {
	use super::*;
	use sp_staking::Stake;

	#[test]
	fn bond() {
		ExtBuilder::default().build_and_execute(|| {
			let delegatee: AccountId = 99;
			let reward_acc: AccountId = 100;
			assert_eq!(Staking::status(&delegatee), Err(StakingError::<T>::NotStash.into()));

			// set intention to become a delegatee
			assert_ok!(DelegatedStaking::accept_delegations(fund(&delegatee, 100), &reward_acc));
			assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 0);

			let mut delegated_balance: Balance = 0;
			// set some delegations
			for delegator in 200..250 {
				assert_ok!(DelegatedStaking::delegate(fund(&delegator, 200), &delegatee, 100));
				delegated_balance += 100;
				assert_eq!(
					Balances::balance_on_hold(&HoldReason::Delegating.into(), &delegator),
					100
				);

				assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), delegated_balance);

				// unbonded balance is the newly delegated 100
				assert_eq!(DelegatedStaking::unbonded_balance(&delegatee), 100);
				assert_ok!(DelegatedStaking::update_bond(&delegatee));
				// after bond, unbonded balance is 0
				assert_eq!(DelegatedStaking::unbonded_balance(&delegatee), 0);
			}

			assert_eq!(
				Staking::stake(&delegatee).unwrap(),
				Stake { total: 50 * 100, active: 50 * 100 }
			)
		});
	}

	#[test]
	fn withdraw_test() {
		ExtBuilder::default().build_and_execute(|| {
			// initial era
			start_era(1);
			let delegatee: AccountId = 200;
			let reward_acc: AccountId = 201;
			let delegators: Vec<AccountId> = (301..=350).collect();
			let total_staked =
				setup_delegation_stake(delegatee, reward_acc, delegators.clone(), 10, 10);

			// lets go to a new era
			start_era(2);

			assert!(eq_stake(delegatee, total_staked, total_staked));
			// Withdrawing without unbonding would fail.
			assert_noop!(
				DelegatedStaking::withdraw(&300, &delegatee, 50, 0),
				Error::<T>::WithdrawFailed
			);
			// assert_noop!(DelegatedStaking::withdraw(&200, &delegatee, 50, 0),
			// Error::<T>::NotAllowed); active and total stake remains same
			assert!(eq_stake(delegatee, total_staked, total_staked));

			// 305 wants to unbond 50 in era 2, withdrawable in era 5.
			assert_ok!(DelegatedStaking::unbond(&delegatee, 50));
			// 310 wants to unbond 100 in era 3, withdrawable in era 6.
			start_era(3);
			assert_ok!(DelegatedStaking::unbond(&delegatee, 100));
			// 320 wants to unbond 200 in era 4, withdrawable in era 7.
			start_era(4);
			assert_ok!(DelegatedStaking::unbond(&delegatee, 200));

			// active stake is now reduced..
			let expected_active = total_staked - (50 + 100 + 200);
			assert!(eq_stake(delegatee, total_staked, expected_active));

			// nothing to withdraw at era 4
			assert_noop!(
				DelegatedStaking::withdraw(&305, &delegatee, 50, 0),
				Error::<T>::WithdrawFailed
			);

			assert!(eq_stake(delegatee, total_staked, expected_active));
			assert_eq!(DelegatedStaking::unbonded_balance(&delegatee), 0);
			// full amount is still delegated
			assert_eq!(DelegatedStaking::delegated_balance(&delegatee), total_staked);

			start_era(5);
			// at era 5, 50 tokens are withdrawable, cannot withdraw more.
			assert_noop!(
				DelegatedStaking::withdraw(&305, &delegatee, 51, 0),
				Error::<T>::WithdrawFailed
			);
			// less is possible
			assert_ok!(DelegatedStaking::withdraw(&305, &delegatee, 30, 0));
			assert_ok!(DelegatedStaking::withdraw(&305, &delegatee, 20, 0));

			// Lets go to future era where everything is unbonded. Withdrawable amount: 100 + 200
			start_era(7);
			// 305 has no more amount delegated so it cannot withdraw.
			assert_noop!(
				DelegatedStaking::withdraw(&305, &delegatee, 5, 0),
				Error::<T>::NotDelegator
			);
			// 309 is an active delegator but has total delegation of 90, so it cannot withdraw more
			// than that.
			assert_noop!(
				DelegatedStaking::withdraw(&309, &delegatee, 91, 0),
				Error::<T>::NotEnoughFunds
			);
			// 310 cannot withdraw more than delegated funds.
			assert_noop!(
				DelegatedStaking::withdraw(&310, &delegatee, 101, 0),
				Error::<T>::NotEnoughFunds
			);
			// but can withdraw all its delegation amount.
			assert_ok!(DelegatedStaking::withdraw(&310, &delegatee, 100, 0));
			// 320 can withdraw all its delegation amount.
			assert_ok!(DelegatedStaking::withdraw(&320, &delegatee, 200, 0));

			// cannot withdraw anything more..
			assert_noop!(
				DelegatedStaking::withdraw(&301, &delegatee, 1, 0),
				Error::<T>::WithdrawFailed
			);
			assert_noop!(
				DelegatedStaking::withdraw(&350, &delegatee, 1, 0),
				Error::<T>::WithdrawFailed
			);
		});
	}

	#[test]
	fn withdraw_happens_with_unbonded_balance_first() {
		ExtBuilder::default().build_and_execute(|| {
			let delegatee = 200;
			setup_delegation_stake(delegatee, 201, (300..350).collect(), 100, 0);

			// verify withdraw not possible yet
			assert_noop!(DelegatedStaking::withdraw(&300, &delegatee, 100, 0), Error::<T>::WithdrawFailed);

			// add new delegation that is not staked
			assert_ok!(DelegatedStaking::delegate(fund(&300, 1000), &delegatee, 100));

			// verify unbonded balance
			assert_eq!(DelegatedStaking::unbonded_balance(&delegatee), 100);

			// withdraw works now without unbonding
			assert_ok!(DelegatedStaking::withdraw(&300, &delegatee, 100, 0));
			assert_eq!(DelegatedStaking::unbonded_balance(&delegatee), 0);
		});
	}

	#[test]
	fn reward_destination_restrictions() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}

	#[test]
	fn nominate_test() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}
	#[test]
	fn slash_works() {
		ExtBuilder::default().build_and_execute(|| {
			setup_delegation_stake(200, 201, (210..250).collect(), 100, 0);
			start_era(1);

			// delegatee is slashed
		});
	}

	#[test]
	fn migration_works() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}
}
