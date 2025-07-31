import { createSignal } from 'solid-js';

export default function Counter() {
    const [count, setCount] = createSignal(0);

    return (
        <div>
            {count()}
            <button onClick={() => setCount(count() + 1)}>
                Increment
            </button>
            <button onClick={() => setCount(count() - 1)}>
                Decrement
            </button>
            <a href="/">Home</a>
            <a href="/ssr">SSR</a>
        </div>
    );
}
