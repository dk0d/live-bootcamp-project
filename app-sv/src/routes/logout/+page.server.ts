import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { getAuthUrl } from "$lib/utils";
import ky from 'ky'


export const load: PageServerLoad = async ({
  cookies
}) => {
  const res = await ky.post(`${getAuthUrl()}/logout`, {
    headers: {
      cookie: `jwt_auth_token=${cookies.get("jwt_auth_token")}`,
    }
  });
  if (!res.ok) {
    console.log(res)
    console.error("Failed to logout from auth server");
    redirect(303, "/app");
  }
  cookies.delete("jwt_auth_token", { path: "/" });
  redirect(303, "/login");
}
