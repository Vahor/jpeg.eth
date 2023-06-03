import {contractAbi, contractAddress} from "@/lib/contract";
import {wagmiConfig} from "@/lib/wagmi/index";

export const getBaseURI = async () => {
    return await wagmiConfig.publicClient.readContract({
        address: contractAddress,
        abi: contractAbi,
        functionName: 'baseURI',
    })
}

export const getAllOwnedTokens = async (address: Web3Address) => {
    const balanceOf = await wagmiConfig.publicClient.readContract({
        address: contractAddress,
        abi: contractAbi,
        functionName: 'balanceOf',
        args: [address]
    })

    return await Promise.all(Array.from({length: Number(balanceOf)}, async (_, i) => {
        return await wagmiConfig.publicClient.readContract({
            address: contractAddress,
            abi: contractAbi,
            functionName: 'tokenOfOwnerByIndex',
            args: [address, BigInt(i)]
        })
    }))
}