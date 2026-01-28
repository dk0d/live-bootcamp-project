import type { LayoutServerLoad } from "./$types";


export const load: LayoutServerLoad = async ({ locals, cookies }) => {
  return {
    session: cookies.get("jwt_auth_token") || null,
  }
}
