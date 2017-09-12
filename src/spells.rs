pub fn get_spell(spell: &str) -> Option<&str> {
    match spell {
        "accio" => Some("wget"),
        "alohomora" => Some("open"),
        "aparecium" => Some("cat"),
        "avada kedavra" => Some("kill"),
        "confundo" => Some("rev"),
        "crucio" => Some("yes"),
        "gemino" => Some("cp"),
        "imperio" => Some("sudo"),
        "legilimens" => Some("less"),
        "locomotor" => Some("mv"),
        "lumos" => Some("ls"),
        "obliviate" => Some("rm"),
        "portus" => Some("ln"),
        "reducio" => Some("tar"),
        _ => None
    }
}
