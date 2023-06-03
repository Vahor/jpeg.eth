import React, {type FC, type PropsWithChildren} from 'react';
import Navbar from "@/components/Navbar";

const Layout: FC<PropsWithChildren<{}>> = ({ children }) => {
    return (
        <div className="w-full h-screen">
            <Navbar/>
            <div className="flex flex-col justify-between min-h-screen container mx-auto px-4 pt-8">
                {children}
            </div>
        </div>
    );
};

export default Layout;