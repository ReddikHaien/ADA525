import Link from "next/link";
import { DefaultLayout } from "../_layouts/default";
import { getAllPosts } from "../api";

interface BlogProps{
    title: string,
    description: string,
    posts: {slug: string, title: string}[]
}

export default function Blog (props: BlogProps) {
    console.log(props);
    return(
        <DefaultLayout title="Home" description="Homepage">
            <p>List of posts:</p>
            <ul>
                {props.posts.map((post, idx) => {
                    return (
                        <li key={idx}>
                            <Link href={`/assignments/${post.slug}`}>
                                {post.title}
                            </Link>
                        </li>
                    );
                })}
            </ul>
        </DefaultLayout>
    );
}

export async function getStaticProps(): Promise<{props: BlogProps}>{
    const posts = await getAllPosts();

    return {
        props:{
            description: "Homepage",
            title: "Home",
            posts
        }
    }
}