import { Program, web3 } from '@project-serum/anchor';
import * as anchor from '@project-serum/anchor';

import { IDL as Jackpot } from "../target/types/jackpot";
import {
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    Transaction,
    ParsedAccountData,
    LAMPORTS_PER_SOL
} from '@solana/web3.js';

import { BET3_WALLET, CODY_WALLET, EXPER_WALLET, GamePool, GAME_SEED, JACKPOT_PROGRAM_ID, JERZY_WALLET, TEAM_WALLET, VAULT_SEED } from './types';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';

let program: Program = null;

// Address of the deployed program.
let programId = new anchor.web3.PublicKey(JACKPOT_PROGRAM_ID);

anchor.setProvider(anchor.AnchorProvider.local("https://divine-attentive-market.solana-mainnet.discover.quiknode.pro/538b8d9673d0c31838019e9f828912ec8365ff07/"));
let provider = anchor.getProvider();

const solConnection = anchor.getProvider().connection;
const payer = anchor.AnchorProvider.local().wallet;
// Generate the program client from IDL.
program = new anchor.Program(Jackpot as anchor.Idl, programId);
console.log('ProgramId: ', program.programId.toBase58());
console.log('Payer: ', payer.publicKey.toBase58());



const main = async () => {

   
    // await initialize();
    await playGame(0.05);
    // 9hZk7AzjKahjkq6EBVoxc6P4njHwLms4ynYqPsmhib18
    
    // await enterGame(new PublicKey('FP1Hi18MmXHRHk8aFDvn9U6TCvqbi5EyBEWqF9LT7kkG'), 0.1);
    
    // await claimReward(new PublicKey('FP1Hi18MmXHRHk8aFDvn9U6TCvqbi5EyBEWqF9LT7kkG'));
    // console.log(await getStateByKey(new PublicKey('FP1Hi18MmXHRHk8aFDvn9U6TCvqbi5EyBEWqF9LT7kkG')));
};

export const initialize = async () => {
    const tx = await createInitializeTx(payer.publicKey);
    const txId = await provider.sendAndConfirm(tx, [], {commitment: "confirmed"});
    console.log("Signature:", txId)
}

export const playGame = async (amount: number) => {
    const tx = await createPlayGameTx(payer.publicKey, amount);
    // const txId = await provider.sendAndConfirm(tx, [], {commitment: "confirmed"});
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = await (await solConnection.getLatestBlockhash()).blockhash;
    payer.signTransaction(tx);
    const txId = await solConnection.sendTransaction(tx, [(payer as NodeWallet).payer]);
    console.log("Signature:", txId)
}
export const enterGame = async (pda: PublicKey, amount: number) => {
    const tx = await createEnterGameTx(payer.publicKey, pda, amount);
    const txId = await provider.sendAndConfirm(tx, [], {commitment: "confirmed"});
    console.log("Signature:", txId)
}
export const claimReward = async (pda: PublicKey) => {
    const tx = await createClaimRewardTx(payer.publicKey, pda);
    const txId = await provider.sendAndConfirm(tx, [], {commitment: "confirmed"});
    console.log("Signature:", txId)
}

/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
////////// create TX
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////

export const createInitializeTx = async (userAddress: PublicKey) => {
    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        programId
    );

    console.log(solVault.toBase58())
    let tx = new Transaction();

    tx.add(program.instruction.initialize(
        {
        accounts: {
            admin: userAddress,
            solVault,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY
        },
        instructions: [],
        signers: [],
    }));

    return tx;
}

export const createPlayGameTx = async (
    userAddress: PublicKey,
    amount: number
) => {

    let now = new Date();
    let ts = Math.floor(now.getTime()/1000);

    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        programId
    );

    const [gamePool, gameBump] = await PublicKey.findProgramAddress(
        [Buffer.from(GAME_SEED), userAddress.toBuffer(), new anchor.BN(ts).toArrayLike(Buffer, "le", 8)],
        programId
    );

    console.log("Game PDA: ", gamePool.toBase58());
    const tx = new Transaction();

    tx.add(program.instruction.playGame(
        new anchor.BN(ts),
        new anchor.BN(amount * LAMPORTS_PER_SOL),
        {
            accounts: {
                admin: userAddress,
                gamePool,
                solVault,
                codyWallet: CODY_WALLET,
                bet3Wallet: BET3_WALLET,
                jerzyWallet: JERZY_WALLET,
                experWallet: EXPER_WALLET,
                teamWallet: TEAM_WALLET,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY
            },
            instructions: [],
            signers: [],
        }));

    return tx;

}

export const createEnterGameTx = async (
    userAddress: PublicKey,
    gamePool: PublicKey,
    amount: number
) => {

    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        programId
    );
    console.log(solVault.toBase58())
    
    const tx = new Transaction();

    tx.add(program.instruction.enterGame(
        new anchor.BN(amount * LAMPORTS_PER_SOL), {
            accounts: {
                admin: userAddress,
                gamePool,
                solVault,
                codyWallet: CODY_WALLET,
                bet3Wallet: BET3_WALLET,
                jerzyWallet: JERZY_WALLET,
                experWallet: EXPER_WALLET,
                teamWallet: TEAM_WALLET,
                systemProgram: SystemProgram.programId,
            },
            instructions: [],
            signers: [],
        }));

    return tx;

}


export const createClaimRewardTx = async (
    userAddress: PublicKey,
    gamePool: PublicKey,
) => {

    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        programId
    );
    const state = await getStateByKey(gamePool);
    let total = state.rand.toNumber() % state.totalDeposit.toNumber();
    console.log("state", state);
    console.log("state.depositAmounts", state.depositAmounts);

    let valid = 0;
    let index = 0;
    for(let i = 0 ; i < state.depositAmounts.length; i++) {
        if (total > state.depositAmounts[i].toNumber()) {
            total -= state.depositAmounts[i].toNumber();
        } else {
            index = i;
            valid = 1;
            break;
        }
    }
    if (valid != 1) return;

    const tx = new Transaction();

    tx.add(program.instruction.claimReward(
        bump, {
            accounts: {
                admin: userAddress,
                gamePool,
                winner: state.entrants[index],
                solVault,
                systemProgram: SystemProgram.programId,
            },
            instructions: [],
            signers: [],
        }));

    return tx;

}


export const getStateByKey = async (
    gameKey: PublicKey
): Promise<GamePool | null> => {
    try {
        const gameState = await program.account.gamePool.fetch(gameKey);
        return gameState as unknown as GamePool;
    } catch {
        return null;
    }
}

main()