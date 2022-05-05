fn str_list_to_vec(str_list: Vec<&str>) -> Vec<String> {
  str_list.iter()
    .map(|val| val.to_string())
    .collect()
}

pub fn get_suffixes() -> Vec<&'static str> {
  let suffixes = vec!["prime", "",
"B", "",
"alpha", "",
"proxima", "",
"IV", "",
"V", "",
"C", "",
"VI", "",
"VII", "",
"VIII", "",
"X", "",
"IX", "",
"D", "",
"", ""];
suffixes
}
