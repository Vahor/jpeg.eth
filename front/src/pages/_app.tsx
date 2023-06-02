import '../styles/globals.css'
import {Inter} from 'next/font/google'
import {WagmiConfig} from 'wagmi';
import {ConnectKitProvider} from "connectkit";
import type {AppProps} from "next/app";

import {wagmiConfig} from "@/lib/wagmi";

const inter = Inter({subsets: ['latin'], variable: '--font-inter', adjustFontFallback: false})


const MyApp = ({
                   Component,
                   pageProps: {session, ...pageProps},
               }: AppProps) => {
    return (
        <WagmiConfig config={wagmiConfig}>
            <ConnectKitProvider debugMode>
                <div className={`${inter.variable} font-sans`}>
                    <Component {...pageProps} />
                </div>
            </ConnectKitProvider>
        </WagmiConfig>
    )
}

export default MyApp
