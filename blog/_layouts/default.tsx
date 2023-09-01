import Head from "next/head";
import Header from "../_includes/header";
import React from "react";
import Footer from "../_includes/footer";


interface DefaultLayoutProps{
    title: string,
    description: string,
    children: React.ReactNode

}
export function DefaultLayout(props: DefaultLayoutProps){
    return (
        <main>
            <Head>
                <title>
                    {props.title}
                </title>
                <meta name="description" content={props.description}/>
            </Head>
            <Header/>
            {props.children}
            <Footer/>           
        </main>
    );
}