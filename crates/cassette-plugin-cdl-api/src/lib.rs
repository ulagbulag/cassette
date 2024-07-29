mod zone;

use actix_web::Scope;

pub fn build_services(scope: Scope) -> Scope {
    scope.service(self::zone::list)
}
