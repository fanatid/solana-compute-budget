use anchor_lang::InstructionData;
use solana_bpf_loader_program::{
    create_vm,
    serialization::{deserialize_parameters, serialize_parameters},
    syscalls, BpfError, ThisInstructionMeter,
};
use solana_rbpf::{
    elf::EBpfElf,
    vm::{Config, Executable},
};
use solana_sdk::{
    bpf_loader,
    entrypoint::SUCCESS,
    keyed_account::KeyedAccount,
    process_instruction::{MockComputeMeter, MockInvokeContext},
    pubkey::Pubkey,
};
use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use solana_compute_budget::instruction as budget_instruction;

fn load_program(name: &str) -> Vec<u8> {
    let mut file = File::open(name).unwrap();

    let mut program = Vec::new();
    file.read_to_end(&mut program).unwrap();
    program
}

fn run_program(
    program: &[u8],
    program_id: &Pubkey,
    parameter_accounts: &[KeyedAccount],
    instruction_data: &[u8],
) -> u64 {
    let loader_id = bpf_loader::id();
    let mut invoke_context = MockInvokeContext::new(parameter_accounts.into());

    let mut executable =
        EBpfElf::<BpfError, ThisInstructionMeter>::load(Config::default(), &program)
            .expect("failed to load program");
    executable.set_syscall_registry(
        syscalls::register_syscalls(&mut invoke_context)
            .expect("failed to create syscalls register"),
    );

    let mut parameter_bytes =
        serialize_parameters(&loader_id, program_id, parameter_accounts, instruction_data)
            .expect("failed to serialize");

    let mut vm = create_vm(
        &loader_id,
        &executable,
        parameter_bytes.as_slice_mut(),
        &mut invoke_context,
    )
    .expect("failed to create vm");

    let compute_meter = Rc::new(RefCell::new(MockComputeMeter {
        remaining: u64::MAX,
    }));
    let mut instruction_meter = ThisInstructionMeter { compute_meter };
    assert_eq!(
        vm.execute_program_interpreted(&mut instruction_meter)
            .expect("failed to execute"),
        SUCCESS,
        "Program executed with error"
    );

    deserialize_parameters(&loader_id, parameter_accounts, parameter_bytes.as_slice())
        .expect("failed to deserialize");

    vm.get_total_instruction_count()
}

#[test]
fn count_instructions() {
    let program = load_program("../../target/deploy/solana_compute_budget.so");
    let program_id = Pubkey::new_unique();
    let accounts: &[KeyedAccount] = &[];

    let data_empty = budget_instruction::Empty;
    let count_empty = run_program(&program, &program_id, accounts, &data_empty.data());
    println!("Instructions for call `empty`: {:?}", count_empty);

    let mut data_u128 = budget_instruction::U128Native {
        count: 1,
        rate: 1000,
        last_update_timestamp: 1628088929,
        current_timestamp: 1628088989,
    };
    let count_u128 = run_program(&program, &program_id, accounts, &data_u128.data());
    println!("Instructions for call `u128_native` (iters: 1): {:?}", count_u128);

    data_u128.count = 10;
    let count_u128 = run_program(&program, &program_id, accounts, &data_u128.data());
    println!("Instructions for call `u128_native` (iters: 10): {:?}", count_u128);

    let mut data_u128 = budget_instruction::U128Uint {
        count: 1,
        rate: 1000,
        last_update_timestamp: 1628088929,
        current_timestamp: 1628088989,
    };
    let count_u128 = run_program(&program, &program_id, accounts, &data_u128.data());
    println!("Instructions for call `u128_uint` (iters: 1): {:?}", count_u128);

    data_u128.count = 10;
    let count_u128 = run_program(&program, &program_id, accounts, &data_u128.data());
    println!("Instructions for call `u128_uint` (iters: 10): {:?}", count_u128);

    let mut data_u256 = budget_instruction::U256Uint {
        count: 1,
        rate: 1000,
        last_update_timestamp: 1628088929,
        current_timestamp: 1628088989,
    };
    let count_u256 = run_program(&program, &program_id, accounts, &data_u256.data());
    println!("Instructions for call `u256_uint` (iters: 1): {:?}", count_u256);

    data_u256.count = 10;
    let count_u256 = run_program(&program, &program_id, accounts, &data_u256.data());
    println!("Instructions for call `u256_uint` (iters: 10): {:?}", count_u256);
}
