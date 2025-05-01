document.addEventListener("DOMContentLoaded", function() {
    const quoteElement = document.getElementById("quote");
    const recipeElement = document.getElementById("recipe");

    async function fetchContent() {
        try {
            const response = await fetch("/api/content");
            const data = await response.json();
            if (data.quote) {
                quoteElement.textContent = data.quote;
            }
            if (data.recipe) {
                recipeElement.textContent = data.recipe;
            }
        } catch (error) {
            console.error("Error fetching content:", error);
        }
    }

    fetchContent();
});