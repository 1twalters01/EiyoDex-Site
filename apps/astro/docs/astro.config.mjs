// @ts-check
import { defineConfig } from 'astro/config';
import solidJs from "@astrojs/solid-js"

import dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import path from 'path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const stylesPath = path.resolve(__dirname, '../../../styles')
const envPath = path.resolve(__dirname, '../../../.env');
let res = dotenv.config({ path: envPath });
if (res.error) {
    console.warn('dotenv failed to load .env:', res.error);
}

const HOST = process.env.DOCS_SERVER_HOST ?? 'localhost';
const PORT = parseInt(process.env.DOCS_SERVER_PORT ?? '2003');
console.log(PORT)
// https://astro.build/config
export default defineConfig({
    integrations: [solidJs()],
    vite: {
        resolve: {
            alias: {
                "@styles": stylesPath,
            },
        },
    },
    server: {
        host: HOST,
        port: PORT,
    },
});
