

#[derive(Debug, Clone)]
pub enum Tag<MSG> {
    Node(Node<MSG>),
    Component(Element<MSG>)
}


pub fn html_tag<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, /*props: &[Attribute<MSG>],*/ children: &[Node<MSG>]) -> Tag<MSG> {
    Tag::Node(create_element(namespace, tag, /*props: &[Attribute<MSG>],*/ &children))
}

pub fn component_tag<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, /*props: &[Attribute<MSG>],*/ children: &[Node<MSG>]) -> Tag<MSG> {
   Tag::Component(Element {
        tag,
        namespace,
        props: props.to_vec(),
        children: children.to_vec()
    })
}

pub fn text_tag<MSG: Clone>(
    value: &str,
) -> nodes::Node<MSG>
{
    Tag::Node(create_text_leaf(None, "text", value))
}


pub fn comment_tag<MSG: Clone>(
    value: &'static str,
) -> nodes::Node<MSG>
{
    Tag::Node(create_comment_leaf(None, "comment", value))
}

pub fn doctype_tag<MSG: Clone>(
    value: &'static str,
) -> nodes::Node<MSG>
{
    Tag::Node(create_doctype_leaf(None, "document-type", value))
}

pub fn create_element<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, /*props: &[Attribute<MSG>],*/ children: &[Node<MSG>]) -> Node<MSG> {
    
    Node::Element(Element {
        tag,
        namespace,
        //props: props.to_vec(),
        children: children.to_vec()
    })
}


pub fn create_fragment<MSG: Clone>(nodes: &[Node<MSG>]) -> Node<MSG> {
    Node::Fragment(nodes.to_vec())
}

pub fn create_text_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::Text(Leaf {
        tag,
        namespace,
        //props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}

pub fn create_comment_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::Comment(Leaf {
        tag,
        namespace,
        //props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}

pub fn create_doctype_leaf<MSG: Clone>(namespace: Option<&'static str>, tag: &'static str, value: &str) -> Node<MSG> {
    
    Node::DocType(Leaf {
        tag,
        namespace,
        //props: Vec::<Attribute<MSG>>::with_capacity(0),
        value: value.to_owned()
    })
}