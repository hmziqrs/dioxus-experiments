# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

# Tailwind/Bun:

```bash
bun install
```

```bash
bunx --bun @tailwindcss/cli -i input.css -o assets/tailwind.css --watch
```

# Tailwind/Npm:

```bash
npm install
```

```bash
npx tailwindcss -i input.css -o assets/tailwind.css --watch
```
