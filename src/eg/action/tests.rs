use super::*;
use crate::core::event::Event;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
struct TestEvent {
    id: String,
}

impl Event for TestEvent {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn get_type(&self) -> crate::core::event::EventType {
        crate::core::event::EventType::System
    }
    
    fn get_payload(&self) -> &crate::core::event::EventPayload {
        unimplemented!()
    }
    
    fn get_timestamp(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }
    
    fn get_source(&self) -> Option<&str> {
        None
    }
}

#[test]
fn test_action_item() {
    let plugin_id = Uuid::new_v4();
    let mut executed = false;
    
    let mut action = ActionItem::new(
        "Test Action",
        "A test action",
        plugin_id,
        move |_| {
            executed = true;
            Ok(())
        },
    );
    
    assert_eq!(action.get_name(), "Test Action");
    assert_eq!(action.get_description(), "A test action");
    assert_eq!(action.get_plugin_id(), plugin_id);
    
    action.execute(None).unwrap();
    assert!(executed);
}

#[test]
fn test_action_group() {
    let plugin_id = Uuid::new_v4();
    let mut group = ActionGroup::new("Test Group", "A test group", plugin_id);
    
    let mut count = 0;
    let action1 = ActionItem::new("Action 1", "First action", plugin_id, move |_| {
        count += 1;
        Ok(())
    });
    
    let mut count2 = 0;
    let action2 = ActionItem::new("Action 2", "Second action", plugin_id, move |_| {
        count2 += 1;
        Ok(())
    });
    
    group.add_action(Box::new(action1));
    group.add_action(Box::new(action2));
    
    assert_eq!(group.get_actions().len(), 2);
    group.execute(None).unwrap();
    
    assert_eq!(count, 1);
    assert_eq!(count2, 1);
}

#[test]
fn test_conditional_execution() {
    let plugin_id = Uuid::new_v4();
    let mut executed = false;
    
    let mut action = ActionItem::new(
        "Conditional Action",
        "Only executes for specific events",
        plugin_id,
        move |_| {
            executed = true;
            Ok(())
        },
    ).with_can_execute(|event| {
        event.map_or(false, |e| e.get_id() == "test_event")
    });
    
    let wrong_event = TestEvent { id: "wrong".to_string() };
    let right_event = TestEvent { id: "test_event".to_string() };
    
    assert!(!action.can_execute(Some(&wrong_event)));
    assert!(action.can_execute(Some(&right_event)));
    
    action.execute(Some(&right_event)).unwrap();
    assert!(executed);
} 