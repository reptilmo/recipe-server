pub struct Recipe {
    pub name: String,
    pub ingredients: String,
    pub preparation: String,
}

pub fn get_recipe() -> Recipe {
    let mut ingreds = Vec::<String>::new();
    ingreds.push("4 apples".to_string());
    ingreds.push("1/2 cup water".to_string());
    ingreds.push("8 tablespoon sugar".to_string());
    ingreds.push("1/2 teaspoon cinnamon".to_string());
    ingreds.push("2 tablespoon butter".to_string());

    Recipe {
        name: "Baked Apples".to_string(),
        ingredients: ingreds.join(", "),
        preparation: "Select apples of uniform size.\n
Wash and core.\n
Place in a pan, cover the bottom with water.\n
Fill each cavity with sugar, a dash of powdered cinnamon and a tiny lump of butter.
Bake for thirty minutes, basting occasionally.\n
Serve around the platter of pork chops.\n"
            .to_string(),
    }
}
