export type Metadata = {
    name: string
    description: string
    image: string
    attributes: {
        trait_type: string
        value: string
    }[]
}

export const getMetadata = async (tokenId: string, baseURI: string) => {
    const metadataURI = `${baseURI}data/${tokenId}`
    const metadata = await fetch(metadataURI)
    return await metadata.json() as Metadata
}