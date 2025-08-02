// @ts-check
import { defineConfig } from 'astro/config';
import * as dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const envPath = path.resolve(__dirname, '../../../.env');
dotenv.config({ path: envPath });

// https://astro.build/config
export default defineConfig({
    server: {
        port: process.env.DOCS_SERVER_URL
    }
});
