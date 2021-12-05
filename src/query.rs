#[derive(PartialEq, Debug)]
pub struct QueryParams {
    title: Option<String>,
    designer_id: Option<u32>,
    geek_item_name: Option<String>,
    publisher_id: Option<u32>,
    yearpublished_min: Option<u16>,
    yearpublished_max: Option<u16>,
    minimum_age: Option<u8>,
    avg_rating_min: Option<f32>,
    avg_rating_max: Option<f32>,
    num_voters_min: Option<u16>,
    avg_weight_min: Option<f32>,
    avg_weight_max: Option<f32>,
    num_weights_min: Option<u16>,
    colfiltertype: Option<String>,
    search_user: Option<String>,
    players_min: Option<u8>,
    players_max: Option<u8>,
    player_range_type: Option<String>,
    playtime_min: Option<u8>,
    playtime_max: Option<u8>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            title: None,
            designer_id: None,
            geek_item_name: None,
            publisher_id: None,
            yearpublished_min: None,
            yearpublished_max: None,
            minimum_age: None,
            avg_rating_min: None,
            avg_rating_max: None,
            num_voters_min: None,
            avg_weight_min: None,
            avg_weight_max: None,
            num_weights_min: None,
            colfiltertype: None,
            search_user: None,
            players_min: None,
            players_max: None,
            player_range_type: None,
            playtime_min: None,
            playtime_max: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn designer_id(mut self, designer_id: u32) -> Self {
        self.designer_id = Some(designer_id);
        self
    }

    pub fn geek_item_name(mut self, geek_item_name: impl Into<String>) -> Self {
        self.geek_item_name = Some(geek_item_name.into());
        self
    }

    pub fn publisher_id(mut self, publisher_id: u32) -> Self {
        self.publisher_id = Some(publisher_id);
        self
    }

    pub fn yearpublished_min(mut self, yearpublished_min: u16) -> Self {
        self.yearpublished_min = Some(yearpublished_min);
        self
    }

    pub fn yearpublished_max(mut self, yearpublished_max: u16) -> Self {
        self.yearpublished_max = Some(yearpublished_max);
        self
    }

    pub fn minimum_age(mut self, minimum_age: u8) -> Self {
        self.minimum_age = Some(minimum_age);
        self
    }

    pub fn avg_rating_min(mut self, avg_rating_min: f32) -> Self {
        self.avg_rating_min = Some(avg_rating_min);
        self
    }

    pub fn avg_rating_max(mut self, avg_rating_max: f32) -> Self {
        self.avg_rating_max = Some(avg_rating_max);
        self
    }

    pub fn num_voters_min(mut self, num_voters_min: u16) -> Self {
        self.num_voters_min = Some(num_voters_min);
        self
    }

    pub fn avg_weight_min(mut self, avg_weight_min: f32) -> Self {
        self.avg_weight_min = Some(avg_weight_min);
        self
    }

    pub fn avg_weight_max(mut self, avg_weight_max: f32) -> Self {
        self.avg_weight_max = Some(avg_weight_max);
        self
    }

    pub fn num_weights_min(mut self, num_weights_min: u16) -> Self {
        self.num_weights_min = Some(num_weights_min);
        self
    }

    pub fn colfiltertype(mut self, colfiltertype: impl Into<String>) -> Self {
        self.colfiltertype = Some(colfiltertype.into());
        self
    }

    pub fn search_user(mut self, search_user: impl Into<String>) -> Self {
        self.search_user = Some(search_user.into());
        self
    }

    pub fn players_min(mut self, players_min: u8) -> Self {
        self.players_min = Some(players_min);
        self
    }

    pub fn players_max(mut self, players_max: u8) -> Self {
        self.players_max = Some(players_max);
        self
    }

    pub fn player_range_type(mut self, player_range_type: impl Into<String>) -> Self {
        self.player_range_type = Some(player_range_type.into());
        self
    }

    pub fn playtime_min(mut self, playtime_min: u8) -> Self {
        self.playtime_min = Some(playtime_min);
        self
    }

    pub fn playtime_max(mut self, playtime_max: u8) -> Self {
        self.playtime_max = Some(playtime_max);
        self
    }
}

pub fn make_reqwest_query(params: QueryParams) -> Vec<(String, String)> {
    let mut queries = Vec::new();
    queries.push(make_elem("advsearch", "1"));
    queries.push(make_elem("objecttype", "boardgame"));
    queries.push(make_elem("action", "search"));
    queries.push(make_elem("B1", "Submit"));

    assemble_query_param(&mut queries, "include[designerid]", params.designer_id);
    assemble_query_param(&mut queries, "geekitemname", params.geek_item_name);
    assemble_query_param(&mut queries, "include[publisherid]", params.publisher_id);
    assemble_query_param(
        &mut queries,
        "range[yearpublished][min]",
        params.yearpublished_min,
    );
    assemble_query_param(
        &mut queries,
        "range[yearpublished][max]",
        params.yearpublished_max,
    );
    assemble_query_param(&mut queries, "range[minage][max]", params.minimum_age);
    assemble_query_param(
        &mut queries,
        "floatrange[avgrating][min]",
        params.avg_rating_min,
    );
    assemble_query_param(
        &mut queries,
        "floatrange[avgrating][max]",
        params.avg_rating_max,
    );
    assemble_query_param(&mut queries, "range[numvoters][min]", params.num_voters_min);
    assemble_query_param(
        &mut queries,
        "floatrange[avgweight][min]",
        params.avg_weight_min,
    );
    assemble_query_param(
        &mut queries,
        "floatrange[avgweight][max]",
        params.avg_weight_max,
    );
    assemble_query_param(
        &mut queries,
        "range[numweights][min]",
        params.num_weights_min,
    );
    assemble_query_param(&mut queries, "colfiltertype", params.colfiltertype);
    assemble_query_param(&mut queries, "searchuser", params.search_user);
    assemble_query_param(&mut queries, "range[minplayers][max]", params.players_min);
    assemble_query_param(&mut queries, "range[maxplayers][min]", params.players_max);
    assemble_query_param(&mut queries, "playerrangetype", params.player_range_type);
    assemble_query_param(
        &mut queries,
        "range[leastplaytime][min]",
        params.playtime_min,
    );
    assemble_query_param(&mut queries, "range[playtime][max]", params.playtime_max);

    queries
}

fn assemble_query_param<T: ToString>(
    queries: &mut Vec<(String, String)>,
    key: &str,
    val: Option<T>,
) {
    if let Some(elem) = val {
        queries.push(make_elem(key, elem));
    }
}

fn make_elem<T: ToString>(key: &str, value: T) -> (String, String) {
    (key.to_string(), value.to_string())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        let reqwest_query = make_reqwest_query(
            QueryParams::new()
                .title("root")
                .designer_id(1000)
                .geek_item_name("geek_item_name")
                .publisher_id(1500)
                .yearpublished_min(2000)
                .yearpublished_max(2020)
                .minimum_age(3)
                .avg_rating_min(1.2)
                .avg_rating_max(9.8)
                .num_voters_min(123)
                .avg_weight_min(2.3)
                .avg_rating_max(4.99)
                .num_weights_min(345)
                .search_user("hiroaqii")
                .players_min(1)
                .players_max(8)
                .playtime_min(15)
                .playtime_max(90),
        );
        println!("{:?}", reqwest_query);
    }
}
