mod station;
mod module;

pub use station::Station;
pub use module::Module;

struct TestTorus {
    major: usize,
    minor: usize,
}

struct TestMoudle<'a> {
    torus: &'a TestTorus,
}

struct TestStation<'a> {
    torus: TestTorus,
    modules: Vec<TestMoudle<'a>>,
}

impl<'a> TestStation<'a> {
    fn new() {
        let torus = TestTorus {
            major: 12,
            minor: 2,
        };

        let mut station = TestStation {
            torus,
            modules: Vec::new(),
        };

        let module = TestMoudle {
            torus: &station.torus,
        };

        station.modules.push(module);
    }
}