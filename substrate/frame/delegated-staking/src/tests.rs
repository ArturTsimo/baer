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
use crate::{mock::*, Event};
use frame_support::{assert_noop, assert_ok, traits::fungible::InspectHold};
use pallet_staking::Error as StakingError;
use sp_staking::{StakeBalanceType, StakingDelegationSupport};

#[test]
fn create_a_delegatee_with_first_delegator() {
	ExtBuilder::default().build_and_execute(|| {
		let delegatee: AccountId = 200;
		let reward_account: AccountId = 201;
		let delegator: AccountId = 202;

		// set intention to accept delegation.
		assert_ok!(DelegatedStaking::accept_delegations(&fund(delegatee, 1000), &reward_account));

		// delegate to this account
		assert_ok!(DelegatedStaking::delegate(&fund(delegator, 1000), &delegatee, 100));

		// verify
		assert_eq!(DelegatedStaking::stake_type(&delegatee), StakeBalanceType::Delegated);
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
		assert_eq!(DelegatedStaking::stake_type(&fund(delegatee, 1000)), StakeBalanceType::Direct);
		assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 1000);

		// set intention to accept delegation.
		assert_ok!(DelegatedStaking::accept_delegations(&delegatee, &reward_account));

		// create 100 delegators
		for i in 202..302 {
			assert_ok!(DelegatedStaking::delegate(
				&fund(i, 100 + ExistentialDeposit::get()),
				&delegatee,
				100
			));
			// Balance of 100 held on delegator account for delegating to the delegatee.
			assert_eq!(Balances::balance_on_hold(&HoldReason::Delegating.into(), &i), 100);
		}

		// verify
		assert_eq!(DelegatedStaking::stake_type(&delegatee), StakeBalanceType::Delegated);
		assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 100 * 100);
	});
}

#[test]
fn withdraw_delegation() {
	// Similar to creating a nomination pool
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

#[test]
fn apply_pending_slash() {
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

#[test]
fn distribute_rewards() {
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

#[test]
fn migrate_to_delegator() {
	ExtBuilder::default().build_and_execute(|| assert!(true));
}

/// Integration tests with pallet-staking.
mod integration {
	use super::*;

	#[test]
	fn bond() {
		ExtBuilder::default().build_and_execute(|| {
			let delegatee: AccountId = 99;
			let reward_acc: AccountId = 100;
			assert_eq!(Staking::status(&delegatee), Err(StakingError::<T>::NotStash.into()));
			assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 0);
			assert_eq!(Balances::free_balance(delegatee), 0);

			// set intention to become a delegatee
			assert_ok!(DelegatedStaking::accept_delegations(&fund(delegatee, 100), &reward_acc));
			// set some delegations
			for delegator in 200..250 {
				assert_ok!(DelegatedStaking::delegate(&fund(delegator, 1000), &delegatee, 100));
				assert_eq!(
					Balances::balance_on_hold(&HoldReason::Delegating.into(), &delegator),
					100
				);
				assert_eq!(DelegatedStaking::stakeable_balance(&delegatee), 100);

				assert_ok!(Staking::bond(RuntimeOrigin::signed(delegatee),));
			}

			//
		});
	}

	#[test]
	fn bond_extra() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}

	#[test]
	fn partial_withdraw() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}

	#[test]
	fn claim_reward() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}

	#[test]
	fn slash_works() {
		ExtBuilder::default().build_and_execute(|| assert!(true));
	}
}