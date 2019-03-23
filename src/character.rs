use crate::datastore::Datastore;

pub struct Character<'d> {
    pub name : String,
    data : &'d Datastore,
}

impl<'d> Character<'d> {
    /// Creates a new character that will draw on the data in the store passed to it here.
    /// This means characters cannot outlive the existence of the data store, which is reasonable
    /// since they need the data in it to know what e.g. feats do.
    pub fn new(data : &'d Datastore) -> Self {
        Character {
            name : String::new(),
            data : data,
        }
    }
}

mod test_character;