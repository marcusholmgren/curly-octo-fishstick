// frontend/src/routes/signin/+page.server.ts
// This file defines the server actions for the sign-in page.
// It maps the default action to the Auth.js signIn function.
// RELEVANT FILES: frontend/src/routes/signin/+page.svelte, frontend/src/auth.ts, frontend/src/hooks.server.ts

import { signIn } from '../../auth';
import type { Actions } from './$types';
export const actions: Actions = { default: signIn };
