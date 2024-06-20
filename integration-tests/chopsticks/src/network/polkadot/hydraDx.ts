import { SetupOption } from '@acala-network/chopsticks-testing'

import { initialBalanceHDX, initialBalanceKILT, toNumber } from '../../helper/utils.js'

/// Options used to create the HydraDx context
export const getSetupOptions = ({
	blockNumber,
	wasmOverride,
}: {
	blockNumber?: number
	wasmOverride?: string
} = {}) =>
	({
		endpoint: process.env.HYDRADX_WS || ['wss://hydradx-rpc.dwellir.com', 'wss://rpc.hydradx.cloud'],
		db: './db/hydradx.db.sqlite',
		port: toNumber(process.env.HYDRADX_PORT) || 9001,
		blockNumber,
		wasmOverride,
	}) as SetupOption

/// Sets the [TechnicalCommittee] and [Council] governance to the given accounts
export function setGovernance(addr: string[]) {
	return {
		TechnicalCommittee: { Members: addr },
		Council: { Members: addr },
	}
}

/// Assigns the native tokens to an accounts
export function assignNativeTokensToAccounts(addr: string[], balance: bigint = initialBalanceHDX) {
	return {
		System: {
			Account: addr.map((address) => [[address], { providers: 1, data: { free: balance } }]),
		},
	}
}

/// Assigns KILT tokens to accounts
export function assignKiltTokensToAccounts(addr: string[], balance: bigint = initialBalanceKILT) {
	return {
		Tokens: {
			Accounts: addr.map((address) => [[address, kiltTokenId], { free: balance }]),
		},
	}
}

export const kiltTokenId = 28

/// HydraDX ParaId
export const paraId = 2034

/// Omnipool account
export const omnipoolAccount = '7L53bUTBbfuj14UpdCNPwmgzzHSsrsTWBHX5pys32mVWM3C1'
