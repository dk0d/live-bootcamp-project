// place files you want to import through the `$lib` alias in this folder.

declare module "bun" {
  interface Env {

    AUTH_SERVICE_URL: string;

  }

}
