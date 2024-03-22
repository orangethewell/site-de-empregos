use pyo3::{prelude::*, prepare_freethreaded_python};

const CODE: &'static str = include_str!("../../scripts/payment_api.py");

pub fn generate_payment_url() -> String {
    prepare_freethreaded_python();
    Python::with_gil(|vm| {
        let fun: Py<PyAny> = PyModule::from_code(vm, CODE, "payment_api.py", "payment_api")
            .unwrap()
            .getattr("generate_payment_url")
            .unwrap()
            .into();

        fun.call0(vm)
            .unwrap()
            .extract(vm)
            .unwrap()
    })
}