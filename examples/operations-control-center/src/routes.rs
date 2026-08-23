use domius_web::Router;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppRoute {
    Overview,
    ServiceDetail,
    Incidents,
    Reports,
    NotFound,
}

pub fn router() -> Router<AppRoute> {
    let mut router = Router::new();
    router.register("/overview", AppRoute::Overview);
    router.register("/services/:id", AppRoute::ServiceDetail);
    router.register("/incidents", AppRoute::Incidents);
    router.register("/reports", AppRoute::Reports);
    router.register("*", AppRoute::NotFound);
    router
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_all_application_routes_and_service_parameter() {
        let router = router();

        assert_eq!(
            *router.match_route("/overview").unwrap().0,
            AppRoute::Overview
        );
        assert_eq!(
            *router.match_route("/incidents").unwrap().0,
            AppRoute::Incidents
        );
        assert_eq!(
            *router.match_route("/reports").unwrap().0,
            AppRoute::Reports
        );
        let (route, params) = router.match_route("/services/svc-03").unwrap();
        assert_eq!(*route, AppRoute::ServiceDetail);
        assert_eq!(params.get("id").map(String::as_str), Some("svc-03"));
        assert_eq!(
            *router.match_route("/absent").unwrap().0,
            AppRoute::NotFound
        );
    }
}
