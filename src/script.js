const quotes = [
    "God told me to make TempleOS.",
    "I have a direct line to God.",
    "TempleOS is the Third Temple.",
    "I built an operating system in 1,000 days."
];

function newQuote() {
    const index = Math.floor(Math.random() * quotes.length);
    document.getElementById('quote').innerText = quotes[index];
}
