mod singly_linked_list;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::singly_linked_list::{*};

    #[test]
    fn test() {
        let mut list: LinkedList<i32> = LinkedList::new();
        test_remove_first(&mut list);
        test_remove_first_value(&mut list);
        test_remove_last_value(&mut list);
        println!("test");
    }
    #[warn(dead_code)]
    fn test_remove_first(list: &mut LinkedList<i32>) {
        assert_eq!(list.del_first(), None);
        list.add_last(10);
        list.add_last(10);
        assert_eq!(list.del_first(), Some(10));
        assert_eq!(list.del_first(), Some(10));
        assert_eq!(list.del_first(), None);
        assert_eq!(list.del_last(), None);
        assert_eq!(list.to_array(), vec![]);
    }

    fn test_remove_first_value(list: &mut LinkedList<i32>) {
        assert_eq!(list.del_first_value(10), None);
        list.add_last(10);
        assert_eq!(list.del_first_value(10), Some(10));
        list.add_first(20);
        assert_eq!(list.del_first_value(20), Some(20));
        assert_eq!(list.del_first_value(20), None);
        assert_eq!(list.del_first_value(10), None);
        assert_eq!(list.to_array(), vec![]);
    }

    fn test_remove_last_value(list: &mut LinkedList<i32>){
        // assert_eq!(list.del_last_value(10), None);
        list.add_last(20);
        list.add_last(10);
        list.add_last(20);
        let option = list.del_last_value(20);
        assert_eq!(option, Some(20));
        assert_eq!(list.to_array(), vec![20,10]);
        assert_eq!(list.del_last_value(10), Some(10));
        assert_eq!(list.del_last_value(20), Some(20));
        assert_eq!(list.del_last_value(20), None);
        assert_eq!(list.del_last_value(10), None);
        assert_eq!(list.to_array(), vec![]);
    }
}