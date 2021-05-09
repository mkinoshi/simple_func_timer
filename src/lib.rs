use proc_macro::TokenStream;
use syn::Lit;
use syn::Meta;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

const VALID_LOG_LEVELS: [&str; 4] = ["info", "warn", "error", "debug"];
const VALID_UNIT: [&str; 3] = ["ms", "ns", "s"];

fn parse_args(args: &[NestedMeta]) -> (String, String) {
    let mut log = "debug".to_string();
    let mut unit = "ms".to_string();
    for arg in args {
        if let NestedMeta::Meta(meta) = arg {
            match meta {
                Meta::NameValue(name_value) => {
                    if name_value.path.segments[0].ident == "log" {
                        let log_level = match &name_value.lit {
                            Lit::Str(s) => s.value(),
                            _ => log,
                        };
                        log = log_level;
                    } else if name_value.path.segments[0].ident == "unit" {
                        let measurement_unit = match &name_value.lit {
                            Lit::Str(s) => s.value(),
                            _ => unit,
                        };
                        unit = measurement_unit;
                    } else {
                        panic!("Illegal attribute");
                    }
                }
                _ => panic!("Illegal attribute"),
            }
        }
    }

    if !VALID_LOG_LEVELS.contains(&(log.as_str())) {
        panic!("Invalid value for log attribue");
    }

    if !VALID_UNIT.contains(&(unit.as_str())) {
        panic!("Invalid value for unit attribue");
    }
    (log, unit)
}

#[proc_macro_attribute]
pub fn timer(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    let parsed_attrs = parse_macro_input!(attr as AttributeArgs);
    let (log, unit) = parse_args(&parsed_attrs);
    let func_name = sig.ident.to_string();

    TokenStream::from(quote::quote! {
        #(#attrs)* #vis #sig {
            let now = std::time::Instant::now();
            let ret = (|| #block )();
            let end_time = now.elapsed();

            let time = match #unit {
                "s" => format!("{:?} s", end_time.as_secs()),
                "ns" => format!("{:?} ns", end_time.as_nanos()),
                _ => format!("{:?} ms", end_time.as_millis()),
            };

            let res = format!("{} function took {}", #func_name, time);
            match #log {
                "info" => log::info!("{}", res),
                "warn" => log::warn!("{}", res),
                "error" => log::error!("{}", res),
                _ => log::debug!("{}", res),
            };
            ret
        }
    })
}
