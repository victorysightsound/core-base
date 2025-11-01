import { createSignal, onMount } from "solid-js";

function App() {
  const [data, setData] = createSignal("Loading...");

  // Load data from the backend API using the URL in your .env file
  onMount(async () => {
    try {
      const apiUrl = import.meta.env.VITE_API_URL;
      const res = await fetch(`${apiUrl}/health`);
      const json = await res.json();
      setData(JSON.stringify(json, null, 2));
    } catch (err) {
      setData("Error: " + err.message);
    }
  });

  return (
    <main class="min-h-screen bg-base-200 flex flex-col items-center justify-center p-10">
      <h1 class="text-5xl font-bold mb-8 text-primary">CORE Dashboard</h1>

      <div class="card bg-base-100 shadow-xl p-6">
        <pre class="text-lg whitespace-pre-wrap">{data()}</pre>
      </div>

      <div class="mt-8">
        <a
          href={`${import.meta.env.VITE_API_URL}/health`}
          target="_blank"
          class="btn btn-primary"
        >
          Open API Health Endpoint
        </a>
      </div>
    </main>
  );
}

export default App;

