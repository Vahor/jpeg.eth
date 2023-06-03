import React from 'react';
import {ConnectKitButton} from "connectkit";
import Link from "next/link";

const Navbar = () => {
    return (
        <div className="bg-white border-b">
            <div className="h-20 container mx-auto px-4 flex flex-row justify-between items-center">
                <div className="flex flex-row items-center space-x-4">
                    <div>
                        <h1 className="font-bold text-2xl text-black/80">JPEG Explorer</h1>
                    </div>
                    <div>
                        <ul className="flex flex-row space-x-4">
                            <li className="text-black/80 hover:text-black/90 cursor-pointer">
                                <Link href={"/"}>Home</Link>
                            </li>
                            <li className="text-black/80 hover:text-black/90 cursor-pointer">
                                <Link href={"/inventory"}>Inventory</Link>
                            </li>
                        </ul>
                    </div>
                </div>
                <div>
                    <ConnectKitButton/>
                </div>
            </div>
        </div>
    );
};

export default Navbar;