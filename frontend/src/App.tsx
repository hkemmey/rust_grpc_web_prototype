import { useState } from 'react'
import './App.css'

function App() {
	const [message, setMessage] = useState("");
	const [response, setResponse] = useState("");

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();

		const res = await fetch("http://localhost:8080/submit", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ message }),
		});

		const data = await res.json();
		setResponse(data.message);
	};

  return (
		<div style={{ padding: "2rem" }}>
			<h1> gRPC Form Submission</h1>
			<form onSubmit={handleSubmit}>
				<input
					type="text"
					placeholder="message"
					value={message}
					onChange={(e) => setMessage(e.target.value)}
				/>
				<button type="submit">Submit</button>
			</form>
			{response && <p>Response: {response}</p>}
		</div>
  );
}

export default App
