use spatialos_sys::schema::Object;
use std::os::raw::c_void;

pub trait Type: Sized {
    type Data: Clone;
    type Update: Clone;

    fn type_data_deserialize(user_data: *mut c_void, source: &mut Object) -> Self::Data;

    fn type_data_serialize(user_data: *mut c_void, data: &mut Self::Data, target: &mut Object);

    fn type_update_deserialize(user_data: *mut c_void, source: &mut Object) -> Self::Update;

    fn type_update_serialize(user_data: *mut c_void, data: &mut Self::Update, target: &mut Object);

    fn type_update_free(user_data: *mut c_void, data: Self::Update);

    fn type_update_copy(user_data: *mut c_void, data: &Self::Update) -> Self::Update;
}
