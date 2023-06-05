import {erc721ABI} from "wagmi";

export const nftContractABI = [
    ...erc721ABI,
    {
        "name": "tokenOfOwnerByIndex",
        "stateMutability": "view",
        "type": "function",
        "inputs": [
            {
                "internalType": "address",
                "name": "owner",
                "type": "address"
            },
            {
                "internalType": "uint256",
                "name": "index",
                "type": "uint256"

            }
        ],
        "outputs": [
            {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"

            }
        ],
    },
    {
        "name": "baseURI",
        "stateMutability": "view",
        "type": "function",
        "inputs": [],
        "outputs": [
            {
                "internalType": "string",
                "name": "",
                "type": "string"
            }
        ]
    }
] as const;

export const faucetContractABI = [
    {
        "name": "mint",
        "stateMutability": "nonpayable",
        "type": "function",
        "inputs": [
            {
                "internalType": "address",
                "name": "who",
                "type": "address"
            },
            {
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
            }
        ],
        "outputs": []
    },
    {
        "name": "lastMinted",
        "stateMutability": "view",
        "type": "function",
        "inputs": [
            {
                "internalType": "address",
                "name": "",
                "type": "address"
            }
        ],
        "outputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ]
    },
    {
        "name": "MAX_MINT",
        "stateMutability": "view",
        "type": "function",
        "inputs": [],
        "outputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ]
    }
] as const;


export const nftContractAddress = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS! as `0x${string}`
export const faucetContractAddress = process.env.NEXT_PUBLIC_FAUCET_CONTRACT_ADDRESS! as `0x${string}`
