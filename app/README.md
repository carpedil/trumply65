# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Creating a project

```bash
# create a new project in app
npm create svelte@latest app

# Add TailwindCSS
npx svelte-add tailwindcss

# Install dependencies
npm install

# Run the CLI
npx shadcn-svelte@latest init
# Configure components.json
Would you like to use TypeScript (recommended)? › Yes
Which style would you like to use? › Default
Which color would you like to use as base color? › Slate
Where is your global CSS file? › src/app.pcss
Where is your tailwind.config.[cjs|js|ts] located? › tailwind.config.js
Configure the import alias for components: › $lib/components
Configure the import alias for utils: › $lib/utils

# You can now start adding components to your project.
npx shadcn-svelte@latest add button
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

# node version
```bash
> nvm list
* 20.14.0 (Currently using 64-bit executable)
  18.16.0
```