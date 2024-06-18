use diesel::{
    prelude::*, dsl::*, mysql::Mysql,
    expression::expression_types::NotSelectable
};
use std::str::FromStr;
use crate::errors::model_errors::ParseOrderError;
use crate::helpers::{database::*, config};
use crate::helpers::words::words_helper::{WordResult, WordsHelper};
use crate::models::words_en::*;
use crate::schema::*;

impl WordsBuilder {
    pub fn new() -> Self {
        Self {
            uid: None,
            limit: config::get("MAX_LIMIT").parse::<i64>().unwrap(),
            offset: Some(0),
            order: Order::Date(OrderDirection::Desc),
            enword_id_list: None,
            exclude_id_list: None,
            searched_value: None,
        }
    }

    pub fn user_id(mut self, uid: i32) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn limit(mut self, mut limit: i64) -> Self {
        if limit > config::get("MAX_LIMIT").parse::<i64>().unwrap() {
            limit = config::get("MAX_LIMIT").parse::<i64>().unwrap();
            println!("limit exceeded max value {}", limit);
        }
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn page_number(self, page_number: i64) -> Self {
        let offset = (page_number - 1) * self.limit;
        self.offset(offset)
    }

    pub fn order(mut self, order_str: &String) -> Self {
        let order_str = order_str.trim();
        self.order = Order::from_str(&order_str).unwrap();
        self
    }

    pub fn enword_id_list(mut self, ids: Vec<i32>) -> Self {
        self.enword_id_list = Some(ids);
        self
    }

    pub fn exclude_id_list(mut self, ids: Vec<i32>) -> Self {
        self.exclude_id_list = Some(ids);
        self
    }

    pub fn find(mut self, value: String) -> Self {
        self.searched_value = Some(value);
        self
    }

    pub async fn get_words(self) -> Vec<WordResult> {
        let enwords = self.get_enwords();
        let words = WordsHelper::new().await;
        words.get_words_by_enword_list(enwords).await
    }

    fn get_enwords(self) -> Vec<EnWord> {
        match self.uid {
            Some(_) => self.get_enwords_for_reg_user(),
            None => match self.enword_id_list {
                Some(_) => self.get_enwords_for_anon_user(),
                None => self.find_enwords(),
            }
        }
    }

    fn get_enwords_for_reg_user(self) -> Vec<EnWord> {
        use crate::schema::*;
        let connection = &mut establish_connection();

        let mut words = words_en::table
            .select(EnWord::as_select())
            .inner_join(user_words::table
                .on(user_words::words_en_id.eq(words_en::id)))
            .filter(user_words::user_id.eq(self.uid.unwrap()))
            .filter(user_words::hidden.eq(false))
            .offset(self.offset.unwrap())
            .limit(self.limit)
            .into_boxed();

        if let Some(value) = self.searched_value {
            words = words.filter(words_en::value.like(value + "%"));
        }

        if let Some(exlude_id_list) = self.exclude_id_list {
            words = words.filter(words_en::id.ne_all(exlude_id_list));
        }

        let order: RegUserOrder = self.order.into();
        words = words.order(order);

        let words_list: Vec<EnWord> =
            words.load(connection).expect("Error to load enword list");

        words_list
    }

    fn get_enwords_for_anon_user(self) -> Vec<EnWord> {
        use crate::schema::*;
        let connection = &mut establish_connection();

        let mut words = words_en::table
            .select(EnWord::as_select())
            .filter(words_en::id.eq_any(self.enword_id_list.as_ref().unwrap()))
            .offset(self.offset.unwrap())
            .limit(self.limit)
            .into_boxed();

        if let Some(value) = &self.searched_value {
            words = words.filter(words_en::value.like(value.to_owned() + "%"));
        }

        let order: AnonUserOrder = self.order.into();
        words = words.order(order);

        let words_list = words
            .load(connection)
            .expect("Error to load enword list");

        self.sort_enwords_by_date_for_anon_user_if_passed(words_list)
    }

    fn find_enwords(self) -> Vec<EnWord> {
        use crate::schema::words_en::dsl::*;
        let connection = &mut establish_connection();

        let mut words = words_en
            .select(EnWord::as_select())
            .filter(value.like(self.searched_value.unwrap().to_owned() + "%"))
            .into_boxed();

        if let Some(exlude_id_list) = self.exclude_id_list {
            words = words.filter(id.ne_all(exlude_id_list));
        }

        let words_list = words.offset(self.offset.unwrap())
            .order(value.asc())
            .limit(self.limit)
            .load(connection)
            .expect("Error to load enword list");

        words_list
    }

    fn sort_enwords_by_date_for_anon_user_if_passed(
        &self, enwords: Vec<EnWord>) -> Vec<EnWord> {

        if self.order != Order::Date(OrderDirection::Asc) &&
            self.order != Order::Date(OrderDirection::Desc) {
            return enwords;
        }

        let mut sorted_enwords = enwords;
        sorted_enwords.sort_by(|a, b| {
            let index_a = self.enword_id_list.as_ref().unwrap().iter().enumerate()
                .filter(|(_, &id)| id == a.id.unwrap())
                .map(|(index, _)| index)
                .collect::<Vec<_>>().first().copied().unwrap();

            let index_b = self.enword_id_list.as_ref().unwrap().iter().enumerate()
                .filter(|(_, &id)| id == b.id.unwrap())
                .map(|(index, _)| index)
                .collect::<Vec<_>>().first().copied().unwrap();

            match self.order {
                Order::Date(OrderDirection::Asc) => index_a.cmp(&index_b),
                _ => index_b.cmp(&index_a),
            }
        });

        sorted_enwords
    }
}

#[derive(Debug, Clone)]
pub struct WordsBuilder {
    uid: Option<i32>,
    limit: i64,
    offset: Option<i64>,
    order: Order,
    enword_id_list: Option<Vec<i32>>,
    exclude_id_list: Option<Vec<i32>>,
    searched_value: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Order {
    Date(OrderDirection),
    Value(OrderDirection),
    Random,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl FromStr for Order {
    type Err = ParseOrderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "value,asc" => Ok(Self::Value(OrderDirection::Asc)),
            "value,desc" => Ok(Self::Value(OrderDirection::Desc)),
            "date,asc" => Ok(Self::Date(OrderDirection::Asc)),
            "date,desc" => Ok(Self::Date(OrderDirection::Desc)),
            "rand" => Ok(Self::Random),

            _ => Err(ParseOrderError {
                incorrect_order_expr: s.to_string()
            }),
        }
    }
}

type RegUserOrder = Box<dyn BoxableExpression<diesel::dsl::InnerJoinQuerySource<
    crate::schema::words_en::table, crate::schema::user_words::table,
        Eq<crate::schema::user_words::words_en_id, crate::schema::words_en::id>>,
    Mysql, SqlType = NotSelectable>>;

impl From<Order> for RegUserOrder {
    fn from(value: Order) -> Self {
        match value {
            Order::Value(OrderDirection::Asc) =>
                Box::new(words_en::value.asc()),

            Order::Value(OrderDirection::Desc) =>
                Box::new(words_en::value.desc()),

            Order::Date(OrderDirection::Asc) =>
                Box::new(user_words::create_date.asc()),

            Order::Date(OrderDirection::Desc) =>
                Box::new(user_words::create_date.desc()),

            Order::Random => {
                sql_function!(fn rand() -> Text);
                Box::new(rand().asc())
            }
        }
    }
}

type AnonUserOrder = Box<dyn BoxableExpression<
    crate::schema::words_en::table, Mysql, SqlType = NotSelectable>>;
impl From<Order> for AnonUserOrder {
    fn from(value: Order) -> Self {
        match value {
            Order::Value(OrderDirection::Asc) =>
                Box::new(words_en::value.asc()),

            Order::Value(OrderDirection::Desc) =>
                Box::new(words_en::value.desc()),

            Order::Random => {
                sql_function!(fn rand() -> Text);
                Box::new(rand().asc())
            },

            _ => Box::new(words_en::id.asc()),
        }
    }
}
