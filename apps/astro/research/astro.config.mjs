// @ts-check
import { defineConfig } from 'astro/config';
import dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import path from 'path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const envPath = path.resolve(__dirname, '../../../.env');
let res = dotenv.config({ path: envPath });
if (res.error) {
    console.warn('dotenv failed to load .env:', res.error);
}

const HOST = process.env.RESEARCH_SERVER_HOST ?? 'localhost';
const PORT = parseInt(process.env.RESEARCH_SERVER_PORT ?? '2001');

// https://astro.build/config
export default defineConfig({
    server: {
        host: HOST,
        port: PORT,
    }
});


