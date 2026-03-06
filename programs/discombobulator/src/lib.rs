use anchor_lang::prelude::*;

declare_id!("Disco11111111111111111111111111111111");

#[program]
pub mod discombobulator {
    use super::*;

    pub fn shuffle(_ctx: Context<Shuffle>, _seed: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Shuffle<'info> {
    /// CHECK: devnet test account
    pub target: UncheckedAccount<'info>,
}
