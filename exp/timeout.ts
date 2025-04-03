import { Server } from "bun";

const server = Bun.serve({
  port: 3000,
  async fetch(req: Request) {
    const url = new URL(req.url);

    // Check if it's a GET request to the root path
    if (req.method === "GET" && url.pathname === "/") {
      // Create a promise that resolves after 1 second with the success message
      const responsePromise = new Promise<Response>((resolve) => {
        setTimeout(() => {
          resolve(
            new Response(JSON.stringify({ message: "success" }), {
              status: 200,
              headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "*",
                "Access-Control-Allow-Methods":
                  "GET, POST, PUT, DELETE, OPTIONS",
                "Access-Control-Allow-Headers": "Content-Type, Authorization",
                "Access-Control-Max-Age": "86400", // 24 hours
              },
            }),
          );
        }, 1000); // 1 second timeout
      });

      return responsePromise;
    }

    // Handle other routes with a 404
    return new Response("Not Found", { status: 404 });
  },
});

console.log(`Server running at http://localhost:${server.port}`);
