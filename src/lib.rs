extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use chemistru_elements::raw::RawElement;

static DATA: &str = include_str!("../periodic-table-data/periodic-table.json");

#[proc_macro]
pub fn element_map(_: TokenStream) -> TokenStream {
    let elements: Vec<RawElement> = serde_json::from_str(DATA).unwrap();

    let element_init = init_all_elements(&elements);

    let proton_numbers = elements.iter().map(|e| e.number).collect::<Vec<_>>();

    let tokens = quote! {
        pub static PROTON_NUMBER_MAP: phf::OrderedMap<u8, chemistru_elements::element::Element> = phf::phf_ordered_map! {
            #( #proton_numbers => #element_init),*
        };
    };

    TokenStream::from(tokens)
}


fn init_all_elements(elements: &[RawElement]) -> Vec<proc_macro2::TokenStream> {
    let mut quote_buf = vec![];
    
    for element in elements {
        let name = element.name;
        let symbol = element.symbol;
        let proton_number = element.number;
        let mass_number = element.atomic_mass;
        let inner = element.clone().into_inner();

        let stream = quote! { chemistru_elements::element::Element::new(#name, #symbol, #mass_number, #proton_number, #inner) };

        quote_buf.push(stream);
    }

    quote_buf
}
