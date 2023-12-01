use {
    crossbeam_channel::{Receiver, Sender},
    nexis_sdk::{hash::Hash, pubkey::Pubkey},
    nexis_vote_program::vote_state::Vote,
};

pub type ReplayedVote = (Pubkey, Vote, Option<Hash>);
pub type ReplayVoteSender = Sender<ReplayedVote>;
pub type ReplayVoteReceiver = Receiver<ReplayedVote>;
