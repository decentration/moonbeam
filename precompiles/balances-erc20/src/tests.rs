// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use std::assert_matches::assert_matches;

use crate::mock::*;
use crate::*;

use pallet_evm::PrecompileSet;
use precompile_utils::{
	error, EvmDataReader, EvmDataWriter, EvmResult, Gasometer, LogsBuilder, RuntimeHelper,
};

#[test]
fn selector_less_than_four_bytes() {
	ExtBuilder::default().build().execute_with(|| {
		// This selector is only three bytes long when four are required.
		let bogus_selector = vec![1u8, 2u8, 3u8];

		assert_eq!(
			Precompiles::<Runtime>::execute(
				Account::Precompile.into(),
				&bogus_selector,
				None,
				&evm::Context {
					address: Account::Precompile.into(),
					caller: Account::Alice.into(),
					apparent_value: From::from(0),
				},
			),
			Some(Err(error("tried to parse selector out of bounds")))
		);
	});
}

#[test]
fn no_selector_exists_but_length_is_right() {
	ExtBuilder::default().build().execute_with(|| {
		let bogus_selector = vec![1u8, 2u8, 3u8, 4u8];

		assert_eq!(
			Precompiles::<Runtime>::execute(
				Account::Precompile.into(),
				&bogus_selector,
				None,
				&evm::Context {
					address: Account::Precompile.into(),
					caller: Account::Alice.into(),
					apparent_value: From::from(0),
				},
			),
			Some(Err(error("unknown selector")))
		);
	});
}

#[test]
fn get_total_supply() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000), (Account::Bob, 2500)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::TotalSupply)
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: EvmDataWriter::new().write(U256::from(3500u64)).build(),
					cost: Default::default(),
					logs: Default::default(),
				}))
			);
		});
}

#[test]
fn get_balances_known_user() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::BalanceOf)
						.write(Address(Account::Alice.into()))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: EvmDataWriter::new().write(U256::from(1000u64)).build(),
					cost: Default::default(),
					logs: Default::default(),
				}))
			);
		});
}

#[test]
fn get_balances_unknown_user() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::BalanceOf)
						.write(Address(Account::Bob.into()))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: EvmDataWriter::new().write(U256::from(0u64)).build(),
					cost: Default::default(),
					logs: Default::default(),
				}))
			);
		});
}

#[test]
fn approve() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::Approve)
						.write(Address(Account::Bob.into()))
						.write(U256::from(500))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: Default::default(),
					cost: 1756u64,
					logs: LogsBuilder::new(Account::Precompile.into())
						.log3(
							SELECTOR_LOG_APPROVAL,
							Account::Alice,
							Account::Bob,
							EvmDataWriter::new().write(U256::from(500)).build(),
						)
						.build(),
				}))
			);
		});
}

#[test]
fn check_allowance_existing() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			Precompiles::<Runtime>::execute(
				Account::Precompile.into(),
				&EvmDataWriter::new()
					.write_selector(Action::Approve)
					.write(Address(Account::Bob.into()))
					.write(U256::from(500))
					.build(),
				None,
				&evm::Context {
					address: Account::Precompile.into(),
					caller: Account::Alice.into(),
					apparent_value: From::from(0),
				},
			);

			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::Allowance)
						.write(Address(Account::Alice.into()))
						.write(Address(Account::Bob.into()))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: EvmDataWriter::new().write(U256::from(500u64)).build(),
					cost: 0u64,
					logs: Default::default(),
				}))
			);
		});
}

#[test]
fn check_allowance_not_existing() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::Allowance)
						.write(Address(Account::Alice.into()))
						.write(Address(Account::Bob.into()))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: EvmDataWriter::new().write(U256::from(0u64)).build(),
					cost: 0u64,
					logs: Default::default(),
				}))
			);
		});
}

#[test]
fn transfer() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_eq!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::Transfer)
						.write(Address(Account::Bob.into()))
						.write(U256::from(400))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: Default::default(),
					cost: 198269756u64, // 1 weight => 1 gas in mock
					logs: LogsBuilder::new(Account::Precompile.into())
						.log3(
							SELECTOR_LOG_TRANSFER,
							Account::Alice,
							Account::Bob,
							EvmDataWriter::new().write(U256::from(400)).build(),
						)
						.build(),
				}))
			);
		});
}

#[test]
fn transfer_not_enough_founds() {
	ExtBuilder::default()
		.with_balances(vec![(Account::Alice, 1000)])
		.build()
		.execute_with(|| {
			assert_matches!(
				Precompiles::<Runtime>::execute(
					Account::Precompile.into(),
					&EvmDataWriter::new()
						.write_selector(Action::Transfer)
						.write(Address(Account::Bob.into()))
						.write(U256::from(1400))
						.build(),
					None,
					&evm::Context {
						address: Account::Precompile.into(),
						caller: Account::Alice.into(),
						apparent_value: From::from(0),
					},
				),
				Some(Err(ExitError::Other(str)))
					if str.contains("Dispatched call failed with error: DispatchErrorWithPostInfo")
					&& str.contains("InsufficientBalance")
			);
		});
}
