//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.

mod home;
pub use home::Home;

mod signup;
pub use signup::Signup;

mod app_navbar;
pub use app_navbar::AppNavbar;

mod login;
pub use login::Login;
