import { AppContext } from "next/app"
import PostLayout from "../../_layouts/assignment"
import { getAllPosts, get_post } from "../../api"
import { GetStaticProps, GetStaticPropsContext, NextPageContext } from "next"
import { useRouter } from "next/router"
import path from "path"

interface PostProps{
        title: string,
        description: string,
        content: string
}
export default function Post(props: PostProps){
    return <PostLayout title={props.title} description={props.description} content={props.content}/>
}

export async function getStaticProps({ params }: GetStaticPropsContext) {
    const slug = params.slug;
    return {
        props: await get_post(slug as string)
    }
}

export async function getStaticPaths() {
    const posts = await getAllPosts();
    console.log(posts);
    const paths = posts.map(p => {
        return {
            params: {slug:p.slug}
        }
    });
    console.log(paths);
    return {
        paths,
        fallback: false
    }
}