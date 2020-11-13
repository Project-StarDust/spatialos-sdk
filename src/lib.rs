use spatialos_sys::schema::ComponentData;
use spatialos_sys::schema::ComponentUpdate;
use spatialos_sys::worker::component_vtable::ComponentVtable;
use spatialos_sys::worker::ComponentDataHandle;
use spatialos_sys::worker::ComponentId;
use spatialos_sys::worker::ComponentUpdateHandle;
use spatialos_sys::Schema_ComponentData;
use spatialos_sys::Schema_ComponentUpdate;
use std::os::raw::c_void;

pub trait Component: Sized {
    type Data: Clone;
    type Update: Clone;

    const ID: ComponentId;

    fn component_data_deserialize(
        component_id: ComponentId,
        user_data: *mut c_void,
        source: ComponentData,
    ) -> Self::Data;

    fn component_data_serialize(
        component_id: ComponentId,
        user_data: *mut c_void,
        handle: &mut Self::Data,
    ) -> ComponentData;

    fn component_update_deserialize(
        component_id: ComponentId,
        user_data: *mut c_void,
        source: ComponentUpdate,
    ) -> Self::Update;

    fn component_update_serialize(
        component_id: ComponentId,
        user_data: *mut c_void,
        handle: &mut Self::Update,
    ) -> ComponentUpdate;

    fn component_update_free(
        component_id: ComponentId,
        user_data: *mut c_void,
        handle: Self::Update,
    );

    fn component_update_copy(
        component_id: ComponentId,
        user_data: *mut c_void,
        handle: &Self::Update,
    ) -> Self::Update;

    fn get_vtable() -> ComponentVtable {
        ComponentVtable {
            component_id: Self::ID,
            user_data: std::ptr::null_mut(),
            command_request_free: None,
            command_request_copy: None,
            command_request_deserialize: None,
            command_request_serialize: None,
            command_response_free: None,
            command_response_copy: None,
            command_response_deserialize: None,
            command_response_serialize: None,
            component_data_free: Some(component_data_free::<Self>),
            component_data_copy: Some(component_data_copy::<Self>),
            component_data_deserialize: Some(component_data_deserialize::<Self>),
            component_data_serialize: Some(component_data_serialize::<Self>),
            component_update_free: Some(component_update_free::<Self>),
            component_update_copy: Some(component_update_copy::<Self>),
            component_update_deserialize: Some(component_update_deserialize::<Self>),
            component_update_serialize: Some(component_update_serialize::<Self>),
        }
    }
}


pub unsafe extern "C" fn component_data_deserialize<T: Component>(
    component_id: ComponentId,
    user_data: *mut c_void,
    source: *mut Schema_ComponentData,
    handle_out: *mut *mut ComponentDataHandle,
) -> u8 {
    assert_eq!(component_id, T::ID);
    let source = source.into();
    let new_data = Box::new(T::component_data_deserialize(
        component_id,
        user_data,
        source,
    ));
    *(handle_out as *mut *mut T::Data) = Box::into_raw(new_data);
    1
}

pub unsafe extern "C" fn component_data_serialize<T: Component>(
    component_id: ComponentId,
    user_data: *mut c_void,
    handle: *mut ComponentDataHandle,
    target_out: *mut *mut Schema_ComponentData,
) {
    assert_eq!(component_id, T::ID);
    let handle = handle as *mut T::Data;
    let mut data = Box::from_raw(handle);
    let component_data = T::component_data_serialize(component_id, user_data, &mut *data).into();
    *target_out = component_data;
    Box::into_raw(data);
}

pub unsafe extern "C" fn component_update_deserialize<T: Component>(
    component_id: ComponentId,
    _: *mut c_void,
    _source: *mut Schema_ComponentUpdate,
    _handle_out: *mut *mut ComponentUpdateHandle,
) -> u8 {
    assert_eq!(component_id, T::ID);
    unimplemented!()
}

pub unsafe extern "C" fn component_update_serialize<T: Component>(
    component_id: ComponentId,
    _: *mut c_void,
    _handle: *mut ComponentUpdateHandle,
    _target_out: *mut *mut Schema_ComponentUpdate,
) {
    assert_eq!(component_id, T::ID);
    unimplemented!()
}

pub unsafe extern "C" fn component_data_copy<T: Component>(
    component_id: ComponentId,
    _: *mut c_void,
    handle: *mut ComponentDataHandle,
) -> *mut ComponentDataHandle {
    assert_eq!(component_id, T::ID);
    let handle = handle as *mut T::Data;
    let ptr = Box::from_raw(handle);
    let new_data = ptr.clone();
    Box::into_raw(ptr);
    Box::into_raw(new_data) as *mut ComponentDataHandle
}

pub unsafe extern "C" fn component_data_free<T>(
    component_id: ComponentId,
    _: *mut c_void,
    handle: *mut ComponentDataHandle,
) {
    assert_eq!(component_id, T::ID);
    Box::from_raw(handle);
}

pub unsafe extern "C" fn component_update_free<T: Component>(
    component_id: ComponentId,
    user_data: *mut c_void,
    handle: *mut ComponentUpdateHandle,
) {
    assert_eq!(component_id, T::ID);
    let handle = handle as *mut T::Update;
    let handle = Box::from_raw(handle);
    T::component_update_free(component_id, user_data, *handle)
}

pub unsafe extern "C" fn component_update_copy<T: Component>(
    component_id: ComponentId,
    user_data: *mut c_void,
    handle: *mut ComponentUpdateHandle,
) -> *mut ComponentUpdateHandle {
    assert_eq!(component_id, T::ID);
    let handle = handle as *mut T::Update;
    let ptr = Box::from_raw(handle);
    let new_data = Box::new(T::component_update_copy(component_id, user_data, &*ptr));

    Box::into_raw(ptr);
    Box::into_raw(new_data) as *mut ComponentDataHandle
}

pub mod sys_exports {
    pub use spatialos_sys::*;
}