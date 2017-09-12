pub fn get_spell(spell: &str) -> Option<&str> {
    match spell {
        "accio" => Some("wget"),
        "alohomora" => Some("open"),
        "aparecium" => Some("cat"),
        "avada" => Some("kill"),
        "confundo" => Some("rev"),
        "crucio" => Some("yes"),
        "depulso" => Some("ping"),
        "gemino" => Some("cp"),
        "imperio" => Some("sudo"),
        "legilimens" => Some("less"),
        "locomotor" => Some("mv"),
        "lumos" => Some("ls"),
        "obliviate" => Some("clear"),
        "portus" => Some("ln"),
        "reducio" => Some("tar"),
        "reducto" => Some("rm"),
        "reparo" => Some("fsck"),
        "sectumsempra" => Some("cut"),
        "stupefy" => Some("sleep"),
        _ => None
    }
}
