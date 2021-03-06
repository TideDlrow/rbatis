use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, Data, FnArg, ItemFn, parse_macro_input, ReturnType};

use crate::proc_macro::TokenStream;
use crate::string_util;

///TODO Redundant code deletion
pub(crate) fn impl_macro_py_sql(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    match &target_fn.sig.output {
        ReturnType::Type(_, b) => {
            return_ty = b.to_token_stream();
        }
        _ => {}
    }

    let mut s = format!("{}", return_ty);
    if !s.starts_with("rbatis_core :: Result") && !s.starts_with("Result") && !s.starts_with("std :: result :: Result") {
        return_ty = quote! {
             rbatis_core :: Result <#return_ty>
        };
    }

    let func_name = format!("{}", target_fn.sig.ident.to_token_stream());
    let rbatis_meta = args.get(0).unwrap();
    let field_name = format!("{}", rbatis_meta.to_token_stream());

    let sql_meta = args.get(1).unwrap();
    let sql = format!("{}", sql_meta.to_token_stream()).trim().to_string();

    //fetch fn arg names
    let mut fn_arg_name_vec = vec![];
    let mut tx_id_ident = quote! {""};
    for arg in &target_fn.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                let arg_name = format!("{}", t.pat.to_token_stream());
                if arg_name.eq(&field_name) {
                    continue;
                }
                if arg_name.contains("tx_id") {
                    tx_id_ident = t.pat.to_token_stream();
                    continue;
                }
                fn_arg_name_vec.push(arg_name);
                //println!("arg_name {}", arg_name);
            }
            _ => {}
        }
    }
    let sql_ident = sql_meta;
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let rbatis_ident = Ident::new(&field_name, Span::call_site());
    //append all args
    let mut args_gen = quote! {
         let mut args = serde_json::Map::new();
    };
    for item in fn_arg_name_vec {
        let item_ident = Ident::new(&item, Span::call_site());
        let item_name_stream = item.to_token_stream();
        args_gen = quote! {
            #args_gen
            args.insert(#item_name_stream.to_string(),serde_json::to_value(#item_ident).unwrap_or(serde_json::Value::Null));
       };
    }

    args_gen = quote! {
        #args_gen
        let mut args = serde_json::Value::from(args);
    };

    let is_select = sql.starts_with("select ") || sql.starts_with("SELECT ") || sql.starts_with("\"select ") || sql.starts_with("\"SELECT ");

    if is_select {
        let gen = quote! {
        pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
              #args_gen
              return #rbatis_ident.py_fetch("",#sql_ident,&args).await;
        }
    };
        return gen.into();
    } else {
        let gen = quote! {
        pub async fn #func_name_ident(#func_args_stream) -> #return_ty {
              #args_gen
              return #rbatis_ident.py_exec("",#sql_ident,&args).await;
        }
    };
        return gen.into();
    }
}