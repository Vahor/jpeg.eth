import {createConfig} from "wagmi";
import {getDefaultConfig} from "connectkit";

const config = createConfig(
    getDefaultConfig({
        alchemyId: process.env.NEXT_PUBLIC_ALCHEMY_API_KEY,
        walletConnectProjectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID!,

        appName: "JPEG Explorer",
    }),
);

export {config as wagmiConfig};