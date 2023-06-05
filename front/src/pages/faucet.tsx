import {useAccount, useContractWrite, usePrepareContractWrite} from "wagmi";
import {useEffect, useState} from "react";
import toast from "react-hot-toast";
import {getLastMintedFaucet, getMaxMintFaucet} from "@/lib/wagmi/utils";
import {faucetContractABI, faucetContractAddress} from "@/lib/contract";
import {useRouter} from "next/router";
import dayjs from "dayjs";
import {Button} from "@/components/ui/button";

const relativeTime = require('dayjs/plugin/relativeTime');
dayjs.extend(relativeTime);

const defaultLastMinted = BigInt(0)
export default function Inventory() {

    const {push} = useRouter();
    const {address} = useAccount()

    const [lastMintedDate, setLastMintedDate] = useState<number | 0>(0)
    const [requestToken, setRequestToken] = useState<bigint | 0n>(defaultLastMinted)
    const canUseFaucet = address && (lastMintedDate === 0 || dayjs(lastMintedDate).add(30, 'minute').isBefore(dayjs()));

    const {config, isLoading, isError, error} = usePrepareContractWrite({
        address: faucetContractAddress,
        abi: faucetContractABI,
        functionName: 'mint',
        enabled: canUseFaucet,
        args: [address!, requestToken],
    })
    const {
        write,
        reset,
        isSuccess: writeSuccess,
        isLoading: writeLoading,
        isError: writeError
    } = useContractWrite(config)

    if (error) {
        console.error(error)
    }

    useEffect(() => {
        if (writeSuccess) {
            toast.success("Successfully minted " + requestToken + " coins", {
                id: "minting"
            })
            reset()
        }
        if (writeLoading) {
            toast.loading("Minting...", {
                id: "minting"
            })
        }
        if (writeError) {
            toast.error("Failed to mint coins", {
                id: "minting"
            })
        }
    }, [writeSuccess, writeLoading, writeError, requestToken, reset])

    const mint = async () => {
        if (!canUseFaucet) return
        if (isLoading) return
        if (!write) return

        console.log(write)
        await write()
        setLastMintedDate(Date.now())
    }

    useEffect(() => {
        if (address) {
            Promise.all([
                getLastMintedFaucet(address),
                getMaxMintFaucet()
            ]).then(([lastMinted, requestToken]) => {
                setLastMintedDate(Number.parseInt(lastMinted.toString() + '000'))
                setRequestToken(requestToken)
            }).catch((e) => {
                console.error(e);
                toast.error("Failed to load faucet history")
            });
        } else {
            push('/').then(() => {
                toast.error("You must be connected to use the faucet");
            })
        }
    }, [address])

    return (
        <div>

            <div>
                <h1 className="text-2xl font-bold">Faucet</h1>
                <p className="text-muted-foreground">
                    You can use the faucet once every 30 minutes.
                    <br/>
                    {/* @ts-ignore */}
                    {lastMintedDate !== 0 && "You last used the faucet " + dayjs(lastMintedDate).fromNow()}
                    {lastMintedDate === 0 && "You have never used the faucet."}
                </p>
            </div>

            <div className="pt-4">
                <Button
                    size='lg'
                    disabled={typeof window !== 'undefined' && (!canUseFaucet || isLoading || isError || writeLoading)}
                    onClick={mint}
                >
                    {(isLoading || writeLoading) ? "Loading..." : `GIVE ME ${requestToken} COINS`}
                </Button>
            </div>


        </div>
    )
}
