/* Wiki specific stuff */
pub fn to_full_wiki_url(url: &str, base_url: &String) -> String {
    let mut url = url.to_owned();
    // no empty case here, come on
    // here we consider it as complete, no other edge cases
    if url.starts_with(base_url) {
        return url;
    }

    // lot of optimizations here isntead of starts_with chain but for small strings it will work
    // not handling "wiki/Something" case
    if !url.starts_with('/') {
        url.insert_str(0, "/wiki/");
    } else if !url.starts_with("/wiki/") {
        url.insert_str(0, "/wiki");
    }
    url.insert_str(0, base_url);
    url
}

/// no empty str edge case. We die as heroes here
pub fn to_relative_wiki_url(url: &str, base_url: &String) -> String {
    let mut url = url.to_owned();
    let wiki_url = format!("{base_url}/wiki");

    if url.starts_with(&wiki_url) {
        url = String::from(&url[base_url.len()..]);
    } else if !url.starts_with("/") {
        url.insert_str(0, "/wiki/");
    } else if !url.starts_with("/wiki/") {
        url.insert_str(1, "wiki/");
    }
    url
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_relative_wiki_url_test() {
        let target = String::from("/wiki/Pokémon");
        let base_url = String::from("https://en.wikipedia.org");

        let complete_url = String::from("https://en.wikipedia.org/wiki/Pokémon");
        let result = to_relative_wiki_url(&complete_url, &base_url);
        assert_eq!(result, target);

        let wiki_url = String::from("/wiki/Pokémon");
        let result = to_relative_wiki_url(&wiki_url, &base_url);
        assert_eq!(result, target);

        let partial_wiki_url = String::from("Pokémon");
        let result = to_relative_wiki_url(&partial_wiki_url, &base_url);
        assert_eq!(result, target);
    }

    #[test]
    fn to_complete_wiki_url_test() {
        let target = String::from("https://en.wikipedia.org/wiki/Pokémon");
        let base_url = String::from("https://en.wikipedia.org");

        let already_complete_url = String::from("https://en.wikipedia.org/wiki/Pokémon");
        let incomplete_url_with_prefix = String::from("/wiki/Pokémon");
        let incomplete_url_without_prefix_with_slash = String::from("/Pokémon");
        let incomplete_url_without_prefix_without_slash = String::from("Pokémon");

        let result = to_full_wiki_url(&already_complete_url, &base_url);
        assert_eq!(result, target);

        let result = to_full_wiki_url(&incomplete_url_with_prefix, &base_url);
        assert_eq!(result, target);

        let result = to_full_wiki_url(&incomplete_url_without_prefix_with_slash, &base_url);
        assert_eq!(result, target);

        let result = to_full_wiki_url(&incomplete_url_without_prefix_without_slash, &base_url);
        assert_eq!(result, target);
    }
}
