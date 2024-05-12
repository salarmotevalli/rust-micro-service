use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use async_trait::async_trait;
use std::{str::FromStr, sync::Arc};

use crate::{
    application::repository::OrderRepository,
    domain::entity::order::{CreateOrder, Order},
    error::HandlerError,
};

pub struct Repository {
    pub pool: Arc<Client>,
}

impl Repository {
    pub fn new(db: Arc<Client>) -> Self {
        Self { pool: db }
    }
}

#[async_trait]
impl OrderRepository for Repository {
    async fn get_all(&self) -> Result<Vec<Order>, HandlerError> {
        todo!()
    }

    async fn create(&self, order: CreateOrder) -> Result<(), HandlerError> {
        let coll: Collection<CreateOrder> = self.pool.database("order").collection("orders");
        coll.insert_one(order, None).await?;
        Ok(())
    }

    async fn get_by_id(&self, id: String) -> Result<Option<Order>, HandlerError> {
        let coll: Collection<Order> = self.pool.database("order").collection("orders");
        let res = coll
            .find_one(doc! {"_id": ObjectId::from_str(id.as_str())?}, None)
            .await?;

        Ok(res)
    }
}
