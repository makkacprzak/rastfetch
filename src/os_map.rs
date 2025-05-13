use phf::phf_map;

pub static OS_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "Fedora Linux" => "fedora",
    "Ubuntu Linux" => "ubuntu"
};