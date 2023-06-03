import React, {FC, useEffect} from 'react';
import {useContractContext} from "@/context/ContractContext";
import {getMetadata, Metadata} from "@/lib/metadata";
import Image from "next/image";
import {cn} from "@/lib/utils";
import {Card, CardDescription, CardFooter, CardHeader, CardTitle,} from "@/components/ui/card"
import {Button} from "@/components/ui/button";
import {Store} from "lucide-react";

const Nft: FC<{
    id: string
}> = ({id}) => {

    const {baseUri} = useContractContext();
    const [metadata, setMetadata] = React.useState<Metadata | undefined | null>(undefined);

    useEffect(() => {
        getMetadata(id, baseUri)
            .then(setMetadata)
            .catch(() => {
                setMetadata(null);
            })
    }, [id, baseUri])


    return (
        <Card className="flex flex-col justify-center overflow-hidden bg-white">
            <div className="h-[200px] w-full relative flex justify-center items-center">
            <Image
                src={metadata?.image ?? "/nft-placeholder.svg"}
                alt={metadata?.name ?? "Loading..."}
                width={metadata?.image ? 200 : 80}
                height={metadata?.image ? 200 : 80}
                className={cn(
                    "transition-all hover:scale-105 duration-300",
                )}
            />
            </div>
            <CardHeader>
                <CardTitle>{metadata?.name ?? "Loading..."}</CardTitle>
                <CardDescription className="text-slate-400 font-light"># {id}</CardDescription>
            </CardHeader>
            <CardFooter>
                <Button className="w-full">
                    <Store className="mr-2 h-4 w-4"/> Sell in store
                </Button>
            </CardFooter>
        </Card>
    );
};

export default Nft;