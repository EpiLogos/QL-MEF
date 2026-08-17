use crate::{
    DeterministicProvenance, DeterministicResult, FourPlusTwoClass, KERNEL_VERSION, OperatorValue,
    QlAddress, QlOperator, SCHEMA_VERSION,
};

pub fn apply_operator(
    operator: QlOperator,
    input: QlAddress,
) -> DeterministicResult<OperatorValue> {
    let value = match operator {
        QlOperator::ConjugateAddress => {
            OperatorValue::Address(input.with_face(input.face().conjugate()))
        }
        QlOperator::ComplementAddress => {
            OperatorValue::Address(input.with_position(input.position().complement()))
        }
        QlOperator::ClassifyFourPlusTwo => {
            let class = match input.position().value() {
                0 | 5 => FourPlusTwoClass::Implicate,
                _ => FourPlusTwoClass::Explicate,
            };
            OperatorValue::FourPlusTwo(class)
        }
    };

    DeterministicResult {
        provenance: DeterministicProvenance {
            schema_version: SCHEMA_VERSION,
            kernel_version: KERNEL_VERSION,
            operation: operator.as_str(),
            input: input.to_string(),
            output: value.to_string(),
        },
        value,
    }
}
