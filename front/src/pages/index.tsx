import {ConnectKitButton} from "connectkit";
import {wagmiConfig} from "@/lib/wagmi";
import {contractAbi, contractAddress} from "@/lib/contract";
import {useEffect, useState} from "react";
import {useAccount} from "wagmi";

const getAllOwnedTokens = async (address: Web3Address) => {
    const balanceOf = await wagmiConfig.publicClient.readContract({
        address: contractAddress,
        abi: contractAbi,
        functionName: 'balanceOf',
        args: [address]
    })

    return await Promise.all(Array.from({ length: Number(balanceOf) }, async (_, i) => {
        return await wagmiConfig.publicClient.readContract({
            address: contractAddress,
            abi: contractAbi,
            functionName: 'tokenOfOwnerByIndex',
            args: [address, BigInt(i)]
        })
    }))
}

const getBaseURI = async () => {
    return await wagmiConfig.publicClient.readContract({
        address: contractAddress,
        abi: contractAbi,
        functionName: 'baseURI',
    })
}

export default function Home() {

    const { address, connector, isConnected } = useAccount()

    const [ownedTokens, setOwnedTokens] = useState<bigint[]>([])
    const [baseURI, setBaseURI] = useState<string>('')

    useEffect(() => {
        if (address) {
            getAllOwnedTokens(address).then(setOwnedTokens)
            getBaseURI().then(setBaseURI)
        }
        else {
            setOwnedTokens([])
            setBaseURI('')
        }
    }, [address])

    return (
        <div>
            <ConnectKitButton/>

            {ownedTokens.map((token, index) => (
                <div key={index}>
                    <img src={`${baseURI}image/${token}`} alt={`WAGMI #${token}`} width={200} height={200}/>
                </div>
            ))}
        </div>
    )
}
