import { redirect } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";
import { getAuthUrl } from "$lib/utils";
import ky from "ky";

export const load: PageServerLoad = async () => { };

export const actions: Actions = {
  signup: async ({ request, cookies }) => {
    const data = await request.formData();
    const email = data.get("email");
    const password = data.get("password");
    const confirmPassword = data.get("confirm_password");
    const twoFactor = data.get("two_factor");

    if (password !== confirmPassword) {
      return { error: "Passwords do not match" };
    }
    const body = {
      method: "email_password",
      email,
      password,
      two_factor: twoFactor,
    };
    console.log("Signup data:", body);

    // Here you would typically send the data to your backend
    // For demonstration, we'll just log it to the console
    const response = await ky.post(`${getAuthUrl()}/signup`, {
      json: body,
    });
    console.log("Signup response:", response);
  },

  login: async ({ request, cookies, setHeaders }) => {
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

    type Result =
      | {
        status: "two_factor";
        email: string;
        url: string;
        method: string;
      }
      | {
        status: "success";
        token: string;
        email: string;
      };

    if (response.ok) {
      const result: Result = await response.json();
      console.log("Login response:", result);
      switch (result.status) {
        case "two_factor": {
          return redirect(302, result.url);
        }
        case "success": {
          const { token } = result;
          // response.headers.getSetCookie()?.map((c) => {
          //   console.log(c);
          // });
          if (token) {
            cookies.set("jwt_auth_token", token, {
              httpOnly: true,
              sameSite: "lax",
              path: "/",
            });
          }
          return redirect(302, "/app");
        }
      }
    }
  },
};
