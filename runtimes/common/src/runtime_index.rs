pub enum RuntimeSpiritnet {
	Error = -1,
	System = 0,
	RandomnessCollectiveFlip = 1,
	Timestamp = 2,
	Indices = 5,
	Balances = 6,
	TransactionPayment = 7,
	Aura = 23,
	Session = 22,
	ParachainStaking = 21,
	Authorship = 20,
	AuraExt = 24,
	Democracy = 30,
	Council = 31,
	TechnicalCommittee = 32,
	TechnicalMembership = 34,
	Treasury = 35,
	Utility = 40,
	Vesting = 41,
	Scheduler = 42,
	Proxy = 43,
	Preimage = 44,
	TipsMembership = 45,
	Tips = 46,
	Ctype = 61,
	Attestation = 62,
	Delegation = 63,
	Did = 64,
	Inflation = 66,
	DidLookup = 67,
	Web3Names = 68,
	PublicCredentials = 69,
	ParachainSystem = 80,
	ParachainInfo = 81,
	XcmpQueue = 82,
	PolkadotXcm = 83,
	CumulusXcm = 84,
	DmpQueue = 85,
}

impl From<u32> for RuntimeSpiritnet {
	fn from(num: u32) -> Self {
		match num {
			0 => RuntimeSpiritnet::System,
			1 => RuntimeSpiritnet::RandomnessCollectiveFlip,
			2 => RuntimeSpiritnet::Timestamp,
			5 => RuntimeSpiritnet::Indices,
			6 => RuntimeSpiritnet::Balances,
			7 => RuntimeSpiritnet::TransactionPayment,
			23 => RuntimeSpiritnet::Aura,
			22 => RuntimeSpiritnet::Session,
			21 => RuntimeSpiritnet::ParachainStaking,
			20 => RuntimeSpiritnet::Authorship,
			24 => RuntimeSpiritnet::AuraExt,
			30 => RuntimeSpiritnet::Democracy,
			31 => RuntimeSpiritnet::Council,
			32 => RuntimeSpiritnet::TechnicalCommittee,
			34 => RuntimeSpiritnet::TechnicalMembership,
			35 => RuntimeSpiritnet::Treasury,
			40 => RuntimeSpiritnet::Utility,
			41 => RuntimeSpiritnet::Vesting,
			42 => RuntimeSpiritnet::Scheduler,
			43 => RuntimeSpiritnet::Proxy,
			44 => RuntimeSpiritnet::Preimage,
			45 => RuntimeSpiritnet::TipsMembership,
			46 => RuntimeSpiritnet::Tips,
			61 => RuntimeSpiritnet::Ctype,
			62 => RuntimeSpiritnet::Attestation,
			63 => RuntimeSpiritnet::Delegation,
			64 => RuntimeSpiritnet::Did,
			66 => RuntimeSpiritnet::Inflation,
			67 => RuntimeSpiritnet::DidLookup,
			68 => RuntimeSpiritnet::Web3Names,
			69 => RuntimeSpiritnet::PublicCredentials,
			80 => RuntimeSpiritnet::ParachainSystem,
			81 => RuntimeSpiritnet::ParachainInfo,
			82 => RuntimeSpiritnet::XcmpQueue,
			83 => RuntimeSpiritnet::PolkadotXcm,
			84 => RuntimeSpiritnet::CumulusXcm,
			85 => RuntimeSpiritnet::DmpQueue,
			_ => RuntimeSpiritnet::Error,
		}
	}
}

pub enum CallIndexDid {
	Error = -1,
	Create = 0,
}

impl From<u32> for CallIndexDid {
	fn from(num: u32) -> Self {
		match num {
			0 => CallIndexDid::Create,
			_ => CallIndexDid::Error,
		}
	}
}
pub enum CallIndexUtility {
	Error = -1,
	Batch = 0,
	BatchAll = 2,
	ForceBatch = 4,
}

impl From<u32> for CallIndexUtility {
	fn from(num: u32) -> Self {
		match num {
			0 => CallIndexUtility::Batch,
			2 => CallIndexUtility::BatchAll,
			4 => CallIndexUtility::ForceBatch,
			_ => CallIndexUtility::Error,
		}
	}
}
