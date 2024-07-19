pub const APP_REVIEWS: &'static str = "https://store.steampowered.com/appreviews/{}?json=1?filter=all?review_type=all&purchase_type=all&language=all&num_per_page=0";
pub const APP_INFO: &'static str =
    "https://store.steampowered.com/api/appdetails/?appids={1}&l={2}";
pub const APP_PLAYERS: &'static str =
    "https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid={}";
pub const APP_LIST: &'static str = "https://api.steampowered.com/ISteamApps/GetAppList/v1";
