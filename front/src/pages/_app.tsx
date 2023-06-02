import '../styles/globals.css'
import {Inter} from 'next/font/google'
import {FC, PropsWithChildren} from "react";
import {WagmiConfig} from 'wagmi';
import {ConnectKitProvider} from "connectkit";

import {wagmiConfig} from "@/lib/wagmi";

const inter = Inter({subsets: ['latin'], variable: '--font-inter', adjustFontFallback: false})


const MyApp: FC<PropsWithChildren<{}>> = ({children}) => {
    console.log(process.env.NEXT_PUBLIC_ALCHEMY_API_KEY)

    return (
        <WagmiConfig config={wagmiConfig}>
            <ConnectKitProvider debugMode>
                <div className={`${inter.variable} font-sans`}>
                    {children}
                </div>
            </ConnectKitProvider>
        </WagmiConfig>
    )
}

export default MyApp
