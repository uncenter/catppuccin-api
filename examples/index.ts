const API_URL = "https://catppuccin-api.fly.dev";

const ports: Record<
	string,
	Array<{
		categories: string;
	}>
> = await fetch(`${API_URL}/ports`).then((response) => response.json());

const categories: Record<string, any> = await fetch(
	`${API_URL}/categories`
).then((response) => response.json());

const result = Object.keys(categories)
	.map((category) => {
		return {
			category,
			count: Object.values(ports).filter((port) =>
				port.some(({ categories }) => categories.includes(category))
			).length,
		};
	})
	.sort((a, b) => b.count - a.count);
console.table(result);

export {};
