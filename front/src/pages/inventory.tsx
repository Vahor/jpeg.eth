import {useAccount} from "wagmi";
import {useEffect, useState} from "react";
import {useRouter} from "next/router";
import toast from "react-hot-toast";
import {getAllOwnedTokens} from "@/lib/wagmi/utils";
import NFT from "@/components/inventory/NFT";

export default function Inventory() {

    const {push} = useRouter();

    const {address} = useAccount()

    const [ownedTokens, setOwnedTokens] = useState<bigint[] | undefined>(undefined)

    useEffect(() => {
        if (address) {
            getAllOwnedTokens(address).then(setOwnedTokens).catch(() => {
                toast.error("Failed to load inventory")
            })
        } else {
            push('/').then(() => {
                toast.error("You must be connected to view your inventory");
            })
        }
    }, [address])

    if (ownedTokens === undefined) {
        return <div>Loading...</div>
    }

    return (
        <div>

            <div>
                <h1 className="text-2xl font-bold">Inventory</h1>
                <p className="text-slate-400">You own {ownedTokens.length} NFTs</p>
            </div>

            <div className="pt-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {ownedTokens.map((token) => (
                    <NFT id={token.toString()} key={token.toString()}/>
                ))}
            </div>


        </div>
    )
}
