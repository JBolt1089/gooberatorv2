use crate::memory;

#[test]
fn default_ram_is_zero() {
    let memory = memory::Memory::new();
    assert_eq!(memory.get_ram(0), 0);
}

#[test]
fn read_and_write() {
    let mut memory = memory::Memory::new();
    memory.set_ram(69, 420);
    assert_eq!(memory.get_ram(69), 420);
}

#[test]
fn stack_push_pop_works() {
    let mut memory = memory::Memory::new();
    memory.push_stack(420);
    memory.push_stack(69);

    assert_eq!(memory.pop_stack(), 69);
}
