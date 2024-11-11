pub mod endpoints {
    pub const ROOT: &'static str = "/api";
    pub const AUTHORS: &'static str = "/authors";
    pub const USERS: &'static str = "/users";
    pub const BOOKS: &'static str = "/books";

    pub const GET_ALL: &'static str = "/";
    pub const GET_DETAIL: &'static str = "/:id/detail";
    pub const UPDATE: &'static str = "/:id/update";
    pub const DELETE: &'static str = "/:id/delete";
    pub const CREATE: &'static str = "/create";
}
