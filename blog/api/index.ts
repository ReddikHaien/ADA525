import matter from 'gray-matter';
import { marked } from 'marked';

export async function getAllPosts(): Promise<{slug: string, title: string}[]> {
    const ctx = (require as NextJsRequire).context("../_posts", false, /\.md/);
    const posts = [];
    for (const key of ctx.keys()){
        const post = key.slice(2);
        const content = await import(`../_posts/${post}`);
        const meta = matter(content.default);
        posts.push({
            slug: post.replace(".md",""),
            title: meta.data.title
        });
    }
    console.log(posts);
    return posts;
}

export async function get_post(slug: string){
    const filleContent = await import(`../_posts/${slug}.md`);
    const meta = matter(filleContent.default);
    const content = marked(meta.content);
    return {
        title: meta.data.title,
        description: "blogpost",
        content
    };
}