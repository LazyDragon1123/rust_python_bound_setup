use ordered_float::OrderedFloat;
use pyo3::prelude::*;

#[derive(Clone)]
struct NumericFunction<'a> {
    py: Python<'a>,
    callback: PyObject,
}

impl NumericFunction<'_> {
    fn calc(&self, x: f64) -> OrderedFloat<f64> {
        let res = self.callback.call(self.py, (x,), None).unwrap();
        let res: f64 = res.as_ref(self.py).extract().unwrap();
        OrderedFloat(res)
    }
}

#[pyclass(module = "outputfilter")]
struct OutputFilter {
    numeric_function: PyObject,
    upper: OrderedFloat<f64>,
    lower: OrderedFloat<f64>,
}

#[pymethods]
impl OutputFilter {
    #[new]
    fn new(numeric_function: PyObject, upper: f64, lower: f64) -> Self {
        OutputFilter {
            numeric_function: numeric_function,
            upper: OrderedFloat(upper),
            lower: OrderedFloat(lower),
        }
    }
    fn run(self_: PyRef<'_, Self>, x_values: Vec<f64>) -> PyResult<PyObject> {
        let func = NumericFunction {
            py: self_.py(),
            callback: self_.numeric_function.clone(),
        };
        let filtered: Vec<f64> = x_values.clone().iter().filter(|x|
            func.calc(**x) < self_.upper && func.calc(**x) > self_.lower
        ).map(|x| *x)
        .collect();
        Ok(filtered.to_object(self_.py()))
    }
}

/// Packaging for Python.
#[pymodule]
fn outputfilter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<OutputFilter>()?;
    Ok(())
}
