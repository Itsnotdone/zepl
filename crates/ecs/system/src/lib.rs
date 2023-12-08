use std::{
    fs::{File, OpenOptions, Permissions},
    io::{Read, Write},
};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{discouraged::Speculative, *},
    *,
};

#[proc_macro_attribute]
pub fn system(
    attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cloned_item = item.clone();

    let Function { name } = parse_macro_input!(cloned_item as Function);

    let item = TokenStream::from(item);

    let runner_function_name = Ident::new(
        format!("__internal_runner_{}", name).as_str(),
        Span::call_site(),
    );
    let runner = quote!(
        pub fn #runner_function_name((world, resources): (&mut World, &mut Resources)){
            #name(world, resources);
        }
    );

    let builder_function_name = Ident::new(
        format!("__internal_builder_{}", name).as_str(),
        Span::call_site(),
    );
    let builder = match attribute.to_string().as_str() {
        "Update" => {
            quote!(
                #[no_mangle]
                pub fn #builder_function_name(graph: &mut ScheduleGraph){
                    graph.insert(Update, #runner_function_name);
                }
            )
        }
        "Setup" => {
            quote!(
                #[no_mangle]
                pub fn #builder_function_name(graph: &mut ScheduleGraph){
                    graph.insert(Setup, #runner_function_name);
                }
            )
        }
        _ => panic!("Invalid Schedule Label"),
    };

    let systems_conf_dir =
        std::env::var("ZEPL_PROJECT_OUT_DIR").expect("Cannot find cargo env var");
    let mut systems_conf_file = String::from(systems_conf_dir);

    systems_conf_file.push_str("/systems.list");

    let mut builder_function_name = builder_function_name.to_string();

    builder_function_name.push('\n');

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .read(true)
        .open(systems_conf_file.clone())
    {
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();

        let systems = file_data.trim().split('\n').collect::<Vec<&str>>();
        if !systems.contains(&builder_function_name.trim()) {
            let mut perms = file.metadata().unwrap().permissions();
            perms.set_readonly(false);

            file.set_permissions(perms).unwrap();
            file.write_all(builder_function_name.as_bytes())
                .expect("Cannot write to file");
        }
    } else {
        let mut file = File::create(systems_conf_file).expect("Cannot create file");
        file.write_all(builder_function_name.to_string().as_bytes())
            .expect("Cannot write to file");
    }

    let output = quote!(
        #item
        #runner
        #builder
    );

    proc_macro::TokenStream::from(output)
}

struct Function {
    name: Ident,
}

impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Visibility>()?;
        input.parse::<Token![fn]>()?;

        let name = input.parse::<Ident>()?;

        input.parse::<TokenStream>()?;

        Ok(Function { name })
    }
}
