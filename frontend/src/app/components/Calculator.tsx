"use client";

import { useState } from "react";

export default function Calculator() {
  const [a, setValueA] = useState(0);
  const [b, setValueB] = useState(0);
  const [result, setResult] = useState<number | string>("");

  const calculation = async (operation: string) => {
    const url = `http://127.0.0.1:8080/${operation}`;
    const response = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ value_a: a, value_b: b }),
    });
    if (!response.ok) {
      setResult("Error: " + (await response.text()));
    } else {
      const calculationResult = await response.json();
      setResult(calculationResult);
    }
  };

  return (
    <div>
      <input
        type="number"
        value={a}
        onChange={(e) => setValueA(Number(e.target.value))}
      />
      <input
        type="number"
        value={b}
        onChange={(e) => setValueB(Number(e.target.value))}
      />
      <div>
        <button onClick={() => calculation("add")}> Addition </button>
        <button onClick={() => calculation("substract")}> Substraction </button>
      </div>
      <div>Result: {result}</div>
    </div>
  );
}
