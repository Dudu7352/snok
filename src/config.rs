use pson::Expr;

pub enum SizeConfig {
    Percent(f64),
    Chars(u16),
}

pub struct Config {
    pub width: SizeConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: SizeConfig::Percent(100.0)
        }
    }
}

pub fn read_config(path: String) -> Option<Config>{
    let content = std::fs::read_to_string(path).ok()?;
    let mut parser = pson::PsonParser::new(content.chars());
    parser.parse().unwrap();
    let config_map = parser
        .get().ok()?
        .as_array()
        .into_iter()
        .filter(|a| a.len() == 1)
        .find_map(|it| it.first().map(|it| it.as_map())?)?;
    let mut conf = Config::default();
    config_map.iter().try_for_each(|(k, v)|
        match (k.as_str(), v) {
            ("width", Expr::String(v)) => {
                (&mut conf).width = match v.as_str() {
                    "auto" => SizeConfig::Percent(100.0),
                    perc if &perc[perc.len() - 1..] == "%" => 
                        SizeConfig::Percent(perc[..perc.len()-1].parse().ok()?),
                    s if (s.parse::<u16>()).is_ok() => SizeConfig::Chars(s.parse().ok()?),
                    _ => None?
                };
                Some(())
            }
            _ => None
        }
    )?;
    Some(conf)
}

