/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rsx;
extern crate rsx_dom;
extern crate rsx_shared;
extern crate rsx_stylesheet;

use rsx::css;
use rsx::rsx;
use rsx_dom::types::*;
use rsx_stylesheet::types::*;

type DOMNode = rsx_dom::types::DOMNode<StyleDeclarations, ComputedStyles, ()>;

#[test]
fn test_rsx_to_dom() {
    struct Props {
        visible: bool,
        menu: MenuProps
    }

    struct MenuProps {
        icon: String
    }

    fn should_do_something_fun() -> bool {
        true
    }

    fn what_fun() -> String {
        "Something Fun!".to_string()
    }

    fn what_else() -> String {
        "Something Else".to_string()
    }

    let props = Props {
        visible: true,
        menu: MenuProps {
            icon: "icon.png".to_string()
        }
    };

    let node = rsx! {
        <Dropdown show={props.visible}>
            A dropdown list
            <Menu icon={props.menu.icon}>
                <MenuItem>Do Something</MenuItem>
                {
                    if should_do_something_fun() {
                        <MenuItem>Do{ what_fun() }</MenuItem>
                    } else {
                        <MenuItem>Do{ what_else() }</MenuItem>
                    }
                }
            </Menu>
        </Dropdown>
    };

    let expected = DOMNode::from((
        DOMTagName::from("Dropdown"),
        box [
            DOMAttribute::from((
                DOMAttributeName::from("show"),
                DOMAttributeValue::from(true)
            )),
        ],
        box [
            DOMNode::from("A dropdown list"),
            DOMNode::from((
                DOMTagName::from("Menu"),
                box [
                    DOMAttribute::from((
                        DOMAttributeName::from("icon"),
                        DOMAttributeValue::from("icon.png")
                    )),
                ],
                box [
                    DOMNode::from((
                        DOMTagName::from("MenuItem"),
                        box [DOMNode::from("Do Something")]
                    )),
                    DOMNode::from({
                        DOMNode::from((
                            DOMTagName::from("MenuItem"),
                            box [DOMNode::from("Do"), DOMNode::from("Something Fun!")]
                        ))
                    }),
                ]
            )),
        ]
    ));

    assert_eq!(node, expected);
}

#[test]
fn test_css_to_stylesheet_1() {
    let stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".foo".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);

    stylesheet.ignore_unused();
    expected.ignore_unused();
}

#[test]
fn test_css_to_stylesheet_2() {
    let stylesheet = css! {
        .foo, .bar {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".foo".into(),
                    specificity: 1024u32
                },
                StyleSelector {
                    css_string: ".bar".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);

    stylesheet.ignore_unused();
    expected.ignore_unused();
}

#[test]
fn test_css_to_stylesheet_3() {
    let stylesheet = css! {
        .foo, .bar-baz {
            margin: 0 auto;
            padding: 10px;
            flex-wrap: nowrap;
            flex-direction: row-reverse;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".foo".into(),
                    specificity: 1024u32
                },
                StyleSelector {
                    css_string: ".bar-baz".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexWrap(Wrap::NoWrap)),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::RowReverse)),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);

    stylesheet.ignore_unused();
    expected.ignore_unused();
}

#[test]
fn test_css_to_stylesheet_4() {
    let stylesheet = css!("tests/fixtures/test_1.css");

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".root".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    components: ColorComponents {
                        red: 255,
                        green: 0,
                        blue: 0,
                        alpha: 255
                    },
                    authored: Some("red".into())
                })),
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
            ]))
        },
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".image".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    components: ColorComponents {
                        red: 0,
                        green: 128,
                        blue: 0,
                        alpha: 255
                    },
                    authored: Some("green".into())
                })),
                StyleDeclaration::Theme(ThemeStyle::Opacity(0.5.into())),
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
            ]))
        },
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector {
                    css_string: ".text".into(),
                    specificity: 1024u32
                },
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    components: ColorComponents {
                        red: 0,
                        green: 0,
                        blue: 255,
                        alpha: 255
                    },
                    authored: Some("blue".into())
                })),
                StyleDeclaration::Theme(ThemeStyle::Color(Color {
                    components: ColorComponents {
                        red: 255,
                        green: 255,
                        blue: 0,
                        alpha: 255
                    },
                    authored: None
                })),
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);

    stylesheet.ignore_unused();
    expected.ignore_unused();
}

#[test]
fn test_rsx_and_css_1() {
    let mut stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let node = rsx! {
        <div style={stylesheet.get(".foo")}>
            Hello world!
        </div>
    };

    let expected = DOMNode::from((
        DOMTagName::from(KnownElementName::Div),
        box [
            DOMAttribute::from((
                DOMAttributeName::from(KnownAttributeName::Style),
                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                    StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                    StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10.0.into()))),
                ])))
            )),
        ],
        box [DOMNode::from("Hello world !")]
    ));

    assert_eq!(node, expected);
}

#[test]
fn test_rsx_and_css_2() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let node = rsx! {
        <div style={stylesheet.get(".bar")}>
            Hello world!
        </div>
    };

    let expected = DOMNode::from((
        DOMTagName::from(KnownElementName::Div),
        box [
            DOMAttribute::from((
                DOMAttributeName::from(KnownAttributeName::Style),
                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::default()))
            )),
        ],
        box [DOMNode::from("Hello world !")]
    ));

    assert_eq!(node, expected);

    stylesheet.ignore_unused();
}

#[test]
fn test_example_1() {
    let mut stylesheet = css! {
        .root {
            width: 500px;
            height: 120px;
            flex-direction: row;
            padding: 20px;
        }
        .image {
            width: 80px;
            margin-right: 20px;
        }
        .text {
            height: 25px;
            align-self: center;
            flex-grow: 1;
        }
    };

    let node = rsx! {
        <view style={stylesheet.get(".root")}>
            <image style={stylesheet.get(".image")} src="..." />
            <text style={stylesheet.get(".text")}>
                Hello world!
            </text>
        </view>
    };

    let expected = DOMNode::from((
        DOMTagName::from(KnownElementName::View),
        box [
            DOMAttribute::from((
                DOMAttributeName::from(KnownAttributeName::Style),
                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                ])))
            )),
        ],
        box [
            DOMNode::from((
                DOMTagName::from(KnownElementName::Image),
                box [
                    DOMAttribute::from((
                        DOMAttributeName::from(KnownAttributeName::Style),
                        DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                            StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                            StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                        ])))
                    )),
                    DOMAttribute::from((
                        DOMAttributeName::from(KnownAttributeName::Src),
                        DOMAttributeValue::from("...")
                    )),
                ]
            )),
            DOMNode::from((
                DOMTagName::from(KnownElementName::Text),
                box [
                    DOMAttribute::from((
                        DOMAttributeName::from(KnownAttributeName::Style),
                        DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                            StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                            StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                            StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                        ])))
                    )),
                ],
                box [DOMNode::from("Hello world !")]
            )),
        ]
    ));

    assert_eq!(node, expected);
}

#[test]
fn test_example_2() {
    fn greeting_str(name: &str) -> String {
        format!("Hello {}!", name)
    }

    fn render_greeting(name: &str) -> DOMNode {
        let mut stylesheet = css!("tests/fixtures/test_1.css").ignore_unused();

        rsx! {
            <text style={stylesheet.get(".text")}>
                { greeting_str(name) }
            </text>
        }
    }

    fn render_children(name: Option<&str>, image: DOMNode) -> DOMNode {
        rsx! {
            <view>
                { image }
                {
                    match name {
                        Some(ref n) => render_greeting(n),
                        None => <text>No greetings!</text>
                    }
                }
            </view>
        }
    }

    fn render_root() -> DOMNode {
        let mut stylesheet = css!("tests/fixtures/test_1.css").ignore_unused();

        rsx! {
            <view style={stylesheet.get(".root")}>
                {
                    let name = Some("world");
                    let image = <image style={stylesheet.get(".image")} src="..." />;
                    render_children(name, image)
                }
            </view>
        }
    }

    let node = render_root();

    let expected = DOMNode::from((
        DOMTagName::from(KnownElementName::View),
        box [
            DOMAttribute::from((
                DOMAttributeName::from(KnownAttributeName::Style),
                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                        components: ColorComponents {
                            red: 255,
                            green: 0,
                            blue: 0,
                            alpha: 255
                        },
                        authored: Some("red".into())
                    })),
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                ])))
            )),
        ],
        box [
            DOMNode::from((
                DOMTagName::from(KnownElementName::View),
                box [
                    DOMNode::from((
                        DOMTagName::from(KnownElementName::Image),
                        box [
                            DOMAttribute::from((
                                DOMAttributeName::from(KnownAttributeName::Style),
                                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                                        components: ColorComponents {
                                            red: 0,
                                            green: 128,
                                            blue: 0,
                                            alpha: 255
                                        },
                                        authored: Some("green".into())
                                    })),
                                    StyleDeclaration::Theme(ThemeStyle::Opacity(0.5.into())),
                                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                                ])))
                            )),
                            DOMAttribute::from((
                                DOMAttributeName::from(KnownAttributeName::Src),
                                DOMAttributeValue::from("...")
                            )),
                        ]
                    )),
                    DOMNode::from((
                        DOMTagName::from(KnownElementName::Text),
                        box [
                            DOMAttribute::from((
                                DOMAttributeName::from(KnownAttributeName::Style),
                                DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                    StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                                        components: ColorComponents {
                                            red: 0,
                                            green: 0,
                                            blue: 255,
                                            alpha: 255
                                        },
                                        authored: Some("blue".into())
                                    })),
                                    StyleDeclaration::Theme(ThemeStyle::Color(Color {
                                        components: ColorComponents {
                                            red: 255,
                                            green: 255,
                                            blue: 0,
                                            alpha: 255
                                        },
                                        authored: None
                                    })),
                                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                                    StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                                    StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                                ])))
                            )),
                        ],
                        box [DOMNode::from("Hello world!")]
                    )),
                ]
            )),
        ]
    ));

    assert_eq!(node, expected);
}
