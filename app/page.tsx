
import Image from 'next/image'
import Link from 'next/link';

interface response{
  posts: {name: string, title: string}[]
}


const postList = `
{
  "posts": [
      {
          "name": "assignment00",
          "title": "Assignment 00: Plan and Sketch"
      }
      
  ]
}
`;


export default async function Home() {
  
  const posts = JSON.parse(postList) as response;

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <h1>List of posts:</h1>
            <ul>
                {posts.posts.map((post, idx) => {
                    return (
                        <li key={idx}>
                            <Link href={`/posts/${post.name}`}>
                                {post.title}
                            </Link>
                        </li>
                    );
                })}
            </ul>
    </main>
  )
}

