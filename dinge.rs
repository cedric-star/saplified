        let mut method_list: Vec<String> = Vec::new();
        let mut method_str = String::new();
        for l in methods_str.lines() {
            let mut is_new_method = false;
            for http_method in HTPP_METHODS {
                if l.starts_with(http_method) {
                    is_new_method = true;
                    break;
                }
            }

            if is_new_method && !method_str.is_empty() {
                method_list.push(method_str.clone());
                method_str.clear();
            }

            method_str.push_str(l);
            method_str.push('\n');
        }

        if !method_str.is_empty() {
            method_list.push(method_str);
        }
        OpenApiPath::new(
            path_str,
            method_list
                .into_iter()
                .map(|s| OpenApiMethod::from_yml_string(s))
                .collect::<Vec<OpenApiMethod>>(),
        )
