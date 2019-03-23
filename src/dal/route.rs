use config::DataRoute;
use config::MemRoute;
use dal::db::open_db;
use dal::db::DB;
use dal::mem::Mem;
use error::Error;
use dal::table::Table;

pub struct Route {
    alias: String,
    route: DataRoute,
    db: DB,
    mem: Option<Mem>,
}

impl Route {
    pub fn new(alias: &str, route: &DataRoute) -> Result<Route, Error> {
        let db = open_db(&route.db)?;
        let mem = Self::init_mem(&route.mem)?;
        let route = Route {
            alias: alias.to_string(),
            route: route.clone(),
            db,
            mem,
        };
        Ok(route)
    }

    pub fn eq(&self, route: &DataRoute) -> bool {
        self.route == route.clone()
    }

    pub fn get_db(&self) -> &DB {
        &self.db
    }

    pub fn get_mem(&self) -> &Option<Mem> {
        &self.mem
    }

    pub fn update(&mut self, route: &DataRoute) -> Result<(), Error> {
        if route.db != self.route.db {
            let db = open_db(&route.db)?;
            self.db = db;
            self.route.db = route.db.clone()
        }
        if route.mem != self.route.mem {
            let mem = Self::init_mem(&route.mem)?;
            self.mem = mem;
            self.route.mem = route.mem.clone()
        }
        Ok(())
    }

    fn init_mem(mr: &Option<MemRoute>) -> Result<Option<Mem>, Error> {
        match mr {
            Some(m) => {
                let mem = Mem::instance(m)?;
                Ok(Some(mem))
            }
            None => Ok(None),
        }
    }

    pub fn get_table(&self, table: String) -> Option<&Table> {
        self.db.tables.get(&table)
    }
}
