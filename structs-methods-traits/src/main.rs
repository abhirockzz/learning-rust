#[derive(Debug)]
struct Programmer {
    email: String,
    github: String,
    blog: String,
}
impl Programmer {
    fn is_same_as(&self, other: &Programmer) -> bool {
        return self.email == other.email;
    }

    fn details(self) {
        println!(
            "Email: {},\nGitHub repo: {},\nBlog: {}",
            self.email, self.github, self.blog
        )
    }

    fn to_upper(&mut self) {
        self.email = self.email.to_uppercase();
    }

    fn new(email: String, github: String, blog: String) -> Self {
        return Programmer {
            email: email,
            github: github,
            blog: blog,
        };
    }
}

trait Pretty_Print {
    fn pretty_print(&self);
}

impl Pretty_Print for Programmer {
    fn pretty_print(&self) {
        println!(
            "{{\n\t\"email\": {},\n\t\"github_repo\" repo: {},\n\t\"blog_url\": {}\n}}",
            self.email, self.github, self.blog
        )
    }
}

/*use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;*/

impl std::fmt::Display for Programmer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.email, self.github, self.blog)
    }
}

fn print_the_printable(p: impl Pretty_Print) {
    p.pretty_print()
}

fn get_printable(info: Vec<String>) -> impl Pretty_Print {
    Programmer::new(
        String::from(&info[0]),
        String::from(&info[1]),
        String::from(&info[2]),
    )
}

fn main() {
    let pg1 = Programmer {
        email: String::from("abhirockzz@gmail.com"),
        github: String::from("https://github.com/abhirockzz"),
        blog: String::from("https://dev.to/abhirockzz"),
    };

    let mut pg2 = Programmer {
        email: String::from("abhirockzz@gmail.com"),
        github: String::from("https://github.com/abhirockzz"),
        blog: String::from("https://medium.com/@abhishek1987"),
    };
    println!("pg1 same as pg2? {}", pg1.is_same_as(&pg2));
    pg2.to_upper();
    pg2.details();
    let pg3 = Programmer::new(
        String::from("abhirockzz@gmail.com"),
        String::from("https://github.com/abhirockzz"),
        String::from("https://medium.com/@abhishek1987"),
    );
    pg3.pretty_print();
    println!("programmer details: {}", pg3);
    println!("{:#?}", pg3);
    print_the_printable(pg3);
    //uncomment to see compiler error
    //pg2.is_same_as(&pg1);
    let info: Vec<String> = vec![
        String::from("abhirockzz@gmail.com"),
        String::from("https://github.com/abhirockzz"),
        String::from("https://medium.com/@abhishek1987"),
    ];
    get_printable(info).pretty_print();
}
