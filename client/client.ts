import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Guarderia;

(async () => {
  const user = provider.wallet.publicKey;

  // Cuenta nueva (NO PDA, más fácil)
  const guarderia = anchor.web3.Keypair.generate();

  try {
    // CREATE
    await program.methods
      .crear("Guarderia1", "Av flores")
      .accounts({
        guarderia: guarderia.publicKey,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([guarderia])
      .rpc();

    console.log("Creada:", guarderia.publicKey.toString());

    // READ
    let data = await program.account.guarderia.fetch(guarderia.publicKey);
    console.log("Datos:", data);

    // UPDATE
    await program.methods
      .actualizar("Nueva", "Otra direccion")
      .accounts({
        guarderia: guarderia.publicKey,
      })
      .rpc();

    data = await program.account.guarderia.fetch(guarderia.publicKey);
    console.log("Actualizada:", data);

    // DELETE
    await program.methods
      .eliminar()
      .accounts({
        guarderia: guarderia.publicKey,
        user: user,
      })
      .rpc();

    console.log("Eliminada");
  } catch (error) {
    console.error("Error:", error);
  }
})();
