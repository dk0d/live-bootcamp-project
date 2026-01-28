import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ cookies }) => {
  if (!cookies.get('jwt_auth_token')) {
    return redirect(302, '/login');
  }
}
