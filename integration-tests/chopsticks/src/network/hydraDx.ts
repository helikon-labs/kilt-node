import { setupContext, SetupOption } from '@acala-network/chopsticks-testing'
import type { Config } from './types.js'
import { u8aToHex } from '@polkadot/util'
import { decodeAddress } from '@polkadot/util-crypto'
import * as SpiritnetConfig from './spiritnet.js'
import { initBalance, toNumber } from './utils.js'

export const options: SetupOption = {
	endpoint: process.env.HYDRADX_WS || ['wss://hydradx-rpc.dwellir.com', 'wss://rpc.hydradx.cloud'],
	db: './db/hydradx.db.sqlite',
	port: toNumber(process.env.HYDRADX_PORT) || 9001,
}

export const kiltTokenId = 60

export const defaultStorage = (addr: string) => ({
	TechnicalCommittee: { Members: [addr] },
	Council: { Members: [addr] },
	Tokens: {
		Accounts: [[[addr, kiltTokenId], { free: initBalance }]],
	},
	assetRegistry: {
		assetLocations: [[[kiltTokenId], { parents: 1, interior: { X1: { Parachain: SpiritnetConfig.paraId } } }]],
		assetIds: [[['KILT'], kiltTokenId]],
		locationAssets: [[[{ parents: 1, interior: { X1: { Parachain: SpiritnetConfig.paraId } } }], kiltTokenId]],
		assets: [
			[
				[kiltTokenId],
				{
					name: 'KILT',
					assetType: 'Token',
					existentialDeposit: 500,
					symbol: 'KLT',
					decimals: 18,
					xcmRateLimit: null,
					isSufficient: true,
				},
			],
		],
	},
	multiTransactionPayment: {
		acceptedCurrencies: [[[kiltTokenId], 100_000]],
	},

	System: {
		Account: [[[addr], { providers: 1, data: { free: initBalance } }]],
	},
})

export const paraId = 2034
export const omnipoolAccount = '7L53bUTBbfuj14UpdCNPwmgzzHSsrsTWBHX5pys32mVWM3C1'

export const spiritnetDestinationAccount = (addr: string) => ({
	V3: {
		parents: 1,
		interior: {
			X2: [
				{ Parachain: SpiritnetConfig.paraId },
				{
					AccountId32: {
						id: u8aToHex(decodeAddress(addr)),
					},
				},
			],
		},
	},
})

export async function getContext(): Promise<Config> {
	return setupContext(options)
}
