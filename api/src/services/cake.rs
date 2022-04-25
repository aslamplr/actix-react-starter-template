use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use entity::sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait,
};
use entity::{cake::Entity as Cake, fruit::Entity as Fruit};

#[async_trait]
pub trait CakeService {
    async fn get_all_cakes(&self) -> Result<CakeResponseModel, DbErr>;
    async fn get_cake_by_id(&self, id: i32) -> Result<CakeModel, DbErr>;
    async fn create_cake(&self, cake_input: CreateCakeModel) -> Result<(i32, Vec<i32>), DbErr>;
}

#[derive(Debug, Clone)]
pub struct PgCakeService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PgCakeService<'a> {
    pub fn spawn(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CakeService for PgCakeService<'_> {
    async fn get_all_cakes(&self) -> Result<CakeResponseModel, DbErr> {
        Ok(Cake::find()
            .find_with_related(Fruit)
            .all(self.db)
            .await?
            .into())
    }

    async fn get_cake_by_id(&self, id: i32) -> Result<CakeModel, DbErr> {
        let result = Cake::find_by_id(id)
            .find_with_related(Fruit)
            .all(self.db)
            .await?;
        Ok(CakeModel::try_from(result)
            .map_err(|_| DbErr::RecordNotFound("Not found!".to_string()))?)
    }

    async fn create_cake(&self, cake_input: CreateCakeModel) -> Result<(i32, Vec<i32>), DbErr> {
        self.db
            .transaction::<_, (i32, Vec<i32>), DbErr>(|txn| {
                Box::pin(async move {
                    let CreateCakeModel { name, fruits } = cake_input;

                    let cake = entity::cake::ActiveModel {
                        name: Set(name),
                        ..Default::default()
                    };
                    let cake = cake.insert(txn).await?;

                    let cake_id = cake.id;
                    let mut fruit_ids = Vec::<i32>::new();

                    for fruit in fruits {
                        let fruit = entity::fruit::ActiveModel {
                            name: Set(fruit),
                            cake_id: Set(Some(cake_id)),
                            ..Default::default()
                        };
                        let fruit = fruit.insert(txn).await?;
                        fruit_ids.push(fruit.id);
                    }

                    Ok((cake_id, fruit_ids))
                })
            })
            .await
            .map_err(|err| DbErr::Custom(err.to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCakeModel {
    pub name: String,
    pub fruits: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CakeResponseModel {
    pub data: Vec<CakeModel>,
}

#[derive(Debug, Serialize)]
pub struct CakeModel {
    pub id: i32,
    pub name: String,
    pub fruits: Vec<FruitModel>,
}

#[derive(Debug, Serialize)]
pub struct FruitModel {
    pub id: i32,
    pub name: String,
}

type CakeRecord = (entity::cake::Model, Vec<entity::fruit::Model>);

impl From<Vec<CakeRecord>> for CakeResponseModel {
    fn from(cakes: Vec<CakeRecord>) -> Self {
        let len = cakes.len();
        let data = cakes
            .into_iter()
            .fold(Vec::<CakeModel>::with_capacity(len), |mut acc, item| {
                acc.push(item.into());
                acc
            });
        Self { data }
    }
}

impl TryFrom<Vec<CakeRecord>> for CakeModel {
    type Error = ();

    fn try_from(cakes: Vec<CakeRecord>) -> Result<Self, Self::Error> {
        let len = cakes.len();
        cakes
            .into_iter()
            .fold(Vec::<CakeModel>::with_capacity(len), |mut acc, item| {
                acc.push(item.into());
                acc
            })
            .into_iter()
            .next()
            .ok_or(())
    }
}

impl From<CakeRecord> for CakeModel {
    fn from((cake, fruits): CakeRecord) -> Self {
        let fruits = fruits.into_iter().map(FruitModel::from).collect::<Vec<_>>();
        Self {
            id: cake.id,
            name: cake.name,
            fruits,
        }
    }
}

impl From<entity::fruit::Model> for FruitModel {
    fn from(fruit: entity::fruit::Model) -> Self {
        Self {
            id: fruit.id,
            name: fruit.name,
        }
    }
}
