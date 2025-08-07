export default function SSRTest() {
    const time = new Date().toISOString();
    return (
        <div>
            <p>Solid rendered at: {time}</p>
            <a href="/">Home</a>
            <a href="/counter">Counter Test</a>
        </div>
    )
}
