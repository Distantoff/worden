
// src models words mod 
// Динамичная сортирока
type AnonUserOrder<T> = Box<dyn BoxableExpression<T, Mysql, SqlType = NotSelectable>>;
impl<T> From<Order> for AnonUserOrder<T>
    where
        T: BoxedDsl<'static, Mysql>,
        crate::schema::words_en::value: SelectableExpression<T>,
{

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

            _ => Box::new(words_en::value.asc()),
        }
    }
}

let order: AnonUserOrder<crate::schema::words_en::table> = self.order.into();
words = words.order(order);

async fn _get_user_word_ids(uid: i32) -> Vec<i32> {
    use crate::schema::user_words::dsl::*;
    let connection = &mut establish_connection();
    let user_word_ids = user_words
        .select(words_en_id)
        .filter(user_id.eq(uid))
        .load(connection)
        .expect("Error to load user word list");

    user_word_ids
}

async fn _get_words() -> Vec<EnWord> {
    use crate::schema::words_en::dsl::*;
    let connection = &mut establish_connection();
    let words = words_en
        .select(EnWord::as_select())
        .limit(10)
        .load(connection)
        .expect("Error to load word list");

    words
}

async fn _get_enwords_by_id_vec(ids: Vec<i32>) -> Vec<EnWord> {
    use crate::schema::words_en::dsl::*;
    let connection = &mut establish_connection();
    let enwords = words_en
        .select(EnWord::as_select())
        .filter(id.eq_any(ids))
        // .limit(10)
        .load(connection)
        .expect("Error to load enword list");

    enwords

}
