// todo: create a new macro to wrap this.
include!(env!("VIEW_FILES"));

fn main() {
    let view = basic_views::get_view("test" /*"comp/index*/);
    if let Some(view) = view {
        let output = view.render();
        println!("{}", output);
    }
}
