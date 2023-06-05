import {
    faucetContractABI,
    faucetContractAddress, mewoContractABI,
    mewoContractAddress,
    nftContractABI,
    nftContractAddress
} from "@/lib/contract";
import {wagmiConfig} from "@/lib/wagmi/index";

export const getBaseURI = async () => {
    return await wagmiConfig.publicClient.readContract({
        address: nftContractAddress,
        abi: nftContractABI,
        functionName: 'baseURI',
    })
}

export const getAllOwnedTokens = async (address: Web3Address) => {
    const balanceOf = await wagmiConfig.publicClient.readContract({
        address: nftContractAddress,
        abi: nftContractABI,
        functionName: 'balanceOf',
        args: [address]
    })

    return await Promise.all(Array.from({length: Number(balanceOf)}, async (_, i) => {
        return await wagmiConfig.publicClient.readContract({
            address: nftContractAddress,
            abi: nftContractABI,
            functionName: 'tokenOfOwnerByIndex',
            args: [address, BigInt(i)]
        })
    }))
}

export const getLastMintedFaucet = async (address: Web3Address) => {
    return await wagmiConfig.publicClient.readContract({
        address: faucetContractAddress,
        abi: faucetContractABI,
        functionName: 'lastMinted',
        args: [address]
    });
}

export const getMaxMintFaucet = async () => {
    return await wagmiConfig.publicClient.readContract({
        address: faucetContractAddress,
        abi: faucetContractABI,
        functionName: 'MAX_MINT'
    });
}


export const getMewoBalance = async (address: Web3Address) => {
    return await wagmiConfig.publicClient.readContract({
        address: mewoContractAddress,
        abi: mewoContractABI,
        functionName: 'balanceOf',
        args: [address]
    })
}
