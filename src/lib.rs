use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn texto() -> String {
    String::from("Bem-vindo ao fantástico mundo do WebAssembly.")
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
