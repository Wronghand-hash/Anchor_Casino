import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        include: ['./tests/*.test.ts'], // Include test files
        environment: 'node', // Use Node.js environment
        globals: true, // Enable global variables (e.g., describe, it, expect)
        coverage: {
            provider: 'v8', // Use V8 for coverage
            reporter: ['text', 'json', 'html'], // Generate coverage reports
        },
    },
});