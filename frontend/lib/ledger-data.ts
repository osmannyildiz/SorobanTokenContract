import { TESTNET_RPC_URL } from "@/config";
import { rpc, scValToNative, xdr } from "@stellar/stellar-sdk";
import * as Client from "../packages/token/dist/index";

const server = new rpc.Server(TESTNET_RPC_URL);
const contractId = Client.networks.testnet.contractId;

export const getTokenMetadata = async (): Promise<{
  decimal: number;
  name: string;
  symbol: string;
}> => {
  const ledgerEntry = await server.getContractData(
    contractId,
    xdr.ScVal.scvLedgerKeyContractInstance()
  );

  const metadata = scValToNative(
    ledgerEntry.val
      .contractData()
      .val()
      .instance()
      .storage()!
      .filter((entry) => scValToNative(entry.key()) === "METADATA")
      .pop()!
      .val()!
  );

  return metadata;
};
