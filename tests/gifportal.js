// NPM Packages
import anchor from '@coral-xyz/anchor';

// Constants
const GIF_URL = 'https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExYm5nNHhhMDFxc3BjcG82d2Uza2dlZDNvdDM5cTNxemZ2OWI4cWtqeiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/duNowzaVje6Di3hnOu/giphy.gif';

async function main() {
  console.log("Starting tests ...");

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gifportal;

  const gifsAccount = anchor.web3.Keypair.generate();

  console.log('gifsAccount: ', gifsAccount.publicKey);
  console.log('user: ', provider.wallet.publicKey);
  console.log('systemProgram: ', anchor.web3.SystemProgram.programId);

  const tx = await program.rpc.initialize({
    accounts: {
      gifsAccount: gifsAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [
      gifsAccount,
    ],
  });

  console.log('tx: ', tx);

 let account = await program.account.gifsAccount
   .fetch(gifsAccount.publicKey);

  console.log("GIF Count: ", account.gifCount.toString());

  await program.rpc.createGif(GIF_URL, {
      accounts: {
        gifsAccount: gifsAccount.publicKey,
        user: provider.wallet.publicKey,
      },
  });

  account = await program.account.gifsAccount
    .fetch(gifsAccount.publicKey);

  console.log("GIF Count: ", account.gifCount.toString());
  console.log("GIFs: ", account.gifs);
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
