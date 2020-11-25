use std::collections::HashMap;

#[derive(Default)]
struct Cache {
    data: HashMap<String, String>,
}

#[derive(Default)]
struct App {
    cache1: Option<Cache>,
    cache2: Option<Cache>,
}

impl App {
    fn new() -> Self {
        App {
            cache1: Some(Cache::default()),
            cache2: Some(Cache::default()),
        }
    }

    fn insert(&mut self, id: String, data: String) {
        if let Some(cache) = self.cache2.as_mut(){
            cache.data.insert(id, data);
        }
    }

    fn clean(&mut self) {
        self.cache1 = self.cache2.take();
        self.cache2 = Some(Cache::default())
    }
}

fn main() {
    println!("Hello, world!");
    let mut app = App::new();


    let mut count = 1;
    loop {
        count = count + 1;
        if count > 10 {
            break;
        }

        app.clean();

        app.insert("2123".to_string(), "dataString".to_string());

    }
}
