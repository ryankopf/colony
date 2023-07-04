use crate::prelude::*;

pub fn task_system_playing(_commands: Commands, mut query: Query<(&mut Brain, &mut Status)>) {
    for (mut brain, mut status) in query.iter_mut() {
        if brain.task != Some(Task::Play) {
            continue;
        }
        if let Some(n) = &mut status.needs_entertainment {
            n.current += 10.0;
            if n.current >= n.max {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
}
