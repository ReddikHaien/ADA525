import Head from "next/head"
import { DefaultLayout } from "./default"
import { describe } from "node:test"
import React from "react"
import Link from "next/link"

interface PostLayoutProps{
    title: string,
    description: string,
    content: string
}
export default function PostLayout(props: PostLayoutProps){
    return (
        <DefaultLayout title={props.title} description={props.description}>
            <article>
                <div dangerouslySetInnerHTML={{__html:props.content}}></div>
                <div><Link href="/">Home</Link></div>
            </article>
        </DefaultLayout>
    )
}