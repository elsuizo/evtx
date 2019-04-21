use crate::{ensure_env_logger_initialized, EvtxParser, ParserSettings};

#[test]
// https://github.com/omerbenamram/evtx/issues/10
fn test_dirty_sample_single_threaded() {
    ensure_env_logger_initialized();
    let evtx_file = include_bytes!("../../samples/2-system-Security-dirty.evtx");

    let mut parser = EvtxParser::from_buffer(evtx_file.to_vec()).unwrap();

    let mut count = 0;
    for r in parser.records() {
        r.unwrap();
        count += 1;
    }
    assert_eq!(count, 14621, "Single threaded iteration failed");
}

#[test]
fn test_dirty_sample_parallel() {
    ensure_env_logger_initialized();
    let evtx_file = include_bytes!("../../samples/2-system-Security-dirty.evtx");

    let mut parser = EvtxParser::from_buffer(evtx_file.to_vec())
        .unwrap()
        .with_configuration(ParserSettings::new().num_threads(8));

    let mut count = 0;

    for r in parser.records() {
        r.unwrap();
        count += 1;
    }

    assert_eq!(count, 14621, "Parallel iteration failed");
}

#[test]
fn test_parses_sample_with_irregular_boolean_values() {
    ensure_env_logger_initialized();
    // This sample contains boolean values which are not zero or one.
    let evtx_file = include_bytes!("../../samples/sample-with-irregular-bool-values.evtx");

    let mut parser = EvtxParser::from_buffer(evtx_file.to_vec()).unwrap();

    for r in parser.records() {
        r.unwrap();
    }
}