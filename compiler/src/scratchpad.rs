// struct Baggage<T> {
//     payload: Vec<T>,
// }

// impl<T> Baggage<T> {
//     fn push(&mut self, v: T) -> &T {
//         self.payload.push(v);
//         &self.payload.last().unwrap()
//     }
// }

// fn mk<'b, 't>(baggage: &'b mut Baggage<Id>) -> MyStruct<'t>
// where
//     'b: 't,
// {
//     let id = Id {
//         name: "hi".to_string(),
//     };

//     MyStruct {
//         something: Something {
//             x: baggage.push(id),
//         },
//     }
// }

// // Can't change anything below, it's from an upstream library

// #[derive(Debug)]
// pub struct Id {
//     pub name: String,
// }

// #[derive(Debug)]
// pub struct Something<'t> {
//     pub x: &'t Id,
// }
