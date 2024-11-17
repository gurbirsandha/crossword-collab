pub mod postgres {
    use sqlx::{postgres::PgRow, Execute, Executor, PgPool, Postgres, QueryBuilder, Row};

    #[derive(Debug)]
    pub struct Board {
        contents: String,
        height: i32,
        width: i32,
    }
    pub struct DB {
        pg_pool: Option<PgPool>,
    }

    impl DB {
        pub fn new() -> DB {
        // use option or something? and handle error outside?
            // let pg_pool = PgPool::connect("postgres://localhost:5432/postgres").await.unwrap_or_else(op)
            DB {
                pg_pool: None
            }
        }
        pub async fn get_connection_pool(&self) -> Option<PgPool> {
            if let Some(pg_pool) = self.pg_pool.clone() {
                Some(pg_pool)
            } else {
                match PgPool::connect("postgres://gurbir:crossword@localhost:5432/postgres").await {
                    Ok(pool) => {
                        Some(pool)
                    },
                    Err(e) => {
                        println!("Error connecting to DB: {:?}", e);
                        None
                    }
                }
                
            }
        }
        pub async fn get_board(&self, guid: String) -> Option<Board> {
            if let Some(pool) = self.get_connection_pool().await {
                let mut query_builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT height, width, board_contents FROM board WHERE join_code = ");
                query_builder.push_bind(guid);
                let query = query_builder.build();
                println!("{:?}", query.sql());
                match query.fetch_one(&pool).await {
                    Ok(res) => {
                        println!("{:?}",res);
                        Some(
                            Board {
                                contents: res.try_get("board_contents").unwrap_or(String::new()),
                                height: res.try_get("height").unwrap(),
                                width: res.try_get("width").unwrap(),
                            }
                        )
                    }, // TODO: wtf comes out of here
                    Err(e) => 
                    {
                        println!("Error fetching: {:?}", e);
                        None
                    }
                }
                // println!("{}", guid);
            } else {
                None
            }
           
        }

        // async fn get_first_result(&self, query: &Query<'_, Postgres, PgArguments>) ->  Option<PgRow> {
        //     if let Some(pool) = self.get_connection_pool().await {
        //         match query.fetch_one().await {
        //             Ok(res) => Some(res),
        //             Err(e) => {
        //                 println!("Error fetching: {:?}", e);
        //                 None
        //             }
        //         }
        //     } else {
        //         None
        //     }
        // }
    
    }

    
}