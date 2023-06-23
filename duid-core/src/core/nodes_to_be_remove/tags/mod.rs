




#[cfg(test)] 
mod tag_test {
    use crate::core::nodes::{
        TagEl, Tag
    };

    #[test]
    fn test_new() {
        let tag_el_1 = TagEl::new("12-25-23", "div", []);
        let tag_el_2 = TagEl::new("12-25-24", "div", []);
        let tag_el_3 = Tag::new("12-25-25", "div", [tag_el_1, tag_el_2]);
        let tag_el_4 = Tag::new_node(tag_el_3);

        let root_component = Tag::new_component(tag_el_4);
        println!("tag_el: {:#?}", tag_el);
    }
}