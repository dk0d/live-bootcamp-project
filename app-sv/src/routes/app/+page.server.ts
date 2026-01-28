import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ request, cookies }) => {
  if (!cookies.get('session')) {
    return redirect(302, '/login');
  }
}
