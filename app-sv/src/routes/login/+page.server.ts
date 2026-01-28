import { redirect } from "@sveltejs/kit";
import type { PageServerLoad, Actions } from "./$types";

function getAuthUrl() {
  return typeof process === "undefined"
    ? "http://localhost:5170"
    : (process.env.AUTH_URL ?? "http://localhost:5170");
}

export const load: PageServerLoad = async () => {

};


export const actions: Actions = {
  signup: async ({ request, cookies }) => {
    const data = await request.formData();
    const email = data.get("email");
    const password = data.get("password");
    const confirmPassword = data.get("confirm_password");

    if (password !== confirmPassword) {
      return { error: "Passwords do not match" };
    }

    // Here you would typically send the data to your backend
    // For demonstration, we'll just log it to the console
    const response = await fetch(`${getAuthUrl()}/signup`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        method: "email_password",
        email,
        password,
        two_factor: "optional",
      }),
    });
  },


  login: async ({ request, cookies, setHeaders }) => {
    const data = await request.formData();
    const email = data.get("email");
    const password = data.get("password");
    const authUrl = getAuthUrl();

    // Here you would typically send the data to your backend
    // For demonstration, we'll just log it to the console
    const response = await fetch(`${authUrl}/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        method: "email_password",
        email,
        password,
      }),
    });

    if (response.ok) {
      const token: { token: string } = await response.json();
      if (token) {
        cookies.set('session', token.token, {
          httpOnly: true,
          sameSite: 'lax',
          path: "/"
        })
      }
      return redirect(302, '/app');
    }
  }
}
