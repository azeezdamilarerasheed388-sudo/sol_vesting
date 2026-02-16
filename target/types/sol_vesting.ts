/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/sol_vesting.json`.
 */
export type SolVesting = {
  "address": "9XG9rnCVfEQL2Hf8rE2MX2sCoMQ9ppUPC4UDXBuKX8K3",
  "metadata": {
    "name": "solVesting",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addToBatch",
      "docs": [
        "Add recipient to existing batch with client-provided index"
      ],
      "discriminator": [
        36,
        130,
        236,
        47,
        205,
        2,
        252,
        197
      ],
      "accounts": [
        {
          "name": "batchAccount",
          "writable": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "recipientIndex",
          "type": "u64"
        },
        {
          "name": "newRecipient",
          "type": "pubkey"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "schedule",
          "type": {
            "defined": {
              "name": "vestingSchedule"
            }
          }
        }
      ]
    },
    {
      "name": "batchClaim",
      "docs": [
        "Batch claim with client-provided claim batch ID"
      ],
      "discriminator": [
        3,
        1,
        13,
        209,
        198,
        215,
        144,
        13
      ],
      "accounts": [
        {
          "name": "batchAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  116,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "batch_account.creator",
                "account": "batchVestingAccount"
              },
              {
                "kind": "account",
                "path": "batch_account.batch_id",
                "account": "batchVestingAccount"
              }
            ]
          }
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "recipient",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "claimBatchId",
          "type": "u64"
        },
        {
          "name": "claimIndices",
          "type": {
            "vec": "u32"
          }
        }
      ]
    },
    {
      "name": "claimVested",
      "docs": [
        "Claim vested SOL"
      ],
      "discriminator": [
        208,
        190,
        166,
        114,
        203,
        225,
        140,
        208
      ],
      "accounts": [
        {
          "name": "vestingAccount",
          "writable": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "recipient",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "createBatchVesting",
      "docs": [
        "Create batch vesting with client-provided batch ID"
      ],
      "discriminator": [
        109,
        188,
        69,
        207,
        46,
        177,
        203,
        44
      ],
      "accounts": [
        {
          "name": "batchAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  116,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "batch_params.batch_id"
              }
            ]
          }
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "batchParams",
          "type": {
            "defined": {
              "name": "batchVestingParams"
            }
          }
        }
      ]
    },
    {
      "name": "createVesting",
      "docs": [
        "Create single vesting with client-provided ID"
      ],
      "discriminator": [
        135,
        184,
        171,
        156,
        197,
        162,
        246,
        44
      ],
      "accounts": [
        {
          "name": "vestingAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "params.recipient"
              },
              {
                "kind": "arg",
                "path": "vestingId"
              }
            ]
          }
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "recipient"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "vestingParams"
            }
          }
        },
        {
          "name": "vestingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "Initialize platform configuration (Owner only)"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "feeCollector",
          "type": "pubkey"
        },
        {
          "name": "singleVestingFee",
          "type": "u64"
        },
        {
          "name": "batchVestingFeeBps",
          "type": "u16"
        },
        {
          "name": "batchMinFee",
          "type": "u64"
        },
        {
          "name": "batchMaxFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "revokeVesting",
      "docs": [
        "Revoke vesting"
      ],
      "discriminator": [
        12,
        252,
        252,
        168,
        39,
        101,
        98,
        9
      ],
      "accounts": [
        {
          "name": "vestingAccount",
          "writable": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "revokeAuthority",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "setPause",
      "docs": [
        "Emergency pause (Owner only)"
      ],
      "discriminator": [
        63,
        32,
        154,
        2,
        56,
        103,
        79,
        45
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "paused",
          "type": "bool"
        }
      ]
    },
    {
      "name": "transferOwnership",
      "docs": [
        "Transfer ownership (Owner only)"
      ],
      "discriminator": [
        65,
        177,
        215,
        73,
        53,
        45,
        99,
        47
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newOwner",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "updateFeeConfig",
      "docs": [
        "Update fee configuration (Owner only)"
      ],
      "discriminator": [
        104,
        184,
        103,
        242,
        88,
        151,
        107,
        20
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "singleVestingFee",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "batchVestingFeeBps",
          "type": {
            "option": "u16"
          }
        },
        {
          "name": "batchMinFee",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "batchMaxFee",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "claimFixedFee",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "claimFeeEnabled",
          "type": {
            "option": "bool"
          }
        },
        {
          "name": "feeCollector",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "updateVestingSchedule",
      "docs": [
        "Update vesting schedule (only if not started)"
      ],
      "discriminator": [
        249,
        18,
        68,
        193,
        205,
        46,
        146,
        114
      ],
      "accounts": [
        {
          "name": "vestingAccount",
          "writable": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newParams",
          "type": {
            "defined": {
              "name": "vestingParamsUpdate"
            }
          }
        }
      ]
    },
    {
      "name": "withdrawFees",
      "docs": [
        "Withdraw accumulated fees (Owner only)"
      ],
      "discriminator": [
        198,
        212,
        171,
        109,
        144,
        215,
        174,
        89
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "feeCollector",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": {
            "option": "u64"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "batchVestingAccount",
      "discriminator": [
        78,
        109,
        52,
        34,
        167,
        22,
        232,
        155
      ]
    },
    {
      "name": "feeConfig",
      "discriminator": [
        143,
        52,
        146,
        187,
        219,
        123,
        76,
        155
      ]
    },
    {
      "name": "vestingAccount",
      "discriminator": [
        102,
        73,
        10,
        233,
        200,
        188,
        228,
        216
      ]
    }
  ],
  "events": [
    {
      "name": "batchClaimed",
      "discriminator": [
        60,
        42,
        120,
        40,
        81,
        126,
        128,
        160
      ]
    },
    {
      "name": "batchUpdated",
      "discriminator": [
        142,
        123,
        95,
        53,
        37,
        90,
        159,
        254
      ]
    },
    {
      "name": "batchVestingCreated",
      "discriminator": [
        136,
        224,
        195,
        89,
        226,
        48,
        238,
        97
      ]
    },
    {
      "name": "feeConfigUpdated",
      "discriminator": [
        45,
        50,
        42,
        173,
        193,
        67,
        52,
        244
      ]
    },
    {
      "name": "feesWithdrawn",
      "discriminator": [
        234,
        15,
        0,
        119,
        148,
        241,
        40,
        21
      ]
    },
    {
      "name": "ownershipTransferred",
      "discriminator": [
        172,
        61,
        205,
        183,
        250,
        50,
        38,
        98
      ]
    },
    {
      "name": "platformInitialized",
      "discriminator": [
        16,
        222,
        212,
        5,
        213,
        140,
        112,
        162
      ]
    },
    {
      "name": "platformPaused",
      "discriminator": [
        110,
        72,
        152,
        13,
        0,
        222,
        149,
        129
      ]
    },
    {
      "name": "vestingClaimed",
      "discriminator": [
        166,
        62,
        135,
        158,
        137,
        1,
        85,
        15
      ]
    },
    {
      "name": "vestingCreated",
      "discriminator": [
        181,
        223,
        229,
        220,
        204,
        6,
        169,
        125
      ]
    },
    {
      "name": "vestingRevoked",
      "discriminator": [
        215,
        148,
        193,
        127,
        237,
        245,
        90,
        75
      ]
    },
    {
      "name": "vestingUpdated",
      "discriminator": [
        100,
        10,
        186,
        179,
        104,
        221,
        249,
        217
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidAmount",
      "msg": "Invalid amount"
    },
    {
      "code": 6001,
      "name": "invalidTimeRange",
      "msg": "Invalid time range"
    },
    {
      "code": 6002,
      "name": "invalidCliffTime",
      "msg": "Invalid cliff time"
    },
    {
      "code": 6003,
      "name": "revoked",
      "msg": "Vesting is revoked"
    },
    {
      "code": 6004,
      "name": "unauthorized",
      "msg": "Unauthorized access"
    },
    {
      "code": 6005,
      "name": "nothingToClaim",
      "msg": "Nothing to claim"
    },
    {
      "code": 6006,
      "name": "invalidBatchData",
      "msg": "Invalid batch data"
    },
    {
      "code": 6007,
      "name": "batchTooLarge",
      "msg": "Batch too large (max 25)"
    },
    {
      "code": 6008,
      "name": "emptyBatch",
      "msg": "Empty batch not allowed"
    },
    {
      "code": 6009,
      "name": "vestingAlreadyStarted",
      "msg": "Vesting already started"
    },
    {
      "code": 6010,
      "name": "alreadyRevoked",
      "msg": "Already revoked"
    },
    {
      "code": 6011,
      "name": "batchFull",
      "msg": "Batch is full"
    },
    {
      "code": 6012,
      "name": "invalidBasisPoints",
      "msg": "Invalid basis points (max 10000)"
    },
    {
      "code": 6013,
      "name": "invalidFeeRange",
      "msg": "Invalid fee range"
    },
    {
      "code": 6014,
      "name": "insufficientBalance",
      "msg": "Insufficient balance for withdrawal"
    },
    {
      "code": 6015,
      "name": "invalidFeeCollector",
      "msg": "Invalid fee collector"
    },
    {
      "code": 6016,
      "name": "insufficientForFee",
      "msg": "Insufficient amount for fee"
    },
    {
      "code": 6017,
      "name": "invalidOwner",
      "msg": "Invalid owner"
    },
    {
      "code": 6018,
      "name": "platformPaused",
      "msg": "Platform is paused"
    },
    {
      "code": 6019,
      "name": "emptyClaimBatch",
      "msg": "Empty claim batch"
    },
    {
      "code": 6020,
      "name": "claimBatchTooLarge",
      "msg": "Claim batch too large (max 10)"
    },
    {
      "code": 6021,
      "name": "mathOverflow",
      "msg": "Math overflow"
    }
  ],
  "types": [
    {
      "name": "batchClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "batchId",
            "type": "u64"
          },
          {
            "name": "claimBatchId",
            "type": "u64"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "feePaid",
            "type": "u64"
          },
          {
            "name": "numClaims",
            "type": "u32"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "batchStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "created"
          },
          {
            "name": "partiallyDistributed"
          },
          {
            "name": "fullyDistributed"
          },
          {
            "name": "cancelled"
          }
        ]
      }
    },
    {
      "name": "batchUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "batchId",
            "type": "u64"
          },
          {
            "name": "recipientIndex",
            "type": "u64"
          },
          {
            "name": "newRecipient",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "additionalFee",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "batchVestingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "batchId",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "totalRecipients",
            "type": "u32"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "metadataUri",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "batchStatus"
              }
            }
          },
          {
            "name": "feePaid",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "batchVestingCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "batchId",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "totalRecipients",
            "type": "u32"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "feePaid",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "batchVestingParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "batchId",
            "type": "u64"
          },
          {
            "name": "recipients",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "amounts",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "schedules",
            "type": {
              "vec": {
                "defined": {
                  "name": "vestingSchedule"
                }
              }
            }
          },
          {
            "name": "metadataUri",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    },
    {
      "name": "feeConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "feeCollector",
            "type": "pubkey"
          },
          {
            "name": "singleVestingFixedFee",
            "type": "u64"
          },
          {
            "name": "batchVestingFeeBps",
            "type": "u16"
          },
          {
            "name": "batchMinFee",
            "type": "u64"
          },
          {
            "name": "batchMaxFee",
            "type": "u64"
          },
          {
            "name": "claimFixedFee",
            "type": "u64"
          },
          {
            "name": "claimFeeEnabled",
            "type": "bool"
          },
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "feeConfigUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "updatedBy",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feesWithdrawn",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "withdrawnBy",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "ownershipTransferred",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "previousOwner",
            "type": "pubkey"
          },
          {
            "name": "newOwner",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "platformInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "feeCollector",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "platformPaused",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "setBy",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vestingId",
            "type": "u64"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "cliffTime",
            "type": "i64"
          },
          {
            "name": "claimedAmount",
            "type": "u64"
          },
          {
            "name": "revoked",
            "type": "bool"
          },
          {
            "name": "revokeAuthority",
            "type": "pubkey"
          },
          {
            "name": "isMulti",
            "type": "bool"
          },
          {
            "name": "batchId",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "lastClaimTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vestingAccount",
            "type": "pubkey"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "feePaid",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vestingAccount",
            "type": "pubkey"
          },
          {
            "name": "vestingId",
            "type": "u64"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "feePaid",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "cliffTime",
            "type": "i64"
          },
          {
            "name": "revokeAuthority",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "vestingParamsUpdate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "endTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "cliffTime",
            "type": {
              "option": "i64"
            }
          }
        ]
      }
    },
    {
      "name": "vestingRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vestingAccount",
            "type": "pubkey"
          },
          {
            "name": "revokeAuthority",
            "type": "pubkey"
          },
          {
            "name": "unclaimedAmount",
            "type": "u64"
          },
          {
            "name": "claimableForfeited",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vestingSchedule",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "cliffTime",
            "type": "i64"
          },
          {
            "name": "vestingType",
            "type": {
              "defined": {
                "name": "vestingType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "vestingType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "linear"
          },
          {
            "name": "cliffThenLinear"
          },
          {
            "name": "exponential"
          },
          {
            "name": "custom"
          }
        ]
      }
    },
    {
      "name": "vestingUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vestingAccount",
            "type": "pubkey"
          },
          {
            "name": "updatedBy",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
