import React, {useContext, useEffect} from "react";
import {getBaseURI} from "@/lib/wagmi/utils";
import toast from "react-hot-toast";

type TokenContextType = {
    baseUri: string
}

const ContractContext = React.createContext<TokenContextType | undefined>(undefined)

export const useContractContext = () => {
    const context = useContext(ContractContext)
    if (context === undefined) {
        throw new Error('useTokenContext must be used within a TokenProvider')
    }
    return context
}

export const ContractProvider = ({children}: { children: React.ReactNode }) => {

    const [baseUri, setBaseUri] = React.useState<string>('');

    useEffect(() => {
        getBaseURI().then(setBaseUri).catch(() => {
            toast.error("Failed to get base URI")
        })
    }, [])

    return (
        <ContractContext.Provider value={{baseUri}}>
            {children}
        </ContractContext.Provider>
    )
}