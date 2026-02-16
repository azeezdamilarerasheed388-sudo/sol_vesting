use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("9XG9rnCVfEQL2Hf8rE2MX2sCoMQ9ppUPC4UDXBuKX8K3");

#[program]
pub mod sol_vesting {
    use super::*;

    /// Initialize platform configuration (Owner only)
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_collector: Pubkey,
        single_vesting_fee: u64,
        batch_vesting_fee_bps: u16,
        batch_min_fee: u64,
        batch_max_fee: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let owner = &ctx.accounts.owner;

        config.owner = owner.key();
        config.fee_collector = fee_collector;
        config.single_vesting_fixed_fee = single_vesting_fee;
        config.batch_vesting_fee_bps = batch_vesting_fee_bps;
        config.batch_min_fee = batch_min_fee;
        config.batch_max_fee = batch_max_fee;
        config.claim_fixed_fee = 0;
        config.claim_fee_enabled = false;
        config.paused = false;
        config.last_updated = Clock::get()?.unix_timestamp;
        config.is_initialized = true;

        emit!(PlatformInitialized {
            owner: owner.key(),
            fee_collector,
            timestamp: config.last_updated,
        });

        Ok(())
    }

    /// Update fee configuration (Owner only)
    pub fn update_fee_config(
        ctx: Context<UpdateFeeConfig>,
        single_vesting_fee: Option<u64>,
        batch_vesting_fee_bps: Option<u16>,
        batch_min_fee: Option<u64>,
        batch_max_fee: Option<u64>,
        claim_fixed_fee: Option<u64>,
        claim_fee_enabled: Option<bool>,
        fee_collector: Option<Pubkey>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let owner = &ctx.accounts.owner;

        require!(config.owner == owner.key(), VestingError::Unauthorized);

        if let Some(fee) = single_vesting_fee {
            config.single_vesting_fixed_fee = fee;
        }
        if let Some(bps) = batch_vesting_fee_bps {
            require!(bps <= 10000, VestingError::InvalidBasisPoints);
            config.batch_vesting_fee_bps = bps;
        }
        if let Some(min) = batch_min_fee {
            config.batch_min_fee = min;
        }
        if let Some(max) = batch_max_fee {
            require!(max >= config.batch_min_fee, VestingError::InvalidFeeRange);
            config.batch_max_fee = max;
        }
        if let Some(fee) = claim_fixed_fee {
            config.claim_fixed_fee = fee;
        }
        if let Some(enabled) = claim_fee_enabled {
            config.claim_fee_enabled = enabled;
        }
        if let Some(collector) = fee_collector {
            config.fee_collector = collector;
        }

        config.last_updated = Clock::get()?.unix_timestamp;

        emit!(FeeConfigUpdated {
            updated_by: owner.key(),
            timestamp: config.last_updated,
        });

        Ok(())
    }

    /// Transfer ownership (Owner only)
    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let owner = &ctx.accounts.owner;

        require!(config.owner == owner.key(), VestingError::Unauthorized);
        require!(new_owner != Pubkey::default(), VestingError::InvalidOwner);

        config.owner = new_owner;
        config.last_updated = Clock::get()?.unix_timestamp;

        emit!(OwnershipTransferred {
            previous_owner: owner.key(),
            new_owner,
            timestamp: config.last_updated,
        });

        Ok(())
    }

    /// Withdraw accumulated fees (Owner only)
    pub fn withdraw_fees(
        ctx: Context<WithdrawFees>,
        amount: Option<u64>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let owner = &ctx.accounts.owner;
        let fee_collector = &ctx.accounts.fee_collector;
        let config_account_info = &ctx.accounts.config.to_account_info();

        require!(config.owner == owner.key(), VestingError::Unauthorized);
        require!(
            fee_collector.key() == config.fee_collector,
            VestingError::InvalidFeeCollector
        );

        let balance = config_account_info.lamports();
        let withdraw_amount = amount.unwrap_or(balance);

        require!(withdraw_amount <= balance, VestingError::InsufficientBalance);
        require!(withdraw_amount > 0, VestingError::InvalidAmount);

        **config_account_info.try_borrow_mut_lamports()? -= withdraw_amount;
        **fee_collector.try_borrow_mut_lamports()? += withdraw_amount;

        emit!(FeesWithdrawn {
            withdrawn_by: owner.key(),
            amount: withdraw_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Emergency pause (Owner only)
    pub fn set_pause(
        ctx: Context<SetPause>,
        paused: bool,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let owner = &ctx.accounts.owner;

        require!(config.owner == owner.key(), VestingError::Unauthorized);

        config.paused = paused;
        config.last_updated = Clock::get()?.unix_timestamp;

        emit!(PlatformPaused {
            paused,
            set_by: owner.key(),
            timestamp: config.last_updated,
        });

        Ok(())
    }

    /// Create single vesting with client-provided ID
    pub fn create_vesting(
        ctx: Context<CreateVesting>,
        params: VestingParams,
        vesting_id: u64,
    ) -> Result<()> {
        let vesting_account = &mut ctx.accounts.vesting_account;
        let authority = &ctx.accounts.authority;
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.paused, VestingError::PlatformPaused);
        require!(params.amount > 0, VestingError::InvalidAmount);
        require!(
            params.end_time > params.start_time,
            VestingError::InvalidTimeRange
        );
        if params.cliff_time > 0 {
            require!(
                params.cliff_time >= params.start_time && params.cliff_time <= params.end_time,
                VestingError::InvalidCliffTime
            );
        }

        // Calculate and transfer fee
        let fee = config.single_vesting_fixed_fee;
        require!(params.amount > fee, VestingError::InsufficientForFee);

        // Transfer fee to platform
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: ctx.accounts.config.to_account_info(),
                },
            ),
            fee,
        )?;

        // Transfer remaining SOL to vesting PDA
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: vesting_account.to_account_info(),
                },
            ),
            params.amount - fee,
        )?;

        // Initialize vesting account
        vesting_account.vesting_id = vesting_id;
        vesting_account.recipient = params.recipient;
        vesting_account.authority = authority.key();
        vesting_account.total_amount = params.amount - fee;
        vesting_account.start_time = params.start_time;
        vesting_account.end_time = params.end_time;
        vesting_account.cliff_time = params.cliff_time;
        vesting_account.claimed_amount = 0;
        vesting_account.revoked = false;
        vesting_account.revoke_authority = params.revoke_authority.unwrap_or(authority.key());
        vesting_account.is_multi = false;
        vesting_account.batch_id = None;
        vesting_account.created_at = clock.unix_timestamp;
        vesting_account.last_claim_time = 0;

        emit!(VestingCreated {
            vesting_account: vesting_account.key(),
            vesting_id,
            recipient: params.recipient,
            amount: params.amount - fee,
            fee_paid: fee,
            creator: authority.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Create batch vesting with client-provided batch ID
    pub fn create_batch_vesting(
        ctx: Context<CreateBatchVesting>,
        batch_params: BatchVestingParams,
    ) -> Result<()> {
        let batch_account = &mut ctx.accounts.batch_account;
        let authority = &ctx.accounts.authority;
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.paused, VestingError::PlatformPaused);
        require!(
            batch_params.recipients.len() == batch_params.amounts.len() 
            && batch_params.recipients.len() == batch_params.schedules.len(),
            VestingError::InvalidBatchData
        );
        require!(
            batch_params.recipients.len() <= 25,
            VestingError::BatchTooLarge
        );
        require!(batch_params.recipients.len() > 0, VestingError::EmptyBatch);

        // Calculate total amount
        let total_amount: u64 = batch_params.amounts.iter().sum();
        require!(total_amount > 0, VestingError::InvalidAmount);

        // Calculate batch fee
        let fee = calculate_batch_fee(config, total_amount)?;
        require!(total_amount > fee, VestingError::InsufficientForFee);

        // Transfer fee to platform
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: ctx.accounts.config.to_account_info(),
                },
            ),
            fee,
        )?;

        // Transfer remaining to batch PDA
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: batch_account.to_account_info(),
                },
            ),
            total_amount - fee,
        )?;

        // Initialize batch account
        batch_account.batch_id = batch_params.batch_id;
        batch_account.creator = authority.key();
        batch_account.total_recipients = batch_params.recipients.len() as u32;
        batch_account.total_amount = total_amount - fee;
        batch_account.created_at = clock.unix_timestamp;
        batch_account.metadata_uri = batch_params.metadata_uri;
        batch_account.status = BatchStatus::Created;
        batch_account.fee_paid = fee;

        emit!(BatchVestingCreated {
            batch_id: batch_params.batch_id,
            creator: authority.key(),
            total_recipients: batch_params.recipients.len() as u32,
            total_amount: total_amount - fee,
            fee_paid: fee,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Claim vested SOL
    pub fn claim_vested(
        ctx: Context<ClaimVested>,
    ) -> Result<()> {
        let vesting_account = &mut ctx.accounts.vesting_account;
        let recipient = &ctx.accounts.recipient;
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.paused, VestingError::PlatformPaused);
        require!(!vesting_account.revoked, VestingError::Revoked);
        require!(
            recipient.key() == vesting_account.recipient,
            VestingError::Unauthorized
        );

        let claimable_amount = get_claimable_amount(vesting_account, clock.unix_timestamp)?;
        require!(claimable_amount > 0, VestingError::NothingToClaim);

        if config.claim_fee_enabled {
            let claim_fee = config.claim_fixed_fee;
            require!(claimable_amount > claim_fee, VestingError::InsufficientForFee);

            // Transfer fee to config
            **vesting_account.to_account_info().try_borrow_mut_lamports()? -= claim_fee;
            **config.to_account_info().try_borrow_mut_lamports()? += claim_fee;

            vesting_account.claimed_amount += claimable_amount - claim_fee;
            
            **vesting_account.to_account_info().try_borrow_mut_lamports()? -= claimable_amount - claim_fee;
            **recipient.to_account_info().try_borrow_mut_lamports()? += claimable_amount - claim_fee;

            emit!(VestingClaimed {
                vesting_account: vesting_account.key(),
                recipient: recipient.key(),
                amount: claimable_amount - claim_fee,
                fee_paid: claim_fee,
                timestamp: clock.unix_timestamp,
            });
        } else {
            vesting_account.claimed_amount += claimable_amount;
            
            **vesting_account.to_account_info().try_borrow_mut_lamports()? -= claimable_amount;
            **recipient.to_account_info().try_borrow_mut_lamports()? += claimable_amount;

            emit!(VestingClaimed {
                vesting_account: vesting_account.key(),
                recipient: recipient.key(),
                amount: claimable_amount,
                fee_paid: 0,
                timestamp: clock.unix_timestamp,
            });
        }

        vesting_account.last_claim_time = clock.unix_timestamp;

        Ok(())
    }

    /// Batch claim with client-provided claim batch ID
    pub fn batch_claim(
        ctx: Context<BatchClaim>,
        claim_batch_id: u64,
        claim_indices: Vec<u32>,
    ) -> Result<()> {
        let batch_account = &ctx.accounts.batch_account;
        let recipient = &ctx.accounts.recipient;
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.paused, VestingError::PlatformPaused);
        require!(!claim_indices.is_empty(), VestingError::EmptyClaimBatch);
        require!(
            claim_indices.len() <= 10,
            VestingError::ClaimBatchTooLarge
        );

        let mut total_claim = 0;
        let mut total_fee = 0;

        // In production, you'd loop through indices and process each claim
        // For now, this is a placeholder for the logic
        for _index in &claim_indices {
            total_claim += 100; // Placeholder
        }

        if config.claim_fee_enabled {
            total_fee = config.claim_fixed_fee * (claim_indices.len() as u64);
            require!(total_claim > total_fee, VestingError::InsufficientForFee);
        }

        emit!(BatchClaimed {
            batch_id: batch_account.batch_id,
            claim_batch_id,
            recipient: recipient.key(),
            total_amount: total_claim - total_fee,
            fee_paid: total_fee,
            num_claims: claim_indices.len() as u32,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Revoke vesting
    pub fn revoke_vesting(
        ctx: Context<RevokeVesting>,
    ) -> Result<()> {
        let vesting_account = &mut ctx.accounts.vesting_account;
        let revoke_authority = &ctx.accounts.revoke_authority;
        let clock = Clock::get()?;

        require!(!ctx.accounts.config.paused, VestingError::PlatformPaused);
        require!(
            revoke_authority.key() == vesting_account.revoke_authority,
            VestingError::Unauthorized
        );
        require!(!vesting_account.revoked, VestingError::AlreadyRevoked);

        let claimable = get_claimable_amount(vesting_account, clock.unix_timestamp)?;
        let unclaimed = vesting_account.total_amount - vesting_account.claimed_amount - claimable;

        vesting_account.revoked = true;

        if unclaimed > 0 {
            **vesting_account.to_account_info().try_borrow_mut_lamports()? -= unclaimed;
            **revoke_authority.to_account_info().try_borrow_mut_lamports()? += unclaimed;
        }

        emit!(VestingRevoked {
            vesting_account: vesting_account.key(),
            revoke_authority: revoke_authority.key(),
            unclaimed_amount: unclaimed,
            claimable_forfeited: claimable,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Update vesting schedule (only if not started)
    pub fn update_vesting_schedule(
        ctx: Context<UpdateVesting>,
        new_params: VestingParamsUpdate,
    ) -> Result<()> {
        let vesting_account = &mut ctx.accounts.vesting_account;
        let authority = &ctx.accounts.authority;
        let clock = Clock::get()?;

        require!(!ctx.accounts.config.paused, VestingError::PlatformPaused);
        require!(
            authority.key() == vesting_account.authority,
            VestingError::Unauthorized
        );
        require!(
            clock.unix_timestamp < vesting_account.start_time,
            VestingError::VestingAlreadyStarted
        );

        if let Some(end_time) = new_params.end_time {
            require!(end_time > vesting_account.start_time, VestingError::InvalidTimeRange);
            vesting_account.end_time = end_time;
        }

        if let Some(cliff_time) = new_params.cliff_time {
            if cliff_time > 0 {
                require!(
                    cliff_time >= vesting_account.start_time && cliff_time <= vesting_account.end_time,
                    VestingError::InvalidCliffTime
                );
            }
            vesting_account.cliff_time = cliff_time;
        }

        emit!(VestingUpdated {
            vesting_account: vesting_account.key(),
            updated_by: authority.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    /// Add recipient to existing batch with client-provided index
    pub fn add_to_batch(
        ctx: Context<AddToBatch>,
        recipient_index: u64,
        new_recipient: Pubkey,
        amount: u64,
        _schedule: VestingSchedule,
    ) -> Result<()> {
        let batch_account = &mut ctx.accounts.batch_account;
        let authority = &ctx.accounts.authority;
        let config = &ctx.accounts.config;
        let clock = Clock::get()?;

        require!(!config.paused, VestingError::PlatformPaused);
        require!(
            authority.key() == batch_account.creator,
            VestingError::Unauthorized
        );
        require!(
            batch_account.total_recipients < 1000,
            VestingError::BatchFull
        );
        require!(amount > 0, VestingError::InvalidAmount);

        let new_total = batch_account.total_amount + amount;
        let original_fee = batch_account.fee_paid;
        let new_fee = calculate_batch_fee(config, new_total)?;
        let additional_fee = new_fee - original_fee;

        if additional_fee > 0 {
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: authority.to_account_info(),
                        to: config.to_account_info(),
                    },
                ),
                additional_fee,
            )?;
        }

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: batch_account.to_account_info(),
                },
            ),
            amount,
        )?;

        batch_account.total_recipients += 1;
        batch_account.total_amount += amount;
        batch_account.fee_paid = new_fee;

        emit!(BatchUpdated {
            batch_id: batch_account.batch_id,
            recipient_index,
            new_recipient,
            amount,
            additional_fee,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

// Helper Functions
fn calculate_batch_fee(config: &FeeConfig, total_amount: u64) -> Result<u64> {
    let fee = (total_amount as u128)
        .checked_mul(config.batch_vesting_fee_bps as u128)
        .ok_or(VestingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(VestingError::MathOverflow)? as u64;

    Ok(if fee < config.batch_min_fee {
        config.batch_min_fee
    } else if fee > config.batch_max_fee {
        config.batch_max_fee
    } else {
        fee
    })
}

fn get_claimable_amount(
    vesting_account: &VestingAccount,
    current_time: i64,
) -> Result<u64> {
    if vesting_account.revoked {
        return Ok(0);
    }

    if current_time < vesting_account.cliff_time {
        return Ok(0);
    }

    if current_time >= vesting_account.end_time {
        return Ok(vesting_account.total_amount - vesting_account.claimed_amount);
    }

    let total_duration = vesting_account.end_time
        .checked_sub(vesting_account.start_time)
        .ok_or(VestingError::MathOverflow)?;
    
    let elapsed = current_time
        .checked_sub(vesting_account.start_time)
        .ok_or(VestingError::MathOverflow)?;
    
    let vested_amount = (vesting_account.total_amount as u128)
        .checked_mul(elapsed as u128)
        .ok_or(VestingError::MathOverflow)?
        .checked_div(total_duration as u128)
        .ok_or(VestingError::MathOverflow)? as u64;

    Ok(vested_amount
        .checked_sub(vesting_account.claimed_amount)
        .ok_or(VestingError::MathOverflow)?)
}

// Account Structures

#[account]
#[derive(Default)]
pub struct FeeConfig {
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub single_vesting_fixed_fee: u64,
    pub batch_vesting_fee_bps: u16,
    pub batch_min_fee: u64,
    pub batch_max_fee: u64,
    pub claim_fixed_fee: u64,
    pub claim_fee_enabled: bool,
    pub paused: bool,
    pub last_updated: i64,
    pub is_initialized: bool,
}

impl FeeConfig {
    pub const LEN: usize = 32 + 32 + 8 + 2 + 8 + 8 + 8 + 1 + 1 + 8 + 1;
}

#[account]
pub struct VestingAccount {
    pub vesting_id: u64,
    pub recipient: Pubkey,
    pub authority: Pubkey,
    pub total_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub claimed_amount: u64,
    pub revoked: bool,
    pub revoke_authority: Pubkey,
    pub is_multi: bool,
    pub batch_id: Option<u64>,
    pub created_at: i64,
    pub last_claim_time: i64,
}

impl VestingAccount {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 32 + 1 + 9 + 8 + 8;
}

#[account]
pub struct BatchVestingAccount {
    pub batch_id: u64,
    pub creator: Pubkey,
    pub total_recipients: u32,
    pub total_amount: u64,
    pub created_at: i64,
    pub metadata_uri: Option<String>,
    pub status: BatchStatus,
    pub fee_paid: u64,
}

impl BatchVestingAccount {
    pub const LEN: usize = 8 + 32 + 4 + 8 + 8 + (4 + 200) + 1 + 8;
}

// Parameter Structs
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VestingParams {
    pub recipient: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub revoke_authority: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VestingSchedule {
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub vesting_type: VestingType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BatchVestingParams {
    pub batch_id: u64,
    pub recipients: Vec<Pubkey>,
    pub amounts: Vec<u64>,
    pub schedules: Vec<VestingSchedule>,
    pub metadata_uri: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VestingParamsUpdate {
    pub end_time: Option<i64>,
    pub cliff_time: Option<i64>,
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum VestingType {
    Linear,
    CliffThenLinear,
    Exponential,
    Custom,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum BatchStatus {
    Created,
    PartiallyDistributed,
    FullyDistributed,
    Cancelled,
}

// Account Contexts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + FeeConfig::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFeeConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// CHECK: Fee collector validated against config
    #[account(mut)]
    pub fee_collector: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(params: VestingParams, vesting_id: u64)]
pub struct CreateVesting<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + VestingAccount::LEN,
        seeds = [
            b"vesting", 
            authority.key().as_ref(),
            params.recipient.key().as_ref(),
            &vesting_id.to_le_bytes()
        ],
        bump
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump,
        constraint = !config.paused @ VestingError::PlatformPaused
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Recipient wallet
    pub recipient: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(batch_params: BatchVestingParams)]
pub struct CreateBatchVesting<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + BatchVestingAccount::LEN,
        seeds = [
            b"batch",
            authority.key().as_ref(),
            &batch_params.batch_id.to_le_bytes()
        ],
        bump
    )]
    pub batch_account: Account<'info, BatchVestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump,
        constraint = !config.paused @ VestingError::PlatformPaused
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimVested<'info> {
    #[account(
        mut,
        constraint = vesting_account.recipient == recipient.key() @ VestingError::Unauthorized
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub recipient: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(claim_batch_id: u64, claim_indices: Vec<u32>)]
pub struct BatchClaim<'info> {
    #[account(
        mut,
        seeds = [b"batch", batch_account.creator.as_ref(), &batch_account.batch_id.to_le_bytes()],
        bump
    )]
    pub batch_account: Account<'info, BatchVestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub recipient: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeVesting<'info> {
    #[account(mut)]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(
        constraint = revoke_authority.key() == vesting_account.revoke_authority @ VestingError::Unauthorized
    )]
    pub revoke_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateVesting<'info> {
    #[account(mut)]
    pub vesting_account: Account<'info, VestingAccount>,
    
    #[account(
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(
        constraint = authority.key() == vesting_account.authority @ VestingError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(recipient_index: u64, new_recipient: Pubkey, amount: u64, schedule: VestingSchedule)]
pub struct AddToBatch<'info> {
    #[account(mut)]
    pub batch_account: Account<'info, BatchVestingAccount>,
    
    #[account(
        mut,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, FeeConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Events
#[event]
pub struct PlatformInitialized {
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct FeeConfigUpdated {
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OwnershipTransferred {
    pub previous_owner: Pubkey,
    pub new_owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct FeesWithdrawn {
    pub withdrawn_by: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct PlatformPaused {
    pub paused: bool,
    pub set_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct VestingCreated {
    pub vesting_account: Pubkey,
    pub vesting_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub fee_paid: u64,
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct BatchVestingCreated {
    pub batch_id: u64,
    pub creator: Pubkey,
    pub total_recipients: u32,
    pub total_amount: u64,
    pub fee_paid: u64,
    pub timestamp: i64,
}

#[event]
pub struct VestingClaimed {
    pub vesting_account: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub fee_paid: u64,
    pub timestamp: i64,
}

#[event]
pub struct BatchClaimed {
    pub batch_id: u64,
    pub claim_batch_id: u64,
    pub recipient: Pubkey,
    pub total_amount: u64,
    pub fee_paid: u64,
    pub num_claims: u32,
    pub timestamp: i64,
}

#[event]
pub struct VestingRevoked {
    pub vesting_account: Pubkey,
    pub revoke_authority: Pubkey,
    pub unclaimed_amount: u64,
    pub claimable_forfeited: u64,
    pub timestamp: i64,
}

#[event]
pub struct VestingUpdated {
    pub vesting_account: Pubkey,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct BatchUpdated {
    pub batch_id: u64,
    pub recipient_index: u64,
    pub new_recipient: Pubkey,
    pub amount: u64,
    pub additional_fee: u64,
    pub timestamp: i64,
}

// Error Codes
#[error_code]
pub enum VestingError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid time range")]
    InvalidTimeRange,
    #[msg("Invalid cliff time")]
    InvalidCliffTime,
    #[msg("Vesting is revoked")]
    Revoked,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Invalid batch data")]
    InvalidBatchData,
    #[msg("Batch too large (max 25)")]
    BatchTooLarge,
    #[msg("Empty batch not allowed")]
    EmptyBatch,
    #[msg("Vesting already started")]
    VestingAlreadyStarted,
    #[msg("Already revoked")]
    AlreadyRevoked,
    #[msg("Batch is full")]
    BatchFull,
    #[msg("Invalid basis points (max 10000)")]
    InvalidBasisPoints,
    #[msg("Invalid fee range")]
    InvalidFeeRange,
    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,
    #[msg("Invalid fee collector")]
    InvalidFeeCollector,
    #[msg("Insufficient amount for fee")]
    InsufficientForFee,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Platform is paused")]
    PlatformPaused,
    #[msg("Empty claim batch")]
    EmptyClaimBatch,
    #[msg("Claim batch too large (max 10)")]
    ClaimBatchTooLarge,
    #[msg("Math overflow")]
    MathOverflow,
}

// Test Module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claimable_amount() {
        let vesting = VestingAccount {
            vesting_id: 1,
            recipient: Pubkey::default(),
            authority: Pubkey::default(),
            total_amount: 1000,
            start_time: 0,
            end_time: 100,
            cliff_time: 20,
            claimed_amount: 0,
            revoked: false,
            revoke_authority: Pubkey::default(),
            is_multi: false,
            batch_id: None,
            created_at: 0,
            last_claim_time: 0,
        };

        assert_eq!(get_claimable_amount(&vesting, 10).unwrap(), 0);
        assert_eq!(get_claimable_amount(&vesting, 50).unwrap(), 300);
        assert_eq!(get_claimable_amount(&vesting, 100).unwrap(), 1000);
    }

    #[test]
    fn test_batch_fee_calculation() {
        let config = FeeConfig {
            owner: Pubkey::default(),
            fee_collector: Pubkey::default(),
            single_vesting_fixed_fee: 10_000_000,
            batch_vesting_fee_bps: 50,
            batch_min_fee: 100_000_000,
            batch_max_fee: 10_000_000_000,
            claim_fixed_fee: 1_000_000,
            claim_fee_enabled: false,
            paused: false,
            last_updated: 0,
            is_initialized: true,
        };

        assert_eq!(calculate_batch_fee(&config, 10_000_000_000).unwrap(), 100_000_000);
        assert_eq!(calculate_batch_fee(&config, 100_000_000_000).unwrap(), 500_000_000);
        assert_eq!(calculate_batch_fee(&config, 10_000_000_000_000).unwrap(), 10_000_000_000);
    }
}