use anchor_lang::prelude::*;

// Here are the account structures

// Default Account structures here

#[account]
#[derive(Default)]
pub struct GamePool {
    pub start_ts: u64,
    pub rand: u64,
    pub total_deposit: u64,
    pub claimed: u64,
    pub winner: Pubkey,
    pub entrants: Vec<Pubkey>,
    pub deposit_amounts: Vec<u64>
}

impl GamePool {
    pub const MAX_DATA_SIZE: usize = ( 8
        + 8
        + 8
        + 8
        + 32
        + (24 + 32) 
        + (24 + 8)
    );

    pub fn append(&mut self, entrant: Pubkey, amount: u64) {
        if let Some(index) = self.entrants.iter().position(|&r| r == entrant) {
            self.deposit_amounts[index] += amount;
            self.total_deposit += amount;
        } else {
            self.entrants.push(entrant);
            self.deposit_amounts.push(amount);
            self.total_deposit += amount;
        }
    }

    pub fn set_winner(&mut self) {
        let mut total_deposit = self.rand % self.total_deposit;
        let mut valid = 0;
        let mut index = 0;
        let amount = &self.deposit_amounts;
        for i in 0..amount.len() {
            if total_deposit > amount[i as usize] {
                total_deposit -= amount[i as usize];
            } else {
                index = i;
                valid = 1;
                break;
            }
        }
        if valid == 1 {
            self.winner = self.entrants[index as usize];
        }

    }

  
    pub fn space(len: usize) -> usize{
        8 + Self::MAX_DATA_SIZE + len * (32+8)
    }


}

