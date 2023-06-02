import {erc721ABI} from "wagmi";

export const contractAbi = [
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
        "name":"baseURI",
        "stateMutability":"view",
        "type":"function",
        "inputs":[],
        "outputs":[
            {
                "internalType":"string",
                "name":"",
                "type":"string"
            }
        ]
    }
] as const;

export const contractAddress = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS! as `0x${string}`