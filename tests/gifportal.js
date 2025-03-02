// NPM Packages
import anchor from '@coral-xyz/anchor';

// Constants
const GIF_LINK = 'https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExYm5nNHhhMDFxc3BjcG82d2Uza2dlZDNvdDM5cTNxemZ2OWI4cWtqeiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/duNowzaVje6Di3hnOu/giphy.gif';

async function main() {
  console.log("Starting tests ...");

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gifportal;

  const baseAccount = anchor.web3.Keypair.generate();

  console.log('baseAccount: ', baseAccount.publicKey);
  console.log('user: ', provider.wallet.publicKey);
  console.log('systemProgram: ', anchor.web3.SystemProgram.programId);

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [
      baseAccount,
    ],
  });

  console.log('tx: ', tx);

 let account = await program.account.baseAccount
   .fetch(baseAccount.publicKey);

  console.log("GIF Count: ", account.totalGifs.toString());

  await program.rpc.addGif(GIF_LINK, {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
  });

  account = await program.account.baseAccount
    .fetch(baseAccount.publicKey);

  console.log("GIF Count: ", account.totalGifs.toString());
  console.log("GIF List: ", account.gifList);
}

async function runMain() {
  try {
    await main();

    process.exit(0);
  }
  catch(error) {
    console.error(error);

    process.exit(1);
  }
}

runMain();
