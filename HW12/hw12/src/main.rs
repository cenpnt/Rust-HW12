fn main() {}
//1.1
#[allow(dead_code)]
enum Shape1 {
    Circle1(f64, f64, f64),
    Rectangle1(f64, f64, f64, f64),
    Triangle1(f64, f64, f64, f64, f64, f64)
}

#[allow(dead_code)]
impl Shape1 {
    fn rep_string(&self) -> String {
        match self {
            Shape1::Circle1(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
            Shape1::Rectangle1(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
            Shape1::Triangle1(x1, y1, x2, y2, x3, y3) => format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2, y2, x3, y3),
        }
    }
    fn area(&self) -> f64 {
        match self {
            Shape1::Circle1(_, _, r) => std::f64::consts::PI * r * r,
            Shape1::Rectangle1(_, _, w, h) => w * h,
            Shape1::Triangle1(x1, y1, x2, y2, x3, y3) => 0.5 * (((x1 - x3) * (y2 - y1)) - ((x1 - x2) * (y3 - y1))),
        }
    }
}

#[test]
fn test() {
    const INPUT_SHAPES: &[Shape1] = &[
        Shape1::Circle1(0., 0., 1.), Shape1::Circle1(50., 50., 15.),
        Shape1::Rectangle1(40., 40., 20., 20.), Shape1::Rectangle1(10., 40., 15., 10.),
        Shape1::Triangle1(2., 3., 7., 20., 10., -5.)
    ];

    const EXPECTED: &[&str] = &[
        "<Circle: 0, 0, 1>, area: 3.14",
        "<Circle: 50, 50, 15>, area: 706.86",
        "<Rectangle: 40, 40, 20, 20>, area: 400.00",
        "<Rectangle: 10, 40, 15, 10>, area: 150.00",
        "<Triangle: 2, 3, 7, 20, 10, -5>, area: -88.00"
    ];

    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();

    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area())
    );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}
///////////////////////////////////////////////////////////////////////
//1.2
trait Shape2 {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_box(&self) -> Box<dyn Shape2>;
}

struct Circle2 {
    x: f64,
    y: f64,
    radius: f64,
}

struct Rectangle2 {
    t: f64,
    b: f64,
    w: f64,
    h: f64,
}

struct Triangle2 {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64
}

impl Clone for Box<dyn Shape2> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Shape2 for Circle2 {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn clone_box(&self) -> Box<dyn Shape2> {
        Box::new(Circle2{x: self.x, y: self.y, radius: self.radius})
    }
}

impl Shape2 for Rectangle2 {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.t, self.b, self.w, self.h)
    }
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn clone_box(&self) -> Box<dyn Shape2> {
        Box::new(Rectangle2{t: self.t, b: self.b, w: self.w, h: self.h})
    }
}

impl Shape2 for Triangle2 {
    fn rep_string(&self) -> String {
        format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
    }
    fn area(&self) -> f64 {
        0.5 * (((self.x1 - self.x3) * (self.y2 - self.y1)) - ((self.x1 - self.x2) * (self.y3 - self.y1)))
    }
    fn clone_box(&self) -> Box<dyn Shape2> {
        Box::new(Triangle2{x1: self.x1, y1: self.y1, x2: self.x2, y2: self.y2, x3: self.x3, y3: self.y3})
    }
}

#[allow(dead_code)]
impl Circle2 {
    fn new(x: f64, y: f64, radius: f64) -> Box<Circle2>{
        Box::new(Circle2{x, y, radius})
    }
}

#[allow(dead_code)]
impl Rectangle2 {
    fn new(t: f64, b: f64, w: f64, h: f64) -> Box<Rectangle2> {
        Box::new(Rectangle2 {t, b, w, h})
    }
}

#[allow(dead_code)]
impl Triangle2 {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Box<Triangle2> {
        Box::new(Triangle2 { x1, y1, x2, y2, x3, y3 })
    }
}

#[allow(dead_code)]
fn input_shape_list() -> Vec<Box<dyn Shape2>> {
    vec![
        Circle2::new(0., 0., 1.), Circle2::new(50., 50., 15.),
        Rectangle2::new(40., 40., 20., 20.), Rectangle2::new(10., 40., 15., 10.),
        Triangle2::new(2., 3., 7., 20., 10., -5.)
    ]
}

#[allow(dead_code)]
const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>","<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>","<Rectangle: 10, 40, 15, 10>",
    "<Triangle: 2, 3, 7, 20, 10, -5>"
];

#[allow(dead_code)]
const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: 2, 3, 7, 20, 10, -5>, area: -88.00"
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}
///////////////////////////////////////////////////////////////////////
// 2.1
#[derive(Clone)]
#[allow(dead_code)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, usize),
    Joined(Vec<Box<Text>>, Box<Text>)
}

#[allow(dead_code)]
impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(b, u) => b.clone().value().repeat(*u),
            Text::Joined(j, sep) => j.iter().map(| j | j.value()).collect::<Vec<_>>().join(&sep.value())
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> { Box::new(t.clone()) }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text { &self }
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
        tvec.push(t1.into());
        tvec.push(t2.into());
        tvec.push(t3.into());
        tvec.push(t4.into());

    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x","[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}
///////////////////////////////////////////////////////////////////////
//2.2
trait Text2 {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text2>;
}
    
impl Clone for Box<dyn Text2> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText { 
    chars: String
}

struct RepeatedText { 
    chars: String,
    times: usize,
}

struct JoinedText {
    chars: Vec<Box<dyn Text2>>,
    sep: Box<dyn Text2>
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText{ chars: text.to_string() }
    }
}

impl Text2 for PlainText {
    fn value(&self) -> String { self.chars.clone() }
    fn clone_box(&self) -> Box<dyn Text2> { Box::new(self.clone()) } // == Box::new(PlainText { chars: self.chars.clone() })
}

impl AsRef<dyn Text2> for PlainText {
    fn as_ref(&self) -> &(dyn Text2 + 'static) { self }
}

#[allow(dead_code)]
impl RepeatedText {
    fn with_parts(text: &dyn Text2, times: usize) -> RepeatedText {
        RepeatedText{ chars: text.value(), times}
    }
}

impl Text2 for RepeatedText {
   fn value(&self) -> String {
        self.chars.clone().repeat(self.times)
   }
   fn clone_box(&self) -> Box<dyn Text2> {
       Box::new(RepeatedText{ chars : self.chars.clone(), times : self.times})
   }
}

impl AsRef<dyn Text2> for RepeatedText {
    fn as_ref(&self) -> &(dyn Text2 + 'static) { self }
}

#[allow(dead_code)]
impl JoinedText {
    fn with_parts(text: &Vec<Box<dyn Text2>>, sep: &dyn Text2) -> JoinedText {
        JoinedText { chars: text.clone(), sep: sep.clone_box()}
    }
}

impl Text2 for JoinedText {
    fn value(&self) -> String {
        self.chars.clone().iter().map(| j | j.value()).collect::<Vec<_>>().join(&self.sep.value())
    }
    fn clone_box(&self) -> Box<dyn Text2> {
        Box::new(JoinedText { chars: self.chars.clone(), sep: self.sep.clone()})
    }
}

impl AsRef<dyn Text2> for JoinedText {
    fn as_ref(&self) -> &(dyn Text2 + 'static) { self }
}

#[test]
fn test_text_composition_2() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text2>> = Vec::new();
        tvec.push(t1.clone_box());
        tvec.push(t2.clone_box());
        tvec.push(t3.clone_box());
        tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}