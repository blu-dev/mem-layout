mod view;

pub use view::*;

#[test]
fn byte_view_test() {
    #[repr(C)]
    #[derive(Debug)]
    struct SomeStruct {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        a: f32
    }
    let view = ByteView::<40>::zeros();
    panic!("{:?}", view.view_as::<SomeStruct>());
}

#[test]
fn as_part_of_struct() {
    #[repr(C)]
    struct SomeStruct {
        x: u64,
        y: f32,
        bytes: ByteView<32>
    }

    let wrapper = SomeStruct {
        x: 0,
        y: 0.0,
        bytes: ByteView::zeros()
    };

    panic!("{:?}", wrapper.bytes.view_as::<u64>());
}