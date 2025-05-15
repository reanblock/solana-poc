export const printTxLogs = async (provider, tx) => {
    // wait for the tenasaction to be confirmed
    await provider.connection.confirmTransaction(tx, 'confirmed');

    // Fetch the transaction details which include the logs
    const txDetails = await provider.connection.getTransaction(tx, {
        commitment: 'confirmed',
    });

    // Log the program's output messages
    console.log("Program Logs:", txDetails.meta.logMessages.filter(log => log.includes("Program log:")));
}