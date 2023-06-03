import '../styles/globals.css'
import {Inter} from 'next/font/google'
import {WagmiConfig} from 'wagmi';
import {ConnectKitProvider} from "connectkit";
import type {AppProps} from "next/app";

import {wagmiConfig} from "@/lib/wagmi";
import Layout from "@/components/Layout";
import {Toaster} from 'react-hot-toast'
import {ContractProvider} from "@/context/ContractContext";

const inter = Inter({subsets: ['latin'], variable: '--font-inter', adjustFontFallback: false})


const MyApp = ({
                   Component,
                   pageProps: {session, ...pageProps},
               }: AppProps) => {
    return (
        <WagmiConfig config={wagmiConfig}>
            <ConnectKitProvider debugMode theme="auto" mode='dark'>
                <ContractProvider>
                    <div className={`${inter.variable} font-sans`}>
                        <Layout>
                            <Component {...pageProps} />
                            <Toaster/>
                        </Layout>
                    </div>
                </ContractProvider>
            </ConnectKitProvider>
        </WagmiConfig>
    )
}

export default MyApp
