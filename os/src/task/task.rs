//! Types related to task management
use core::mem::size_of;

use super::TaskContext;
use crate::config::kernel_stack_position;
use crate::mm::{MapPermission, MemorySet, VirtAddr, KERNEL_SPACE, TRAMPOLINE_SPACE};
use crate::trap::{trap_handler, TrapContext};

/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

impl TaskControlBlock {
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let task_status = TaskStatus::Ready;
        // map a kernel-stack in kernel space
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        TRAMPOLINE_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::NX | MapPermission::W | MapPermission::D,
        );
        let trap_cx = kernel_stack_top - size_of::<TrapContext>();
        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(trap_cx),
            memory_set,
            base_size: user_sp,
            heap_bottom: user_sp,
            program_brk: user_sp,
        };
        // prepare TrapContext in user space
        let trap_cx = unsafe { (trap_cx as *mut TrapContext).as_mut().unwrap() };
        let kernel_pgdl = KERNEL_SPACE.exclusive_access().token();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            kernel_pgdl,
            trap_handler as usize,
        );
        task_control_block
    }
    /// change the location of the program break. return None if failed.
    pub fn change_program_brk(&mut self, size: i32) -> Option<usize> {
        let old_break = self.program_brk;
        let new_brk = self.program_brk as isize + size as isize;
        if new_brk < self.heap_bottom as isize {
            return None;
        }
        let result = if size < 0 {
            self.memory_set
                .shrink_to(VirtAddr(self.heap_bottom), VirtAddr(new_brk as usize))
        } else {
            self.memory_set
                .append_to(VirtAddr(self.heap_bottom), VirtAddr(new_brk as usize))
        };
        if result {
            self.program_brk = new_brk as usize;
            Some(old_break)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}
