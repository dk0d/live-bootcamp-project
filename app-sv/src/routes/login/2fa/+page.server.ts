import { redirect } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import * as JWT from "jose";

import { getAuthUrl } from "$lib/utils";
import ky from "ky";
import { Try } from "$lib/result";

export const load: PageServerLoad = async ({ request, url }) => {
  let payload = url.searchParams.get("payload");
  if (!payload) throw redirect(302, "/login");

  // Use the built-in atob function to decode the base64 string to a binary string
  // Convert the binary string to a UTF-8 string
  let token = decodeURIComponent(atob(payload));
  let jwtSet = JWT.createRemoteJWKSet(
    new URL(`${getAuthUrl()}/.well-known/jwks.json`),
  );

  let { data: jwt, error } = await Try(JWT.jwtVerify(token, jwtSet));

  if (error)
    return { error: { message: "Invalid token", details: `${error}` } };

  if (jwt) {
    const { payload: claims, protectedHeader } = jwt;
    return {
      email: claims.email as string,
      id: claims.sub as string,
      exp: new Date((claims.exp as number) * 1000),
    };
  }

  throw redirect(302, "/login");
};

export const actions: Actions = {
  default: async ({ request, cookies, setHeaders }) => {
    const data = await request.formData();
    const email = data.get("email");
    const password = data.get("password");
    const authUrl = getAuthUrl();

    // Here you would typically send the data to your backend
    // For demonstration, we'll just log it to the console
    const response = await ky.post(`${authUrl}/login`, {
      json: {
        method: "email_password",
        email,
        password,
      },
    });

    if (response.ok) {
      const token: { token: string } = await response.json();
      response.headers.getSetCookie()?.map((c) => {
        console.log(c);
      });
      if (token) {
        cookies.set("jwt_auth_token", token.token, {
          httpOnly: true,
          sameSite: "lax",
          path: "/",
        });
      }

      return redirect(302, "/app");
    }
  },
};
