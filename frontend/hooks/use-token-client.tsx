import { TESTNET_RPC_URL } from "@/config";
import { useMemo } from "react";
import * as Client from "../packages/token/dist/index";

export const useTokenClient = () => {
  const tokenClient = useMemo(
    () =>
      new Client.Client({
        ...Client.networks.testnet,
        rpcUrl: TESTNET_RPC_URL,
      }),
    []
  );

  return tokenClient;
};
